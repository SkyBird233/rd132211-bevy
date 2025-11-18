#import bevy_pbr::{
    forward_io::VertexOutput,
    mesh_view_bindings::view,
    mesh_functions,
}

@group(2) @binding(0)
var terrain_texture: texture_2d<f32>;
@group(2) @binding(1)
var terrain_sampler: sampler;

@fragment
fn fragment(
    mesh_data: VertexOutput,
) -> @location(0) vec4<f32> {
    // Get the object's center position
    let model_matrix = mesh_functions::get_world_from_local(mesh_data.instance_index);
    let object_center = vec3<f32>(
        model_matrix[3][0],
        model_matrix[3][1],
        model_matrix[3][2]
    );
    
    // Determine block type based on center position
    var tile_index = 0u;
    if(object_center.y != 0.0) {
        tile_index = 1u;
    }
    
    let tiles_per_row = 16u;
    let tile_x = tile_index % tiles_per_row;
    let tile_y = tile_index / tiles_per_row;
    
    // Get UV coordinates within the current face (0-1 range)
    let face_uv = fract(mesh_data.uv);
    
    // ((x,y) + face_uv) / tiles_per_row
    let atlas_uv = (vec2<f32>(f32(tile_x), f32(tile_y)) + face_uv) / f32(tiles_per_row);
    
    let color = textureSample(terrain_texture, terrain_sampler, atlas_uv);
    return color;
}
