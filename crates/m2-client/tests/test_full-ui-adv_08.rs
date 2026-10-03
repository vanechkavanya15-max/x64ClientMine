#[path = "../src/bar3d_world_billboard.rs"]
mod bar3d_world_billboard;

use bar3d_world_billboard::{CharacterHpBar3D, D3dxColor};

#[test]
fn test_width_calculation_full_hp() {
    let bar = CharacterHpBar3D::new(100, 100, 50.0, 5.0);
    assert_eq!(bar.calculate_width(), 50.0);
}

#[test]
fn test_width_calculation_half_hp() {
    let bar = CharacterHpBar3D::new(50, 100, 50.0, 5.0);
    assert_eq!(bar.calculate_width(), 25.0);
}

#[test]
fn test_width_calculation_over_max_hp() {
    let bar = CharacterHpBar3D::new(150, 100, 50.0, 5.0);
    assert_eq!(bar.calculate_width(), 50.0); // Clamped at max_width
}

#[test]
fn test_width_calculation_negative_hp() {
    let bar = CharacterHpBar3D::new(-10, 100, 50.0, 5.0);
    assert_eq!(bar.calculate_width(), 0.0); // Clamped at 0
}

#[test]
fn test_width_calculation_zero_max_hp() {
    let bar = CharacterHpBar3D::new(50, 0, 50.0, 5.0);
    assert_eq!(bar.calculate_width(), 0.0); // Handled safely
}

#[test]
fn test_color_high_hp() {
    let bar = CharacterHpBar3D::new(80, 100, 50.0, 5.0);
    assert_eq!(bar.calculate_color(), D3dxColor::new(0.0, 1.0, 0.0, 1.0)); // Green
}

#[test]
fn test_color_medium_hp() {
    let bar = CharacterHpBar3D::new(40, 100, 50.0, 5.0);
    assert_eq!(bar.calculate_color(), D3dxColor::new(1.0, 1.0, 0.0, 1.0)); // Yellow
}

#[test]
fn test_color_low_hp() {
    let bar = CharacterHpBar3D::new(20, 100, 50.0, 5.0);
    assert_eq!(bar.calculate_color(), D3dxColor::new(1.0, 0.0, 0.0, 1.0)); // Red
}

#[test]
fn test_color_zero_hp() {
    let bar = CharacterHpBar3D::new(0, 100, 50.0, 5.0);
    assert_eq!(bar.calculate_color(), D3dxColor::new(1.0, 0.0, 0.0, 1.0)); // Red
}

#[test]
fn test_color_zero_max_hp() {
    let bar = CharacterHpBar3D::new(10, 0, 50.0, 5.0);
    assert_eq!(bar.calculate_color(), D3dxColor::new(1.0, 0.0, 0.0, 1.0)); // Red fallback
}

#[test]
fn test_project_world_to_screen_front() {
    // Identity view-projection matrix for simple test
    let vp_matrix: [f32; 16] = [
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ];

    let screen_w = 800.0;
    let screen_h = 600.0;

    let result = CharacterHpBar3D::project_world_to_screen(
        0.0, 0.0, 0.5,
        &vp_matrix,
        screen_w, screen_h
    );

    // ndc_x = 0, ndc_y = 0
    // screen_x = (0+1)/2 * 800 = 400
    // screen_y = (1-0)/2 * 600 = 300
    assert_eq!(result, Some((400.0, 300.0)));
}

#[test]
fn test_project_world_to_screen_behind() {
    // Modify vp_matrix so w becomes negative (behind camera)
    let vp_matrix: [f32; 16] = [
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 1.0, -1.0, // w = z * (-1) + 1.0 * w? No, vp[11] is -1.0
    ];

    // let clip_w = world_x * vp[3] + world_y * vp[7] + world_z * vp[11] + 1.0 * vp[15];
    // if vp[3]=0, vp[7]=0, vp[11]=-1.0, vp[15]=0
    // world_z = 2.0 -> clip_w = 2.0 * -1.0 + 1.0 * 0.0 = -2.0
    let mut vp_matrix2 = vp_matrix;
    vp_matrix2[15] = 0.0;

    let screen_w = 800.0;
    let screen_h = 600.0;

    let result = CharacterHpBar3D::project_world_to_screen(
        0.0, 0.0, 2.0,
        &vp_matrix2,
        screen_w, screen_h
    );

    assert_eq!(result, None);
}
