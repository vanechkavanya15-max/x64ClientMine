use m2_client::terrain_attr_collision_v2::{
    m2_terrain_get_attr,
    m2_terrain_is_walkable_v2,
    m2_terrain_load_attr_grid,
    FLAG_BLOCKED,
    FLAG_WATER,
    FLAG_SAFE_ZONE,
    FLAG_CANT_BUILD,
    TERRAIN_SECTOR_SIZE,
    EXPECTED_GRID_LEN
};
use std::sync::{Mutex, OnceLock};

static TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn get_test_lock() -> &'static Mutex<()> {
    TEST_LOCK.get_or_init(|| Mutex::new(()))
}

#[test]
fn test_terrain_attr_collision_basic() {
    let _guard = get_test_lock().lock().unwrap();

    let mut grid = vec![0u8; EXPECTED_GRID_LEN];
    grid[0] = FLAG_BLOCKED;
    grid[1] = FLAG_WATER;
    grid[2] = FLAG_SAFE_ZONE;
    grid[3] = FLAG_CANT_BUILD;
    grid[127 * TERRAIN_SECTOR_SIZE + 127] = FLAG_BLOCKED | FLAG_SAFE_ZONE;

    assert!(m2_terrain_load_attr_grid(grid.as_ptr(), grid.len()));

    assert_eq!(m2_terrain_get_attr(0.0, 0.0), FLAG_BLOCKED);
    assert_eq!(m2_terrain_get_attr(1.0, 0.0), FLAG_WATER);
    assert_eq!(m2_terrain_get_attr(2.0, 0.0), FLAG_SAFE_ZONE);
    assert_eq!(m2_terrain_get_attr(3.0, 0.0), FLAG_CANT_BUILD);
    assert_eq!(m2_terrain_get_attr(127.0, 127.0), FLAG_BLOCKED | FLAG_SAFE_ZONE);
}

#[test]
fn test_terrain_attr_collision_out_of_bounds() {
    let _guard = get_test_lock().lock().unwrap();

    let grid = vec![0u8; EXPECTED_GRID_LEN];
    assert!(m2_terrain_load_attr_grid(grid.as_ptr(), grid.len()));

    assert_eq!(m2_terrain_get_attr(-1.0, 0.0), FLAG_BLOCKED);
    assert_eq!(m2_terrain_get_attr(0.0, -1.0), FLAG_BLOCKED);
    assert_eq!(m2_terrain_get_attr(128.0, 0.0), FLAG_BLOCKED);
    assert_eq!(m2_terrain_get_attr(0.0, 128.0), FLAG_BLOCKED);
}

#[test]
fn test_terrain_attr_is_walkable() {
    let _guard = get_test_lock().lock().unwrap();

    let mut grid = vec![0u8; EXPECTED_GRID_LEN];
    grid[0] = 0;
    grid[1] = FLAG_BLOCKED;
    grid[2] = FLAG_WATER;
    grid[3] = FLAG_SAFE_ZONE;
    grid[4] = FLAG_CANT_BUILD;
    grid[5] = FLAG_SAFE_ZONE | FLAG_CANT_BUILD;
    
    assert!(m2_terrain_load_attr_grid(grid.as_ptr(), grid.len()));

    assert_eq!(m2_terrain_is_walkable_v2(0.0, 0.0), true);
    assert_eq!(m2_terrain_is_walkable_v2(1.0, 0.0), false);
    assert_eq!(m2_terrain_is_walkable_v2(2.0, 0.0), false);
    assert_eq!(m2_terrain_is_walkable_v2(3.0, 0.0), true);
    assert_eq!(m2_terrain_is_walkable_v2(4.0, 0.0), true);
    assert_eq!(m2_terrain_is_walkable_v2(5.0, 0.0), true);
}

#[test]
fn test_terrain_attr_is_walkable_out_of_bounds() {
    let _guard = get_test_lock().lock().unwrap();

    let grid = vec![0u8; EXPECTED_GRID_LEN];
    assert!(m2_terrain_load_attr_grid(grid.as_ptr(), grid.len()));

    assert_eq!(m2_terrain_is_walkable_v2(-1.0, 0.0), false);
    assert_eq!(m2_terrain_is_walkable_v2(128.0, 0.0), false);
}

#[test]
fn test_terrain_attr_invalid_grid_size() {
    let _guard = get_test_lock().lock().unwrap();

    let grid = vec![0u8; EXPECTED_GRID_LEN - 1];
    assert!(!m2_terrain_load_attr_grid(grid.as_ptr(), grid.len()));
    
    let grid_large = vec![0u8; EXPECTED_GRID_LEN + 1];
    assert!(!m2_terrain_load_attr_grid(grid_large.as_ptr(), grid_large.len()));
}

#[test]
fn test_terrain_attr_fractional_coords() {
    let _guard = get_test_lock().lock().unwrap();

    let mut grid = vec![0u8; EXPECTED_GRID_LEN];
    grid[0] = 0;
    grid[1] = FLAG_BLOCKED;
    assert!(m2_terrain_load_attr_grid(grid.as_ptr(), grid.len()));

    assert_eq!(m2_terrain_get_attr(0.5, 0.5), 0);
    assert_eq!(m2_terrain_get_attr(1.9, 0.1), FLAG_BLOCKED);
    assert_eq!(m2_terrain_is_walkable_v2(0.5, 0.5), true);
    assert_eq!(m2_terrain_is_walkable_v2(1.9, 0.1), false);
}

#[test]
fn test_terrain_attr_null_ptr() {
    let _guard = get_test_lock().lock().unwrap();
    assert!(!m2_terrain_load_attr_grid(std::ptr::null(), EXPECTED_GRID_LEN));
}

#[test]
fn test_terrain_attr_all_combinations() {
    let _guard = get_test_lock().lock().unwrap();
    let mut grid = vec![0u8; EXPECTED_GRID_LEN];
    
    for i in 0..16 {
        grid[i] = i as u8;
    }
    
    assert!(m2_terrain_load_attr_grid(grid.as_ptr(), grid.len()));
    
    for i in 0..16 {
        let is_blocked = (i & FLAG_BLOCKED) != 0 || (i & FLAG_WATER) != 0;
        assert_eq!(m2_terrain_is_walkable_v2(i as f32, 0.0), !is_blocked);
        assert_eq!(m2_terrain_get_attr(i as f32, 0.0), i as u8);
    }
}
