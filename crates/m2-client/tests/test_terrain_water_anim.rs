use m2_client::terrain_water_anim::TerrainWaterAnim;

#[test]
fn test_default_values() {
    let anim = TerrainWaterAnim::default();
    assert_eq!(anim.wave_frequency, 2.0);
    assert_eq!(anim.wave_amplitude, 10.0);
    assert_eq!(anim.uv_flow_speed, (0.05, 0.02));
}

#[test]
fn test_wave_height_boundaries() {
    let anim = TerrainWaterAnim::default();
    let local_x = 100.0;
    let local_y = 50.0;
    
    // Test that the amplitude is always within [-wave_amplitude, wave_amplitude]
    for i in 0..100 {
        let cur_time_sec = i as f32 * 0.1;
        let offset = anim.get_wave_height_offset(local_x, local_y, cur_time_sec);
        assert!(offset >= -10.0 && offset <= 10.0, "Offset {} is out of bounds", offset);
    }
}

#[test]
fn test_wave_height_periodicity() {
    let anim = TerrainWaterAnim::default(); // frequency = 2.0
    // Period T = 2 * PI / frequency
    let period = std::f32::consts::PI; 
    
    let local_x = 0.0;
    let local_y = 0.0;
    
    let t1 = 0.0;
    let offset1 = anim.get_wave_height_offset(local_x, local_y, t1);
    
    let t2 = period;
    let offset2 = anim.get_wave_height_offset(local_x, local_y, t2);
    
    // Values should be very close
    let diff = (offset1 - offset2).abs();
    assert!(diff < 0.001, "Wave is not periodic: diff is {}", diff);
}

#[test]
fn test_uv_flow_correctness() {
    let anim = TerrainWaterAnim::new(2.0, 10.0, (0.1, 0.2));
    
    let (u, v) = anim.get_water_uv(0.0, 0.0, 1.0);
    assert!((u - 0.1).abs() < f32::EPSILON);
    assert!((v - 0.2).abs() < f32::EPSILON);
    
    let (u, v) = anim.get_water_uv(0.5, 0.5, 2.0);
    // 0.5 + 2.0 * 0.1 = 0.7
    // 0.5 + 2.0 * 0.2 = 0.9
    assert!((u - 0.7).abs() < 0.001);
    assert!((v - 0.9).abs() < 0.001);
}

#[test]
fn test_uv_wrapping() {
    let anim = TerrainWaterAnim::new(2.0, 10.0, (0.5, 0.8));
    
    // Exceeding 1.0
    let (u, v) = anim.get_water_uv(0.0, 0.0, 3.0);
    // u = (0.0 + 3.0 * 0.5) % 1.0 = 1.5 % 1.0 = 0.5
    // v = (0.0 + 3.0 * 0.8) % 1.0 = 2.4 % 1.0 = 0.4
    assert!((u - 0.5).abs() < 0.001);
    assert!((v - 0.4).abs() < 0.001);
    
    // Negative flow handling
    let anim_neg = TerrainWaterAnim::new(2.0, 10.0, (-0.3, -0.7));
    let (u, v) = anim_neg.get_water_uv(0.0, 0.0, 1.0);
    // u = (0.0 - 0.3) % 1.0 = -0.3 => +1.0 = 0.7
    // v = (0.0 - 0.7) % 1.0 = -0.7 => +1.0 = 0.3
    assert!((u - 0.7).abs() < 0.001);
    assert!((v - 0.3).abs() < 0.001);
    
    // Extremely negative flow
    let (u, v) = anim_neg.get_water_uv(0.0, 0.0, 10.0);
    // u = -3.0 => 0.0
    // v = -7.0 => 0.0
    assert!((u - 0.0).abs() < 0.001);
    assert!((v - 0.0).abs() < 0.001);
}
