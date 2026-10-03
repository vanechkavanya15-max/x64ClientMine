#[path = "../src/wgpu_magic_effect_mesh_pipeline.rs"]
mod wgpu_magic_effect_mesh_pipeline;

use glam::{Mat4, Vec3};
use std::f32::consts::FRAC_PI_2;
use std::mem;
use wgpu_magic_effect_mesh_pipeline::*;

#[test]
fn test_effect_mesh_vertex_desc() {
    let desc = EffectMeshVertex::desc();
    assert_eq!(desc.array_stride, 20); // 3*4 + 2*4 = 20
    assert_eq!(desc.step_mode, wgpu::VertexStepMode::Vertex);
    assert_eq!(desc.attributes.len(), 2);

    assert_eq!(desc.attributes[0].offset, 0);
    assert_eq!(desc.attributes[0].shader_location, 0);
    assert_eq!(desc.attributes[0].format, wgpu::VertexFormat::Float32x3);

    assert_eq!(desc.attributes[1].offset, 12);
    assert_eq!(desc.attributes[1].shader_location, 1);
    assert_eq!(desc.attributes[1].format, wgpu::VertexFormat::Float32x2);
}

#[test]
fn test_effect_mesh_uniforms_layout() {
    let size = mem::size_of::<EffectMeshUniforms>();
    // 64 (world_matrix) + 64 (view_proj_matrix) + 16 (color_factor) + 4 (color_op_type) + 12 (padding)
    assert_eq!(size, 160);

    let align = mem::align_of::<EffectMeshUniforms>();
    assert_eq!(align, 4); // Standard float/u32 align
}

#[test]
fn test_d3d_blend_mapping() {
    assert_eq!(
        d3d_blend_to_wgpu_factor(D3DBLEND_ZERO),
        wgpu::BlendFactor::Zero
    );
    assert_eq!(
        d3d_blend_to_wgpu_factor(D3DBLEND_ONE),
        wgpu::BlendFactor::One
    );
    assert_eq!(
        d3d_blend_to_wgpu_factor(D3DBLEND_SRCCOLOR),
        wgpu::BlendFactor::Src
    );
    assert_eq!(
        d3d_blend_to_wgpu_factor(D3DBLEND_INVSRCCOLOR),
        wgpu::BlendFactor::OneMinusSrc
    );
    assert_eq!(
        d3d_blend_to_wgpu_factor(D3DBLEND_SRCALPHA),
        wgpu::BlendFactor::SrcAlpha
    );
    assert_eq!(
        d3d_blend_to_wgpu_factor(D3DBLEND_INVSRCALPHA),
        wgpu::BlendFactor::OneMinusSrcAlpha
    );
    assert_eq!(
        d3d_blend_to_wgpu_factor(D3DBLEND_DESTALPHA),
        wgpu::BlendFactor::DstAlpha
    );
    assert_eq!(
        d3d_blend_to_wgpu_factor(D3DBLEND_INVDESTALPHA),
        wgpu::BlendFactor::OneMinusDstAlpha
    );
    assert_eq!(
        d3d_blend_to_wgpu_factor(D3DBLEND_DESTCOLOR),
        wgpu::BlendFactor::Dst
    );
    assert_eq!(
        d3d_blend_to_wgpu_factor(D3DBLEND_INVDESTCOLOR),
        wgpu::BlendFactor::OneMinusDst
    );
    assert_eq!(
        d3d_blend_to_wgpu_factor(D3DBLEND_SRCALPHASAT),
        wgpu::BlendFactor::SrcAlphaSaturated
    );

    // Default fallback check
    assert_eq!(d3d_blend_to_wgpu_factor(255), wgpu::BlendFactor::One);
}

#[test]
fn test_get_wgpu_blend_state() {
    let state = get_wgpu_blend_state(D3DBLEND_SRCALPHA, D3DBLEND_ONE);

    assert_eq!(state.color.src_factor, wgpu::BlendFactor::SrcAlpha);
    assert_eq!(state.color.dst_factor, wgpu::BlendFactor::One);
    assert_eq!(state.color.operation, wgpu::BlendOperation::Add);

    assert_eq!(state.alpha.src_factor, wgpu::BlendFactor::SrcAlpha);
    assert_eq!(state.alpha.dst_factor, wgpu::BlendFactor::One);
    assert_eq!(state.alpha.operation, wgpu::BlendOperation::Add);
}

#[test]
fn test_calculate_effect_mesh_world_matrix_all() {
    // view matrix looking slightly off-axis to test full rotation
    let view = Mat4::look_at_rh(
        Vec3::new(10.0, 10.0, 10.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let result = calculate_effect_mesh_world_matrix(
        MESH_BILLBOARD_TYPE_ALL,
        view,
        Mat4::IDENTITY,
        Vec3::ZERO,
        Vec3::ZERO,
    );

    // The rotation should be RotX(90) * InvView(RotOnly)
    let inv_view_rot = Mat4::from_quat(view.inverse().to_scale_rotation_translation().1);
    let expected = Mat4::from_rotation_x(FRAC_PI_2) * inv_view_rot;

    for i in 0..16 {
        assert!((result.to_cols_array()[i] - expected.to_cols_array()[i]).abs() < 1e-5);
    }
}

#[test]
fn test_calculate_effect_mesh_world_matrix_y() {
    let view = Mat4::look_at_rh(
        Vec3::new(10.0, 5.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let result = calculate_effect_mesh_world_matrix(
        MESH_BILLBOARD_TYPE_Y,
        view,
        Mat4::IDENTITY,
        Vec3::ZERO,
        Vec3::ZERO,
    );

    let inv_view = view.inverse();
    let mut expected = Mat4::IDENTITY;
    expected.col_mut(0)[0] = inv_view.col(0)[0];
    expected.col_mut(0)[1] = inv_view.col(0)[1];
    expected.col_mut(1)[0] = inv_view.col(1)[0];
    expected.col_mut(1)[1] = inv_view.col(1)[1];

    for i in 0..16 {
        assert!((result.to_cols_array()[i] - expected.to_cols_array()[i]).abs() < 1e-5);
    }
}

#[test]
fn test_calculate_effect_mesh_world_matrix_move() {
    let pos = Vec3::new(1.0, 0.0, 0.0);
    let last_pos = Vec3::new(0.0, 0.0, 0.0);

    let result = calculate_effect_mesh_world_matrix(
        MESH_BILLBOARD_TYPE_MOVE,
        Mat4::IDENTITY,
        Mat4::IDENTITY,
        pos,
        last_pos,
    );

    // movement vector is (1, 0, 0)
    // we calculate rotation from (0, -1, 0) to (1, 0, 0)
    // cross((0, -1, 0), (1, 0, 0)) = (0, 0, 1)
    // dot((0, -1, 0), (1, 0, 0)) = 0 -> angle = 90 deg = PI/2

    let expected = Mat4::from_rotation_z(FRAC_PI_2);

    for i in 0..16 {
        assert!((result.to_cols_array()[i] - expected.to_cols_array()[i]).abs() < 1e-5);
    }
}

#[test]
fn test_calculate_effect_mesh_world_matrix_none() {
    let input = Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0));

    let result = calculate_effect_mesh_world_matrix(
        MESH_BILLBOARD_TYPE_NONE,
        Mat4::IDENTITY,
        input,
        Vec3::ZERO,
        Vec3::ZERO,
    );

    // Result should be unchanged
    for i in 0..16 {
        assert!((result.to_cols_array()[i] - input.to_cols_array()[i]).abs() < 1e-5);
    }
}
