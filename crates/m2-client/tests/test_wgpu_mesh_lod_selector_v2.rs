#[path = "../src/wgpu_mesh_lod_selector_v2.rs"]
mod wgpu_mesh_lod_selector_v2;

use wgpu_mesh_lod_selector_v2::{WgpuMeshLodSelectorV2, LodLevel, ANIFPS_MAX, ANIFPS_MIN};

#[test]
fn test_center_exception() {
    let mut selector = WgpuMeshLodSelectorV2::new(5000.0, 4);
    
    // Test that when distance_from_center <= LOD_APPLY_MIN (500.0), it always uses Lod0 and ANIFPS_MAX
    let result = selector.update_lod_level(300.0, 10000.0);
    assert_eq!(result.level, LodLevel::Lod0);
    assert_eq!(result.ani_fps, ANIFPS_MAX);
    assert_eq!(selector.get_current_level(), LodLevel::Lod0);
}

#[test]
fn test_model_count_one() {
    let mut selector = WgpuMeshLodSelectorV2::new(5000.0, 1);
    
    // If only 1 model, it should always return Lod0 and ANIFPS_MAX
    let result = selector.update_lod_level(1000.0, 1000.0);
    assert_eq!(result.level, LodLevel::Lod0);
    assert_eq!(result.ani_fps, ANIFPS_MAX);
}

#[test]
fn test_distance_thresholds_no_hysteresis() {
    let mut selector = WgpuMeshLodSelectorV2::new(5000.0, 4);
    selector.hysteresis_threshold = 0.0; // Remove hysteresis for simple threshold test
    
    // Set current to Lod0, camera distance 400.0 (< 500.0) -> Lod0
    selector.set_current_level(LodLevel::Lod0);
    let result = selector.update_lod_level(1000.0, 400.0);
    assert_eq!(result.level, LodLevel::Lod0);
    
    // Camera distance 1000.0 (between 500 and 1500) -> Lod1
    let result = selector.update_lod_level(1000.0, 1000.0);
    assert_eq!(result.level, LodLevel::Lod1);
    
    // Camera distance 2000.0 (between 1500 and 2500) -> Lod2
    let result = selector.update_lod_level(1000.0, 2000.0);
    assert_eq!(result.level, LodLevel::Lod2);
    
    // Camera distance 3000.0 (> 2500) -> Lod3
    let result = selector.update_lod_level(1000.0, 3000.0);
    assert_eq!(result.level, LodLevel::Lod3);
}

#[test]
fn test_hysteresis() {
    let mut selector = WgpuMeshLodSelectorV2::new(5000.0, 4);
    let h = 50.0;
    selector.hysteresis_threshold = h;
    
    // Start at Lod0
    selector.set_current_level(LodLevel::Lod0);
    
    // Move to 510.0 (just over 500 threshold, but within hysteresis 500 + 50 = 550) -> should stay Lod0
    let result = selector.update_lod_level(1000.0, 510.0);
    assert_eq!(result.level, LodLevel::Lod0);
    
    // Move to 560.0 (over 550 threshold) -> should switch to Lod1
    let result = selector.update_lod_level(1000.0, 560.0);
    assert_eq!(result.level, LodLevel::Lod1);
    
    // Move back to 510.0 (just under 500 threshold normally, but hysteresis for moving down is 500 - 50 = 450)
    // So should stay at Lod1
    let result = selector.update_lod_level(1000.0, 510.0);
    assert_eq!(result.level, LodLevel::Lod1);
    
    // Move to 440.0 (under 450 threshold) -> should switch back to Lod0
    let result = selector.update_lod_level(1000.0, 440.0);
    assert_eq!(result.level, LodLevel::Lod0);
}

#[test]
fn test_min_lod_mode() {
    let mut selector = WgpuMeshLodSelectorV2::new(5000.0, 4);
    selector.min_lod_mode_enable = true;
    
    // Even if far away, min lod mode forces Lod0
    let result = selector.update_lod_level(1000.0, 3000.0);
    assert_eq!(result.level, LodLevel::Lod0);
}

#[test]
fn test_long_distance_calculation() {
    let mut selector = WgpuMeshLodSelectorV2::new(25000.0, 4); // Building LOD distance
    
    // If distance > 5000, it uses linear step calculation
    // f_lod_factor goes from 25000 to 0.
    // Lod step is 25000 / 4 = 6250.
    
    // Camera distance 20000 -> f_lod_factor = 5000
    // raw_lod (before invert) = 5000 / 6250 = 0.
    // max_possible = 3. inverted: 3 - 0 = 3 (Lod3)
    let result = selector.update_lod_level(1000.0, 20000.0);
    assert_eq!(result.level, LodLevel::Lod3);
    
    // Camera distance 5000 -> f_lod_factor = 20000
    // raw_lod (before invert) = 20000 / 6250 = 3.
    // max_possible = 3. inverted: 3 - 3 = 0 (Lod0)
    let result = selector.update_lod_level(1000.0, 5000.0);
    assert_eq!(result.level, LodLevel::Lod0);
}

#[test]
fn test_anifps_calculation() {
    let mut selector = WgpuMeshLodSelectorV2::new(5000.0, 4);
    
    // At very far distance, fps should drop to ANIFPS_MIN
    let result = selector.update_lod_level(1000.0, 6000.0); // f_lod_factor = 0
    assert_eq!(result.ani_fps, (ANIFPS_MIN / 10) * 10);
    
    // At distance 0, fps should be ANIFPS_MAX
    let result = selector.update_lod_level(1000.0, 0.0); // f_lod_factor = 5000
    assert_eq!(result.ani_fps, (ANIFPS_MAX / 10) * 10);
}
