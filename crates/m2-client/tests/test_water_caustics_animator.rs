use m2_client::wgpu_water_caustics_animator::WgpuWaterCausticsAnimator;

#[test]
fn test_default_initialization() {
    let animator = WgpuWaterCausticsAnimator::default();
    assert_eq!(animator.num_frames, 30);
    assert_eq!(animator.frame_duration_ms, 70);
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
}

#[test]
fn test_edge_cases() {
    // Zero frames or zero duration
    let animator_zero_frames = WgpuWaterCausticsAnimator::new(0, 70);
    assert_eq!(animator_zero_frames.get_current_frame(100), 0);

    let animator_zero_duration = WgpuWaterCausticsAnimator::new(30, 0);
    assert_eq!(animator_zero_duration.get_current_frame(100), 0);

    // Zero or negative max depth for intensity
    let animator = WgpuWaterCausticsAnimator::default();
    assert_eq!(animator.calculate_intensity(5.0, 0.0, 0.8), 0.0);
    assert_eq!(animator.calculate_intensity(5.0, -10.0, 0.8), 0.0);
}
