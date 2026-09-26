use m2_client::terrain_tile_map::{TileMap, TILEMAP_RAW_SIZE};

#[test]
fn test_tile_map_from_bytes() {
    let invalid_data = vec![0u8; 100];
    let result = TileMap::from_bytes(&invalid_data);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid tile map data size");

    let valid_data = vec![0u8; TILEMAP_RAW_SIZE];
    let result = TileMap::from_bytes(&valid_data);
    assert!(result.is_ok());
}

#[test]
fn test_get_tile() {
    let mut data = vec![0u8; TILEMAP_RAW_SIZE];
    
    // Set some specific values
    // Index = y * 258 + x
    data[0] = 5;
    data[258 + 1] = 10; // x=1, y=1
    data[257 * 258 + 257] = 15; // x=257, y=257

    let tile_map = TileMap::from_bytes(&data).unwrap();

    assert_eq!(tile_map.get_tile(0, 0), Some(5));
    assert_eq!(tile_map.get_tile(1, 1), Some(10));
    assert_eq!(tile_map.get_tile(257, 257), Some(15));

    // Out of bounds
    assert_eq!(tile_map.get_tile(258, 0), None);
    assert_eq!(tile_map.get_tile(0, 258), None);
    assert_eq!(tile_map.get_tile(258, 258), None);
}

#[test]
fn test_count_unique_textures() {
    let mut data = vec![0u8; TILEMAP_RAW_SIZE];
    data[0] = 1;
    data[1] = 2;
    data[2] = 1;
    data[3] = 3;
    data[4] = 2;
    data[5] = 255;

    let tile_map = TileMap::from_bytes(&data).unwrap();
    // Unique textures: 0, 1, 2, 3, 255 -> 5 unique
    assert_eq!(tile_map.count_unique_textures(), 5);
}

#[test]
fn test_get_patch_texture_counts() {
    let mut data = vec![0u8; TILEMAP_RAW_SIZE];
    
    // Let's populate patch 0, 0 (which corresponds to x: 1..33, y: 1..33)
    // with some known texture IDs
    
    // Set (1, 1) to 10
    data[1 * 258 + 1] = 10;
    
    // Set (2, 1) to 10
    data[1 * 258 + 2] = 10;
    
    // Set (32, 32) to 20
    data[32 * 258 + 32] = 20;

    // Set (0, 0) to 30 - this is in the border, should not be included in patch 0,0
    data[0] = 30;

    let tile_map = TileMap::from_bytes(&data).unwrap();

    let counts = tile_map.get_patch_texture_counts(0, 0);

    assert_eq!(counts[10], 2);
    assert_eq!(counts[20], 1);
    assert_eq!(counts[30], 0); // border not included
    // 32 * 32 = 1024. Total assigned in patch: 3. Remaining: 1021 zeros.
    assert_eq!(counts[0], 1021);
    
    // Ensure total count is exactly 1024
    let total_count: u32 = counts.iter().sum();
    assert_eq!(total_count, 1024);
}
