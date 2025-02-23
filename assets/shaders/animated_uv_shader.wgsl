#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_pbr::mesh_view_bindings::globals,


@group(2) @binding(0) var<uniform> frame: f32;
@group(2) @binding(1) var atlas_texture: texture_2d<f32>;
@group(2) @binding(2) var atlas_sampler: sampler;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
  
  //let frame2 = floor((globals.time * 60.) / 16.);
  let frame2 = floor((globals.time * 15.) % 16.);

  let atlas_uv = vec2<f32>(
    ((frame2 % 4.) * 0.25) + (mesh.uv.x / 4.),
    (floor(frame2/4.) * 0.25) + (mesh.uv.y / 4.),
  );
  return textureSample(atlas_texture, atlas_sampler, atlas_uv);
}