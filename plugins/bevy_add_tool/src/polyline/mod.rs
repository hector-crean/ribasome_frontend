use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct PolylineTool {
    open: bool,
    interpolation: Option<fn(&Vec3, &Vec3, f32) -> Vec3>,
}

impl PolylineTool {
    pub fn emit_polyline_tool_event(mut commands: Commands) {}
    pub fn consume_polyline_tool_event(mut commands: Commands) {}
}
