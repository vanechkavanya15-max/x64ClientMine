use m2_client::terrain_ffi::*;
use std::ptr;
use std::sync::{Mutex, OnceLock};

static TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn lock_tests() -> std::sync::MutexGuard<'static, ()> {
    TEST_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
}

#[test]
fn test_ffi_null_pointers() {
    let _guard = lock_tests();
    assert_eq!(m2_terrain_load_raw_heightmap(ptr::null(), 100), false);
    assert_eq!(m2_terrain_load_attrmap(ptr::null(), 100), false);
    assert_eq!(m2_terrain_load_watermap(ptr::null(), 100), false);
    
    let mut hit_x = 0.0;
    let mut hit_y = 0.0;
    assert_eq!(m2_terrain_raycast_check(0.0, 0.0, 10.0, 10.0, ptr::null_mut(), &mut hit_y), false);
    assert_eq!(m2_terrain_raycast_check(0.0, 0.0, 10.0, 10.0, &mut hit_x, ptr::null_mut()), false);
}

#[test]
fn test_ffi_load_heightmap() {
    let _guard = lock_tests();
    // A valid heightmap size is 131 * 131 * 2 = 34322 bytes
    let valid_size = 131 * 131 * 2;
    let mut data = vec![0u8; valid_size];
    
    // Write a specific height value at (0, 0)
    // 1000 in u16
    let height_val: u16 = 1000;
    let bytes = height_val.to_ne_bytes();
    data[0] = bytes[0];
    data[1] = bytes[1];

    assert_eq!(m2_terrain_load_raw_heightmap(data.as_ptr(), valid_size), true);
    
    // Test height query, at 0.0, 0.0 it should give 1000.0 (interpolation might affect it, but exactly at 0,0 it's exact)
    let height = m2_terrain_get_height(0.0, 0.0);
    assert_eq!(height, 1000.0);

    // out of bounds or invalid sizes
    assert_eq!(m2_terrain_load_raw_heightmap(data.as_ptr(), 100), false);
}

#[test]
fn test_ffi_load_attrmap() {
    let _guard = lock_tests();
    // Let's load a 2x2 map (4 bytes)
    let size = 4;
    let mut data = vec![0u8; size];
    // 0 = walkable, 1 = blocked (ATTR_BLOCK)
    // ATTR_BLOCK is 1 << 0
    data[0] = 0; // walkable
    data[1] = 1; // blocked
    data[2] = 1; // blocked
    data[3] = 0; // walkable

    assert_eq!(m2_terrain_load_attrmap(data.as_ptr(), size), true);

    // Now query
    assert_eq!(m2_terrain_is_blocked(0, 0), false);
    assert_eq!(m2_terrain_is_blocked(1, 0), true);
    assert_eq!(m2_terrain_is_blocked(0, 1), true);
    assert_eq!(m2_terrain_is_blocked(1, 1), false);

    // Out of bounds defaults to blocked
    assert_eq!(m2_terrain_is_blocked(5, 5), true);
    
    // Invalid load
    assert_eq!(m2_terrain_load_attrmap(data.as_ptr(), 3), false);
}

#[test]
fn test_ffi_load_watermap() {
    let _guard = lock_tests();
    let size = 4; // 2x2
    let mut data = vec![0u8; size];
    data[0] = 0; // no water
    data[1] = 255; // water
    
    assert_eq!(m2_terrain_load_watermap(data.as_ptr(), size), true);

    assert_eq!(m2_terrain_is_water(0, 0), false);
    assert_eq!(m2_terrain_is_water(1, 0), true);
    assert_eq!(m2_terrain_is_water(0, 1), false);
    
    // Out of bounds -> false
    assert_eq!(m2_terrain_is_water(5, 5), false);
    
    // invalid load
    assert_eq!(m2_terrain_load_watermap(data.as_ptr(), 3), false);
}

#[test]
fn test_ffi_raycast() {
    let _guard = lock_tests();
    // Load a 3x3 attrmap
    let size = 9;
    let mut data = vec![0u8; size];
    // map:
    // W W B
    // W W W
    // W W W
    // (where W=0 walkable, B=1 blocked)
    // Coords: 0,0 is top left.
    // 2,0 is blocked.
    data[2] = 1; 

    assert_eq!(m2_terrain_load_attrmap(data.as_ptr(), size), true);

    let mut hit_x = 0.0;
    let mut hit_y = 0.0;

    // Raycast from 0.5, 0.5 to 2.5, 0.5 (moves purely in X axis)
    // Should hit the block at x=2
    let hit = m2_terrain_raycast_check(0.5, 0.5, 2.5, 0.5, &mut hit_x, &mut hit_y);
    assert_eq!(hit, true);
    assert_eq!(hit_x, 2.0); // Entering the cell x=2
    assert_eq!(hit_y, 0.5); // Y is unchanged

    // Raycast from 0.5, 0.5 to 0.5, 2.5 (Pure Y axis)
    let hit = m2_terrain_raycast_check(0.5, 0.5, 0.5, 2.5, &mut hit_x, &mut hit_y);
    // no block in that path
    assert_eq!(hit, false);
    
    // Raycast starting in a block
    let hit = m2_terrain_raycast_check(2.5, 0.5, 2.5, 0.5, &mut hit_x, &mut hit_y);
    assert_eq!(hit, true);
    assert_eq!(hit_x, 2.5);
    assert_eq!(hit_y, 0.5);
    
    // Raycast hitting out of bounds
    let hit = m2_terrain_raycast_check(1.5, 0.5, 10.5, 0.5, &mut hit_x, &mut hit_y);
    assert_eq!(hit, true);
    // Should hit x=2.0 (since x=2, y=0 is blocked) or x=3.0 (out of bounds)
    assert!(hit_x == 2.0 || hit_x == 3.0);
}
