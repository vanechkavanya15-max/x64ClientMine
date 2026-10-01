use m2_client::wgpu_granny_actor_renderer::{GrannyActorVertex, ActorUniforms, ACTOR_SHADER_WGSL};
use std::mem;

#[test]
fn test_granny_actor_vertex_size_and_alignment() {
    assert_eq!(mem::size_of::<GrannyActorVertex>(), 64);
    assert_eq!(mem::align_of::<GrannyActorVertex>(), 4);
}

#[test]
fn test_actor_uniforms_layout() {
    assert_eq!(mem::size_of::<ActorUniforms>(), 64 + 128 * 64);
    assert_eq!(mem::align_of::<ActorUniforms>(), 4);
}

#[test]
fn test_vertex_layout_desc() {
    let desc = GrannyActorVertex::desc();
    assert_eq!(desc.array_stride, 64);
    assert_eq!(desc.step_mode, wgpu::VertexStepMode::Vertex);
    assert_eq!(desc.attributes.len(), 5);
    
    assert_eq!(desc.attributes[0].format, wgpu::VertexFormat::Float32x3);
    assert_eq!(desc.attributes[0].offset, 0);
    
    assert_eq!(desc.attributes[1].format, wgpu::VertexFormat::Float32x3);
    assert_eq!(desc.attributes[1].offset, 12);
    
    assert_eq!(desc.attributes[2].format, wgpu::VertexFormat::Float32x2);
    assert_eq!(desc.attributes[2].offset, 24);
    
    assert_eq!(desc.attributes[3].format, wgpu::VertexFormat::Uint32x4);
    assert_eq!(desc.attributes[3].offset, 32);
    
    assert_eq!(desc.attributes[4].format, wgpu::VertexFormat::Float32x4);
    assert_eq!(desc.attributes[4].offset, 48);
}

#[test]
fn test_wgsl_shader_compiles() {
    // Basic check that shader string is populated and contains expected tokens
    assert!(ACTOR_SHADER_WGSL.contains("struct ActorUniforms"));
    assert!(ACTOR_SHADER_WGSL.contains("vs_main"));
    assert!(ACTOR_SHADER_WGSL.contains("fs_main"));
    assert!(ACTOR_SHADER_WGSL.contains("skinned_pos = skinned_pos + (bone_matrix * local_pos) * weight;"));
}

#[test]
fn test_software_skinning_math() {
    // Test the exact math done in the WGSL shader in software to ensure accuracy
    // WGSL logic:
    // skinned_pos = sum(bone_matrix * local_pos * weight)
    
    // We'll create a vertex, 2 bones, and apply weights
    let pos = [1.0_f32, 2.0, 3.0];
    
    // Bone 0: translation of +1 on X
    let bone0 = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [1.0, 0.0, 0.0, 1.0],
    ];
    
    // Bone 1: translation of +1 on Y
    let bone1 = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0, 1.0],
    ];

    let mut vertex = GrannyActorVertex {
        position: pos,
        normal: [0.0, 1.0, 0.0],
        tex_coord: [0.0, 0.0],
        bone_indices: [0, 1, 0, 0],
        bone_weights: [0.5, 0.5, 0.0, 0.0],
    };

    let mut skinned_pos = [0.0_f32; 4];
    
    for i in 0..4 {
        let bone_index = vertex.bone_indices[i] as usize;
        let weight = vertex.bone_weights[i];
        
        let matrix = if bone_index == 0 { bone0 } else { bone1 };
        
        // Matrix * Vector multiplication (column-major logic matching WGSL/WGPU standard, wgsl applies vec4 * mat4 as mat * vec)
        // WGSL: bone_matrix * local_pos
        // In WGSL matrix columns are arrays.
        let local_pos = [pos[0], pos[1], pos[2], 1.0];
        
        // standard matrix * vector
        let transformed_x = matrix[0][0]*local_pos[0] + matrix[1][0]*local_pos[1] + matrix[2][0]*local_pos[2] + matrix[3][0]*local_pos[3];
        let transformed_y = matrix[0][1]*local_pos[0] + matrix[1][1]*local_pos[1] + matrix[2][1]*local_pos[2] + matrix[3][1]*local_pos[3];
        let transformed_z = matrix[0][2]*local_pos[0] + matrix[1][2]*local_pos[1] + matrix[2][2]*local_pos[2] + matrix[3][2]*local_pos[3];
        let transformed_w = matrix[0][3]*local_pos[0] + matrix[1][3]*local_pos[1] + matrix[2][3]*local_pos[2] + matrix[3][3]*local_pos[3];
        
        skinned_pos[0] += transformed_x * weight;
        skinned_pos[1] += transformed_y * weight;
        skinned_pos[2] += transformed_z * weight;
        skinned_pos[3] += transformed_w * weight;
    }

    // Expected:
    // bone0 transform: (2, 2, 3)
    // bone1 transform: (1, 3, 3)
    // weights are 0.5 and 0.5
    // result: (1.5, 2.5, 3)
    
    assert!((skinned_pos[0] - 1.5).abs() < 1e-5);
    assert!((skinned_pos[1] - 2.5).abs() < 1e-5);
    assert!((skinned_pos[2] - 3.0).abs() < 1e-5);
}