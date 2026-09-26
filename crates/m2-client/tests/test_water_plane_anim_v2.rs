use m2_client::water_plane_anim_v2::*;
use std::sync::Mutex;

#[test]
fn test_water_plane_anim_initial_state() {
    let anim = WaterPlaneAnimV2::new(100);
    assert_eq!(anim.current_height_offset, 0.0);
    assert_eq!(anim.height_begin, 0.0);
    assert_eq!(anim.height_end, 0.0);
    assert_eq!(anim.last_height_change_time, 100);
    assert_eq!(anim.blend_time, 300);
    assert_eq!(anim.u, 0.0);
    assert_eq!(anim.v, 0.0);
    assert_eq!(anim.speed_u, 0.05);
    assert_eq!(anim.speed_v, 0.02);
}

#[test]
fn test_water_height_animation_logic() {
    let mut anim = WaterPlaneAnimV2::new(1000);
    
    // First update at 1000, nothing changes yet
    anim.update(1000, 0.0);
    assert_eq!(anim.current_height_offset, 0.0);

    // Update at 1100, still within blend_time (300)
    anim.update(1100, 0.0);
    assert_eq!(anim.current_height_offset, 0.0); // height_begin and height_end are both 0.0

    // Update at 1400 (exceeds 300 blend time)
    // It should trigger a blend_time and height_end randomization
    anim.update(1400, 0.0);
    assert!(anim.blend_time >= 1000 && anim.blend_time <= 3000);
    assert!(anim.height_end <= 0.0 && anim.height_end >= -15.0);
    assert_eq!(anim.height_begin, 0.0);
    assert_eq!(anim.last_height_change_time, 1400);

    // Remember values for the next interpolation
    let blend_time = anim.blend_time;
    let height_end = anim.height_end;
    let height_begin = anim.height_begin;

    // Update halfway through the blend time
    let half_time = 1400 + blend_time / 2;
    anim.update(half_time, 0.0);
    
    // Should be halfway interpolated
    let expected_height = height_begin + (height_end - height_begin) * 0.5;
    assert!((anim.current_height_offset - expected_height).abs() < 0.001);

    // Update past blend time, should hit exactly height_end before next blend cycle triggers
    // We update to exactly 1400 + blend_time so it evaluates at 1.0 progress but doesn't trigger new cycle
    let end_time = 1400 + blend_time;
    anim.update(end_time, 0.0);
    assert!((anim.current_height_offset - height_end).abs() < 0.001);

    // Update slightly past blend time (triggers next cycle)
    let next_cycle_time = end_time + 1;
    anim.update(next_cycle_time, 0.0);
    // height_end should now be 0.0 because the previous cycle had a non-zero height_end
    assert_eq!(anim.height_end, 0.0);
    assert_eq!(anim.height_begin, height_end);
}

#[test]
fn test_uv_flow_animation() {
    let mut anim = WaterPlaneAnimV2::new(0);
    
    anim.update(0, 10.0);
    assert!((anim.u - 0.5).abs() < 0.001);
    assert!((anim.v - 0.2).abs() < 0.001);

    anim.update(0, 20.0);
    // u += 20 * 0.05 = 1.0. u = (0.5 + 1.0).fract() = 0.5
    // v += 20 * 0.02 = 0.4. v = (0.2 + 0.4).fract() = 0.6
    assert!((anim.u - 0.5).abs() < 0.001);
    assert!((anim.v - 0.6).abs() < 0.001);

    // Test negative dt and wrap around
    anim.set_uv_speed(-0.05, -0.02);
    anim.update(0, 10.0);
    // u = (0.5 - 0.5) = 0.0
    // v = (0.6 - 0.2) = 0.4
    assert!((anim.u - 0.0).abs() < 0.001);
    assert!((anim.v - 0.4).abs() < 0.001);

    anim.update(0, 10.0);
    // u = (0.0 - 0.5) = -0.5 -> wrapped to 0.5
    // v = (0.4 - 0.2) = 0.2
    assert!((anim.u - 0.5).abs() < 0.001);
    assert!((anim.v - 0.2).abs() < 0.001);
}

#[test]
fn test_sector_height_storage() {
    let mut anim = WaterPlaneAnimV2::new(0);
    anim.set_sector_water_height(0, 0, 100.0);
    anim.set_sector_water_height(1, 0, 200.0);
    anim.set_sector_water_height(-1, -1, 300.0);

    assert_eq!(anim.get_sector_water_height(0, 0), Some(100.0));
    assert_eq!(anim.get_sector_water_height(1, 0), Some(200.0));
    assert_eq!(anim.get_sector_water_height(-1, -1), Some(300.0));
    assert_eq!(anim.get_sector_water_height(2, 2), None);
}

#[test]
fn test_is_underwater_logic() {
    let mut anim = WaterPlaneAnimV2::new(0);
    anim.set_sector_water_height(0, 0, 50.0);
    
    // With offset 0.0, water level is 50.0
    assert!(anim.is_underwater(1000.0, 1000.0, 49.0)); // Under
    assert!(!anim.is_underwater(1000.0, 1000.0, 50.0)); // Equal, not under
    assert!(!anim.is_underwater(1000.0, 1000.0, 51.0)); // Above

    // Apply some offset manually to test dynamic change
    anim.current_height_offset = -10.0; // Water level is now 40.0
    
    assert!(anim.is_underwater(1000.0, 1000.0, 39.0)); // Under
    assert!(!anim.is_underwater(1000.0, 1000.0, 41.0)); // Above

    // Test different sectors mapping correctly (25600 divisor)
    anim.set_sector_water_height(1, 1, 20.0); // level is 20 + (-10) = 10
    
    let x_sec1 = 25600.0 + 10.0;
    let y_sec1 = 25600.0 + 10.0;
    
    assert!(anim.is_underwater(x_sec1, y_sec1, 9.0)); // Under
    assert!(!anim.is_underwater(x_sec1, y_sec1, 11.0)); // Above

    // Sector without water data
    let x_sec2 = 51200.0 + 10.0;
    assert!(!anim.is_underwater(x_sec2, y_sec1, -100.0)); // No data, not underwater
}

#[test]
fn test_m2_water_is_under_ffi() {
    m2_water_anim_init(0);
    m2_water_set_sector_height(0, 0, 100.0);
    
    // Base level is 100.0
    assert!(m2_water_is_under(0.0, 0.0, 99.0));
    assert!(!m2_water_is_under(0.0, 0.0, 101.0));
    
    // Simulate some time so height_end is calculated and animated
    m2_water_anim_update(500, 0.0); // should trigger randomize because 500 > 300
    m2_water_anim_update(2500, 0.0); // some time later
    
    // The water height offset should be between -15.0 and 0.0
    let anim = get_global_water_anim().lock().unwrap();
    let offset = anim.current_height_offset;
    assert!(offset <= 0.0 && offset >= -15.0);
    
    let current_level = 100.0 + offset;
    drop(anim);
    
    assert!(m2_water_is_under(0.0, 0.0, current_level - 1.0));
    assert!(!m2_water_is_under(0.0, 0.0, current_level + 1.0));
}
