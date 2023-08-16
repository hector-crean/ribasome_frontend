use bevy::pbr::{AlphaMode, Material, MaterialPipeline, MaterialPipelineKey};
use bevy::render::{
    mesh::MeshVertexBufferLayout, render_asset::RenderAssets, render_resource::*, texture::Image,
};
use bevy::{asset::Handle, render::renderer::RenderDevice};
use bevy::{math::Vec3, render::texture::FallbackImage};
use bevy::{prelude::Vec4, reflect::TypeUuid};

#[derive(Clone, TypeUuid)]
#[uuid = "491b5c3c-bafc-442c-b45c-61eaa041d18c"]
pub struct PaintMaskMaterial {
    pub base_color: Vec4,
    pub pos: Vec3,
    pub brush_painting: bool,
    pub paint_mask_texture: Option<Handle<Image>>,
    pub cull_mode: Option<Face>,
}

impl AsBindGroup for PaintMaskMaterial {
    type Data = PaintMaskMaterialKey;
    fn as_bind_group(
        &self,
        layout: &BindGroupLayout,
        render_device: &RenderDevice,
        images: &RenderAssets<Image>,
        fallback_image: &FallbackImage,
    ) -> Result<PreparedBindGroup<Self>, AsBindGroupError> {
        let bindings = <[_]>::into_vec(Box::new([
            {
                let mut buffer = encase::UniformBuffer::new(Vec::new());
                let converted: PaintMaskMaterialUniform = self.as_bind_group_shader_type(&images);
                buffer.write(&converted).unwrap();
                OwnedBindingResource::Buffer(render_device.create_buffer_with_data(
                    &BufferInitDescriptor {
                        label: None,
                        usage: BufferUsages::COPY_DST | BufferUsages::UNIFORM,
                        contents: buffer.as_ref(),
                    },
                ))
            },
            OwnedBindingResource::TextureView({
                let handle: Option<Handle<Image>> = self.paint_mask_texture.clone();
                if let Some(handle) = handle {
                    images
                        .get(&handle)
                        .ok_or_else(|| AsBindGroupError::RetryNextUpdate)?
                        .texture_view
                        .clone()
                } else {
                    fallback_image.texture_view.clone()
                }
            }),
        ]));
        let bind_group = {
            let descriptor = BindGroupDescriptor {
                entries: &[
                    BindGroupEntry {
                        binding: 0u32,
                        resource: bindings[0usize].get_binding(),
                    },
                    BindGroupEntry {
                        binding: 1u32,
                        resource: bindings[1usize].get_binding(),
                    },
                ],
                label: None,
                layout: &layout,
            };
            render_device.create_bind_group(&descriptor)
        };
        Ok(PreparedBindGroup {
            bindings,
            bind_group,
            data: self.into(),
        })
    }
    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0u32,
                    visibility: ShaderStages::all(),
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: Some(<PaintMaskMaterialUniform as ShaderType>::min_size()),
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1u32,
                    visibility:  ShaderStages::FRAGMENT,
                    ty: BindingType::StorageTexture {
                        access: StorageTextureAccess::WriteOnly,
                        format: TextureFormat::Rgba8Unorm,
                        view_dimension: TextureViewDimension::D2,
                    },
                    count: None,
                },
            ],
            label: None,
        })
    }
}

impl Default for PaintMaskMaterial {
    fn default() -> Self {
        PaintMaskMaterial {
            pos: Vec3::ZERO,
            paint_mask_texture: None,
            brush_painting: false,
            cull_mode: None,
            base_color: Vec4::ZERO,
        }
    }
}

// NOTE: These must match the bit flags in bevy_pbr/src/render/pbr_types.wgsl!
bitflags::bitflags! {
    #[repr(transparent)]
    pub struct PaintMaskMaterialFlags: u32 {
        const BRUSH_PAINTING             = (1 << 0);
        const NONE                       = 0;
        const UNINITIALIZED              = 0xFFFF;
    }
}

/// The GPU representation of the uniform data of a [`StandardMaterial`].
#[derive(Clone, ShaderType)]
pub struct PaintMaskMaterialUniform {
    pub pos: Vec3,
    pub flags: u32,
}

impl AsBindGroupShaderType<PaintMaskMaterialUniform> for PaintMaskMaterial {
    fn as_bind_group_shader_type(&self, _images: &RenderAssets<Image>) -> PaintMaskMaterialUniform {
        let mut flags = PaintMaskMaterialFlags::NONE;
        if self.brush_painting {
            flags |= PaintMaskMaterialFlags::BRUSH_PAINTING;
        }

        PaintMaskMaterialUniform {
            flags: flags.bits(),
            pos: self.pos,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PaintMaskMaterialKey {
    cull_mode: Option<Face>,
}

impl From<&PaintMaskMaterial> for PaintMaskMaterialKey {
    fn from(material: &PaintMaskMaterial) -> Self {
        PaintMaskMaterialKey {
            cull_mode: material.cull_mode,
        }
    }
}

impl Material for PaintMaskMaterial {
    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = key.bind_group_data.cull_mode;
        if let Some(label) = &mut descriptor.label {
            *label = format!("paint_mask: pbr_{}", *label).into();
        }
        Ok(())
    }
    fn fragment_shader() -> ShaderRef {
        "shaders/paint_mask_material.wgsl".into()
    }

    #[inline]
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Mask(0.5)
    }

    #[inline]
    fn depth_bias(&self) -> f32 {
        0.0
    }
}
