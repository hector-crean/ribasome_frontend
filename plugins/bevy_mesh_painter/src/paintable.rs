use bevy::asset::Handle;
use bevy::math::Vec4;
use bevy::pbr::{AlphaMode, Material, MaterialPipeline, MaterialPipelineKey};
use bevy::reflect::TypeUuid;
use bevy::render::{
    color::Color, mesh::MeshVertexBufferLayout, render_asset::RenderAssets, render_resource::*,
    texture::Image,
};

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "0dd7ebce-058c-48fd-a72e-8fb72f7d5c65"]
#[bind_group_data(PaintMaterialKey)]
#[uniform(0, PaintMaterialUniform)]
pub struct PaintableMaterial {
    pub base_color: Color,
    #[texture(1)]
    #[sampler(2)]
    pub base_color_texture: Option<Handle<Image>>,

    #[texture(3)]
    #[sampler(4)]
    pub emissive_texture: Option<Handle<Image>>,

    #[texture(5)]
    #[sampler(6)]
    pub metallic_roughness_texture: Option<Handle<Image>>,

    #[texture(7)]
    #[sampler(8)]
    pub occlusion_texture: Option<Handle<Image>>,

    #[texture(9)]
    #[sampler(10)]
    pub normal_map_texture: Option<Handle<Image>>,

    #[texture(11)]
    #[sampler(12)]
    pub paint_mask_texture: Option<Handle<Image>>,

    pub emissive: Color,
    pub perceptual_roughness: f32,
    pub metallic: f32,
    pub reflectance: f32,
    pub flip_normal_map_y: bool,
    pub double_sided: bool,
    pub cull_mode: Option<Face>,
    pub unlit: bool,
    pub alpha_mode: AlphaMode,
    pub depth_bias: f32,
}

impl Default for PaintableMaterial {
    fn default() -> Self {
        PaintableMaterial {
            base_color: Color::rgb(1.0, 1.0, 1.0),
            base_color_texture: None,
            emissive: Color::BLACK,
            emissive_texture: None,
            perceptual_roughness: 0.089,
            metallic: 0.01,
            metallic_roughness_texture: None,
            reflectance: 0.5,
            occlusion_texture: None,
            normal_map_texture: None,
            flip_normal_map_y: false,
            double_sided: false,
            cull_mode: Some(Face::Back),
            unlit: false,
            alpha_mode: AlphaMode::Opaque,
            depth_bias: 0.0,
            paint_mask_texture: None,
        }
    }
}

impl From<Color> for PaintableMaterial {
    fn from(color: Color) -> Self {
        PaintableMaterial {
            base_color: color,
            alpha_mode: if color.a() < 1.0 {
                AlphaMode::Blend
            } else {
                AlphaMode::Opaque
            },
            ..Default::default()
        }
    }
}

impl From<Handle<Image>> for PaintableMaterial {
    fn from(texture: Handle<Image>) -> Self {
        PaintableMaterial {
            base_color_texture: Some(texture),
            ..Default::default()
        }
    }
}

// NOTE: These must match the bit flags in bevy_pbr/src/render/pbr_types.wgsl!
bitflags::bitflags! {
    #[repr(transparent)]
    pub struct PaintMaterialFlags: u32 {
        const BASE_COLOR_TEXTURE         = (1 << 0);
        const EMISSIVE_TEXTURE           = (1 << 1);
        const METALLIC_ROUGHNESS_TEXTURE = (1 << 2);
        const OCCLUSION_TEXTURE          = (1 << 3);
        const DOUBLE_SIDED               = (1 << 4);
        const UNLIT                      = (1 << 5);
        const ALPHA_MODE_OPAQUE          = (1 << 6);
        const ALPHA_MODE_MASK            = (1 << 7);
        const ALPHA_MODE_BLEND           = (1 << 8);
        const TWO_COMPONENT_NORMAL_MAP   = (1 << 9);
        const FLIP_NORMAL_MAP_Y          = (1 << 10);
        const NONE                       = 0;
        const UNINITIALIZED              = 0xFFFF;
    }
}

/// The GPU representation of the uniform data of a [`StandardMaterial`].
#[derive(Clone, Default, ShaderType)]
pub struct PaintMaterialUniform {
    pub base_color: Vec4,
    pub emissive: Vec4,
    pub roughness: f32,
    pub metallic: f32,
    pub reflectance: f32,
    pub flags: u32,
    pub alpha_cutoff: f32,
}

impl AsBindGroupShaderType<PaintMaterialUniform> for PaintableMaterial {
    fn as_bind_group_shader_type(&self, images: &RenderAssets<Image>) -> PaintMaterialUniform {
        let mut flags = PaintMaterialFlags::NONE;
        if self.base_color_texture.is_some() {
            flags |= PaintMaterialFlags::BASE_COLOR_TEXTURE;
        }
        if self.emissive_texture.is_some() {
            flags |= PaintMaterialFlags::EMISSIVE_TEXTURE;
        }
        if self.metallic_roughness_texture.is_some() {
            flags |= PaintMaterialFlags::METALLIC_ROUGHNESS_TEXTURE;
        }
        if self.occlusion_texture.is_some() {
            flags |= PaintMaterialFlags::OCCLUSION_TEXTURE;
        }
        if self.double_sided {
            flags |= PaintMaterialFlags::DOUBLE_SIDED;
        }
        if self.unlit {
            flags |= PaintMaterialFlags::UNLIT;
        }
        let has_normal_map = self.normal_map_texture.is_some();
        if has_normal_map {
            if let Some(texture) = images.get(self.normal_map_texture.as_ref().unwrap()) {
                match texture.texture_format {
                    // All 2-component unorm formats
                    TextureFormat::Rg8Unorm
                    | TextureFormat::Rg16Unorm
                    | TextureFormat::Bc5RgUnorm
                    | TextureFormat::EacRg11Unorm => {
                        flags |= PaintMaterialFlags::TWO_COMPONENT_NORMAL_MAP;
                    }
                    _ => {}
                }
            }
            if self.flip_normal_map_y {
                flags |= PaintMaterialFlags::FLIP_NORMAL_MAP_Y;
            }
        }
        // NOTE: 0.5 is from the glTF default - do we want this?
        let mut alpha_cutoff = 0.5;
        match self.alpha_mode {
            AlphaMode::Opaque => flags |= PaintMaterialFlags::ALPHA_MODE_OPAQUE,
            AlphaMode::Mask(c) => {
                alpha_cutoff = c;
                flags |= PaintMaterialFlags::ALPHA_MODE_MASK;
            }
            AlphaMode::Blend => flags |= PaintMaterialFlags::ALPHA_MODE_BLEND,
        };

        PaintMaterialUniform {
            base_color: self.base_color.as_linear_rgba_f32().into(),
            emissive: self.emissive.into(),
            roughness: self.perceptual_roughness,
            metallic: self.metallic,
            reflectance: self.reflectance,
            flags: flags.bits(),
            alpha_cutoff,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PaintMaterialKey {
    normal_map: bool,
    cull_mode: Option<Face>,
}

impl From<&PaintableMaterial> for PaintMaterialKey {
    fn from(material: &PaintableMaterial) -> Self {
        PaintMaterialKey {
            normal_map: material.normal_map_texture.is_some(),
            cull_mode: material.cull_mode,
        }
    }
}

impl Material for PaintableMaterial {
    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        if key.bind_group_data.normal_map {
            descriptor
                .fragment
                .as_mut()
                .unwrap()
                .shader_defs
                .push(String::from("STANDARDMATERIAL_NORMAL_MAP"));
        }
        descriptor.primitive.cull_mode = key.bind_group_data.cull_mode;
        if let Some(label) = &mut descriptor.label {
            *label = format!("paintable_material: pbr_{}", *label).into();
        }
        Ok(())
    }
    fn fragment_shader() -> ShaderRef {
        "shaders/paintable_material.wgsl".into()
    }

    #[inline]
    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }

    #[inline]
    fn depth_bias(&self) -> f32 {
        self.depth_bias
    }
}
