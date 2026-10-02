use m2_client::map_outdoor_vfs_loader::*;

#[test]
fn test_parse_setting_txt() {
    let setting_txt = r#"
        ScriptType MapSetting
        CellScale 200
        HeightScale 0.5
        ViewRadius 128
        MapSize 2 3
        TextureSet d:/ymir work/textureset/snow.txt
        BasePosition 1000 2000
        WaterHeight 10
    "#;

    let mut loader = MapOutdoorVfsLoader::new();
    let result = loader.load_setting(setting_txt);
    assert!(result.is_ok(), "Failed to parse valid Setting.txt");

    let setting = &loader.setting;
    assert_eq!(setting.map_size_x, 2);
    assert_eq!(setting.map_size_y, 3);
    // The key parser splits by whitespace and removes the first part.
    // If we have `TextureSet d:/ymir work/textureset/snow.txt`,
    // it will be parsed as ["d:/ymir", "work/textureset/snow.txt"].
    // In our `load_setting` we take `.first()`, so it will just be "d:/ymir".
    // We update the test to match this behavior, or we can fix the loader to join the rest.
    // Given the simplicity, we'll join the rest in the loader, let's fix loader first and adjust this test.
    assert_eq!(setting.texture_set, "d:/ymir work/textureset/snow.txt");
    assert_eq!(setting.base_x, 1000);
    assert_eq!(setting.base_y, 2000);
    assert_eq!(setting.view_radius, 128);
    assert_eq!(setting.height_scale, 0.5);
    assert_eq!(setting.water_height, 10);
}

#[test]
fn test_parse_setting_txt_invalid() {
    let setting_txt_invalid_type = r#"
        ScriptType WrongType
        CellScale 200
        HeightScale 0.5
        ViewRadius 128
        MapSize 2 3
        TextureSet snow.txt
    "#;

    let mut loader = MapOutdoorVfsLoader::new();
    assert!(loader.load_setting(setting_txt_invalid_type).is_err());
}

#[test]
fn test_load_height_map() {
    let mut loader = MapOutdoorVfsLoader::new();
    
    let mut data = vec![0u8; HEIGHTMAP_RAW_BYTES];
    // Write a height value of 1000 to (1, 1)
    let idx = 1 * HEIGHTMAP_RAW_XSIZE + 1;
    let val: u16 = 1000;
    data[idx * 2] = (val & 0xFF) as u8;
    data[idx * 2 + 1] = (val >> 8) as u8;

    let res = loader.load_height_map(&data);
    assert!(res.is_ok());

    let map = loader.height_map.unwrap();
    assert_eq!(map.get_height(1, 1), 1000);
    assert_eq!(map.get_height(0, 0), 0);
}

#[test]
fn test_load_tile_map() {
    let mut loader = MapOutdoorVfsLoader::new();
    let mut data = vec![0u8; TILEMAP_RAW_BYTES];
    
    // Write tile index 5 to (10, 10)
    data[10 * TILEMAP_RAW_XSIZE + 10] = 5;

    let res = loader.load_tile_map(&data);
    assert!(res.is_ok());

    let map = loader.tile_map.unwrap();
    assert_eq!(map.get_tile(10, 10), 5);
}

#[test]
fn test_load_attr_map() {
    let mut loader = MapOutdoorVfsLoader::new();
    
    let mut data = Vec::new();
    // Header
    data.extend_from_slice(&ATTRMAP_MAGIC.to_le_bytes()); // magic
    data.extend_from_slice(&(ATTRMAP_XSIZE as u16).to_le_bytes()); // width
    data.extend_from_slice(&(ATTRMAP_YSIZE as u16).to_le_bytes()); // height
    
    // Body
    let mut body = vec![0u8; ATTRMAP_XSIZE * ATTRMAP_YSIZE];
    body[5 * ATTRMAP_XSIZE + 5] = ATTRIBUTE_BLOCK;
    data.extend_from_slice(&body);

    let res = loader.load_attr_map(&data);
    assert!(res.is_ok());

    let map = loader.attr_map.unwrap();
    assert_eq!(map.get_attr(5, 5), ATTRIBUTE_BLOCK);
}

#[test]
fn test_get_terrain_height_at() {
    let mut loader = MapOutdoorVfsLoader::new();
    loader.setting.height_scale = 1.0;
    loader.setting.base_x = 0;
    loader.setting.base_y = 0;

    let mut data = vec![0u8; HEIGHTMAP_RAW_BYTES];
    // Fill heights:
    // (1, 1) -> 100
    // (2, 1) -> 100
    // (1, 2) -> 200
    // (2, 2) -> 200

    fn set_height(data: &mut [u8], x: usize, y: usize, h: u16) {
        let idx = y * HEIGHTMAP_RAW_XSIZE + x;
        data[idx * 2] = (h & 0xFF) as u8;
        data[idx * 2 + 1] = (h >> 8) as u8;
    }

    set_height(&mut data, 1, 1, 100);
    set_height(&mut data, 2, 1, 100);
    set_height(&mut data, 1, 2, 200);
    set_height(&mut data, 2, 2, 200);

    loader.load_height_map(&data).unwrap();

    // The function takes (world_x, world_y).
    // world coordinates map to cell_x = world_x / CELLSCALE.
    // In our map:
    // Cell (0, 0) corresponds to cell_x=0, cell_y=0.
    // Interpolation reads sx1=1, sy1=1.
    // world (0, 0) -> cell (0, 0), xdist=0, ydist=0. Reads h(1,1) = 100.
    let h = loader.get_terrain_height_at(0.0, 0.0);
    assert_eq!(h, 100.0);

    // world (0, 100) -> cell (0, 0), xdist=0, ydist=100. 
    // ydist (100) > xdist (0), so it's the "left triangle" (xdist <= ydist in C++).
    // This will interpolate halfway between h(1,1)=100 and h(1,2)=200.
    let h = loader.get_terrain_height_at(0.0, 100.0);
    assert_eq!(h, 150.0);

    // out of bounds should return 0
    let out_h = loader.get_terrain_height_at((TERRAIN_XSIZE * CELLSCALE + 10) as f32, 0.0);
    assert_eq!(out_h, 0.0);
}
