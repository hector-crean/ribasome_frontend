use bevy::prelude::*;
use bevy_ui_widgets::widget::tooltip::*;

use bevy_cameras::{
    mode::CameraModeImpl,
    pan_orbit_camera::{OrbitCameraController, OrbitCameraControllerPlugin},
};

/// A simple tooltip that updates every frame.
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            OrbitCameraControllerPlugin::<CameraModeImpl>::default(),
        ))
        .add_plugins(TooltipPlugin)
        .add_systems(Startup, (setup_camera, setup_scene))
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let mesh_entity = commands
        // When any of this entity's children are interacted with using a pointer, those events will
        // propagate up the entity hierarchy until they reach this parent. By referring to the
        // `target` entity instead of the `listener` entity, we can do things to specific target
        // entities, even though they lack `OnPointer` components.
        .spawn((PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::WHITE.into()),
            ..Default::default()
        },))
        .id();

    let tooltip_content = commands
        .spawn((
            TextBundle {
                text: Text::from_section(
                    "This is example text",
                    TextStyle {
                        font: asset_server.load("fonts\\iosevka-extendedheavyoblique.ttf"),
                        font_size: 10.,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ),
                style: Style {
                    width: Val::Px(120.),
                    height: Val::Px(120.),
                    border: UiRect::all(Val::Px(2.)),
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,

                    ..default()
                },
                background_color: Color::rgb(108. / 180., 130. / 180., 142. / 180.).into(),

                ..default()
            },
            Label,
        ))
        .id();

    commands
        .spawn(
            (TooltipBundle {
                anchor: TooltipAnchor::AlignedToMesh(mesh_entity),
                ..default()
            }),
        )
        .add_child(tooltip_content);

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 0., -2.).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        OrbitCameraController::default(),
    ));
}
