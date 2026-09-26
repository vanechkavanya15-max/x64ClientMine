use m2_client::texture_splatting_blend_v2::*;
use std::ptr;

#[test]
fn test_terrain_splat_patch_initialization() {
    let patch = TerrainSplatPatch::new();
    assert_eq!(patch.tile_count.len(), MAX_TERRAIN_TEXTURES);
    assert_eq!(patch.patch_tile_count.len(), TERRAIN_PATCHCOUNT * TERRAIN_PATCHCOUNT);
    assert_eq!(patch.splats.len(), MAX_TERRAIN_TEXTURES);
    assert_eq!(patch.needs_update, false);
}

#[test]
fn test_terrain_initialization() {
    let terrain = Terrain::new(5);
    assert_eq!(terrain.texture_count, 5);
    assert_eq!(terrain.tile_map.len(), SPLATALPHA_RAW_XSIZE * SPLATALPHA_RAW_YSIZE);
}

#[test]
fn test_generate_splat_no_update_needed() {
    let mut terrain = Terrain::new(5);
    terrain.terrain_splat_patch.needs_update = false;
    
    terrain.generate_splat();
    
    assert_eq!(terrain.terrain_splat_patch.splats[1].active, false);
}

#[test]
fn test_generate_splat_no_tiles() {
    let mut terrain = Terrain::new(5);
    terrain.terrain_splat_patch.needs_update = true;
    terrain.terrain_splat_patch.splats[1].needs_update = true;
    terrain.terrain_splat_patch.tile_count[1] = 0;
    
    terrain.generate_splat();
    
    assert_eq!(terrain.terrain_splat_patch.splats[1].active, false);
    assert_eq!(terrain.terrain_splat_patch.splats[1].needs_update, false);
    assert!(terrain.terrain_splat_patch.splats[1].alpha_map.is_none());
}

#[test]
fn test_generate_splat_with_tiles_single_pixel() {
    let mut terrain = Terrain::new(5);
    terrain.terrain_splat_patch.needs_update = true;
    terrain.terrain_splat_patch.splats[1].needs_update = true;
    terrain.terrain_splat_patch.tile_count[1] = 1;
    
    // Set one pixel in the tile map to texture 1
    let x = 10;
    let y = 10;
    terrain.tile_map[y * SPLATALPHA_RAW_XSIZE + x] = 1;
    
    terrain.generate_splat();
    
    let splat = &terrain.terrain_splat_patch.splats[1];
    assert_eq!(splat.active, true);
    assert_eq!(splat.needs_update, false);
    
    let alpha_map = splat.alpha_map.as_ref().unwrap();
    assert_eq!(alpha_map.len(), SPLATALPHA_RAW_XSIZE * SPLATALPHA_RAW_YSIZE);
    
    // Pixel (10, 10) should be 0xFF
    assert_eq!(alpha_map[10 * SPLATALPHA_RAW_XSIZE + 10], 0xFF);
    // Other pixels should be 0x00
    assert_eq!(alpha_map[10 * SPLATALPHA_RAW_XSIZE + 11], 0x00);
}

#[test]
fn test_generate_splat_overlapping_textures() {
    let mut terrain = Terrain::new(5);
    terrain.terrain_splat_patch.needs_update = true;
    
    // Setup for texture 1
    terrain.terrain_splat_patch.splats[1].needs_update = true;
    terrain.terrain_splat_patch.tile_count[1] = 1;
    
    // Setup for texture 2
    terrain.terrain_splat_patch.splats[2].needs_update = true;
    terrain.terrain_splat_patch.tile_count[2] = 1;
    
    // Set pixel (10,10) to texture 2, which will also affect texture 1 if adjacent
    terrain.tile_map[10 * SPLATALPHA_RAW_XSIZE + 10] = 2;
    terrain.tile_map[10 * SPLATALPHA_RAW_XSIZE + 11] = 1;
    
    terrain.generate_splat();
    
    // Check texture 1
    let splat1 = &terrain.terrain_splat_patch.splats[1];
    let alpha_map1 = splat1.alpha_map.as_ref().unwrap();
    assert_eq!(alpha_map1[10 * SPLATALPHA_RAW_XSIZE + 11], 0xFF); // Exact match
    assert_eq!(alpha_map1[10 * SPLATALPHA_RAW_XSIZE + 10], 0xFF); // Adjacency with higher texture id
    
    // Check texture 2
    let splat2 = &terrain.terrain_splat_patch.splats[2];
    let alpha_map2 = splat2.alpha_map.as_ref().unwrap();
    assert_eq!(alpha_map2[10 * SPLATALPHA_RAW_XSIZE + 10], 0xFF); // Exact match
    assert_eq!(alpha_map2[10 * SPLATALPHA_RAW_XSIZE + 11], 0x00); // No match
}

#[test]
fn test_ffi_get_texture_layers_combined() {
    // Run all FFI tests sequentially in one function to avoid multithreading data race
    // on the global ptr.
    
    // 1. Out of bounds
    m2_terrain_set_context(ptr::null());
    assert_eq!(m2_terrain_get_texture_layers(TERRAIN_PATCHCOUNT as u16, 0), 0);
    assert_eq!(m2_terrain_get_texture_layers(0, TERRAIN_PATCHCOUNT as u16), 0);
    
    // 2. Null pointer
    m2_terrain_set_context(ptr::null());
    assert_eq!(m2_terrain_get_texture_layers(0, 0), 0);
    
    // 3. Valid
    let mut terrain = Terrain::new(10);
    let patch_index = 0;
    terrain.terrain_splat_patch.patch_tile_count[patch_index][1] = 10;
    terrain.terrain_splat_patch.patch_tile_count[patch_index][3] = 5;
    terrain.terrain_splat_patch.patch_tile_count[patch_index][5] = 1;
    
    m2_terrain_set_context(&terrain as *const Terrain);
    assert_eq!(m2_terrain_get_texture_layers(0, 0), 3);
    m2_terrain_set_context(ptr::null());
    
    // 4. Max limit
    let mut terrain = Terrain::new(15);
    let patch_x = 1;
    let patch_y = 1;
    let patch_index = (patch_y * TERRAIN_PATCHCOUNT) + patch_x;
    
    for i in 1..=10 {
        terrain.terrain_splat_patch.patch_tile_count[patch_index][i] = 1;
    }
    
    m2_terrain_set_context(&terrain as *const Terrain);
    assert_eq!(m2_terrain_get_texture_layers(patch_x as u16, patch_y as u16), 8);
    m2_terrain_set_context(ptr::null());
}
