#[path = "../src/water_fx_math_complete.rs"]
mod water_fx_math_complete;

use water_fx_math_complete::{calculate_water_fresnel, calculate_water_foam};
use glam::Vec3;

#[test]
fn test_fresnel_straight_down() {
    let normal = Vec3::new(0.0, 0.0, 1.0);
    let view_dir = Vec3::new(0.0, 0.0, 1.0);
    // bias = 0.02, power = 5.0
    let fresnel = calculate_water_fresnel(view_dir, normal, 0.02, 5.0);
    
    // dot = 1.0, 1-1 = 0, so should be 0.02
    assert!((fresnel - 0.02).abs() < 1e-6);
}

#[test]
fn test_fresnel_glancing_angle() {
    let normal = Vec3::new(0.0, 0.0, 1.0);
    let view_dir = Vec3::new(1.0, 0.0, 0.0);
    let fresnel = calculate_water_fresnel(view_dir, normal, 0.02, 5.0);
    
    // dot = 0.0, 1-0=1, 1^5=1, 0.02 + 0.98 * 1 = 1.0
    assert!((fresnel - 1.0).abs() < 1e-6);
}

#[test]
fn test_fresnel_45_degrees() {
    let normal = Vec3::new(0.0, 0.0, 1.0);
    let view_dir = Vec3::new(0.70710678, 0.0, 0.70710678); // cos(45) = ~0.707
    let fresnel = calculate_water_fresnel(view_dir, normal, 0.02, 5.0);
    
    let cos_theta = 0.70710678f32;
    let expected = 0.02 + (1.0 - 0.02) * (1.0 - cos_theta).powf(5.0);
    assert!((fresnel - expected).abs() < 1e-6);
}

#[test]
fn test_fresnel_clamp() {
    let normal = Vec3::new(0.0, 0.0, 1.0);
    // Negative dot product - should clamp to 0.0 internally, making 1-0 = 1.0
    let view_dir = Vec3::new(0.0, 0.0, -1.0);
    let fresnel = calculate_water_fresnel(view_dir, normal, 0.02, 5.0);
    
    assert!((fresnel - 1.0).abs() < 1e-6);
}

#[test]
fn test_foam_deep_water() {
    let foam = calculate_water_foam(0.5, 0.3, 0.0);
    assert_eq!(foam, 0.0);
}

#[test]
fn test_foam_near_shore() {
    let foam = calculate_water_foam(0.1, 0.3, 0.0);
    
    // depth_factor = 1.0 - (0.1 / 0.3) = 0.6666...
    // time_fluctuation = sin(0) * 0.1 + 0.9 = 0.9
    // total = 0.6666... * 0.9 = 0.6
    assert!((foam - 0.6).abs() < 1e-6);
}

#[test]
fn test_foam_exactly_at_shore() {
    let foam = calculate_water_foam(0.0, 0.3, 0.0);
    // depth_factor = 1.0
    assert!((foam - 0.9).abs() < 1e-6);
}

#[test]
fn test_foam_with_time_fluctuation() {
    // time_sec = PI/4 -> sin(PI/2) = 1.0
    // time_fluctuation = 1.0 * 0.1 + 0.9 = 1.0
    let foam = calculate_water_foam(0.1, 0.3, std::f32::consts::PI / 4.0);
    // depth_factor = 0.666...
    assert!((foam - 0.6666667).abs() < 1e-5);
}

#[test]
fn test_foam_clamp() {
    let foam = calculate_water_foam(-0.1, 0.3, 0.0);
    // negative depth makes factor > 1.0 -> final > 1.0 -> clamps to 1.0
    // depth_factor = 1.333..., time_fluc = 0.9 -> 1.2 -> clamp 1.0
    assert!((foam - 1.0).abs() < 1e-6);
}
