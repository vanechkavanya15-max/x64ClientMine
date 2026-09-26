#[path = "../src/terrain_sector_grid.rs"]
mod terrain_sector_grid;

use terrain_sector_grid::*;

#[test]
fn test_exact_sector_origin() {
    let global_x = 10000;
    let global_y = 20000;
    let base_x = 10000;
    let base_y = 20000;
    
    let coords = global_to_sector(global_x, global_y, base_x, base_y);
    assert_eq!(coords.col, 0);
    assert_eq!(coords.row, 0);
    assert_eq!(coords.local_cm_x, 0.0);
    assert_eq!(coords.local_cm_y, 0.0);
    assert_eq!(coords.tile_x, 0.0);
    assert_eq!(coords.tile_y, 0.0);
}

#[test]
fn test_middle_of_sector() {
    let base_x = 0;
    let base_y = 0;
    
    // Half of SECTOR_SIZE_CM (25600 / 2 = 12800)
    let global_x = 12800;
    let global_y = 12800;
    
    let coords = global_to_sector(global_x, global_y, base_x, base_y);
    assert_eq!(coords.col, 0);
    assert_eq!(coords.row, 0);
    assert_eq!(coords.local_cm_x, 12800.0);
    assert_eq!(coords.local_cm_y, 12800.0);
    assert_eq!(coords.tile_x, 64.0);
    assert_eq!(coords.tile_y, 64.0);
}

#[test]
fn test_transition_between_sectors() {
    let base_x = 100;
    let base_y = 100;
    
    // Exactly at the boundary of first sector
    let global_x1 = base_x + SECTOR_SIZE_CM - 1;
    let global_y1 = base_y + SECTOR_SIZE_CM - 1;
    let coords1 = global_to_sector(global_x1, global_y1, base_x, base_y);
    assert_eq!(coords1.col, 0);
    assert_eq!(coords1.row, 0);
    assert_eq!(coords1.local_cm_x, (SECTOR_SIZE_CM - 1) as f32);
    assert_eq!(coords1.local_cm_y, (SECTOR_SIZE_CM - 1) as f32);
    assert_eq!(coords1.tile_x, (SECTOR_SIZE_CM - 1) as f32 / 200.0);
    assert_eq!(coords1.tile_y, (SECTOR_SIZE_CM - 1) as f32 / 200.0);

    // Exactly at the start of the next sector
    let global_x2 = base_x + SECTOR_SIZE_CM;
    let global_y2 = base_y + SECTOR_SIZE_CM;
    let coords2 = global_to_sector(global_x2, global_y2, base_x, base_y);
    assert_eq!(coords2.col, 1);
    assert_eq!(coords2.row, 1);
    assert_eq!(coords2.local_cm_x, 0.0);
    assert_eq!(coords2.local_cm_y, 0.0);
    assert_eq!(coords2.tile_x, 0.0);
    assert_eq!(coords2.tile_y, 0.0);
}

#[test]
fn test_get_surrounding_sectors_center() {
    let max_w = 10;
    let max_h = 10;
    let center_c = 5;
    let center_r = 5;
    
    let sectors = get_surrounding_sectors(center_c, center_r, max_w, max_h);
    assert_eq!(sectors.len(), 9);
    
    let expected = vec![
        (4, 4), (5, 4), (6, 4),
        (4, 5), (5, 5), (6, 5),
        (4, 6), (5, 6), (6, 6)
    ];
    
    assert_eq!(sectors, expected);
}

#[test]
fn test_get_surrounding_sectors_corner() {
    let max_w = 10;
    let max_h = 10;
    
    // Top-left corner
    let tl_sectors = get_surrounding_sectors(0, 0, max_w, max_h);
    assert_eq!(tl_sectors.len(), 4);
    assert_eq!(tl_sectors, vec![
        (0, 0), (1, 0),
        (0, 1), (1, 1)
    ]);
    
    // Bottom-right corner
    let br_sectors = get_surrounding_sectors(9, 9, max_w, max_h);
    assert_eq!(br_sectors.len(), 4);
    assert_eq!(br_sectors, vec![
        (8, 8), (9, 8),
        (8, 9), (9, 9)
    ]);
}

#[test]
fn test_get_surrounding_sectors_edges() {
    let max_w = 10;
    let max_h = 10;
    
    // Top edge
    let te_sectors = get_surrounding_sectors(5, 0, max_w, max_h);
    assert_eq!(te_sectors.len(), 6);
    assert_eq!(te_sectors, vec![
        (4, 0), (5, 0), (6, 0),
        (4, 1), (5, 1), (6, 1)
    ]);
    
    // Right edge
    let re_sectors = get_surrounding_sectors(9, 5, max_w, max_h);
    assert_eq!(re_sectors.len(), 6);
    assert_eq!(re_sectors, vec![
        (8, 4), (9, 4),
        (8, 5), (9, 5),
        (8, 6), (9, 6)
    ]);
}

#[test]
fn test_get_surrounding_sectors_zero_dimensions() {
    let s1 = get_surrounding_sectors(0, 0, 0, 10);
    assert_eq!(s1.len(), 0);
    
    let s2 = get_surrounding_sectors(0, 0, 10, 0);
    assert_eq!(s2.len(), 0);
    
    let s3 = get_surrounding_sectors(5, 5, 0, 0);
    assert_eq!(s3.len(), 0);
}

#[test]
fn test_get_surrounding_sectors_out_of_bounds() {
    let s1 = get_surrounding_sectors(15, 15, 10, 10);
    assert_eq!(s1.len(), 0);
}
