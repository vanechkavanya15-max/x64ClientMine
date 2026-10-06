#[path = "../src/wgpu_speedtree_billboard_instancing_v2.rs"]
mod wgpu_speedtree_billboard_instancing_v2;

use wgpu_speedtree_billboard_instancing_v2::{
    C_F_FAR_LOD_FACTOR, C_F_NEAR_LOD_FACTOR, C_N_DEFAULT_ALPHA_TEST_VALUE, C_N_NUM_WIND_MATRICES,
    SBillboardInstance, SBillboardVertex, SpeedTreeBillboardManager,
};

#[test]
fn test_billboard_vertex_layout_size() {
    assert_eq!(
        std::mem::size_of::<SBillboardVertex>(),
        20,
        "SBillboardVertex size must be exactly 20 bytes (5x 4-byte floats)"
    );
    assert_eq!(
        std::mem::align_of::<SBillboardVertex>(),
        4,
        "SBillboardVertex alignment must be 4 bytes"
    );
}

#[test]
fn test_billboard_instance_layout_size() {
    assert_eq!(
        std::mem::size_of::<SBillboardInstance>(),
        24,
        "SBillboardInstance size must be exactly 24 bytes (6x 4-byte floats)"
    );
    assert_eq!(
        std::mem::align_of::<SBillboardInstance>(),
        4,
        "SBillboardInstance alignment must be 4 bytes"
    );
}

#[test]
fn test_config_constants() {
    assert_eq!(C_F_NEAR_LOD_FACTOR, 2.0);
    assert_eq!(C_F_FAR_LOD_FACTOR, 9.0);
    assert_eq!(C_N_DEFAULT_ALPHA_TEST_VALUE, 84);
    assert_eq!(C_N_NUM_WIND_MATRICES, 4);
}

#[test]
fn test_lod_factor_calculation() {
    let tree_height = 100.0;
    
    // Distance < Near Limit
    let factor_near = SpeedTreeBillboardManager::calculate_lod_factor(150.0, tree_height);
    assert_eq!(factor_near, 1.0, "Distance under near limit should return 1.0");

    // Distance > Far Limit
    let factor_far = SpeedTreeBillboardManager::calculate_lod_factor(950.0, tree_height);
    assert_eq!(factor_far, 0.0, "Distance over far limit should return 0.0");

    // Distance exactly halfway
    let limit_range = (tree_height * C_F_FAR_LOD_FACTOR) - (tree_height * C_F_NEAR_LOD_FACTOR);
    let halfway = (tree_height * C_F_NEAR_LOD_FACTOR) + (limit_range / 2.0);
    let factor_mid = SpeedTreeBillboardManager::calculate_lod_factor(halfway, tree_height);
    assert!((factor_mid - 0.5).abs() < f32::EPSILON, "Distance exactly halfway should return 0.5");
}

#[test]
fn test_wind_sway_calculation() {
    // With strength 0, sway should be 0.
    let sway_zero = SpeedTreeBillboardManager::calculate_wind_sway(1.0, 0.0, 0.0);
    assert_eq!(sway_zero, 0.0);

    // With time 0 and offset 0, sin(0) is 0, so sway should be 0.
    let sway_zero_time = SpeedTreeBillboardManager::calculate_wind_sway(0.0, 1.0, 0.0);
    assert_eq!(sway_zero_time, 0.0);
    
    // Sway calculation test
    let sway = SpeedTreeBillboardManager::calculate_wind_sway(10.0, 1.0, 1.5);
    let expected = ((10.0 * 2.0_f32) + 1.5).sin() * 1.0 * 0.1;
    assert!((sway - expected).abs() < f32::EPSILON);
}
