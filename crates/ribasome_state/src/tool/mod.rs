use bevy::{
    ecs::entity::Entity,
    prelude::*,
    prelude::{NextState, ResMut, Resource, States},
};
// use ribasome_models::linear_algebra::Vec3;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum ToolState {
    //translate/scale/rotate: this is effectively the default mode
    #[default]
    Transform,
    //Modify the structure of an object (vertices, edges, faces), or add labels etc.: requires selecting an object
    Edit,
    // Add a new geometry to the scene
    Add,
    //Paint directly onto vertices of objects: don't need to be selecting a particualr object
    VertexPaint,
    // Paint textures directly onto an object:  don't need to be selecting a particualr object
    TexturePaint,
}
