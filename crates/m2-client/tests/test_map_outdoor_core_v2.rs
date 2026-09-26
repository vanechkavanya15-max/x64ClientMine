use m2_client::map_outdoor_core_v2::*;

#[test]
fn test_get_terrain_num_from_coord() {
    let mut core = MapOutdoorCore::new();
    core.cur_coordinate = OutdoorMapCoordinate {
        terrain_coord_x: 10,
        terrain_coord_y: 10,
    };

    // Center point (10, 10) -> index 4
    assert_eq!(core.get_terrain_num_from_coord(10, 10), Some(4));

    // Top-left point (9, 9) -> index 0
    assert_eq!(core.get_terrain_num_from_coord(9, 9), Some(0));

    // Bottom-right point (11, 11) -> index 8
    assert_eq!(core.get_terrain_num_from_coord(11, 11), Some(8));

    // Out of bounds cases
    assert_eq!(core.get_terrain_num_from_coord(8, 10), None);
    assert_eq!(core.get_terrain_num_from_coord(12, 10), None);
    assert_eq!(core.get_terrain_num_from_coord(10, 8), None);
    assert_eq!(core.get_terrain_num_from_coord(10, 12), None);
}

#[test]
fn test_convert_to_map_coords() {
    let fx = 51200.0;
    let fy = 51200.0;

    let (cell_x, cell_y, sub_x, sub_y, tx, ty) = MapOutdoorCore::convert_to_map_coords(fx, fy);
    
    // terrain_num_x = 51200 / 25600 = 2
    assert_eq!(tx, 2);
    // terrain_num_y = 51200 / 25600 = 2
    assert_eq!(ty, 2);

    // fx after mod = 0, fy after mod = 0
    assert_eq!(cell_x, 0);
    assert_eq!(cell_y, 0);
    assert_eq!(sub_x, 0);
    assert_eq!(sub_y, 0);
}

#[test]
fn test_convert_to_map_coords_with_remainder() {
    let fx = 25650.0; // Terrain 1, localized fx = 50.0
    let fy = 25750.0; // Terrain 1, localized fy = 150.0

    let (cell_x, cell_y, sub_x, sub_y, tx, ty) = MapOutdoorCore::convert_to_map_coords(fx, fy);
    
    assert_eq!(tx, 1);
    assert_eq!(ty, 1);

    // localized fx = 50.0. cell_x = 50.0 / 100.0 = 0.5 -> int = 0
    // localized fy = 150.0. cell_y = 150.0 / 100.0 = 1.5 -> int = 1
    assert_eq!(cell_x, 0);
    assert_eq!(cell_y, 1);

    // subcell_x = 50.0 * (2 / 100) = 1.0 -> int = 1. 1 % 2 = 1
    // subcell_y = 150.0 * (2 / 100) = 3.0 -> int = 3. 3 % 2 = 1
    assert_eq!(sub_x, 1);
    assert_eq!(sub_y, 1);
}

#[test]
fn test_convert_to_map_coords_negative() {
    let fx = -25600.0;
    let fy = -25600.0;
    
    let (cell_x, cell_y, sub_x, sub_y, tx, ty) = MapOutdoorCore::convert_to_map_coords(fx, fy);
    // When fy < 0, fy = -fy = 25600.0
    // ix = -25600, iy = 25600
    // tx = -25600 / 25600 = 0 (unsigned integer conversion from signed) Wait!
    // In C++: int ix; PR_FLOAT_TO_INT(fx, ix); -> ix = -25600
    // pwTerrainNumX = ix / TERRAIN_XSIZE = -25600 / 25600 = -1.
    // In our rust code, it's `(ix as f32 / TERRAIN_XSIZE) as u16` -> -1 as f32 -> -1_i32 -> u16 -> 65535
    assert_eq!(tx, 65535); // This mimics the C++ behavior when converting negative int to u16
    assert_eq!(ty, 1); // fy was inverted, so it's 25600 / 25600 = 1

    // localized fx: starts at -25600.0. while fx < 0 { fx += 25600 } -> fx = 0
    // localized fy: starts at 25600.0. while fy >= 25600 { fy -= 25600 } -> fy = 0
    assert_eq!(cell_x, 0);
    assert_eq!(cell_y, 0);
    assert_eq!(sub_x, 0);
    assert_eq!(sub_y, 0);
}

#[test]
fn test_ffi_m2_terrain_world_to_local() {
    let gx = 25650.0;
    let gy = 25750.0;
    let coords = m2_terrain_world_to_local(gx, gy);
    
    assert_eq!(coords.terrain_x, 1);
    assert_eq!(coords.terrain_y, 1);

    // Expected cell_x = i_cell_x (0) + sub_x (1) / 2 = 0.5
    // Expected cell_y = i_cell_y (1) + sub_y (1) / 2 = 1.5
    assert_eq!(coords.cell_x, 0.5);
    assert_eq!(coords.cell_y, 1.5);
}

#[test]
fn test_ffi_m2_terrain_world_to_local_edge() {
    let gx = 0.0;
    let gy = 0.0;
    let coords = m2_terrain_world_to_local(gx, gy);
    
    assert_eq!(coords.terrain_x, 0);
    assert_eq!(coords.terrain_y, 0);
    assert_eq!(coords.cell_x, 0.0);
    assert_eq!(coords.cell_y, 0.0);
}
