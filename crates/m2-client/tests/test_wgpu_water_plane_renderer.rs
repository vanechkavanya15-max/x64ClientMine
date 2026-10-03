use m2_client::wgpu_water_plane_renderer::{WaterVertex, WaterUniforms, calculate_water_uvs, calculate_water_specular, calculate_water_alpha, WgpuWaterPlaneRenderer};

#[test]
fn test_wgpu_water_plane_renderer_basic() {
    assert_eq!(std::mem::size_of::<WaterVertex>(), 16);
}

#[test]
fn test_calculate_water_uvs() {
    let base_uv = [0.0, 0.0];
    let time = 10.0;
    let dir1 = [0.05, 0.05];
    let dir2 = [-0.03, 0.04];
    
    let (uv1, uv2) = calculate_water_uvs(base_uv, time, dir1, dir2);
    
    // Allow for small float precision differences
    assert!((uv1[0] - 0.5).abs() < 1e-6);
    assert!((uv1[1] - 0.5).abs() < 1e-6);
    assert!((uv2[0] - -0.3).abs() < 1e-6);
    assert!((uv2[1] - 0.4).abs() < 1e-6);
}

#[test]
fn test_calculate_water_specular() {
    let normal = glam::Vec3::new(0.0, 0.0, 1.0);
    let view_dir = glam::Vec3::new(0.0, -1.0, 1.0).normalize();
    let sun_dir = glam::Vec3::new(0.5, 1.0, 0.3).normalize();
    
    let spec = calculate_water_specular(normal, view_dir, sun_dir, 32.0);
    
    // Just a sanity check that it's between 0 and 1
    assert!(spec >= 0.0 && spec <= 1.0);
}

#[test]
fn test_calculate_water_alpha() {
    // Water surface above terrain
    let alpha1 = calculate_water_alpha(100.0, 0.0, 100.0);
    assert_eq!(alpha1, 1.0); // max depth -> max opacity
    
    // Very shallow water
    let alpha2 = calculate_water_alpha(10.0, 0.0, 100.0);
    assert!(alpha2 > 0.4 && alpha2 < 0.5); 
    
    // Terrain above water (should return base transparency)
    let alpha3 = calculate_water_alpha(0.0, 10.0, 100.0);
    assert_eq!(alpha3, 0.4); 
}

#[test]
fn test_wgpu_water_plane_renderer_uniforms_default() {
    let uniforms = WaterUniforms::default();
    assert_eq!(uniforms.dir1, [0.05, 0.05]);
    assert_eq!(uniforms.dir2, [-0.03, 0.04]);
    assert_eq!(uniforms.sun_dir, [0.5, 1.0, 0.3]);
    assert_eq!(uniforms.water_depth_scale, 1.0);
    assert_eq!(uniforms.time, 0.0);
}

#[test]
fn test_wgpu_water_plane_renderer_is_accessible() {
    let _renderer = std::mem::size_of::<WgpuWaterPlaneRenderer>();
    assert!(_renderer > 0);
}
