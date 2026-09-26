use m2_client::map_setting_parser::MapSetting;

#[test]
fn test_parse_complete_setting() {
    let config = "
        // this is a comment
        # this is another comment
        MapSize 3 3
        BasePosition 200000 300000
        TextureSet textureset/metin2_a1.txt
        Environment d:/ymir work/environment/test.msenv
    ";

    let setting = MapSetting::parse(config).unwrap();
    assert_eq!(setting.width_sectors, 3);
    assert_eq!(setting.height_sectors, 3);
    assert_eq!(setting.base_x, 200000);
    assert_eq!(setting.base_y, 300000);
    assert_eq!(setting.texture_set_path, "textureset/metin2_a1.txt");
    assert_eq!(setting.environment_path, "d:/ymir work/environment/test.msenv");
}

#[test]
fn test_ignore_comments_and_empty_lines() {
    let config = "
        
        // This is a test
        # Another test

        MapSize 2 2
        
        BasePosition 0 0
        
        // Comment here
        TextureSet some_path.txt
        
        Environment env.msenv
        # The End
    ";
    let setting = MapSetting::parse(config).unwrap();
    assert_eq!(setting.width_sectors, 2);
    assert_eq!(setting.base_x, 0);
    assert_eq!(setting.texture_set_path, "some_path.txt");
    assert_eq!(setting.environment_path, "env.msenv");
}

#[test]
fn test_get_world_bounds_cm() {
    let config = "
        MapSize 3 4
        BasePosition 10000 20000
        TextureSet path.txt
        Environment path.msenv
    ";
    let setting = MapSetting::parse(config).unwrap();
    let bounds = setting.get_world_bounds_cm();
    // 1 sektor = 25600 cm
    // min_x = 10000
    // min_y = 20000
    // max_x = 10000 + 3 * 25600 = 86800
    // max_y = 20000 + 4 * 25600 = 122400
    assert_eq!(bounds, (10000, 20000, 86800, 122400));
}

#[test]
fn test_error_missing_mapsize() {
    let config = "
        BasePosition 10000 20000
        TextureSet path.txt
        Environment path.msenv
    ";
    let setting = MapSetting::parse(config);
    assert!(setting.is_err());
    assert_eq!(setting.unwrap_err(), "Missing or invalid MapSize");
}

#[test]
fn test_error_invalid_format() {
    let config_bad_size = "
        MapSize 3 abc
        BasePosition 10000 20000
        TextureSet path.txt
        Environment path.msenv
    ";
    let setting1 = MapSetting::parse(config_bad_size);
    assert!(setting1.is_err());
    assert_eq!(setting1.unwrap_err(), "Missing or invalid MapSize");

    let config_bad_base = "
        MapSize 3 3
        BasePosition 10000
        TextureSet path.txt
        Environment path.msenv
    ";
    let setting2 = MapSetting::parse(config_bad_base);
    assert!(setting2.is_err());
    assert_eq!(setting2.unwrap_err(), "Missing or invalid BasePosition");

    let config_missing_texture = "
        MapSize 3 3
        BasePosition 10000 20000
        Environment path.msenv
    ";
    let setting3 = MapSetting::parse(config_missing_texture);
    assert!(setting3.is_err());
    assert_eq!(setting3.unwrap_err(), "Missing TextureSet");

    let config_missing_env = "
        MapSize 3 3
        BasePosition 10000 20000
        TextureSet path.txt
    ";
    let setting4 = MapSetting::parse(config_missing_env);
    assert!(setting4.is_err());
    assert_eq!(setting4.unwrap_err(), "Missing Environment");
}
