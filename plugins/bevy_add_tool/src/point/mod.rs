use bevy::{a11y::accesskit::Point, prelude::*};
use bevy_mod_picking::{
    backend::HitData,
    events::{Click, Down, Pointer},
};
use bevy_mod_reqwest::*;

#[derive(Resource, Default)]
pub struct PointTool;

impl PointTool {
    pub fn emit_point_tool_event(
        mut pointerdowns: EventReader<Pointer<Down>>,
        mut point_tool_evt_wtr: EventWriter<PointToolCommand>,
    ) {
        for pointerdown in pointerdowns.iter() {
            let HitData { position, .. } = pointerdown.event.hit;

            match position {
                Some(position) => point_tool_evt_wtr.send(PointToolCommand::Create { position }),
                None => {}
            }
        }
    }
    pub fn consume_point_tool_event(
        mut point_tool_evts: EventReader<PointToolCommand>,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        for ev in point_tool_evts.iter() {
            match ev {
                PointToolCommand::Create { position } => {
                    commands.spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.05 })),
                        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                        transform: Transform::from_translation(*position),
                        ..default()
                    });

                    if let Ok(url) = "https://www.boredapi.com/api/activity".try_into() {
                        let req = reqwest::Request::new(reqwest::Method::GET, url);
                        let req = ReqwestRequest::new(req);
                        commands.spawn(req);
                    }
                }
            }
        }
    }
}

#[derive(Event)]
pub enum PointToolCommand {
    Create { position: Vec3 },
}
