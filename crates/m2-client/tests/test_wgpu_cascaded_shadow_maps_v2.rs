#[path = "../src/wgpu_cascaded_shadow_maps_v2.rs"]
mod wgpu_cascaded_shadow_maps_v2;

use glam::{Mat4, Vec3, Vec4};
use wgpu_cascaded_shadow_maps_v2::{
    CascadedShadowMapV2, CsmUniforms, get_metin2_light_dir, update_cascades,
    CSM_SHADOW_MAP_SIZE, CSM_CASCADE_COUNT,
};

#[test]
fn test_cascaded_shadow_map_struct_layout() {
    let size = std::mem::size_of::<CsmUniforms>();
    // Array of 3 Mat4 (3 * 16 * 4) = 192 bytes
    // cascade_distances (4 * 4) = 16 bytes
    // light_dir (4 * 4) = 16 bytes
    // pcf_enabled (4) = 4 bytes
    // _padding (3 * 4) = 12 bytes
    // Total: 240 bytes
    assert_eq!(size, 240, "CsmUniforms layout size must exactly match WGSL buffer alignment");
}

#[test]
fn test_get_metin2_light_dir_matches_c_plus_plus() {
    // Tests the C++ exact equivalent:
    // v3Eye(v3Target.x - 1.732f * 1250.0f,
    //       v3Target.y - 1250.0f,
    //       v3Target.z + 2.0f * 1.732f * 1250.0f);
    let target = Vec3::new(0.0, 0.0, 0.0);
    let eye = Vec3::new(
        target.x - 1.732 * 1250.0,
        target.y - 1250.0,
        target.z + 2.0 * 1.732 * 1250.0,
    );
    let expected_dir = (target - eye).normalize();
    
    let calc_dir = get_metin2_light_dir();
    
    // Light direction vector should exactly match expected.
    assert!((calc_dir.x - expected_dir.x).abs() < 1e-5);
    assert!((calc_dir.y - expected_dir.y).abs() < 1e-5);
    assert!((calc_dir.z - expected_dir.z).abs() < 1e-5);
    
    // Z axis should be heavily pointing back and down
    assert!(calc_dir.z < 0.0);
    assert!(calc_dir.y > 0.0);
    assert!(calc_dir.x > 0.0);
}

#[test]
fn test_update_cascades_frustum_math() {
    // Camera looking down negative Z axis
    let camera_view = Mat4::look_at_rh(
        Vec3::new(0.0, 100.0, 500.0), // Eye
        Vec3::new(0.0, 0.0, 0.0),     // Target
        Vec3::new(0.0, 1.0, 0.0),     // Up
    );
    let camera_proj = Mat4::perspective_rh(std::f32::consts::PI / 4.0, 16.0 / 9.0, 10.0, 10000.0);
    
    let light_dir = get_metin2_light_dir();
    let cascade_splits = [10.0, 1000.0, 4000.0, 10000.0];
    
    let light_view_projs = update_cascades(
        camera_view,
        camera_proj,
        light_dir,
        &cascade_splits,
    );
    
    // There should be 3 cascade matrices
    assert_eq!(light_view_projs.len(), CSM_CASCADE_COUNT);
    
    for i in 0..CSM_CASCADE_COUNT {
        let mat = light_view_projs[i];
        
        // Matrix must not be identity (meaning we did some work)
        assert_ne!(mat, Mat4::IDENTITY);
        
        // Transform the center of the world. It should yield a valid coordinate in light clip space.
        let center_clip = mat * Vec4::new(0.0, 0.0, 0.0, 1.0);
        let w = center_clip.w;
        
        // It's an orthogonal projection, so w should be close to 1.0
        assert!((w - 1.0).abs() < 1e-4);
        
        // The coordinates shouldn't be NaN
        assert!(!center_clip.x.is_nan());
        assert!(!center_clip.y.is_nan());
        assert!(!center_clip.z.is_nan());
    }
}
