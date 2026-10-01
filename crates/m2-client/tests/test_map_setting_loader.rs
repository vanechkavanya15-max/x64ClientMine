#![allow(non_snake_case)]

use m2_client::map_setting_loader::*;
use m2_client::map_outdoor_core_v2::{TERRAIN_XSIZE, TERRAIN_YSIZE};

#[test]
fn test_parse_atlas_info_normal() {
    let content = "
        metin2_map_a1 1000 2000 2 3
        metin2_map_b1 5000 6000 1 1
    ";

    let infos = parse_atlas_info(content);
    assert_eq!(infos.len(), 2);

    assert_eq!(infos[0].name, "metin2_map_a1");
    assert_eq!(infos[0].base_x, 1000);
    assert_eq!(infos[0].base_y, 2000);
    assert_eq!(infos[0].size_x, 2);
    assert_eq!(infos[0].size_y, 3);
    assert_eq!(infos[0].end_x, 1000 + (2.0 * TERRAIN_XSIZE) as i32);
    assert_eq!(infos[0].end_y, 2000 + (3.0 * TERRAIN_YSIZE) as i32);

    assert_eq!(infos[1].name, "metin2_map_b1");
    assert_eq!(infos[1].base_x, 5000);
    assert_eq!(infos[1].base_y, 6000);
}

#[test]
fn test_parse_atlas_info_malformed() {
    let content = "
        metin2_map_c1 1000 2000 2
        
        metin2_map_d1 1000 2000 2 3 extra
    ";

    let infos = parse_atlas_info(content);
    assert_eq!(infos.len(), 1);
    assert_eq!(infos[0].name, "metin2_map_d1");
}

#[test]
fn test_parse_setting_txt_normal() {
    let content = "
        ScriptType MapSetting

        ViewRadius 128
        CellScale 200
        HeightScale 0.5
        MapSize 2 3
        TextureSet textureset\\metin2_a1.txt
        TerrainVisible 1
        BasePosition 1000 2000
        Environment metin2_a1.msenv
    ";

    let config = parse_setting_txt(content).unwrap();
    assert_eq!(config.script_type, "MapSetting");
    assert_eq!(config.view_radius, 128);
    assert_eq!(config.height_scale, 0.5);
    assert_eq!(config.map_size_x, 2);
    assert_eq!(config.map_size_y, 3);
    assert_eq!(config.texture_set, "textureset\\metin2_a1.txt");
    assert_eq!(config.terrain_visible, true);
    assert_eq!(config.base_x, Some(1000));
    assert_eq!(config.base_y, Some(2000));
    assert_eq!(config.environment, Some("metin2_a1.msenv".to_string()));
}

#[test]
fn test_parse_setting_txt_missing_required() {
    let content = "
        ScriptType MapSetting
        ViewRadius 128
        HeightScale 0.5
        // MapSize is missing
        TextureSet test
    ";

    let result = parse_setting_txt(content);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), "FIND 'cellscale' - FAILED");
    
    let content2 = "
        ScriptType MapSetting
        ViewRadius 128
        CellScale 200
        HeightScale 0.5
        // MapSize is missing
        TextureSet test
    ";
    let result2 = parse_setting_txt(content2);
    assert!(result2.is_err());
    assert_eq!(result2.err().unwrap(), "FIND 'mapsize' - FAILED");
}

#[test]
fn test_parse_setting_txt_block_format() {
    let map = parse_setting_txt(
    "
        ScriptType MapSetting
        ViewRadius 128
        CellScale 200
        HeightScale 0.5
        MapSize 2 3
        TextureSet test.txt
    "
    );
    assert!(map.is_ok());
}

#[test]
fn test_locate_map_by_coordinates() {
    let atlas_infos = vec![
        MapInfo {
            name: "map1".to_string(),
            base_x: 0,
            base_y: 0,
            size_x: 1,
            size_y: 1,
            end_x: 25600,
            end_y: 25600,
        },
        MapInfo {
            name: "map2".to_string(),
            base_x: 25600,
            base_y: 0,
            size_x: 2,
            size_y: 1,
            end_x: 25600 + 51200,
            end_y: 25600,
        }
    ];

    let get_setting_content = |name: &str| -> Option<String> {
        let content = "
            ScriptType MapSetting
            ViewRadius 128
            CellScale 200
            HeightScale 0.5
            MapSize 2 3
            TextureSet test
        ".to_string();
        
        if name == "map1" || name == "map2" {
            Some(content)
        } else {
            None
        }
    };

    let found = locate_map_by_coordinates_internal(100, 100, &atlas_infos, get_setting_content);
    assert!(found.is_some());

    let found2 = locate_map_by_coordinates_internal(26000, 100, &atlas_infos, get_setting_content);
    assert!(found2.is_some());

    let not_found = locate_map_by_coordinates_internal(-10, -10, &atlas_infos, get_setting_content);
    assert!(not_found.is_none());

    let not_found2 = locate_map_by_coordinates_internal(0, 26000, &atlas_infos, get_setting_content);
    assert!(not_found2.is_none());
}
