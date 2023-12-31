use bevy::{math::vec4, prelude::*};
use bevy_mod_picking::prelude::*;
use std::{ffi::OsStr, fs, io, path::PathBuf};

use bevy::{
    ecs::system::Command,
    gltf::{Gltf, GltfMesh, GltfNode},
    prelude::*,
};
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn copy_file<P: AsRef<Path>, Q: AsRef<Path>>(
    source_path: P,
    destination_path: Q,
) -> Result<Q, io::Error> {
    fs::copy(&source_path, &destination_path)?;

    Ok(destination_path)
}

enum FileType {
    Gltf,
    Other,
}

fn get_file_type(extension: Option<&OsStr>) -> FileType {
    match extension.and_then(|ext| ext.to_str()) {
        Some("gltf") | Some("glb") => FileType::Gltf,
        _ => FileType::Other,
    }
}

pub struct FileDragAndDropPlugin;

impl Plugin for FileDragAndDropPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (file_drag_and_drop, consume_gltf_loading))
            .add_systems(PostUpdate, (make_pickable));
    }
}

pub fn file_drag_and_drop(
    mut events: EventReader<FileDragAndDrop>,
    asset_server: Res<AssetServer>,
) {
    for ev in events.iter() {
        match ev {
            FileDragAndDrop::DroppedFile { window, path_buf } => {
                match path_buf.extension().and_then(|ext| ext.to_str()) {
                    Some(extension) => match extension {
                        "gltf" | "glb" => {
                            // let path = copy_file(
                            //     path_buf,
                            //     format!("glb/{}.{}", path_buf.file_name()., extension),
                            // );
                            let handle =
                                asset_server.load::<bevy::gltf::Gltf, PathBuf>(path_buf.into());
                        }
                        _ => {}
                    },
                    None => {}
                }
            }

            FileDragAndDrop::HoveredFile { window, path_buf } => {}
            FileDragAndDrop::HoveredFileCanceled { window } => {}
        }
    }
}

/// Used to tint the mesh instead of simply replacing the mesh's material with a single color. See
/// `tinted_highlight` for more details.
const HIGHLIGHT_TINT: Highlight<StandardMaterial> = Highlight {
    hovered: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl.base_color + vec4(-0.5, -0.3, 0.9, 0.8), // hovered is blue
        ..matl.to_owned()
    })),
    pressed: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl.base_color + vec4(-0.4, -0.4, 0.8, 0.8), // pressed is a different blue
        ..matl.to_owned()
    })),
    selected: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl.base_color + vec4(-0.4, 0.8, -0.4, 0.0), // selected is green
        ..matl.to_owned()
    })),
};

fn consume_gltf_loading(
    mut ev_asset: EventReader<AssetEvent<Gltf>>,
    gltfs: ResMut<Assets<Gltf>>,
    meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    for ev in ev_asset.iter() {
        info!("{:?}", &ev);
        match ev {
            AssetEvent::Created { handle } => {
                let gltf = gltfs.get(handle);

                match gltf {
                    Some(gltf) => {
                        for scene in &gltf.scenes {
                            commands.spawn((SceneBundle {
                                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                                scene: scene.clone(),
                                ..default()
                            },));
                        }
                    }
                    None => {}
                }
            }
            AssetEvent::Modified { handle } => {
                // an image was modified
            }
            AssetEvent::Removed { handle } => {
                // an image was unloaded
            }
        }
    }
}

fn make_pickable(
    mut commands: Commands,
    meshes: Query<Entity, (With<Handle<Mesh>>, Without<RaycastPickTarget>)>,
) {
    for entity in meshes.iter() {
        commands.entity(entity).insert((
            PickableBundle::default(),
            RaycastPickTarget::default(),
            HIGHLIGHT_TINT.clone(),
        ));
    }
}
