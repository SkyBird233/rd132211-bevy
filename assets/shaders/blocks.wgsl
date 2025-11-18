#import bevy_pbr::{
    forward_io::VertexOutput,
    mesh_view_bindings::view,
    mesh_functions,
}

@fragment
fn fragment(
    mesh_data: VertexOutput,
) -> @location(0) vec4<f32> {
    // Get the object's center position from the mesh transform
    let model_matrix = mesh_functions::get_world_from_local(mesh_data.instance_index);
    let object_center = vec3<f32>(
        model_matrix[3][0],
        model_matrix[3][1],
        model_matrix[3][2]
    );
    
    // Create color based on object's center world position
    var color = vec3<f32>(0.0,0.0,0.0);
    if(object_center.y != 0) {
        color = vec3<f32>(1.0,1.0,1.0);
    }
    
    return vec4<f32>(color, 1.0);
}
