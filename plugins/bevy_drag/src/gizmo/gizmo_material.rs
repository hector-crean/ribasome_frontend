use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    },
};

pub const GIZMO_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 12246244073709451715);

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "f80c31f9-5111-483a-bf34-a9dc06c2492e"]
pub struct GizmoMaterial {
    #[uniform(0)]
    pub color: Color,
}
impl From<Color> for GizmoMaterial {
    fn from(color: Color) -> Self {
        GizmoMaterial { color }
    }
}

impl Material for GizmoMaterial {
    fn fragment_shader() -> ShaderRef {
        ShaderRef::Handle(GIZMO_SHADER_HANDLE.typed())
    }

    fn vertex_shader() -> ShaderRef {
        ShaderRef::Handle(GIZMO_SHADER_HANDLE.typed())
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Opaque
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
}
