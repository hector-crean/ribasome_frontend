
use bevy::prelude::*;
use bevy_cameras::pan_orbit_camera::{OrbitCameraController, OrbitCameraControllerPlugin};
use bevy_drag::{RaycastPickCamera, Transformable, TransformablePlugin};
use bevy_mod_billboard::{prelude::BillboardPlugin, BillboardTextBundle};
use bevy_mod_outline::{OutlineBundle, OutlineStencil, OutlineVolume};
use bevy_mod_picking::prelude::{Down, Listener, On, Pointer, PointerButton, RaycastPickTarget};

use crate::{state::camera::CameraModeImpl, FontAssets, GlbAssets};

use super::{cleanup, AppState, BILLBOARD_TEXT_SCALE};

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
        .add_systems(OnExit(AppState::Canvas3d), cleanup::<OnCanvas3dScreen>);
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
    glbs: Res<GlbAssets>,
    _fonts: Res<FontAssets>,
) {
    let outline = OutlineBundle {
        outline: OutlineVolume {
            visible: true,
            colour: Color::WHITE,
            width: 10.0,
        },
        stencil: OutlineStencil {
            offset: 5.0,
            ..default()
        },
        ..default()
    };

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
            OutlineBundle {
                outline: OutlineVolume {
                    visible: true,
                    colour: Color::WHITE,
                    width: 5.0,
                },
                stencil: OutlineStencil {
                    offset: 2.0,
                    ..default()
                },
                ..default()
            },
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
                            let Down { button, hit } = &pointerdown.event;

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
