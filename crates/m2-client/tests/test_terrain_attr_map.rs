use m2_client::terrain_attr_map::TerrainAttrMap;

#[test]
fn test_terrain_attr_map_new() {
    let grid = vec![0; 4];
    let map = TerrainAttrMap::new(2, 2, grid);
    assert!(map.is_ok());

    let grid = vec![0; 3];
    let map = TerrainAttrMap::new(2, 2, grid);
    assert!(map.is_err());
}

#[test]
fn test_terrain_attr_map_is_walkable() {
    // 1: ATTR_BLOCK
    // 2: ATTR_WATER
    // 4: ATTR_WARP
    // 8: ATTR_SAFE_ZONE
    let grid = vec![
        0, 1, // open, block
        2, 4, // water, warp
        8, 9, // safe zone, safe zone + block
    ];
    let map = TerrainAttrMap::new(2, 3, grid).unwrap();

    assert!(map.is_walkable(0, 0)); // open
    assert!(!map.is_walkable(1, 0)); // block
    assert!(!map.is_walkable(0, 1)); // water
    assert!(map.is_walkable(1, 1)); // warp (walkable)
    assert!(map.is_walkable(0, 2)); // safe zone (walkable)
    assert!(!map.is_walkable(1, 2)); // safe zone + block

    // Out of bounds
    assert!(!map.is_walkable(2, 0));
    assert!(!map.is_walkable(0, 3));
}

#[test]
fn test_terrain_attr_map_is_safe_zone() {
    let grid = vec![
        0, 1, // open, block
        2, 8, // water, safe zone
        12, 0, // safe zone + warp, open
    ];
    let map = TerrainAttrMap::new(2, 3, grid).unwrap();

    assert!(!map.is_safe_zone(0, 0)); // open
    assert!(!map.is_safe_zone(1, 0)); // block
    assert!(!map.is_safe_zone(0, 1)); // water
    assert!(map.is_safe_zone(1, 1)); // safe zone
    assert!(map.is_safe_zone(0, 2)); // safe zone + warp
    assert!(!map.is_safe_zone(1, 2)); // open

    // Out of bounds
    assert!(!map.is_safe_zone(2, 0));
    assert!(!map.is_safe_zone(0, 3));
}
