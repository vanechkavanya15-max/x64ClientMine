use glam::{Mat4, Vec3};

#[path = "../src/lens_sun_flare_v2.rs"]
mod lens_sun_flare_v2;

use lens_sun_flare_v2::{m2_lens_flare_calc, FlarePiece, TerrainHeightCallback};

extern "C" fn mock_terrain_flat(x: f32, y: f32) -> f32 {
    0.0
}

extern "C" fn mock_terrain_high(x: f32, y: f32) -> f32 {
    5000.0 // Very high, will occlude unless camera/sun is higher
}

fn create_perspective_vp() -> [f32; 16] {
    let proj = Mat4::perspective_rh(std::f32::consts::FRAC_PI_4, 800.0 / 600.0, 1.0, 100000.0);
    // Camera at origin looking along +Z (or -Y depending on convention, let's use +X to be simple)
    let view = Mat4::look_at_rh(Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0), Vec3::Z);
    (proj * view).to_cols_array()
}

#[test]
fn test_sun_flare_null_ptrs() {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut pieces = [FlarePiece::default(); 8];
    let mut bright = 0.0;
    
    let result = m2_lens_flare_calc(
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        800.0,
        600.0,
        mock_terrain_flat,
        &mut x,
        &mut y,
        pieces.as_mut_ptr(),
        &mut bright,
    );
    
    assert_eq!(result, false);
}

#[test]
fn test_sun_flare_behind_camera() {
    let target_pos = [100.0, 0.0, 0.0];
    let eye_pos = [0.0, 0.0, 0.0];
    
    // Camera is looking +X
    let camera_dir = [1.0, 0.0, 0.0];
    
    // Light is coming FROM -X (so sun is at +X) -> this would be visible
    // To make it behind, let's have sun at -X. Sun pos = target - light_dir * large.
    // If we want sun at -X, we need light_dir to be +X.
    let light_dir = [1.0, 0.0, 0.0]; // Light travels +X. Sun is at -99999999 in X.
    
    let vp = Mat4::IDENTITY.to_cols_array();
    
    let mut x = 0.0;
    let mut y = 0.0;
    let mut pieces = [FlarePiece::default(); 8];
    let mut bright = 1.0; // Start bright
    
    let result = m2_lens_flare_calc(
        &target_pos,
        &eye_pos,
        &camera_dir,
        &light_dir,
        &vp,
        800.0,
        600.0,
        mock_terrain_flat,
        &mut x,
        &mut y,
        pieces.as_mut_ptr(),
        &mut bright,
    );
    
    assert_eq!(result, false, "Should return false when sun is behind camera");
    assert_eq!(bright, 0.0, "Brightness should be set to 0.0 when behind camera");
}

#[test]
fn test_sun_flare_occluded_by_terrain() {
    let target_pos = [100.0, 0.0, 0.0];
    let eye_pos = [0.0, 0.0, 0.0];
    let camera_dir = [1.0, 0.0, 0.0];
    
    // Light comes from -X, so sun is at +X.
    let light_dir = [-1.0, 0.0, 0.0]; 
    
    let vp = create_perspective_vp();
    
    let mut x = 0.0;
    let mut y = 0.0;
    let mut pieces = [FlarePiece::default(); 8];
    let mut bright = 1.0;
    
    let result = m2_lens_flare_calc(
        &target_pos,
        &eye_pos,
        &camera_dir,
        &light_dir,
        &vp,
        800.0,
        600.0,
        mock_terrain_high, // Terrain is at Z=5000, ray goes along Z=0 -> occluded!
        &mut x,
        &mut y,
        pieces.as_mut_ptr(),
        &mut bright,
    );
    
    assert_eq!(result, false, "Should return false when occluded by terrain");
    assert_eq!(bright, 0.0, "Brightness should be 0.0 when occluded");
}

#[test]
fn test_sun_flare_visible_calc_pieces() {
    let target_pos = [100.0, 0.0, 0.0];
    let eye_pos = [0.0, 0.0, 0.0];
    let camera_dir = [1.0, 0.0, 0.0];
    
    // Light comes from -X, so sun is at +X.
    let light_dir = [-1.0, 0.0, 0.0]; 
    
    let proj = Mat4::perspective_rh(std::f32::consts::FRAC_PI_4, 800.0 / 600.0, 1.0, 200000000.0);
    let view = Mat4::look_at_rh(Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0), Vec3::Z);
    let vp = (proj * view).to_cols_array();
    
    let mut x = 0.0;
    let mut y = 0.0;
    let mut pieces = [FlarePiece::default(); 8];
    let mut bright = 0.0;
    
    let result = m2_lens_flare_calc(
        &target_pos,
        &eye_pos,
        &camera_dir,
        &light_dir,
        &vp,
        800.0,
        600.0,
        mock_terrain_flat, // Terrain at Z=0, ray along Z=0. We'll set eye a bit higher to avoid occlusion.
        &mut x,
        &mut y,
        pieces.as_mut_ptr(),
        &mut bright,
    );
    
    // We should make eye slightly higher so it doesn't intersect Z=0 flat terrain
    let eye_pos_high = [0.0, 0.0, 100.0];
    let target_pos_high = [100.0, 0.0, 100.0];
    let view_high = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 100.0), Vec3::new(1.0, 0.0, 100.0), Vec3::Z);
    let vp_high = (proj * view_high).to_cols_array();

    let result = m2_lens_flare_calc(
        &target_pos_high,
        &eye_pos_high,
        &camera_dir,
        &light_dir,
        &vp_high,
        800.0,
        600.0,
        mock_terrain_flat,
        &mut x,
        &mut y,
        pieces.as_mut_ptr(),
        &mut bright,
    );
    
    assert_eq!(result, true, "Flare should be visible");
    assert!(bright > 0.0, "Brightness should be > 0");
    
    // Check if pieces got generated properly
    // Check first piece
    assert_eq!(pieces[0].size, 20.0);
    assert_eq!(pieces[0].color_r, 1.0);
    assert_eq!(pieces[0].color_g, 1.0);
    assert_eq!(pieces[0].color_b, 0.0);
    assert_eq!(pieces[0].color_a, 1.0);
    
    // Check array values are propagated
    assert_eq!(pieces[7].size, 250.0);
}

#[test]
fn test_sun_flare_null_optional_out() {
    let eye_pos_high = [0.0, 0.0, 100.0];
    let target_pos_high = [100.0, 0.0, 100.0];
    let camera_dir = [1.0, 0.0, 0.0];
    let light_dir = [-1.0, 0.0, 0.0]; 

    let proj = Mat4::perspective_rh(std::f32::consts::FRAC_PI_4, 800.0 / 600.0, 1.0, 200000000.0);
    let view_high = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 100.0), Vec3::new(1.0, 0.0, 100.0), Vec3::Z);
    let vp_high = (proj * view_high).to_cols_array();

    // Passing null pointers to optional outs shouldn't crash and still return true
    let result = m2_lens_flare_calc(
        &target_pos_high,
        &eye_pos_high,
        &camera_dir,
        &light_dir,
        &vp_high,
        800.0,
        600.0,
        mock_terrain_flat,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );
    
    assert_eq!(result, true);
}

#[test]
fn test_sun_flare_too_far_from_center() {
    let target_pos = [100.0, 100000.0, 100.0]; // Looking diagonally
    let eye_pos = [0.0, 0.0, 100.0];
    let camera_dir = [1.0, 0.0, 0.0]; // But camera still looking +X
    
    // Sun is at +Y mostly
    let light_dir = [0.0, -1.0, 0.0]; 
    
    let proj = Mat4::perspective_rh(std::f32::consts::FRAC_PI_4, 800.0 / 600.0, 1.0, 200000000.0);
    let view = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 100.0), Vec3::new(1.0, 0.0, 100.0), Vec3::Z);
    let vp = (proj * view).to_cols_array();

    let mut bright = 1.0;
    let mut pieces = [FlarePiece::default(); 8];
    
    // Might be in front but far out of screen center -> brightness drops to 0
    let result = m2_lens_flare_calc(
        &target_pos,
        &eye_pos,
        &camera_dir,
        &light_dir,
        &vp,
        800.0,
        600.0,
        mock_terrain_flat,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        pieces.as_mut_ptr(),
        &mut bright,
    );
    
    // In Metin2, if sun is too far from center, bright drops to 0, and returns false if we assume not drawn.
    // Our logic sets after_bright = 0.0 and returns false if after_bright <= 0.0.
    assert_eq!(result, false, "Should return false if too far off center (brightness dropped to 0)");
    assert_eq!(bright, 0.0);
}
