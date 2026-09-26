#[path = "../src/sound_spatial_3d.rs"]
mod sound_spatial_3d;

use sound_spatial_3d::calculate_spatial_sound;

#[test]
fn test_same_position() {
    let result = calculate_spatial_sound(
        100.0, 200.0, 300.0,
        100.0, 200.0, 300.0,
        0.0
    );
    
    assert_eq!(result.is_audible, true);
    assert_eq!(result.effective_volume, 1.0);
    assert_eq!(result.pan, 0.0);
}

#[test]
fn test_out_of_limit() {
    let result = calculate_spatial_sound(
        0.0, 5001.0, 0.0,
        0.0, 0.0, 0.0,
        0.0
    );
    
    assert_eq!(result.is_audible, false);
    assert_eq!(result.effective_volume, 0.0);
    assert_eq!(result.pan, 0.0);
}

#[test]
fn test_exact_limit() {
    let result = calculate_spatial_sound(
        5000.0, 0.0, 0.0,
        0.0, 0.0, 0.0,
        0.0
    );
    
    assert_eq!(result.is_audible, true);
    assert_eq!(result.effective_volume, 0.0); // 1.0 - (5000/5000) = 0.0
}

#[test]
fn test_exact_left() {
    // Sound is at (-100, 0, 0), listener at (0, 0, 0), yaw = 0 (looking at +Y typically in M2, but let's check math)
    // dx = -100, dy = 0, dz = 0
    // rel_x = -100 * (-sin(0)) + 0 * cos(0) = 0? Wait, the formula is:
    // rel_x = dx * (-sin(yaw)) + dy * (cos(yaw))
    // If yaw = 0: rel_x = dy.
    // If we want it to be exactly left or right, we need to understand the relative coordinate system.
    // If we want pan = -1.0, rel_x must be -100. So dy = -100.
    
    // Let's test with dy = -100 (which gives rel_x = -100 when yaw = 0)
    let result = calculate_spatial_sound(
        0.0, -100.0, 0.0,
        0.0, 0.0, 0.0,
        0.0
    );
    
    assert_eq!(result.is_audible, true);
    assert_eq!(result.pan, -1.0);
}

#[test]
fn test_exact_right() {
    // For pan = +1.0, rel_x must be 100. So dy = 100 when yaw = 0.
    let result = calculate_spatial_sound(
        0.0, 100.0, 0.0,
        0.0, 0.0, 0.0,
        0.0
    );
    
    assert_eq!(result.is_audible, true);
    assert_eq!(result.pan, 1.0);
}

#[test]
fn test_yaw_rotation() {
    // If yaw is PI/2 (90 degrees). sin(PI/2) = 1, cos(PI/2) = 0
    // rel_x = dx * (-1) + dy * 0 = -dx
    // If dx = 100, rel_x = -100, pan = -1.0
    
    let result = calculate_spatial_sound(
        100.0, 0.0, 0.0,
        0.0, 0.0, 0.0,
        std::f32::consts::FRAC_PI_2
    );
    
    assert_eq!(result.is_audible, true);
    // There might be some floating point inaccuracy with sin/cos
    assert!((result.pan - (-1.0)).abs() < 1e-5);
}

#[test]
fn test_clamping() {
    // Let's test if pan clamps properly, though by definition rel_x/dist can't exceed 1.0 since rel_x is a projection of dist onto a 2D plane.
    // But we test the max(1.0) for distance.
    
    let result = calculate_spatial_sound(
        0.0, 0.5, 0.0,
        0.0, 0.0, 0.0,
        0.0
    );
    
    // dist is 0.5. dist.max(1.0) is 1.0.
    // rel_x is 0.5.
    // pan is 0.5 / 1.0 = 0.5.
    
    assert_eq!(result.is_audible, true);
    assert_eq!(result.pan, 0.5);
    
    // Another sub-1 distance test
    let result2 = calculate_spatial_sound(
        0.0, 0.0, 0.0,
        0.0, 0.0, 0.0,
        0.0
    );
    assert_eq!(result2.pan, 0.0);
}
