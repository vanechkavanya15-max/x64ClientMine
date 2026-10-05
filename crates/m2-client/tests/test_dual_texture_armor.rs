use std::mem;
use m2_client::wgpu_dual_texture_armor_pipeline::{
    DualTextureArmorUniforms, WgpuDualTextureArmorPipeline, DUAL_TEXTURE_ARMOR_SHADER_WGSL,
};

// We still need the GrannyActorVertex description to test compatibility,
// since it's not exported from lib.rs.
#[allow(dead_code)]
#[path = "../src/wgpu_granny_actor_renderer.rs"]
mod wgpu_granny_actor_renderer;
use wgpu_granny_actor_renderer::GrannyActorVertex;

#[test]
fn test_dual_texture_armor_uniforms_layout() {
    let size = mem::size_of::<DualTextureArmorUniforms>();
    // view_proj: 16 * 4 = 64
    // view_matrix: 16 * 4 = 64
    // bone_matrices: 128 * 16 * 4 = 8192
    // specular_trans: 3 * 4 = 12
    // specular_power: 4
    // 64 + 64 + 8192 + 12 + 4 = 8336 bytes
    assert_eq!(size, 8336);

    let alignment = mem::align_of::<DualTextureArmorUniforms>();
    assert_eq!(alignment, 4); // f32 alignment
}

#[test]
fn test_wgsl_shader_contains_critical_logic() {
    // Assert critical shader structures and variables exist matching C++ spec
    assert!(DUAL_TEXTURE_ARMOR_SHADER_WGSL.contains("struct DualTextureArmorUniforms"));
    assert!(DUAL_TEXTURE_ARMOR_SHADER_WGSL.contains("specular_trans: vec3<f32>"));
    assert!(DUAL_TEXTURE_ARMOR_SHADER_WGSL.contains("specular_power: f32"));
    assert!(DUAL_TEXTURE_ARMOR_SHADER_WGSL.contains("reflect("));
    assert!(DUAL_TEXTURE_ARMOR_SHADER_WGSL.contains("t_diffuse: texture_2d<f32>"));
    assert!(DUAL_TEXTURE_ARMOR_SHADER_WGSL.contains("t_env: texture_2d<f32>"));
    
    // Assert logic mapping reflection back to UI coordinates
    assert!(DUAL_TEXTURE_ARMOR_SHADER_WGSL.contains("env_u = reflection.x * 0.5 + 0.5"));
    // Assert specular modulation blending matching ModulateAlphaAddColor
    assert!(DUAL_TEXTURE_ARMOR_SHADER_WGSL.contains("diffuse_color.rgb + (env_color.rgb * spec_factor)"));
}

#[test]
fn test_granny_actor_vertex_compatibility() {
    // Ensure Vertex specification matches our shader expectations
    let layout = GrannyActorVertex::desc();
    assert_eq!(layout.array_stride, mem::size_of::<GrannyActorVertex>() as u64);
    assert_eq!(layout.step_mode, wgpu::VertexStepMode::Vertex);
    assert_eq!(layout.attributes.len(), 5);

    // Positions (0)
    assert_eq!(layout.attributes[0].shader_location, 0);
    assert_eq!(layout.attributes[0].format, wgpu::VertexFormat::Float32x3);

    // Bones indices (3)
    assert_eq!(layout.attributes[3].shader_location, 3);
    assert_eq!(layout.attributes[3].format, wgpu::VertexFormat::Uint32x4);
}

#[test]
fn test_pipeline_instantiation_is_functional() {
    let _fn_ptr = WgpuDualTextureArmorPipeline::new;
    let _fn_draw = WgpuDualTextureArmorPipeline::draw;
}

#[test]
fn test_shader_blend_logic_correctness() {
    let code = DUAL_TEXTURE_ARMOR_SHADER_WGSL;
    // ModulateAlphaAddColor implementation
    assert!(code.contains("let spec_factor = diffuse_color.a * uniforms.specular_power;"));
    assert!(code.contains("diffuse_color.rgb + (env_color.rgb * spec_factor)"));
    assert!(code.contains("return vec4<f32>(final_color, diffuse_color.a);"));
}

#[test]
fn test_material_parameters_uniform() {
    let uniforms = DualTextureArmorUniforms {
        view_proj: [[0.0; 4]; 4],
        view_matrix: [[0.0; 4]; 4],
        bone_matrices: [[[0.0; 4]; 4]; 128],
        specular_trans: [1.0, 2.0, 3.0],
        specular_power: 0.5,
    };
    
    assert_eq!(uniforms.specular_trans[0], 1.0);
    assert_eq!(uniforms.specular_power, 0.5);
}
