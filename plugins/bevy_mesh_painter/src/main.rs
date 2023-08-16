use std::borrow::BorrowMut;

use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
};
use bevy_mod_picking::*;
use painter_shader_lab::{paint_mask::PaintMaskMaterial, paintable::PaintableMaterial};

#[derive(Component)]
struct Painter;

#[derive(Component)]
struct PrimaryCamera;

#[derive(Bundle)]
struct PaintableMesh {
    #[bundle]
    spatial_bundle: SpatialBundle,
    mesh: Handle<Mesh>,
    paint_mask_material: Handle<PaintMaskMaterial>,
    paintable_material: Handle<PaintableMaterial>,
}

impl PaintableMesh {
    pub fn new(
        transform: Transform,
        mesh: Handle<Mesh>,
        base_color_texture: Handle<Image>,
        images: &mut Assets<Image>,
        paintable_materials: &mut Assets<PaintableMaterial>,
        paint_mask_materials: &mut Assets<PaintMaskMaterial>,
    ) -> Self {
        let paint_mask_handle = new_paint_image_handle(images.borrow_mut());
        PaintableMesh {
            spatial_bundle: SpatialBundle::from_transform(transform),
            mesh,
            paint_mask_material: paint_mask_materials.add(PaintMaskMaterial {
                paint_mask_texture: Some(paint_mask_handle.clone()),
                cull_mode: None,
                ..default()
            }),
            paintable_material: paintable_materials.add(PaintableMaterial {
                base_color: Color::WHITE,
                base_color_texture: Some(base_color_texture),
                paint_mask_texture: Some(paint_mask_handle),
                double_sided: true,
                cull_mode: None,
                ..default()
            }),
        }
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("071f3c").unwrap()))
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(MaterialPlugin::<PaintMaskMaterial>::default())
        .add_plugin(MaterialPlugin::<PaintableMaterial>::default())
        .add_startup_system(setup)
        .add_startup_system(setup_lighting)
        .add_system(draw)
        .run();
}

fn new_paint_image_handle(images: &mut Assets<Image>) -> Handle<Image> {
    let mut image = Image::new_fill(
        Extent3d {
            width: 1028,
            height: 1028,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Rgba8Unorm,
    );
    image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;

    images.add(image)
}

fn setup_lighting(
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::ORANGE_RED,
        brightness: 0.02,
    });

    // blue point light
    commands
        .spawn_bundle(PointLightBundle {
            transform: Transform::from_xyz(0.0, 4.0, 0.0),
            point_light: PointLight {
                intensity: 1600.0,
                color: Color::BLUE,
                shadows_enabled: true,
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere {
                    radius: 0.1,
                    ..default()
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::BLUE,
                    emissive: Color::rgba_linear(0.0, 0.0, 100.0, 0.0),
                    ..default()
                }),
                ..default()
            });
        });
    // directional 'sun' light
    const HALF_SIZE: f32 = 10.0;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            // Configure the projection to better fit the scene
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut paintable_materials: ResMut<Assets<PaintableMaterial>>,
    mut paint_mask_materials: ResMut<Assets<PaintMaskMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
) {
    // camera
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(-3.0, 3.5, 7.0).looking_at(Vec3::Y * 1.5, Vec3::Y),
            ..default()
        })
        .insert(PrimaryCamera)
        .insert_bundle(PickingCameraBundle::default());

    // ball
    commands
        .spawn_bundle(PaintableMesh::new(
            Transform::from_xyz(0.0, 2.5, 0.0),
            meshes.add(Mesh::from(shape::UVSphere {
                radius: 1.0,
                ..default()
            })),
            asset_server.load("concrete/uv.png"),
            &mut images,
            &mut paintable_materials,
            &mut paint_mask_materials,
        ))
        .insert_bundle(PickableBundle::default());

    // ground_plane
    commands
        .spawn_bundle(PaintableMesh::new(
            Transform::from_xyz(0.0, 0.0, 0.0),
            meshes.add(Mesh::from(shape::Plane {
                size: 10.0,
                ..default()
            })),
            asset_server.load("concrete/sekjcawb_2K_Albedo.jpg"),
            &mut images,
            &mut paintable_materials,
            &mut paint_mask_materials,
        ))
        .insert_bundle(PickableBundle::default());

    // left wall
    let mut transform = Transform::from_xyz(2.5, 2.5, 0.0);
    transform.rotate_z(std::f32::consts::FRAC_PI_2);
    commands
        .spawn_bundle(PaintableMesh::new(
            transform,
            meshes.add(Mesh::from(shape::Box::new(5.0, 0.15, 5.0))),
            asset_server.load("concrete/sekjcawb_2K_Albedo.jpg"),
            &mut images,
            &mut paintable_materials,
            &mut paint_mask_materials,
        ))
        .insert_bundle(PickableBundle::default());

    // right wall
    let mut transform = Transform::from_xyz(0.0, 2.5, -2.5);
    transform.rotate_x(std::f32::consts::FRAC_PI_2);
    commands
        .spawn_bundle(PaintableMesh::new(
            transform,
            meshes.add(Mesh::from(shape::Box::new(5.0, 0.15, 5.0))),
            asset_server.load("concrete/sekjcawb_2K_Albedo.jpg"),
            &mut images,
            &mut paintable_materials,
            &mut paint_mask_materials,
        ))
        .insert_bundle(PickableBundle::default());
    // painter
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.1,
                ..default()
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::BLUE,
                ..default()
            }),
            ..default()
        })
        .insert(Painter)
        .insert_bundle(SpatialBundle::default());
}

fn draw(
    primary_camera_query: Query<&PickingCamera, With<PrimaryCamera>>,
    mut materials: ResMut<Assets<PaintMaskMaterial>>,
    mut painter_query: Query<&mut Transform, (With<Painter>, Without<PrimaryCamera>)>,
    input: Res<Input<MouseButton>>,
) {
    let mut painter = painter_query.single_mut();
    let picking = primary_camera_query.single();
    if let Some((_entity, hit_data)) = picking.intersect_top() {
        let pos = hit_data.position();

        painter.translation = pos;
        for material in materials.iter_mut() {
            material.1.pos = painter.translation;
            if input.pressed(MouseButton::Left) {
                material.1.brush_painting = true;
            } else {
                material.1.brush_painting = false;
            }
        }
    }
}
