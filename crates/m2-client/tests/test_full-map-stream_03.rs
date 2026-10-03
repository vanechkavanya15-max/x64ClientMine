#[path = "../src/terrain_attr_tile_loader.rs"]
mod terrain_attr_tile_loader;

use terrain_attr_tile_loader::{
    TerrainAttrTileLoader, TILE_SIZE, TILE_MAGIC, FLAG_BLOCKED, FLAG_WATER, FLAG_SAFE_ZONE
};

fn create_valid_atr_data(fill_val: u8) -> Vec<u8> {
    let mut data = Vec::new();
    data.extend_from_slice(&TILE_MAGIC.to_le_bytes());
    data.extend_from_slice(&(TILE_SIZE as u16).to_le_bytes());
    data.extend_from_slice(&(TILE_SIZE as u16).to_le_bytes());
    data.resize(6 + TILE_SIZE * TILE_SIZE, fill_val);
    data
}

#[test]
fn test_load_valid_tile() {
    let mut loader = TerrainAttrTileLoader::new();
    let data = create_valid_atr_data(0);
    
    let result = loader.load_tile(0, 0, &data);
    assert!(result.is_ok());
    assert!(loader.is_walkable(0, 0));
}

#[test]
fn test_load_invalid_magic() {
    let mut loader = TerrainAttrTileLoader::new();
    let mut data = create_valid_atr_data(0);
    data[0] = 0;
    data[1] = 0; // wrong magic
    
    let result = loader.load_tile(0, 0, &data);
    assert_eq!(result, Err("Invalid magic number"));
}

#[test]
fn test_load_invalid_dimensions() {
    let mut loader = TerrainAttrTileLoader::new();
    
    let mut data2 = create_valid_atr_data(0);
    data2[3] = 0; // width is now invalid (0 instead of 256 for upper byte)
    let result2 = loader.load_tile(0, 0, &data2);
    assert_eq!(result2, Err("Invalid dimensions"));
}

#[test]
fn test_load_invalid_size() {
    let mut loader = TerrainAttrTileLoader::new();
    let mut data = create_valid_atr_data(0);
    data.pop(); // make size wrong
    
    let result = loader.load_tile(0, 0, &data);
    assert_eq!(result, Err("Invalid .atr file size"));
}

#[test]
fn test_walkability_flags() {
    let mut loader = TerrainAttrTileLoader::new();
    let mut data = create_valid_atr_data(0);
    
    // Set some specific blocks
    let idx1 = 6 + (10 * TILE_SIZE + 10);
    data[idx1] = FLAG_BLOCKED;
    
    let idx2 = 6 + (20 * TILE_SIZE + 20);
    data[idx2] = FLAG_WATER;
    
    let idx3 = 6 + (30 * TILE_SIZE + 30);
    data[idx3] = FLAG_SAFE_ZONE;
    
    loader.load_tile(0, 0, &data).unwrap();
    
    assert!(!loader.is_walkable(10, 10)); // blocked
    assert!(!loader.is_walkable(20, 20)); // water
    assert!(loader.is_walkable(30, 30));  // safe zone is walkable
    assert!(loader.is_walkable(40, 40));  // empty is walkable
}

#[test]
fn test_out_of_bounds_blocked() {
    let mut loader = TerrainAttrTileLoader::new();
    let data = create_valid_atr_data(0);
    loader.load_tile(0, 0, &data).unwrap();
    
    // Tile (1, 1) is not loaded, so it should be blocked
    assert!(!loader.is_walkable(TILE_SIZE + 10, TILE_SIZE + 10));
}

#[test]
fn test_multiple_tiles() {
    let mut loader = TerrainAttrTileLoader::new();
    
    let mut data_0_0 = create_valid_atr_data(0);
    data_0_0[6 + 50 * TILE_SIZE + 50] = FLAG_BLOCKED;
    
    let mut data_1_0 = create_valid_atr_data(0);
    data_1_0[6 + 60 * TILE_SIZE + 60] = FLAG_WATER;
    
    loader.load_tile(0, 0, &data_0_0).unwrap();
    loader.load_tile(1, 0, &data_1_0).unwrap();
    
    assert!(!loader.is_walkable(50, 50));
    assert!(!loader.is_walkable(TILE_SIZE + 60, 60));
    assert!(loader.is_walkable(TILE_SIZE + 50, 50));
}

#[test]
fn test_remove_and_clear_tiles() {
    let mut loader = TerrainAttrTileLoader::new();
    let data = create_valid_atr_data(0);
    loader.load_tile(0, 0, &data).unwrap();
    assert!(loader.is_walkable(0, 0));
    
    loader.remove_tile(0, 0);
    assert!(!loader.is_walkable(0, 0));
    
    loader.load_tile(0, 0, &data).unwrap();
    loader.load_tile(1, 1, &data).unwrap();
    loader.clear();
    
    assert!(!loader.is_walkable(0, 0));
    assert!(!loader.is_walkable(TILE_SIZE, TILE_SIZE));
}
