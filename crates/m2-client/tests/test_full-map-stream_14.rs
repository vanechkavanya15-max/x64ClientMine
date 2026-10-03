#[path = "../src/map_coordinate_transformer.rs"]
mod map_coordinate_transformer;

use map_coordinate_transformer::*;

#[test]
fn test_cm_to_meters() {
    assert_eq!(cm_to_meters(100.0), 1.0);
    assert_eq!(cm_to_meters(250.0), 2.5);
    assert_eq!(cm_to_meters(-50.0), -0.5);
    assert_eq!(cm_to_meters(0.0), 0.0);
}

#[test]
fn test_meters_to_cm() {
    assert_eq!(meters_to_cm(1.0), 100.0);
    assert_eq!(meters_to_cm(2.5), 250.0);
    assert_eq!(meters_to_cm(-0.5), -50.0);
    assert_eq!(meters_to_cm(0.0), 0.0);
}

#[test]
fn test_cm_to_cell() {
    assert_eq!(cm_to_cell(0.0), 0);
    assert_eq!(cm_to_cell(199.9), 0);
    assert_eq!(cm_to_cell(200.0), 1);
    assert_eq!(cm_to_cell(400.0), 2);
    assert_eq!(cm_to_cell(-1.0), -1);
    assert_eq!(cm_to_cell(-200.0), -1); // -200 / 200 = -1
    assert_eq!(cm_to_cell(-200.1), -2); // floor(-1.0005) = -2
}

#[test]
fn test_cell_to_cm() {
    assert_eq!(cell_to_cm(0), 0.0);
    assert_eq!(cell_to_cm(1), 200.0);
    assert_eq!(cell_to_cm(2), 400.0);
    assert_eq!(cell_to_cm(-1), -200.0);
}

#[test]
fn test_meters_to_cell() {
    assert_eq!(meters_to_cell(0.0), 0);
    assert_eq!(meters_to_cell(1.99), 0);
    assert_eq!(meters_to_cell(2.0), 1);
    assert_eq!(meters_to_cell(4.0), 2);
    assert_eq!(meters_to_cell(-0.01), -1);
    assert_eq!(meters_to_cell(-2.0), -1);
    assert_eq!(meters_to_cell(-2.01), -2);
}

#[test]
fn test_cell_to_meters() {
    assert_eq!(cell_to_meters(0), 0.0);
    assert_eq!(cell_to_meters(1), 2.0);
    assert_eq!(cell_to_meters(2), 4.0);
    assert_eq!(cell_to_meters(-1), -2.0);
}
