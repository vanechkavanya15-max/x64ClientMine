#[path = "../src/actor_terrain_snapper_v2.rs"]
mod actor_terrain_snapper_v2;

use actor_terrain_snapper_v2::{
    adjust_to_terrain_height, TerrainHeightData, CELLSCALE, XSIZE, YSIZE,
};

fn create_flat_terrain(height_scale: f32) -> TerrainHeightData {
    let mut raw_heights = [100u16; (XSIZE + 1) * (YSIZE + 1)];
    TerrainHeightData::new(raw_heights, height_scale, 0, 0)
}

fn create_sloped_terrain(height_scale: f32) -> TerrainHeightData {
    let mut raw_heights = [0u16; (XSIZE + 1) * (YSIZE + 1)];
    for y in 0..=(YSIZE) {
        for x in 0..=(XSIZE) {
            // Slope upwards along X axis
            raw_heights[y * (XSIZE + 1) + x] = (x * 10) as u16;
        }
    }
    TerrainHeightData::new(raw_heights, height_scale, 0, 0)
}

#[test]
fn test_flat_terrain_height() {
    let terrain = create_flat_terrain(1.0);
    let height = terrain.get_height_bilinear(500.0, 500.0);
    assert_eq!(height, 100.0);
}

#[test]
fn test_flat_terrain_snapper() {
    let terrain = create_flat_terrain(1.0);

    // Actor facing purely along X axis
    let result = adjust_to_terrain_height(500.0, 500.0, 1.0, 0.0, &terrain);

    assert_eq!(result.z_height, 100.0);
    assert_eq!(result.pitch, 0.0);
    assert_eq!(result.roll, 0.0);
    assert_eq!(result.normal, (0.0, 0.0, 1.0));
}

#[test]
fn test_sloped_terrain_height_interpolation() {
    let terrain = create_sloped_terrain(1.0);

    // Each X cell step increases raw_height by 10.
    // At x=0 (fx=0.0), height = 0
    // At x=1 (fx=200.0), height = 10
    // At fx=100.0 (halfway), height should be 5.0
    let height = terrain.get_height_bilinear(100.0, 0.0);
    assert!((height - 5.0).abs() < 0.001);
}

#[test]
fn test_sloped_terrain_snapper_pitch() {
    let terrain = create_sloped_terrain(1.0);

    // Facing up the slope (X axis). Terrain goes up by 10 per cell.
    // normal is calculated using h_left - h_right
    // h_left = (1 * 10), h_right = (3 * 10) for x=2
    // normal.x = -1.0 * (10.0 - 30.0) = 20.0
    // So normal.0 is positive!

    let result = adjust_to_terrain_height(300.0, 300.0, 1.0, 0.0, &terrain);

    // It's sloped up along X, but C++ uses -scale * (left - right).
    // If left is lower than right, left - right is negative, so -scale * negative is positive.
    assert!(result.normal.0 > 0.0);
    assert_eq!(result.normal.1, 0.0);
    assert!(result.normal.2 > 0.0);

    // Our normal.x is positive, forward_slope = -normal.0 * dir_x = negative
    // pitch = forward_slope.atan2(...) which is negative.
    // In many engine conventions, a negative pitch means pitching up / facing uphill!
    assert!(result.pitch < 0.0);
    assert_eq!(result.roll, 0.0);
}

#[test]
fn test_sloped_terrain_snapper_roll() {
    let terrain = create_sloped_terrain(1.0);

    // Facing across the slope (Y axis)
    let result = adjust_to_terrain_height(300.0, 300.0, 0.0, 1.0, &terrain);

    // Facing Y, slope is to our right (along X). So roll should be non-zero.
    assert_eq!(result.pitch, 0.0); // No slope in our forward direction
    assert!(result.roll != 0.0); // Roll exists due to sideways slope
}

#[test]
fn test_out_of_bounds_clamping() {
    let terrain = create_flat_terrain(1.0);

    // Try way out of bounds
    let height_negative = terrain.get_height_bilinear(-100.0, -100.0);
    assert_eq!(height_negative, 0.0);

    let large_coord = (XSIZE * CELLSCALE * 2) as f32;
    let height_positive = terrain.get_height_bilinear(large_coord, large_coord);
    assert_eq!(height_positive, 0.0);
}
