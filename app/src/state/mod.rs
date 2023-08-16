pub mod camera;
use bevy::{
    ecs::entity::Entity,
    prelude::*,
    prelude::{NextState, ResMut, Resource, States},
};

#[derive(Default, Resource)]
pub enum InteractionMode {
    // Select an entity in the scene
    Select {
        selected: Option<Entity>,
        mode: SelectModes,
    },
    // Add new entity to scene
    Add(AddCommand),
    // Move camera around :
    #[default]
    Survey,
}

#[derive(Default)]
pub enum SelectModes {
    #[default]
    Edit,
    Move,
    Delete,
}

pub enum AddCommand {
    AddPoint,
    AddLine,
    AddPolyline { open: bool },
    AddPolygon(Polygon),
    AddProtractor,
    AddRuler,
    AddVolume,
}

pub enum Polygon {
    Triange,
    Rectangle,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum ToolState {
    #[default]
    Move,
    Pen,
    Comment,
}

fn tool_mode(mut next_state: ResMut<NextState<ToolState>>) {
    next_state.set(ToolState::Move)
}

pub struct PenToolPlugin;

impl PenToolPlugin {
    fn setup_pen_tool(commands: Commands) {}
    fn pen_tool(commands: Commands) {}
}

impl Plugin for PenToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ToolState::Pen), (Self::setup_pen_tool))
            .add_systems(Update, Self::pen_tool.run_if(in_state(ToolState::Pen)));
        // .add_systems(OnExit(ToolState::Move), cleanup::<PenToolSystem>);
    }
}
