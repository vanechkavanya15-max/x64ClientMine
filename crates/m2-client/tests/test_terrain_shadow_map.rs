use m2_client::terrain_shadow_map::{TerrainShadowMap, SHADOWMAP_XSIZE};

#[test]
fn test_terrain_shadow_map_black_color() {
    let mut map = TerrainShadowMap::new();
    // 0x0000 -> black
    map.shadow_map[0] = 0x0000;
    
    let color = map.get_shadow_color_at(0.0, 0.0);
    assert_eq!(color, 0xFF000000); // Only Alpha is 0xFF
}

#[test]
fn test_terrain_shadow_map_white_color() {
    let mut map = TerrainShadowMap::new();
    // 0xFFFF -> white
    map.shadow_map[0] = 0xFFFF;
    
    let color = map.get_shadow_color_at(0.0, 0.0);
    assert_eq!(color, 0xFFFFFFFF); // All channels 0xFF
}

#[test]
fn test_terrain_shadow_map_red_color() {
    let mut map = TerrainShadowMap::new();
    // R = 31 (0x1F), G = 0, B = 0 -> 0xF800
    map.shadow_map[0] = 0xF800;
    
    let color = map.get_shadow_color_at(0.0, 0.0);
    assert_eq!(color, 0xFFFF0000); 
}

#[test]
fn test_terrain_shadow_map_green_color() {
    let mut map = TerrainShadowMap::new();
    // R = 0, G = 63 (0x3F), B = 0 -> 0x07E0
    map.shadow_map[0] = 0x07E0;
    
    let color = map.get_shadow_color_at(0.0, 0.0);
    assert_eq!(color, 0xFF00FF00); 
}

#[test]
fn test_terrain_shadow_map_blue_color() {
    let mut map = TerrainShadowMap::new();
    // R = 0, G = 0, B = 31 (0x1F) -> 0x001F
    map.shadow_map[0] = 0x001F;
    
    let color = map.get_shadow_color_at(0.0, 0.0);
    assert_eq!(color, 0xFF0000FF); 
}

#[test]
fn test_terrain_shadow_map_middle_coordinate() {
    let mut map = TerrainShadowMap::new();
    let index = 127 * SHADOWMAP_XSIZE + 127;
    map.shadow_map[index] = 0x0000; // Black
    
    // Map center coordinate (64.0, 64.0) should map to (127, 127) because 64.0/128.0 * 255.0 = 127.5 -> 127 as usize
    let color = map.get_shadow_color_at(64.0, 64.0);
    assert_eq!(color, 0xFF000000);
}

#[test]
fn test_terrain_shadow_map_out_of_bounds() {
    let map = TerrainShadowMap::new();
    
    // Outside the valid range should return pure white (0xFFFFFFFF) as per C++ logic
    assert_eq!(map.get_shadow_color_at(-1.0, 50.0), 0xFFFFFFFF);
    assert_eq!(map.get_shadow_color_at(50.0, -1.0), 0xFFFFFFFF);
    assert_eq!(map.get_shadow_color_at(128.0, 50.0), 0xFFFFFFFF);
    assert_eq!(map.get_shadow_color_at(50.0, 128.0), 0xFFFFFFFF);
}

#[test]
fn test_terrain_shadow_map_clamping_edge() {
    let mut map = TerrainShadowMap::new();
    // Very edge of valid bounds
    // (127.999 / 128.0 * 255.0) -> approx 254.998 -> 254
    // C++ clamps fx = fx * fooMapSize * (SHADOWMAP_XSIZE - 1)
    
    let index = 254 * SHADOWMAP_XSIZE + 254;
    map.shadow_map[index] = 0x0000;
    
    let color = map.get_shadow_color_at(127.99, 127.99);
    assert_eq!(color, 0xFF000000);
}
