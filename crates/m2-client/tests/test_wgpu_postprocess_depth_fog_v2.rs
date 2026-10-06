#[path = "../src/wgpu_postprocess_depth_fog_v2.rs"]
mod wgpu_postprocess_depth_fog_v2;

use wgpu_postprocess_depth_fog_v2::{DepthFogV2, FogModeV2};

#[test]
fn test_fog_disabled() {
    let fog = DepthFogV2::new();
    assert_eq!(fog.calculate_fog_density(100.0, 0), 0.0);
}

#[test]
fn test_fog_linear() {
    let mut fog = DepthFogV2::new();
    fog.set_enabled(true);
    fog.set_mode(FogModeV2::Linear);
    fog.set_distances(100.0, 200.0);
    fog.set_color(1.0, 1.0, 1.0, 1.0); // Use set_color

    // level 1 scale is 1.0
    assert_eq!(fog.calculate_fog_density(50.0, 1), 0.0);
    assert_eq!(fog.calculate_fog_density(150.0, 1), 0.5);
    assert_eq!(fog.calculate_fog_density(250.0, 1), 1.0);

    // level 0 scale is 0.75 -> near: 75.0, far: 150.0
    assert_eq!(fog.calculate_fog_density(50.0, 0), 0.0);
    assert_eq!(fog.calculate_fog_density(112.5, 0), 0.5);
    assert_eq!(fog.calculate_fog_density(200.0, 0), 1.0);

    // level 2 scale is 1.25 -> near: 125.0, far: 250.0
    assert_eq!(fog.calculate_fog_density(100.0, 2), 0.0);
    assert_eq!(fog.calculate_fog_density(187.5, 2), 0.5);
    assert_eq!(fog.calculate_fog_density(300.0, 2), 1.0);
}

#[test]
fn test_fog_exp() {
    let mut fog = DepthFogV2::new();
    fog.set_enabled(true);
    fog.set_mode(FogModeV2::Exp);
    
    // C++ map level
    let base_density = 1000.0;
    fog.set_density(base_density);
    fog.set_color(1.0, 1.0, 1.0, 1.0); // Use set_color

    // level 1 density is 0.000004
    // density * level_density = 1000.0 * 0.000004 = 0.004
    
    let depth = 100.0;
    // f = 1.0 / e^(100.0 * 0.004) = 1.0 / e^0.4 = 1.0 / 1.4918 = 0.6703
    // res = 1.0 - 0.6703 = 0.3297
    let result = fog.calculate_fog_density(depth, 1);
    
    let expected = 1.0 - (1.0 / std::f32::consts::E.powf(100.0 * 0.004));
    assert!((result - expected).abs() < 0.0001);
}

#[test]
fn test_invalid_level() {
    let mut fog = DepthFogV2::new();
    fog.set_enabled(true);
    fog.set_mode(FogModeV2::Linear);
    fog.set_distances(100.0, 200.0);
    assert_eq!(fog.color, [0.0, 0.0, 0.0, 1.0]); // Use color

    // level 5 should be clamped to 2
    let res2 = fog.calculate_fog_density(150.0, 2);
    let res5 = fog.calculate_fog_density(150.0, 5);
    assert_eq!(res2, res5);
}
