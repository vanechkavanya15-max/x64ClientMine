#[path = "../src/wgpu_water_advanced.rs"]
mod wgpu_water_advanced;

use wgpu_water_advanced::{
    blend_skybox_fresnel, calculate_advanced_wave_height, calculate_depth_fade,
    AdvancedWaveParams,
};
use glam::{Vec2, Vec3};
use std::f32::consts::PI;

#[test]
fn test_advanced_wave_height_zero_time() {
    let params = AdvancedWaveParams {
        amplitude: 2.0,
        frequency: 1.0,
        speed: 1.0,
        direction: Vec2::new(1.0, 0.0),
    };
    
    // At pos(PI/2, 0), dir(1,0), dot=PI/2.
    // phase = PI/2 * 1.0 + 0 = PI/2. sin(PI/2) = 1. height = 2.0.
    let height = calculate_advanced_wave_height(Vec2::new(PI / 2.0, 0.0), 0.0, &params);
    assert!((height - 2.0).abs() < 1e-5);
    
    // At pos(PI, 0), phase = PI. sin(PI) = 0.
    let height2 = calculate_advanced_wave_height(Vec2::new(PI, 0.0), 0.0, &params);
    assert!((height2 - 0.0).abs() < 1e-5);
}

#[test]
fn test_advanced_wave_height_time_progression() {
    let params = AdvancedWaveParams {
        amplitude: 1.0,
        frequency: 1.0,
        speed: PI, // speed is PI per second
        direction: Vec2::new(0.0, 1.0),
    };
    
    // At pos(0, 0), time = 0.5s -> phase = PI/2. sin(PI/2) = 1.0.
    let height = calculate_advanced_wave_height(Vec2::new(0.0, 0.0), 0.5, &params);
    assert!((height - 1.0).abs() < 1e-5);
    
    // time = 1.0s -> phase = PI. sin(PI) = 0.
    let height2 = calculate_advanced_wave_height(Vec2::new(0.0, 0.0), 1.0, &params);
    assert!((height2 - 0.0).abs() < 1e-5);
}

#[test]
fn test_blend_skybox_fresnel_clamping() {
    let water_color = Vec3::new(0.0, 0.0, 1.0); // Blue
    let skybox_color = Vec3::new(1.0, 1.0, 1.0); // White
    
    // Below 0 should clamp to 0 (all water color)
    let blend1 = blend_skybox_fresnel(water_color, skybox_color, -0.5);
    assert_eq!(blend1, water_color);
    
    // Above 1 should clamp to 1 (all skybox color)
    let blend2 = blend_skybox_fresnel(water_color, skybox_color, 1.5);
    assert_eq!(blend2, skybox_color);
    
    // At 0.5, it should be 50-50
    let blend3 = blend_skybox_fresnel(water_color, skybox_color, 0.5);
    assert_eq!(blend3, Vec3::new(0.5, 0.5, 1.0));
}

#[test]
fn test_calculate_depth_fade_normal() {
    let max_depth = 10.0;
    
    // At 0 depth or below, fully transparent (0.0)
    assert_eq!(calculate_depth_fade(0.0, max_depth), 0.0);
    assert_eq!(calculate_depth_fade(-5.0, max_depth), 0.0);
    
    // At 5.0 depth, half transparent (0.5)
    assert_eq!(calculate_depth_fade(5.0, max_depth), 0.5);
    
    // At max depth or beyond, fully opaque (1.0)
    assert_eq!(calculate_depth_fade(10.0, max_depth), 1.0);
    assert_eq!(calculate_depth_fade(20.0, max_depth), 1.0);
}

#[test]
fn test_calculate_depth_fade_edge_cases() {
    // If max_depth is 0.0 or negative, we assume immediate fully opaque
    assert_eq!(calculate_depth_fade(5.0, 0.0), 1.0);
    assert_eq!(calculate_depth_fade(5.0, -10.0), 1.0);
    assert_eq!(calculate_depth_fade(-5.0, 0.0), 1.0);
}
