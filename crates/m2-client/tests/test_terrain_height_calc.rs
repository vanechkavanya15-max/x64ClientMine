use m2_client::terrain_height_calc::{
    RawHeightMap, TerrainHeightCalculator, HEIGHTMAP_RAW_SIZE, HEIGHTMAP_RAW_XSIZE,
};

fn create_test_map() -> RawHeightMap {
    let mut data = [0u16; HEIGHTMAP_RAW_SIZE];
    for y in 0..131 {
        for x in 0..131 {
            // Set a gradient-like value so we can check interpolation easily
            // e.g. x + y
            data[y * HEIGHTMAP_RAW_XSIZE + x] = (x + y) as u16;
        }
    }
    RawHeightMap::new(data)
}

#[test]
fn test_exact_vertex() {
    let map = create_test_map();
    let calc = TerrainHeightCalculator::new(map, 1.0);

    // Coordinate (0, 0) corresponds to index (1, 1) due to +1 padding.
    // At index (1, 1), the value is 1 + 1 = 2.
    assert_eq!(calc.get_height(0.0, 0.0), 2.0);

    // Coordinate (5, 10) corresponds to index (6, 11) -> 17.
    assert_eq!(calc.get_height(5.0, 10.0), 17.0);
}

#[test]
fn test_interpolation_middle() {
    let map = create_test_map();
    let calc = TerrainHeightCalculator::new(map, 1.0);

    // Coordinate (0.5, 0.5)
    // h00 (1,1) = 2.0
    // h10 (2,1) = 3.0
    // h01 (1,2) = 3.0
    // h11 (2,2) = 4.0
    // tx = 0.5, ty = 0.5
    // h_bottom = 0.5 * 2.0 + 0.5 * 3.0 = 2.5
    // h_top = 0.5 * 3.0 + 0.5 * 4.0 = 3.5
    // final_height = 0.5 * 2.5 + 0.5 * 3.5 = 3.0
    assert_eq!(calc.get_height(0.5, 0.5), 3.0);
}

#[test]
fn test_interpolation_edges() {
    let map = create_test_map();
    let calc = TerrainHeightCalculator::new(map, 2.0); // Test with height scale

    // Edge on tx (x = 0.75, y = 0.0) -> y is integer
    // h00 (1,1) = 2.0
    // h10 (2,1) = 3.0
    // tx = 0.75, ty = 0.0
    // final_height = 0.25 * 2.0 + 0.75 * 3.0 = 2.75
    // with scale 2.0 -> 5.5
    assert_eq!(calc.get_height(0.75, 0.0), 5.5);

    // Edge on ty (x = 0.0, y = 0.25) -> x is integer
    // h00 (1,1) = 2.0
    // h01 (1,2) = 3.0
    // tx = 0.0, ty = 0.25
    // final_height = 0.75 * 2.0 + 0.25 * 3.0 = 2.25
    // with scale 2.0 -> 4.5
    assert_eq!(calc.get_height(0.0, 0.25), 4.5);
}

#[test]
fn test_clamping() {
    let map = create_test_map();
    let calc = TerrainHeightCalculator::new(map, 1.0);

    // Negative coordinates should clamp to (0.0, 0.0) -> index (1, 1) -> 2.0
    assert_eq!(calc.get_height(-10.0, -5.0), 2.0);

    // Coordinates > 128.0 should clamp to (128.0, 128.0) -> index (129, 129) -> 258.0
    assert_eq!(calc.get_height(130.0, 200.0), 258.0);
}
