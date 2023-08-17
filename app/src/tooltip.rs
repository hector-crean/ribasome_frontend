use bevy::{prelude::*, ui::ContentSize};
use ribasome_state::marker::MainCamera;

#[derive(Component)]
pub struct ProjectedTextFrom3Dto2D;

fn update_text_position(
    mesh_query: Query<&Transform, With<Handle<Mesh>>>,
    mut text_query: Query<(&mut Style, &ContentSize), With<ProjectedTextFrom3Dto2D>>,
    camera_query: Query<(&Camera, &GlobalTransform), (With<MainCamera>, With<Camera3d>)>,
) {
    for (camera, camera_transform) in camera_query.iter() {
        for mesh_position in mesh_query.iter() {
            for (mut style, calculated) in text_query.iter_mut() {
                match camera.world_to_viewport(&camera_transform, mesh_position.translation) {
                    Some(coords) => {
                        style.left = Val::Px(coords.x);
                        style.bottom = Val::Px(coords.y);
                    }
                    None => {
                        // A hack to hide the text when the cube is behind the camera
                        style.bottom = Val::Px(-1000.0);
                    }
                }
            }
        }
    }
}
