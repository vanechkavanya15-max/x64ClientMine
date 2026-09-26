#[path = "../src/terrain_height_map.rs"]
mod terrain_height_map;
use terrain_height_map::{TerrainHeightMap, CELL_SIZE};

#[test]
fn test_exact_grid_node_height() {
    let heights = vec![
        100, 200,
        300, 400
    ];
    let map = TerrainHeightMap::new(2, 2, heights);

    assert_eq!(map.get_height(0.0, 0.0), 100.0);
    assert_eq!(map.get_height(CELL_SIZE, 0.0), 200.0);
    assert_eq!(map.get_height(0.0, CELL_SIZE), 300.0);
    assert_eq!(map.get_height(CELL_SIZE, CELL_SIZE), 400.0);
}

#[test]
fn test_smooth_interpolation_center() {
    let heights = vec![
        100, 200,
        300, 400
    ];
    let map = TerrainHeightMap::new(2, 2, heights);

    let expected = (100.0 * 0.5 + 200.0 * 0.5) * 0.5 + (300.0 * 0.5 + 400.0 * 0.5) * 0.5;
    assert_eq!(map.get_height(CELL_SIZE * 0.5, CELL_SIZE * 0.5), expected);
}

#[test]
fn test_stability_on_edges() {
    let heights = vec![
        100, 200, 300,
        400, 500, 600,
        700, 800, 900
    ];
    let map = TerrainHeightMap::new(3, 3, heights);

    assert_eq!(map.get_height(CELL_SIZE * 1.5, 0.0), 250.0);
    assert_eq!(map.get_height(0.0, CELL_SIZE * 1.5), 550.0);
    assert_eq!(map.get_height(CELL_SIZE * 1.5, CELL_SIZE), 550.0);
    assert_eq!(map.get_height(CELL_SIZE, CELL_SIZE * 1.5), 650.0);
}

#[test]
fn test_out_of_bounds_clamping() {
    let heights = vec![
        10, 20,
        30, 40
    ];
    let map = TerrainHeightMap::new(2, 2, heights);

    assert_eq!(map.get_height(-50.0, -50.0), 10.0);
    
    assert_eq!(map.get_height(-50.0, 50.0), 15.0); 
    
    assert_eq!(map.get_height(CELL_SIZE * 2.0, CELL_SIZE * 2.0), 40.0);
    assert_eq!(map.get_height(CELL_SIZE * 1.5, CELL_SIZE * 2.0), 40.0);
}

#[test]
fn test_complex_interpolation() {
    let heights = vec![
        10, 50,
        100, 150
    ];
    let map = TerrainHeightMap::new(2, 2, heights);

    let _rx = 0.1;
    let _ry = 0.8;
    
    let h0 = 10.0 * 0.9 + 50.0 * 0.1;
    let h1 = 100.0 * 0.9 + 150.0 * 0.1;
    let _expected = h0 * 0.2 + h1 * 0.8;

    let result = map.get_height(20.0, 160.0);
    assert!((result - 86.8).abs() < f32::EPSILON, "Expected 86.8, got {}", result);
}
