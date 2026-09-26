#[path = "../src/sound_logarithmic_volume.rs"]
mod sound_logarithmic_volume;

use sound_logarithmic_volume::*;

#[test]
fn test_ratio_to_apply_volume_zero() {
    assert_eq!(ratio_to_apply_volume(0.0), 0.0);
    assert_eq!(ratio_to_apply_volume(-1.0), 0.0); // Negative value should return 0.0
}

#[test]
fn test_ratio_to_apply_volume_small_values() {
    assert_eq!(ratio_to_apply_volume(0.05), 0.05);
    assert_eq!(ratio_to_apply_volume(0.099), 0.099);
}

#[test]
fn test_ratio_to_apply_volume_logarithmic() {
    // Expected: 10^(-1 + 0.5) = 10^(-0.5) = 0.31622776601683794
    let result = ratio_to_apply_volume(0.5);
    assert!((result - 0.31622776).abs() < 1e-6);
}

#[test]
fn test_ratio_to_apply_volume_full() {
    // Expected: 10^(-1 + 1.0) = 10^0 = 1.0
    assert_eq!(ratio_to_apply_volume(1.0), 1.0);
}

#[test]
fn test_ratio_to_apply_volume_greater_than_one() {
    // Expected: 10^(-1 + 2.0) = 10^1 = 10.0
    let result = ratio_to_apply_volume(2.0);
    assert!((result - 10.0).abs() < 1e-6);
}

#[test]
fn test_grade_to_apply_volume_zero() {
    // Grade 0 -> Ratio 0.0 -> Volume 0.0
    assert_eq!(grade_to_apply_volume(0), 0.0);
}

#[test]
fn test_grade_to_apply_volume_negative() {
    // Grade -1 -> Ratio -0.2 -> Volume 0.0
    assert_eq!(grade_to_apply_volume(-1), 0.0);
}

#[test]
fn test_grade_to_apply_volume_five() {
    // Grade 5 -> Ratio 1.0 -> Volume 1.0
    assert_eq!(grade_to_apply_volume(5), 1.0);
}

#[test]
fn test_grade_to_apply_volume_mid() {
    // Grade 3 -> Ratio 0.6 -> Volume 10^(-0.4) = 0.39810717
    let result = grade_to_apply_volume(3);
    assert!((result - 0.39810717).abs() < 1e-6);
}

#[test]
fn test_grade_to_apply_volume_greater_than_five() {
    // Grade 10 -> Ratio 2.0 -> Volume 10.0
    let result = grade_to_apply_volume(10);
    assert!((result - 10.0).abs() < 1e-6);
}
