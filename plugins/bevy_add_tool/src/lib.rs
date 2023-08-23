pub mod point;
pub mod polyline;

use std::{fmt::Debug, hash::Hash};

use bevy::prelude::*;
use point::{PointTool, PointToolCommand};
use polyline::PolylineTool;
use bevy_ui_widgets::widget::tooltip::TooltipPlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AddToolState {
    #[default]
    Point,
    Polyline,
}

pub struct AddToolPlugin<S: States + Copy + Clone, const I: usize> {
    pub run_states: [S; I],
    pub on_exit_state: S,
}

impl<S, const I: usize> AddToolPlugin<S, I> where S: States + Debug + Copy {}

impl<S, const I: usize> Plugin for AddToolPlugin<S, I>
where
    S: States + Debug + Copy,
{
    fn build(&self, app: &mut App) {
        app     
        .add_plugins(TooltipPlugin)
        .add_state::<AddToolState>()
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
                    .run_if(in_any_state_from(self.run_states)),
            );
        // .add_systems(OnExit(ToolState::Move), cleanup::<PenToolSystem>);
    }
}

pub fn in_any_state_from<S: States, const I: usize>(
    states: [S; I],
) -> impl FnMut(Res<State<S>>) -> bool + Clone {
    move |current_state: Res<State<S>>| states.iter().any(|s| *current_state == *s)
}
