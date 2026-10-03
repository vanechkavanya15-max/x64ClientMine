#[path = "../src/input_coordinate_scaler.rs"]
mod input_coordinate_scaler;

use input_coordinate_scaler::{InputCoordinateScaler, Resolution};

#[test]
fn test_default_instantiation_and_1_to_1_scaling() {
    let physical = Resolution { width: 800, height: 600 };
    let logical = Resolution { width: 800, height: 600 };
    let scaler = InputCoordinateScaler::new(physical, logical);

    // 1:1 test
    let (lx, ly) = scaler.scale_to_logical(100, 200);
    assert_eq!((lx, ly), (100, 200));

    let (px, py) = scaler.scale_to_physical(300, 400);
    assert_eq!((px, py), (300, 400));
}

#[test]
fn test_upscaling_physical_to_logical() {
    // Window is 800x600, game logic is 1920x1080
    let physical = Resolution { width: 800, height: 600 };
    let logical = Resolution { width: 1920, height: 1080 };
    let scaler = InputCoordinateScaler::new(physical, logical);

    let (lx, ly) = scaler.scale_to_logical(400, 300); // center of window
    assert_eq!((lx, ly), (960, 540)); // center of logical

    let (px, py) = scaler.scale_to_physical(960, 540); // center of logical
    assert_eq!((px, py), (400, 300)); // center of window
}

#[test]
fn test_downscaling_physical_to_logical() {
    // Window is 1920x1080, game logic is 800x600
    let physical = Resolution { width: 1920, height: 1080 };
    let logical = Resolution { width: 800, height: 600 };
    let scaler = InputCoordinateScaler::new(physical, logical);

    let (lx, ly) = scaler.scale_to_logical(960, 540); // center of window
    assert_eq!((lx, ly), (400, 300)); // center of logical

    let (px, py) = scaler.scale_to_physical(400, 300); // center of logical
    assert_eq!((px, py), (960, 540)); // center of window
}

#[test]
fn test_rounding_precision() {
    let physical = Resolution { width: 1366, height: 768 };
    let logical = Resolution { width: 1920, height: 1080 };
    let scaler = InputCoordinateScaler::new(physical, logical);

    // X: 136 * (1920 / 1366) = 191.156... -> 191
    // Y: 76 * (1080 / 768) = 106.875 -> 107
    let (lx, ly) = scaler.scale_to_logical(136, 76);
    assert_eq!((lx, ly), (191, 107));
}

#[test]
fn test_edge_case_zero_resolution() {
    let mut scaler = InputCoordinateScaler::new(
        Resolution { width: 0, height: 0 },
        Resolution { width: 800, height: 600 },
    );

    // When physical is 0x0, scale_to_logical should return input coordinates as fallback
    let (lx, ly) = scaler.scale_to_logical(100, 200);
    assert_eq!((lx, ly), (100, 200));

    scaler.set_physical_resolution(800, 600);
    scaler.set_logical_resolution(0, 0);

    // When logical is 0x0, scale_to_physical should return input coordinates as fallback
    let (px, py) = scaler.scale_to_physical(100, 200);
    assert_eq!((px, py), (100, 200));
}
