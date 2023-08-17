use bevy::{a11y::accesskit::Point, prelude::*};
use bevy_mod_picking::{
    backend::HitData,
    events::{Click, Down, Pointer},
};

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
    pub fn consume_point_tool_event(mut point_tool_evts: EventReader<PointToolCommand>) {
        for ev in point_tool_evts.iter() {
            match ev {
                PointToolCommand::Create { position } => info!("Create Point at: {}", &position),
            }
        }
    }
}

#[derive(Event)]
pub enum PointToolCommand {
    Create { position: Vec3 },
}
