use m2_client::wgpu_water_caustics_animator::WgpuWaterCausticsAnimator;

#[test]
fn test_default_initialization() {
    let animator = WgpuWaterCausticsAnimator::default();
    assert_eq!(animator.num_frames, 30);
    assert_eq!(animator.frame_duration_ms, 70);
    assert_eq!(animator.base_path, "d:/ymir Work/special/water/");
}

#[test]
fn test_custom_initialization() {
    let animator = WgpuWaterCausticsAnimator::new(20, 50, Some("custom/path/".to_string()));
    assert_eq!(animator.num_frames, 20);
    assert_eq!(animator.frame_duration_ms, 50);
    assert_eq!(animator.base_path, "custom/path/");
    
    let animator2 = WgpuWaterCausticsAnimator::new(20, 50, None);
    assert_eq!(animator2.base_path, "d:/ymir Work/special/water/");
}

#[test]
fn test_frame_sequence() {
    let animator = WgpuWaterCausticsAnimator::default();

    // time_ms = 0 -> frame 0
    assert_eq!(animator.get_current_frame(0), 0);
    
    // time_ms = 69 -> frame 0
    assert_eq!(animator.get_current_frame(69), 0);

    // time_ms = 70 -> frame 1
    assert_eq!(animator.get_current_frame(70), 1);

    // time_ms = 2100 (30 frames * 70ms) -> frame 0
    assert_eq!(animator.get_current_frame(2100), 0);

    // time_ms = 2170 (31 frames * 70ms) -> frame 1
    assert_eq!(animator.get_current_frame(2170), 1);
}

#[test]
fn test_calculate_intensity() {
    let animator = WgpuWaterCausticsAnimator::default();

    // Zero depth -> full transparency
    assert_eq!(animator.calculate_intensity(0.0, 10.0, 0.8), 0.8);

    // Half depth -> half transparency
    assert_eq!(animator.calculate_intensity(5.0, 10.0, 0.8), 0.4);

    // Max depth -> zero intensity
    assert_eq!(animator.calculate_intensity(10.0, 10.0, 0.8), 0.0);

    // Beyond max depth -> zero intensity
    assert_eq!(animator.calculate_intensity(15.0, 10.0, 0.8), 0.0);
    
    // Testing EPSILON bounds
    assert_eq!(animator.calculate_intensity(f32::EPSILON / 2.0, 10.0, 0.8), 0.8); // effectively 0 depth
    assert_eq!(animator.calculate_intensity(5.0, f32::EPSILON / 2.0, 0.8), 0.0); // effectively 0 max depth
}

#[test]
fn test_get_texture_path() {
    let animator = WgpuWaterCausticsAnimator::default();

    // time_ms = 0 -> frame 0 -> 01.dds
    assert_eq!(animator.get_texture_path(0), "d:/ymir Work/special/water/01.dds");

    // time_ms = 70 -> frame 1 -> 02.dds
    assert_eq!(animator.get_texture_path(70), "d:/ymir Work/special/water/02.dds");

    // time_ms = 2030 (29 frames * 70ms) -> frame 29 -> 30.dds
    assert_eq!(animator.get_texture_path(2030), "d:/ymir Work/special/water/30.dds");

    // time_ms = 2100 (30 frames * 70ms) -> frame 0 -> 01.dds (Wrap around)
    assert_eq!(animator.get_texture_path(2100), "d:/ymir Work/special/water/01.dds");
    
    // With custom path
    let custom_animator = WgpuWaterCausticsAnimator::new(10, 100, Some("test/".to_string()));
    assert_eq!(custom_animator.get_texture_path(0), "test/01.dds");
}

#[test]
fn test_water_height_animator() {
    use m2_client::wgpu_water_caustics_animator::WaterHeightAnimator;
    
    // Initial state check
    let mut animator = WaterHeightAnimator::new(100);
    assert_eq!(animator.current_height, 0.0);
    assert_eq!(animator.last_change_time_ms, 100);
    assert_eq!(animator.blend_time_ms, 300);

    // Update before blend time
    let height = animator.update(200); // 100ms elapsed, blend time 300ms
    assert_eq!(height, 0.0); // should stay 0 since end is 0 and begin is 0
    
    // Trigger the blend time crossing to assign random target height
    // We mock elapsed > blend_time_ms
    let _new_height = animator.update(100 + 300 + 1);
    
    assert!(animator.blend_time_ms >= 1000 && animator.blend_time_ms <= 3000);
    // height_end will be set either to 0.0 or a negative value between 0.0 and -15.0
    assert!(animator.height_end <= 0.0 && animator.height_end >= -15.0);
    assert_eq!(animator.last_change_time_ms, 401);

    // Let's force some values to test the interpolation
    animator.height_begin = 0.0;
    animator.height_end = -10.0;
    animator.blend_time_ms = 1000;
    animator.last_change_time_ms = 1000;

    let interp_height = animator.update(1500); // 500ms elapsed / 1000ms blend = 0.5 progress
    assert_eq!(interp_height, -5.0);
    assert_eq!(animator.current_height, -5.0);
}

#[test]
fn test_edge_cases() {
    // Zero frames or zero duration
    let animator_zero_frames = WgpuWaterCausticsAnimator::new(0, 70, None);
    assert_eq!(animator_zero_frames.get_current_frame(100), 0);

    let animator_zero_duration = WgpuWaterCausticsAnimator::new(30, 0, None);
    assert_eq!(animator_zero_duration.get_current_frame(100), 0);

    // Zero or negative max depth for intensity
    let animator = WgpuWaterCausticsAnimator::default();
    assert_eq!(animator.calculate_intensity(5.0, 0.0, 0.8), 0.0);
    assert_eq!(animator.calculate_intensity(5.0, -10.0, 0.8), 0.0);
}
