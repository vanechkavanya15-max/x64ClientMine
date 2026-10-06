struct Globals {
    view_proj: mat4x4<f32>,
    camera_pos: vec3<f32>,
    wind_matrices: array<mat4x4<f32>, 4>,
}

@group(1) @binding(0)
var<uniform> globals: Globals;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct InstanceInput {
    @location(2) position: vec3<f32>,
    @location(3) scale: f32,
    @location(4) tree_height: f32,
    @location(5) wind_offset: f32, // Passed into wind weight mapping or used as an index/phase
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;

    // 1. Base Local Scaling
    let scaled_pos = model.position * instance.scale;
    
    // 2. Wind simulation
    // Using a simplistic weight for wind offset in billboard since we don't have separate branch vertices.
    // SpeedTree usually applies wind matrices. Let's sample a single wind matrix for the billboard for swaying
    let wind_index = u32(instance.wind_offset) % 4u; 
    let wind_mat = globals.wind_matrices[wind_index];
    
    // Convert scaled position to homogenous space, apply wind transform 
    // Speedtree typically fades wind influence towards the base. We estimate tree height ratio using Z.
    // Normalized height (assuming model.position.z ranges from 0 to 1 in normalized billboard space):
    let height_ratio = clamp(model.position.z, 0.0, 1.0);
    
    let wind_transformed = (wind_mat * vec4<f32>(scaled_pos, 1.0)).xyz;
    let local_pos = mix(scaled_pos, wind_transformed, height_ratio);
    
    // 3. Billboard rotation (Cylindrical or Spherical Billboard)
    // To face camera, we extract the view vectors.
    let to_camera = normalize(globals.camera_pos - instance.position);
    
    // Metin2 UP vector is Z (WRAPPER_UP_POS_Z)
    let up = vec3<f32>(0.0, 0.0, 1.0);
    let right = normalize(cross(to_camera, up));
    // Recompute orthogonal forward (optional, typically billboarding uses right and up directly)
    // let forward = cross(up, right);
    
    // Map local X to right, local Z to up, local Y to forward (or flat)
    let rotated_pos = (right * local_pos.x) + (up * local_pos.z);
    
    // 4. World Position and Projection
    let world_position = rotated_pos + instance.position;
    out.clip_position = globals.view_proj * vec4<f32>(world_position, 1.0);
    out.tex_coords = model.tex_coords;
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let color = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    // Mimic the c_nDefaultAlphaTestValue (84/255) from SpeedTreeConfig.h
    if (color.a < 0.32941) {
        discard;
    }
    return color;
}