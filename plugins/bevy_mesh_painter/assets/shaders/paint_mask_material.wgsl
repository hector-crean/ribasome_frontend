
struct PaintMaskMaterial {
    pos: vec3<f32>,
    flags: u32,
}

let STANDARD_MATERIAL_FLAGS_BRUSH_PAINTING: u32 = 1u;

@group(1) @binding(0)
var <uniform> material: PaintMaskMaterial;
@group(1) @binding(1)
var paint_mask_texture: texture_storage_2d<rgba8unorm, write>;

struct FragmentInput {
    #import bevy_pbr::mesh_vertex_output
};

@fragment
fn fragment(in: FragmentInput) {

  if (material.flags & STANDARD_MATERIAL_FLAGS_BRUSH_PAINTING) !=0u {

    var paint_mask_size = textureDimensions(paint_mask_texture);
    var tex_pos = vec2<f32>(in.uv.x, in.uv.y);
    let tex_pos_x = f32(paint_mask_size.x) * in.uv.x;
    let tex_pos_y = f32(paint_mask_size.y) * in.uv.y;
    let converted_tex_pos  = vec2<i32>(i32(tex_pos_x) ,i32(tex_pos_y));

    var world_pos = vec3 (
      in.world_position[0],
      in.world_position[1],
      in.world_position[2],
    );
    let dist = distance(material.pos, world_pos);
    if dist < 0.2 {
      textureStore(paint_mask_texture, converted_tex_pos, vec4<f32>(1.0, 1.0, 1.0, 1.0));
    }
  }
}

