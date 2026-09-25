use m2_client::background_mgr::{BackgroundMgr, DayMode, DistanceSetNum, ShadowLevel, TERRAIN_XSIZE, TERRAIN_YSIZE};

#[test]
fn test_integration_background_mgr() {
    let instance = BackgroundMgr::new();
    assert!(instance.is_ready());
}

#[test]
fn test_parse_atlas_info() {
    let mut bg = BackgroundMgr::new();
    let atlas_content = "\
metin2_map_a1 10000 10000 1 1
metin2_map_b1 20000 30000 2 2
\n
";
    bg.parse_atlas_info(atlas_content);
    assert_eq!(bg.map_info_vector.len(), 2);
    
    let a1 = &bg.map_info_vector[0];
    assert_eq!(a1.name, "metin2_map_a1");
    assert_eq!(a1.base_x, 10000);
    assert_eq!(a1.base_y, 10000);
    assert_eq!(a1.size_x, 1);
    assert_eq!(a1.size_y, 1);
    assert_eq!(a1.end_x, 10000 + 1 * TERRAIN_XSIZE);
    assert_eq!(a1.end_y, 10000 + 1 * TERRAIN_YSIZE);

    let b1 = &bg.map_info_vector[1];
    assert_eq!(b1.name, "metin2_map_b1");
    assert_eq!(b1.base_x, 20000);
    assert_eq!(b1.base_y, 30000);
    assert_eq!(b1.size_x, 2);
    assert_eq!(b1.size_y, 2);
    assert_eq!(b1.end_x, 20000 + 2 * TERRAIN_XSIZE);
    assert_eq!(b1.end_y, 30000 + 2 * TERRAIN_YSIZE);
}

#[test]
fn test_global_position_to_map_info() {
    let mut bg = BackgroundMgr::new();
    let atlas_content = "metin2_map_a1 10000 10000 1 1";
    bg.parse_atlas_info(atlas_content);
    
    let info = bg.global_position_to_map_info(15000, 15000).unwrap();
    assert_eq!(info.name, "metin2_map_a1");

    let none_info = bg.global_position_to_map_info(5000, 5000);
    assert!(none_info.is_none());
}

#[test]
fn test_warp() {
    let mut bg = BackgroundMgr::new();
    let atlas_content = "metin2_map_a1 10000 10000 1 1";
    bg.parse_atlas_info(atlas_content);
    
    let success = bg.warp(15000, 15000);
    assert!(success);
    assert_eq!(bg.map_name, "metin2_map_a1");
    assert_eq!(bg.base_x, 10000);
    assert_eq!(bg.base_y, 10000);
}

#[test]
fn test_position_conversions() {
    let mut bg = BackgroundMgr::new();
    let atlas_content = "metin2_map_a1 10000 10000 1 1";
    bg.parse_atlas_info(atlas_content);
    bg.warp(15000, 15000);
    
    let mut x = 12000;
    let mut y = 13000;
    bg.global_position_to_local_position(&mut x, &mut y);
    assert_eq!(x, 2000);
    assert_eq!(y, 3000);
    
    bg.local_position_to_global_position(&mut x, &mut y);
    assert_eq!(x, 12000);
    assert_eq!(y, 13000);
}

#[test]
fn test_view_distance_set() {
    let mut bg = BackgroundMgr::new();
    
    bg.set_view_distance_set(DistanceSetNum::Distance2, 10000.0);
    
    // Test selecting the view distance directly
    bg.select_view_distance_num(DistanceSetNum::Distance2);
    assert_eq!(bg.get_far_clip(), 10000.0);
    
    let (num, start, end, far) = bg.get_distance_set_info();
    assert_eq!(num, DistanceSetNum::Distance2);
    assert_eq!(start, 10000.0 * 0.5);
    assert_eq!(end, 10000.0 * 0.7);
    assert_eq!(far, 10000.0);
}

#[test]
fn test_day_night_and_guild_area() {
    let mut bg = BackgroundMgr::new();
    
    assert_eq!(bg.day_mode, DayMode::Light);
    bg.change_to_night();
    assert_eq!(bg.day_mode, DayMode::Dark);
    bg.change_to_day();
    assert_eq!(bg.day_mode, DayMode::Light);

    assert!(!bg.visible_guild_area);
    bg.visible_guild_area();
    assert!(bg.visible_guild_area);
    bg.disable_guild_area();
    assert!(!bg.visible_guild_area);
}

#[test]
fn test_shadow_level() {
    let mut bg = BackgroundMgr::new();
    
    bg.set_shadow_level(ShadowLevel::AllMax);
    assert_eq!(bg.shadow_level, ShadowLevel::AllMax);
}
