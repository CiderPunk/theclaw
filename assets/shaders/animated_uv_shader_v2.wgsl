#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_pbr::mesh_view_bindings::globals,

struct SplosionSettings{
  frame_offset:f32,
  frame_rate:f32,
  frames_wide:f32, 
  frames_deep:f32,
  frame_count:f32,
  display_frames:f32,
  _webgl2_padding:vec2<f32>
}
@group(2) @binding(0) var<uniform> settings: SplosionSettings;
@group(2) @binding(1) var atlas_texture: texture_2d<f32>;
@group(2) @binding(2) var atlas_sampler: sampler;


@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
  let frame = floor(((globals.time * settings.frame_rate) + settings.frame_offset) % settings.frame_count );
  if (frame > settings.display_frames){
    return vec4<f32>(0.,0.,0.,0.);
  }
  let atlas_uv = vec2<f32>(
    ((frame % settings.frames_wide) * (1./settings.frames_wide)) + (mesh.uv.x / settings.frames_wide),
    (floor(frame/settings.frames_deep) * 1./settings.frames_deep) + (mesh.uv.y / settings.frames_deep),
  );
  return textureSample(atlas_texture, atlas_sampler, atlas_uv);
}