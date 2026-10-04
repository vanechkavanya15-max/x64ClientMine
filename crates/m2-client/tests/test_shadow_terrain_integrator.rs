use m2_client::wgpu_shadow_terrain_integrator::{ShadowTerrainIntegrator, TERRAIN_SHADOW_PCF_WGSL};
use glam::{Mat4, Vec3};

// Tests to ensure no empty stubs are present and the matrix logic is sound.

#[test]
fn test_shadow_terrain_integrator_default() {
    let integrator = ShadowTerrainIntegrator::default();
    assert_eq!(integrator.light_view_matrix, Mat4::IDENTITY);
    assert_eq!(integrator.light_proj_matrix, Mat4::IDENTITY);
}

#[test]
fn test_shadow_terrain_integrator_new() {
    let integrator = ShadowTerrainIntegrator::new();
    assert_eq!(integrator.light_view_matrix, Mat4::IDENTITY);
    assert_eq!(integrator.light_proj_matrix, Mat4::IDENTITY);
}

#[test]
fn test_calculate_light_matrices() {
    let mut integrator = ShadowTerrainIntegrator::new();
    let target = Vec3::new(100.0, 50.0, 200.0);
    
    integrator.calculate_light_matrices(target);

    // Eye calculation matching:
    // v3Eye(v3Target.x - 1.732f * 1250.0f,
    //       v3Target.y - 1250.0f,
    //       v3Target.z + 2.0f * 1.732f * 1250.0f);
    let expected_eye = Vec3::new(
        100.0 - 1.732 * 1250.0,
        50.0 - 1250.0,
        200.0 + 2.0 * 1.732 * 1250.0,
    );

    let expected_view = Mat4::look_at_rh(expected_eye, target, Vec3::new(0.0, 0.0, 1.0));

    let w = 2550.0;
    let h = 2550.0;
    let expected_proj = Mat4::orthographic_rh(-w / 2.0, w / 2.0, -h / 2.0, h / 2.0, 1.0, 15000.0);

    assert_eq!(integrator.light_view_matrix, expected_view);
    assert_eq!(integrator.light_proj_matrix, expected_proj);

    let vp = integrator.get_light_view_proj_matrix();
    assert_eq!(vp, expected_proj * expected_view);
}

#[test]
fn test_shader_snippet_contains_pcf() {
    assert!(TERRAIN_SHADOW_PCF_WGSL.contains("fn calculate_shadow_pcf"));
    assert!(TERRAIN_SHADOW_PCF_WGSL.contains("fn get_terrain_shadow_factor"));
    assert!(TERRAIN_SHADOW_PCF_WGSL.contains("textureSampleCompareLevel"));
}
