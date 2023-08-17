use std::{fmt::Debug, hash::Hash};

use bevy::prelude::*;

#[derive(Resource, Default)]
pub enum Add {
    #[default]
    Point,
    Polyline {
        open: bool,
        interpolation: Option<fn(&Vec3, &Vec3, f32) -> Vec3>,
    },
    Volume,
}

pub struct AddToolPlugin<S: Debug + Clone + Copy + Default + Eq + PartialEq + Hash + States> {
    run_state: S,
    on_exit_state: S,
}

impl<S> AddToolPlugin<S>
where
    S: Debug + Clone + Copy + Default + Eq + PartialEq + Hash + States,
{
    fn setup_add_tool(commands: Commands) {}
    fn pen_tool(commands: Commands) {}
}

impl<S> Plugin for AddToolPlugin<S>
where
    S: Debug + Clone + Copy + Default + Eq + PartialEq + Hash + States,
{
    fn build(&self, app: &mut App) {
        app.insert_resource(Add::default())
            .add_systems(OnEnter(self.run_state), (Self::setup_add_tool))
            .add_systems(Update, Self::pen_tool.run_if(in_state(self.run_state)));
        // .add_systems(OnExit(ToolState::Move), cleanup::<PenToolSystem>);
    }
}
