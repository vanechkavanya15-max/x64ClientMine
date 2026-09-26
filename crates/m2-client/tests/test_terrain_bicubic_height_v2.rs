use m2_client::terrain_bicubic_height_v2::{
    get_terrain_v2_ctx, m2_terrain_get_height_and_normal, set_terrain_v2_context_for_test,
    TerrainBicubicHeightV2, CELLSCALE, HEIGHTMAP_RAW_SIZE, HEIGHTMAP_RAW_XSIZE, XSIZE, YSIZE,
};

fn create_test_map() -> Vec<u16> {
    let mut map = vec![10; HEIGHTMAP_RAW_SIZE]; // Default flat height 10

    // Set specific heights for interpolation testing
    // Point (10, 10) grid coords
    let get_index = |x: i32, y: i32| -> usize {
        ((y + 1) * HEIGHTMAP_RAW_XSIZE + x + 1) as usize
    };

    // Make a slope for normal vector testing
    map[get_index(9, 10)] = 20;  // left
    map[get_index(11, 10)] = 0;  // right
    map[get_index(10, 9)] = 20;  // up
    map[get_index(10, 11)] = 0;  // down

    // Make a triangle for interpolation testing at (20, 20)
    map[get_index(20, 20)] = 100;
    map[get_index(21, 20)] = 200;
    map[get_index(20, 21)] = 300;
    map[get_index(21, 21)] = 400;

    map
}

#[test]
fn test_instantiation() {
    let map = vec![0; HEIGHTMAP_RAW_SIZE];
    let terrain = TerrainBicubicHeightV2::new(map, 1.0, 0, 0);
    assert_eq!(terrain.get_height_map_value(0, 0), 0);
}

#[test]
#[should_panic(expected = "Invalid heightmap size")]
fn test_invalid_instantiation() {
    TerrainBicubicHeightV2::new(vec![0; 10], 1.0, 0, 0);
}

#[test]
fn test_point_exact_height_evaluation() {
    let map = create_test_map();
    let terrain = TerrainBicubicHeightV2::new(map, 2.0, 0, 0);

    // Grid (20, 20) => x = 20 * 200 = 4000
    let height = terrain.get_height(4000, 4000);
    assert_eq!(height, 100.0 * 2.0);
}

#[test]
fn test_left_triangle_interpolation() {
    let map = create_test_map();
    let terrain = TerrainBicubicHeightV2::new(map, 1.0, 0, 0);

    // xdist <= ydist (e.g. xdist=50, ydist=100)
    // x = 20 * 200 + 50 = 4050
    // y = 20 * 200 + 100 = 4100
    let height = terrain.get_height(4050, 4100);

    // Manual calculation based on Left triangle logic
    // h1 = 100, h2 = 400, h3 = 300
    // xslope = (400 - 300) / 200 = 0.5
    // yslope = (300 - 100) / 200 = 1.0
    // result = 100 + (50 * 0.5 + 100 * 1.0) = 100 + 25 + 100 = 225
    assert_eq!(height, 225.0);
}

#[test]
fn test_right_triangle_interpolation() {
    let map = create_test_map();
    let terrain = TerrainBicubicHeightV2::new(map, 1.0, 0, 0);

    // xdist > ydist (e.g. xdist=150, ydist=50)
    // x = 20 * 200 + 150 = 4150
    // y = 20 * 200 + 50 = 4050
    let height = terrain.get_height(4150, 4050);

    // Manual calculation based on Right triangle logic
    // h1 = 100, h2 = 400, h3 = 200
    // xslope = (200 - 100) / 200 = 0.5
    // yslope = (400 - 200) / 200 = 1.0
    // result = 100 + (150 * 0.5 + 50 * 1.0) = 100 + 75 + 50 = 225
    assert_eq!(height, 225.0);
}

#[test]
fn test_normal_vector_computation() {
    let map = create_test_map();
    let height_scale = 1.0;
    let terrain = TerrainBicubicHeightV2::new(map, height_scale, 0, 0);

    // Coordinate (10, 10) in grid
    let (nx, ny, nz) = terrain.get_normal_vector(10, 10);

    // Math:
    // nx = -1.0 * (20 - 0) = -20
    // ny = -1.0 * (20 - 0) = -20
    // nz = 2.0 * 200 = 400
    // length = sqrt((-20)^2 + (-20)^2 + 400^2) = sqrt(400 + 400 + 160000) = sqrt(160800) ~= 400.99875
    // scale = 127.0 / 400.99875 ~= 0.316709
    // nx *= scale = -6.334
    // ny *= scale = -6.334
    // nz *= scale = 126.68
    
    let ex_nx: f32 = -20.0;
    let ex_ny: f32 = -20.0;
    let ex_nz: f32 = 400.0;
    let len = (ex_nx * ex_nx + ex_ny * ex_ny + ex_nz * ex_nz).sqrt();
    let scale = 127.0 / len;
    
    assert!((nx - ex_nx * scale).abs() < 0.001);
    assert!((ny - ex_ny * scale).abs() < 0.001);
    assert!((nz - ex_nz * scale).abs() < 0.001);
}

#[test]
fn test_out_of_bounds_handling() {
    let map = vec![10; HEIGHTMAP_RAW_SIZE];
    let terrain = TerrainBicubicHeightV2::new(map, 1.0, 0, 0);

    // Negative coordinates
    assert_eq!(terrain.get_height(-10, -10), 0.0);

    // Coordinates larger than XSIZE * CELLSCALE (128 * 200 = 25600)
    assert_eq!(terrain.get_height(26000, 26000), 0.0);
}

#[test]
fn test_ffi_functions() {
    let map = create_test_map();
    let terrain = TerrainBicubicHeightV2::new(map, 1.0, 0, 0);
    
    set_terrain_v2_context_for_test(terrain);

    // At grid (20, 20) with left triangle offsets (50, 100) -> 225.0
    // Normal vector at grid (20, 20):
    // Left: 10, Right: 200, Up: 10, Down: 300
    // nx = -(10 - 200) = 190
    // ny = -(10 - 300) = 290
    // nz = 400
    
    let res = m2_terrain_get_height_and_normal(4050.0, 4100.0);
    
    assert_eq!(res.height, 225.0);

    let ex_nx: f32 = 190.0;
    let ex_ny: f32 = 290.0;
    let ex_nz: f32 = 400.0;
    let len = (ex_nx * ex_nx + ex_ny * ex_ny + ex_nz * ex_nz).sqrt();
    let scale = 127.0 / len;

    assert!((res.normal_x - ex_nx * scale).abs() < 0.001);
    assert!((res.normal_y - ex_ny * scale).abs() < 0.001);
    assert!((res.normal_z - ex_nz * scale).abs() < 0.001);
}
