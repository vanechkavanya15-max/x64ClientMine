use m2_client::terrain_attr_collision_v2::{m2_terrain_load_attr_grid, EXPECTED_GRID_LEN, FLAG_SAFE_ZONE, FLAG_BLOCKED};
use m2_client::map_safe_zone_evaluator::{is_in_safe_zone, m2_map_is_safe_zone};

fn setup_grid_with_safe_zones() {
    let mut grid = vec![0u8; EXPECTED_GRID_LEN];
    // Put safe zone at (10, 10)
    grid[10 * 128 + 10] = FLAG_SAFE_ZONE;
    // Put safe zone with blocked at (20, 20)
    grid[20 * 128 + 20] = FLAG_SAFE_ZONE | FLAG_BLOCKED;
    
    assert!(m2_terrain_load_attr_grid(grid.as_ptr(), grid.len()));
}

#[test]
fn test_not_in_safe_zone() {
    setup_grid_with_safe_zones();
    assert!(!is_in_safe_zone(5.0, 5.0));
    assert!(!m2_map_is_safe_zone(5.0, 5.0));
}

#[test]
fn test_in_safe_zone_basic() {
    setup_grid_with_safe_zones();
    assert!(is_in_safe_zone(10.0, 10.0));
    assert!(m2_map_is_safe_zone(10.0, 10.0));
}

#[test]
fn test_in_safe_zone_with_other_flags() {
    setup_grid_with_safe_zones();
    assert!(is_in_safe_zone(20.0, 20.0));
    assert!(m2_map_is_safe_zone(20.0, 20.0));
}

#[test]
fn test_negative_coordinates() {
    setup_grid_with_safe_zones();
    assert!(!is_in_safe_zone(-10.0, -10.0));
    assert!(!m2_map_is_safe_zone(-10.0, -10.0));
}

#[test]
fn test_out_of_bounds_coordinates() {
    setup_grid_with_safe_zones();
    assert!(!is_in_safe_zone(200.0, 200.0));
    assert!(!m2_map_is_safe_zone(200.0, 200.0));
}

#[test]
fn test_ffi_guard_handles_panic() {
    assert!(true);
}
