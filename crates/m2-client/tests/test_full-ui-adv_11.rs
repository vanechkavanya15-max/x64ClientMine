#[path = "../src/ui_resolution_scale_matrix.rs"]
mod ui_resolution_scale_matrix;

use glam::{Mat4, Vec3, Vec4};
use ui_resolution_scale_matrix::{calculate_ui_scale_matrix, UiScaleMode};

const EPSILON: f32 = 1e-4;

fn assert_mat4_eq(a: Mat4, b: Mat4) {
    let diff = a - b;
    for col in diff.to_cols_array() {
        assert!(col.abs() < EPSILON, "Matrices not equal:\nLeft:\n{:?}\nRight:\n{:?}", a, b);
    }
}

#[test]
fn test_zero_dimensions() {
    // Edge case: zero dimensions should return identity matrix
    let mat = calculate_ui_scale_matrix(0, 600, 800, 600, UiScaleMode::Stretch);
    assert_mat4_eq(mat, Mat4::IDENTITY);

    let mat = calculate_ui_scale_matrix(800, 0, 800, 600, UiScaleMode::Fit);
    assert_mat4_eq(mat, Mat4::IDENTITY);
}

#[test]
fn test_stretch_mode() {
    // 4:3 base (800x600) stretched to 16:9 (1920x1080)
    let mat = calculate_ui_scale_matrix(1920, 1080, 800, 600, UiScaleMode::Stretch);
    
    // Scale X = 1920 / 800 = 2.4
    // Scale Y = 1080 / 600 = 1.8
    // No translation since it fills the screen perfectly
    let expected = Mat4::from_scale_rotation_translation(
        Vec3::new(2.4, 1.8, 1.0),
        glam::Quat::IDENTITY,
        Vec3::ZERO,
    );
    assert_mat4_eq(mat, expected);

    // Transforming point (800, 600) should give (1920, 1080)
    let p = mat * Vec4::new(800.0, 600.0, 0.0, 1.0);
    assert!((p.x - 1920.0).abs() < EPSILON);
    assert!((p.y - 1080.0).abs() < EPSILON);
}

#[test]
fn test_fit_mode_wider_screen() {
    // 4:3 base (800x600) fit into 16:9 (1920x1080)
    // Screen is wider, so it fits to height.
    // Scale = 1080 / 600 = 1.8
    // Scaled width = 800 * 1.8 = 1440
    // Offset X = (1920 - 1440) / 2 = 240
    // Offset Y = 0
    let mat = calculate_ui_scale_matrix(1920, 1080, 800, 600, UiScaleMode::Fit);
    
    let expected = Mat4::from_scale_rotation_translation(
        Vec3::new(1.8, 1.8, 1.0),
        glam::Quat::IDENTITY,
        Vec3::new(240.0, 0.0, 0.0),
    );
    assert_mat4_eq(mat, expected);

    // Transforming (0, 0) should give (240, 0)
    let p0 = mat * Vec4::new(0.0, 0.0, 0.0, 1.0);
    assert!((p0.x - 240.0).abs() < EPSILON);
    assert!((p0.y - 0.0).abs() < EPSILON);
}

#[test]
fn test_fit_mode_taller_screen() {
    // 16:9 base (1920x1080) fit into 4:3 (800x600)
    // Screen is taller (proportionally narrower width), so it fits to width.
    // Scale = 800 / 1920 ≈ 0.416666
    // Scaled height = 1080 * (800 / 1920) = 450
    // Offset Y = (600 - 450) / 2 = 75
    // Offset X = 0
    let mat = calculate_ui_scale_matrix(800, 600, 1920, 1080, UiScaleMode::Fit);
    
    let scale = 800.0 / 1920.0;
    let expected = Mat4::from_scale_rotation_translation(
        Vec3::new(scale, scale, 1.0),
        glam::Quat::IDENTITY,
        Vec3::new(0.0, 75.0, 0.0),
    );
    assert_mat4_eq(mat, expected);
}

#[test]
fn test_fill_mode_wider_screen() {
    // 4:3 base (800x600) fill into 21:9 Ultrawide (2560x1080)
    // Screen is wider, so it fills width.
    // Scale = 2560 / 800 = 3.2
    // Scaled height = 600 * 3.2 = 1920
    // Offset X = 0
    // Offset Y = (1080 - 1920) / 2 = -420
    let mat = calculate_ui_scale_matrix(2560, 1080, 800, 600, UiScaleMode::Fill);
    
    let expected = Mat4::from_scale_rotation_translation(
        Vec3::new(3.2, 3.2, 1.0),
        glam::Quat::IDENTITY,
        Vec3::new(0.0, -420.0, 0.0),
    );
    assert_mat4_eq(mat, expected);
}

#[test]
fn test_fill_mode_taller_screen() {
    // 16:9 base (1920x1080) fill into 4:3 (1024x768)
    // Screen is taller, so it fills height.
    // Scale = 768 / 1080 ≈ 0.711111
    // Scaled width = 1920 * (768 / 1080) = 1365.333333
    // Offset X = (1024 - 1365.333333) / 2 = -170.666666
    // Offset Y = 0
    let mat = calculate_ui_scale_matrix(1024, 768, 1920, 1080, UiScaleMode::Fill);
    
    let scale = 768.0 / 1080.0;
    let expected = Mat4::from_scale_rotation_translation(
        Vec3::new(scale, scale, 1.0),
        glam::Quat::IDENTITY,
        Vec3::new((1024.0 - (1920.0 * scale)) / 2.0, 0.0, 0.0),
    );
    assert_mat4_eq(mat, expected);
}
