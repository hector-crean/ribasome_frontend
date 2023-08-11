pub mod coordinate_system;
pub mod errors;
pub mod state;

use bevy::{prelude::*, window::PresentMode};
use bevy_asset_loader::prelude::*;



use ribasome_models::marker_3d::Marker3d;


const BILLBOARD_TEXT_SCALE: Vec3 = Vec3::splat(0.0085);

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/iosevka-regular.ttf")]
    pub iosevka_regular: Handle<Font>,
    #[asset(path = "fonts/iosevka-heavy.ttf")]
    pub iosevka_heavy: Handle<Font>,
    #[asset(path = "fonts/iosevka-heavyitalic.ttf")]
    pub iosevka_heavyitalic: Handle<Font>,
    #[asset(path = "fonts/iosevka-extraboldoblique.ttf")]
    pub iosevka_extraboldoblique: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct GlbAssets {
    #[asset(path = "glb/mind_flayer__illithid.glb#Scene0")]
    pub mind_flayer_illithid: Handle<Scene>,
}

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

#[derive(Resource, Debug, Component)]
struct Labels(Vec<Marker3d>);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Loading,
    Menu,
    Canvas3d,
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
enum Tool {
    Pen,
    Labeller,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: true,
                fit_canvas_to_parent: true,
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }),))
        .add_state::<AppState>()
        .insert_resource(Tool::Labeller)
        .add_loading_state(LoadingState::new(AppState::Loading).continue_to_state(AppState::Menu))
        .add_collection_to_loading_state::<AppState, FontAssets>(AppState::Loading)
        .add_collection_to_loading_state::<AppState, GlbAssets>(AppState::Loading)
        .add_systems(Startup, setup)
        // Adds the plugins for each state
        .add_plugins((menu::MenuPlugin, canvas3d::Canvas3dPlugin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

mod canvas3d {
    use bevy::{prelude::*};
    use bevy_cameras::pan_orbit_camera::{OrbitCameraController, OrbitCameraControllerPlugin};
    use bevy_drag::{RaycastPickCamera, Transformable, TransformablePlugin};
    use bevy_eventlistener::prelude::{Listener, On};
    use bevy_mod_billboard::{
        prelude::BillboardPlugin, BillboardTextBundle,
    };
    use bevy_mod_picking::{
        prelude::{Down, Pointer, PointerButton, RaycastPickTarget},
    };

    use crate::{state::camera::CameraModeImpl, FontAssets, GlbAssets};

    use super::{despawn_screen, AppState, BILLBOARD_TEXT_SCALE};

    #[derive(Component)]
    pub struct OnCanvas3dScreen;

    pub struct Canvas3dPlugin;

    impl Plugin for Canvas3dPlugin {
        fn build(&self, app: &mut App) {
            app.add_plugins((
                TransformablePlugin::<CameraModeImpl>::default(),
                OrbitCameraControllerPlugin::<CameraModeImpl>::default(),
                BillboardPlugin,
            ))
            .insert_resource(CameraModeImpl::default())
            .add_systems(
                OnEnter(AppState::Canvas3d),
                (
                    canvas_3d_setup,
                    TransformablePlugin::<CameraModeImpl>::setup_raycast_camera,
                    setup_labels,
                    convert_glb_to_transformable,
                )
                    .chain(),
            )
            .add_systems(Update, canvas_3d.run_if(in_state(AppState::Canvas3d)))
            .add_systems(
                OnExit(AppState::Canvas3d),
                despawn_screen::<OnCanvas3dScreen>,
            );
        }
    }

    fn convert_glb_to_transformable(
        mut commands: Commands,
        meshes_query: Query<Entity, (With<Handle<Mesh>>, Without<RaycastPickTarget>)>,
    ) {
        for entity in meshes_query.iter() {
            commands.entity(entity).insert(Transformable::default());
        }
    }

    fn canvas_3d_setup(
        _app_state: ResMut<NextState<AppState>>,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        camera_2d_query: Query<Entity, (With<Camera2d>, With<Camera>)>,
        glbs: Res<GlbAssets>,
        _fonts: Res<FontAssets>,
    ) {
        for camera_entity in &camera_2d_query {
            commands.entity(camera_entity).despawn_recursive();
        }

        commands
            .spawn((
                Camera3dBundle {
                    transform: Transform::from_xyz(0., 0., -2.).looking_at(Vec3::ZERO, Vec3::Y),
                    // projection: Projection::Orthographic(OrthographicProjection::default()),
                    ..default()
                },
                OrbitCameraController::default(),
                OnCanvas3dScreen,
            ))
            .insert(RaycastPickCamera::default());

        commands
            // When any of this entity's children are interacted with using a pointer, those events will
            // propagate up the entity hierarchy until they reach this parent. By referring to the
            // `target` entity instead of the `listener` entity, we can do things to specific target
            // entities, even though they lack `OnPointer` components.
            .spawn((
                OnCanvas3dScreen,
                Transformable::default(),
                SceneBundle {
                    scene: glbs.mind_flayer_illithid.clone(),
                    transform: Transform::from_xyz(8., 2.0, -5.0),
                    ..Default::default()
                },
            ));

        commands
            // When any of this entity's children are interacted with using a pointer, those events will
            // propagate up the entity hierarchy until they reach this parent. By referring to the
            // `target` entity instead of the `listener` entity, we can do things to specific target
            // entities, even though they lack `OnPointer` components.
            .spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.4 })),
                    material: materials.add(Color::RED.into()),
                    transform: Transform::from_xyz(0.0, 1.0, 0.0),
                    ..Default::default()
                },
                Transformable::default(),
                OnCanvas3dScreen,
            ))
            .with_children(|parent| {
                for i in 1..=5 {
                    parent.spawn((
                        // As noted above, we are adding children here but we don't need to add an event
                        // listener. Events on children will bubble up to the parent!
                        PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.4 })),
                            material: materials.add(Color::RED.into()),
                            transform: Transform::from_xyz(0.0, 1.0 + 0.5 * i as f32, 0.0),
                            ..Default::default()
                        },
                        Transformable::default(),
                        On::<Pointer<Down>>::run(
                            |pointerdown: Listener<Pointer<Down>>,
                             mut commands: Commands,
                             _meshes: ResMut<Assets<Mesh>>,
                             _materials: ResMut<Assets<StandardMaterial>>,
                             keys: Res<Input<KeyCode>>,
                             fonts: Res<FontAssets>| {
                                let Down { button, hit } = pointerdown.event;

                                match hit.position {
                                    Some(position) => match button {
                                        PointerButton::Primary => {
                                            if keys.pressed(KeyCode::Space) {
                                                let _label = commands
                                                    .spawn(BillboardTextBundle {
                                                        transform: Transform::from_translation(
                                                            position,
                                                        )
                                                        .with_scale(BILLBOARD_TEXT_SCALE),

                                                        text: Text::from_sections([
                                                            TextSection {
                                                                value: "IMPORTANT".to_string(),
                                                                style: TextStyle {
                                                                    font_size: 60.0,
                                                                    font: fonts
                                                                        .iosevka_regular
                                                                        .clone(),
                                                                    color: Color::ORANGE,
                                                                },
                                                            },
                                                            TextSection {
                                                                value: " text".to_string(),
                                                                style: TextStyle {
                                                                    font_size: 60.0,
                                                                    font: fonts
                                                                        .iosevka_regular
                                                                        .clone(),
                                                                    color: Color::WHITE,
                                                                },
                                                            },
                                                        ])
                                                        .with_alignment(TextAlignment::Center),
                                                        ..default()
                                                    })
                                                    .id();

                                                // commands
                                                //     .entity(pointerdown.target)
                                                //     .add_child(label);
                                            }
                                        }
                                        _ => {}
                                    },
                                    None => {}
                                }
                            },
                        ),
                    ));
                }
            });

        commands.spawn((
            OnCanvas3dScreen,
            PointLightBundle {
                point_light: PointLight {
                    intensity: 1500.0,
                    shadows_enabled: true,
                    ..Default::default()
                },
                transform: Transform::from_xyz(4.0, 8.0, 4.0),
                ..Default::default()
            },
        ));
    }

    fn setup_labels(mut commands: Commands, fonts: Res<FontAssets>) {
        commands.spawn(BillboardTextBundle {
            transform: Transform::from_scale(Vec3::splat(0.0085)),
            text: Text::from_sections([
                TextSection {
                    value: "IMPORTANT".to_string(),
                    style: TextStyle {
                        font_size: 60.0,
                        font: fonts.iosevka_regular.clone(),
                        color: Color::ORANGE,
                    },
                },
                TextSection {
                    value: " text".to_string(),
                    style: TextStyle {
                        font_size: 60.0,
                        font: fonts.iosevka_regular.clone(),
                        color: Color::WHITE,
                    },
                },
            ])
            .with_alignment(TextAlignment::Center),
            ..default()
        });
    }

    fn canvas_3d(_commands: Commands) {}
}

mod menu {
    use bevy::{app::AppExit, prelude::*};

    use crate::FontAssets;

    use super::{despawn_screen, AppState, Tool, TEXT_COLOR};

    pub struct MenuPlugin;

    impl Plugin for MenuPlugin {
        fn build(&self, app: &mut App) {
            app
                // At start, the menu is not enabled. This will be changed in `menu_setup` when
                // entering the `AppState::Menu` state.
                // Current screen in the menu is handled by an independent state from `AppState`
                .add_state::<MenuState>()
                .add_systems(OnEnter(AppState::Menu), menu_setup)
                // Systems to handle the main menu screen
                .add_systems(OnEnter(MenuState::Main), main_menu_setup)
                .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
                // Systems to handle the settings menu screen
                .add_systems(OnEnter(MenuState::Settings), settings_menu_setup)
                .add_systems(
                    OnExit(MenuState::Settings),
                    despawn_screen::<OnSettingsMenuScreen>,
                )
                // Systems to handle the display settings screen
                .add_systems(
                    OnEnter(MenuState::SettingsDisplay),
                    display_settings_menu_setup,
                )
                .add_systems(
                    Update,
                    (setting_button::<Tool>.run_if(in_state(MenuState::SettingsDisplay)),),
                )
                .add_systems(
                    OnExit(MenuState::SettingsDisplay),
                    despawn_screen::<OnToolSettingsMenuScreen>,
                )
                // Common systems to all screens that handles buttons behavior
                .add_systems(
                    Update,
                    (menu_action, button_system).run_if(in_state(AppState::Menu)),
                );
        }
    }

    #[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
    enum MenuState {
        Main,
        Settings,
        #[default]
        Disabled,
        SettingsDisplay,
    }

    // Tag component used to tag entities added on the main menu screen
    #[derive(Component)]
    struct OnMainMenuScreen;

    // Tag component used to tag entities added on the settings menu screen
    #[derive(Component)]
    struct OnSettingsMenuScreen;

    #[derive(Component)]
    struct OnToolSettingsMenuScreen;

    const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
    const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
    const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
    const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

    // Tag component used to mark which setting is currently selected
    #[derive(Component)]
    struct SelectedOption;

    // All actions that can be triggered from a button click
    #[derive(Component)]
    enum MenuButtonAction {
        Play,
        Settings,
        SettingsDisplay,
        BackToMainMenu,
        BackToSettings,
        Quit,
    }

    // This system handles changing all buttons color based on mouse interaction
    fn button_system(
        mut interaction_query: Query<
            (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
            (Changed<Interaction>, With<Button>),
        >,
    ) {
        for (interaction, mut color, selected) in &mut interaction_query {
            *color = match (*interaction, selected) {
                (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
                (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
                (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
                (Interaction::None, None) => NORMAL_BUTTON.into(),
            }
        }
    }

    // This system updates the settings when a new value for a setting is selected, and marks
    // the button as the one currently selected
    fn setting_button<T: Resource + Component + PartialEq + Copy>(
        interaction_query: Query<(&Interaction, &T, Entity), (Changed<Interaction>, With<Button>)>,
        mut selected_query: Query<(Entity, &mut BackgroundColor), With<SelectedOption>>,
        mut commands: Commands,
        mut setting: ResMut<T>,
    ) {
        for (interaction, button_setting, entity) in &interaction_query {
            if *interaction == Interaction::Pressed && *setting != *button_setting {
                let (previous_button, mut previous_color) = selected_query.single_mut();
                *previous_color = NORMAL_BUTTON.into();
                commands.entity(previous_button).remove::<SelectedOption>();
                commands.entity(entity).insert(SelectedOption);
                *setting = *button_setting;
            }
        }
    }

    fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
        menu_state.set(MenuState::Main);
    }
    fn main_menu_setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        fonts: Res<FontAssets>,
    ) {
        // Common style for all buttons on the screen
        let button_style = Style {
            width: Val::Px(250.0),
            height: Val::Px(65.0),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };
        let button_icon_style = Style {
            width: Val::Px(30.0),
            // This takes the icons out of the flexbox flow, to be positioned exactly
            position_type: PositionType::Absolute,
            // The icon will be close to the left border of the button
            left: Val::Px(10.0),
            right: Val::Auto,
            ..default()
        };
        let button_text_style = TextStyle {
            font_size: 40.0,
            color: TEXT_COLOR,
            font: fonts.iosevka_regular.clone(),
            ..default()
        };

        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                },
                OnMainMenuScreen,
            ))
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::CRIMSON.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        // Display the game name
                        parent.spawn(
                            TextBundle::from_section(
                                "Ribasome",
                                TextStyle {
                                    font: fonts.iosevka_regular.clone(),
                                    font_size: 80.0,
                                    color: TEXT_COLOR,
                                    ..default()
                                },
                            )
                            .with_style(Style {
                                margin: UiRect::all(Val::Px(50.0)),
                                ..default()
                            }),
                        );

                        // Display three buttons for each action available from the main menu:
                        // - new game
                        // - settings
                        // - quit
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                MenuButtonAction::Play,
                            ))
                            .with_children(|parent| {
                                let icon = asset_server.load("textures/Game Icons/right.png");
                                parent.spawn(ImageBundle {
                                    style: button_icon_style.clone(),
                                    image: UiImage::new(icon),
                                    ..default()
                                });
                                parent.spawn(TextBundle::from_section(
                                    "New",
                                    button_text_style.clone(),
                                ));
                            });
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                MenuButtonAction::Settings,
                            ))
                            .with_children(|parent| {
                                let icon = asset_server.load("textures/Game Icons/wrench.png");
                                parent.spawn(ImageBundle {
                                    style: button_icon_style.clone(),
                                    image: UiImage::new(icon),
                                    ..default()
                                });
                                parent.spawn(TextBundle::from_section(
                                    "Settings",
                                    button_text_style.clone(),
                                ));
                            });
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style,
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                MenuButtonAction::Quit,
                            ))
                            .with_children(|parent| {
                                let icon = asset_server.load("textures/Game Icons/exitRight.png");
                                parent.spawn(ImageBundle {
                                    style: button_icon_style,
                                    image: UiImage::new(icon),
                                    ..default()
                                });
                                parent.spawn(TextBundle::from_section("Quit", button_text_style));
                            });
                    });
            });
    }

    fn settings_menu_setup(mut commands: Commands, fonts: Res<FontAssets>) {
        let button_style = Style {
            width: Val::Px(200.0),
            height: Val::Px(65.0),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };

        let button_text_style = TextStyle {
            font_size: 40.0,
            color: TEXT_COLOR,
            font: fonts.iosevka_regular.clone(),
            ..default()
        };

        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                },
                OnSettingsMenuScreen,
            ))
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::CRIMSON.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        for (action, text) in [
                            (MenuButtonAction::SettingsDisplay, "Display"),
                            (MenuButtonAction::BackToMainMenu, "Back"),
                        ] {
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: button_style.clone(),
                                        background_color: NORMAL_BUTTON.into(),
                                        ..default()
                                    },
                                    action,
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        text,
                                        button_text_style.clone(),
                                    ));
                                });
                        }
                    });
            });
    }

    fn display_settings_menu_setup(mut commands: Commands, _display_quality: Res<Tool>) {
        let button_style = Style {
            width: Val::Px(200.0),
            height: Val::Px(65.0),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };
        let button_text_style = TextStyle {
            font_size: 40.0,
            color: TEXT_COLOR,
            ..default()
        };

        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                },
                OnToolSettingsMenuScreen,
            ))
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::CRIMSON.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        // Create a new `NodeBundle`, this time not setting its `flex_direction`. It will
                        // use the default value, `FlexDirection::Row`, from left to right.
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::CRIMSON.into(),
                                ..default()
                            })
                            .with_children(|parent| {
                                // Display a label for the current setting
                                parent.spawn(TextBundle::from_section(
                                    "Display Quality",
                                    button_text_style.clone(),
                                ));
                                // Display a button for each possible value
                                for tool in [Tool::Pen, Tool::Labeller] {
                                    let mut entity = parent.spawn(ButtonBundle {
                                        style: Style {
                                            width: Val::Px(150.0),
                                            height: Val::Px(65.0),
                                            ..button_style.clone()
                                        },
                                        background_color: NORMAL_BUTTON.into(),
                                        ..default()
                                    });
                                    entity.insert(tool).with_children(|parent| {
                                        parent.spawn(TextBundle::from_section(
                                            format!("{tool:?}"),
                                            button_text_style.clone(),
                                        ));
                                    });
                                    if tool == tool {
                                        entity.insert(SelectedOption);
                                    }
                                }
                            });
                        // Display the back button to return to the settings screen
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style,
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                MenuButtonAction::BackToSettings,
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section("Back", button_text_style));
                            });
                    });
            });
    }

    fn menu_action(
        interaction_query: Query<
            (&Interaction, &MenuButtonAction),
            (Changed<Interaction>, With<Button>),
        >,
        mut app_exit_events: EventWriter<AppExit>,
        mut menu_state: ResMut<NextState<MenuState>>,
        mut game_state: ResMut<NextState<AppState>>,
    ) {
        for (interaction, menu_button_action) in &interaction_query {
            if *interaction == Interaction::Pressed {
                match menu_button_action {
                    MenuButtonAction::Quit => app_exit_events.send(AppExit),
                    MenuButtonAction::Play => {
                        game_state.set(AppState::Canvas3d);
                        menu_state.set(MenuState::Disabled);
                    }
                    MenuButtonAction::Settings => menu_state.set(MenuState::Settings),
                    MenuButtonAction::SettingsDisplay => {
                        menu_state.set(MenuState::Settings);
                    }
                    MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::Main),
                    MenuButtonAction::BackToSettings => {
                        menu_state.set(MenuState::Settings);
                    }
                }
            }
        }
    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
