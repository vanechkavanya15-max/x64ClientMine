use m2_client::terrain_height_patch::{RawHeightMap, TERRAIN_PATCHCOUNT};
use m2_client::water_tile_mesh_loader::WaterTileMeshLoader;

#[test]
fn test_load_sector_water_invalid_data() {
    let raw_heights = vec![0.0; 256 * 256];
    let height_map = RawHeightMap::new(256, 256, raw_heights);
    let result = WaterTileMeshLoader::load_sector_water(&[], &height_map, 1.0);
    assert!(result.is_err());
}

#[test]
fn test_load_sector_water_valid_no_water() {
    let mut data = vec![0u8; 6];
    // Magic 5426
    data[0] = (5426u16 & 0xFF) as u8;
    data[1] = (5426u16 >> 8) as u8;
    // Width 128
    data[2] = 128;
    data[3] = 0;
    // Height 128
    data[4] = 128;
    data[5] = 0;
    // Layer count 0
    data.push(0);
    
    // Grid 128x128
    let mut grid = vec![0xFF; 128 * 128];
    data.extend(grid);

    let raw_heights = vec![0.0; 256 * 256];
    let height_map = RawHeightMap::new(256, 256, raw_heights);
    
    let result = WaterTileMeshLoader::load_sector_water(&data, &height_map, 1.0);
    assert!(result.is_ok());
    let vertices = result.unwrap();
    assert_eq!(vertices.len(), 0);
}

#[test]
fn test_load_sector_water_valid_with_water() {
    let mut data = vec![0u8; 7];
    // Magic 5426
    data[0] = (5426u16 & 0xFF) as u8;
    data[1] = (5426u16 >> 8) as u8;
    // Width 128
    data[2] = 128;
    data[3] = 0;
    // Height 128
    data[4] = 128;
    data[5] = 0;
    // Layer count 1
    data[6] = 1;
    
    // Layer 0 height = 100
    data.push(100);
    data.push(0);

    // Grid 128x128
    let mut grid = vec![0xFF; 128 * 128];
    // Set some water at top left
    for y in 0..16 {
        for x in 0..16 {
            grid[y * 128 + x] = 0; // Layer 0
        }
    }
    data.extend(grid);

    let raw_heights = vec![0.0; 256 * 256];
    // Just mock 128x128 height map for simplicity? 
    // WaterMeshBuilder uses start_x = px * TERRAIN_PATCHSIZE
    // max px = 7, py = 7, start_x = 7 * 16 = 112
    // max global_x = 112 + 15 = 127
    // height map needs to be at least 128x128
    let height_map = RawHeightMap::new(128, 128, vec![0.0; 128 * 128]);
    
    let result = WaterTileMeshLoader::load_sector_water(&data, &height_map, 1.0);
    assert!(result.is_ok());
    let vertices = result.unwrap();
    assert!(vertices.len() > 0);
}

#[test]
fn test_load_sector_water_invalid_magic() {
    let mut data = vec![0u8; 6];
    // Magic 0
    data[0] = 0;
    data[1] = 0;
    // Width 128
    data[2] = 128;
    data[3] = 0;
    // Height 128
    data[4] = 128;
    data[5] = 0;
    // Layer count 0
    data.push(0);
    
    let mut grid = vec![0xFF; 128 * 128];
    data.extend(grid);

    let raw_heights = vec![0.0; 256 * 256];
    let height_map = RawHeightMap::new(256, 256, raw_heights);
    
    let result = WaterTileMeshLoader::load_sector_water(&data, &height_map, 1.0);
    assert!(result.is_err());
}

#[test]
fn test_load_sector_water_incomplete_grid() {
    let mut data = vec![0u8; 7];
    // Magic 5426
    data[0] = (5426u16 & 0xFF) as u8;
    data[1] = (5426u16 >> 8) as u8;
    // Width 128
    data[2] = 128;
    data[3] = 0;
    // Height 128
    data[4] = 128;
    data[5] = 0;
    // Layer count 1
    data[6] = 1;
    
    // Layer 0 height = 100
    data.push(100);
    data.push(0);

    // Grid 128x128 but incomplete
    let grid = vec![0xFF; 100];
    data.extend(grid);

    let raw_heights = vec![0.0; 256 * 256];
    let height_map = RawHeightMap::new(256, 256, raw_heights);
    
    let result = WaterTileMeshLoader::load_sector_water(&data, &height_map, 1.0);
    assert!(result.is_err());
}
