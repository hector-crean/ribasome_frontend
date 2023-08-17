pub mod point;
pub mod polyline;

use std::{fmt::Debug, hash::Hash};

use bevy::prelude::*;
use point::{PointTool, PointToolCommand};
use polyline::PolylineTool;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AddToolState {
    #[default]
    Point,
    Polyline,
}

pub struct AddToolPlugin<S: Debug + Clone + Copy + Default + Eq + PartialEq + Hash + States> {
    pub run_state: S,
    pub on_exit_state: S,
}

impl<S> AddToolPlugin<S> where S: Debug + Clone + Copy + Default + Eq + PartialEq + Hash + States {}

impl<S> Plugin for AddToolPlugin<S>
where
    S: Debug + Clone + Copy + Default + Eq + PartialEq + Hash + States,
{
    fn build(&self, app: &mut App) {
        app.add_state::<AddToolState>()
            .insert_resource(PointTool::default())
            .insert_resource(PolylineTool::default())
            .add_event::<PointToolCommand>()
            .add_systems(
                Update,
                (
                    (
                        PointTool::emit_point_tool_event,
                        PointTool::consume_point_tool_event,
                    )
                        .chain()
                        .run_if(in_state(AddToolState::Point)),
                    (
                        PolylineTool::emit_polyline_tool_event,
                        PolylineTool::consume_polyline_tool_event,
                    )
                        .chain()
                        .run_if(in_state(AddToolState::Polyline)),
                )
                    .run_if(in_state(self.run_state)),
            );
        // .add_systems(OnExit(ToolState::Move), cleanup::<PenToolSystem>);
    }
}
