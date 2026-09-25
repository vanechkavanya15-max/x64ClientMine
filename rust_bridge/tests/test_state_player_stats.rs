use m2_client_bridge::state::player_stats::PlayerStatsManager;

#[test]
fn test_create_and_remove_player() {
    let mut manager = PlayerStatsManager::new();
    let vid = 100;

    assert!(manager.create_player(vid));
    assert!(!manager.create_player(vid)); // Gracz juz istnieje

    let stats = manager.get_stats(vid);
    assert!(stats.is_some());
    let stats = stats.unwrap();
    assert_eq!(stats.lvl, 0);

    assert!(manager.remove_player(vid));
    assert!(!manager.remove_player(vid)); // Gracz nie istnieje
    assert!(manager.get_stats(vid).is_none());
}

#[test]
fn test_set_lvl() {
    let mut manager = PlayerStatsManager::new();
    let vid = 200;

    assert!(!manager.set_lvl(vid, 10)); // Gracz nie istnieje
    manager.create_player(vid);
    assert!(manager.set_lvl(vid, 10));
    assert_eq!(manager.get_stats(vid).unwrap().lvl, 10);
}

#[test]
fn test_add_exp() {
    let mut manager = PlayerStatsManager::new();
    let vid = 300;

    manager.create_player(vid);
    manager.set_max_exp(vid, 1000);

    assert!(manager.add_exp(vid, 500));
    assert_eq!(manager.get_stats(vid).unwrap().exp, 500);

    // Przekroczenie max exp
    assert!(manager.add_exp(vid, 600));
    assert_eq!(manager.get_stats(vid).unwrap().exp, 1000);

    // Zabezpieczenie przed przepelnieniem bez limitu (gdy max_exp = 0)
    let vid2 = 301;
    manager.create_player(vid2);
    manager.add_exp(vid2, u64::MAX - 100);
    manager.add_exp(vid2, 200);
    assert_eq!(manager.get_stats(vid2).unwrap().exp, u64::MAX);
}

#[test]
fn test_set_max_exp() {
    let mut manager = PlayerStatsManager::new();
    let vid = 400;

    manager.create_player(vid);
    manager.add_exp(vid, 500);
    manager.set_max_exp(vid, 200);
    
    // Ustawienie mniejszego max exp redukuje exp
    assert_eq!(manager.get_stats(vid).unwrap().exp, 200);
}

#[test]
fn test_hp_mp_operations() {
    let mut manager = PlayerStatsManager::new();
    let vid = 500;

    manager.create_player(vid);
    
    assert!(manager.add_hp(vid, 100));
    assert_eq!(manager.get_stats(vid).unwrap().hp, 100);
    assert!(manager.sub_hp(vid, 40));
    assert_eq!(manager.get_stats(vid).unwrap().hp, 60);
    
    // Saturacja ponizej 0
    assert!(manager.sub_hp(vid, 100));
    assert_eq!(manager.get_stats(vid).unwrap().hp, 0);

    // MP
    assert!(manager.add_mp(vid, 50));
    assert_eq!(manager.get_stats(vid).unwrap().mp, 50);
    assert!(manager.sub_mp(vid, 60));
    assert_eq!(manager.get_stats(vid).unwrap().mp, 0);
    
    // Saturacja max
    assert!(manager.add_hp(vid, u32::MAX - 10));
    assert!(manager.add_hp(vid, 20));
    assert_eq!(manager.get_stats(vid).unwrap().hp, u32::MAX);
}

#[test]
fn test_status_points() {
    let mut manager = PlayerStatsManager::new();
    let vid = 600;

    manager.create_player(vid);
    
    assert!(manager.set_status_points(vid, 10, 20, 30, 40));
    let stats = manager.get_stats(vid).unwrap();
    assert_eq!(stats.str_points, 10);
    assert_eq!(stats.dex_points, 20);
    assert_eq!(stats.int_points, 30);
    assert_eq!(stats.con_points, 40);
    
    manager.add_str(vid, 5);
    manager.add_dex(vid, 10);
    manager.add_int(vid, 15);
    manager.add_con(vid, 20);
    
    let stats2 = manager.get_stats(vid).unwrap();
    assert_eq!(stats2.str_points, 15);
    assert_eq!(stats2.dex_points, 30);
    assert_eq!(stats2.int_points, 45);
    assert_eq!(stats2.con_points, 60);

    // Przekroczenie limitow
    manager.add_str(vid, u32::MAX);
    let stats3 = manager.get_stats(vid).unwrap();
    assert_eq!(stats3.str_points, u32::MAX);
}

#[test]
fn test_invalid_vid_operations() {
    let mut manager = PlayerStatsManager::new();
    let invalid_vid = 999;

    assert!(!manager.set_lvl(invalid_vid, 10));
    assert!(!manager.add_exp(invalid_vid, 100));
    assert!(!manager.set_max_exp(invalid_vid, 100));
    assert!(!manager.add_hp(invalid_vid, 100));
    assert!(!manager.sub_hp(invalid_vid, 100));
    assert!(!manager.add_mp(invalid_vid, 100));
    assert!(!manager.sub_mp(invalid_vid, 100));
    assert!(!manager.set_status_points(invalid_vid, 1, 1, 1, 1));
    assert!(!manager.add_str(invalid_vid, 1));
    assert!(!manager.add_dex(invalid_vid, 1));
    assert!(!manager.add_int(invalid_vid, 1));
    assert!(!manager.add_con(invalid_vid, 1));
}

#[test]
fn test_singleton_instance() {
    let instance = PlayerStatsManager::instance();
    let mut writer = instance.write().unwrap();
    
    let vid = 700;
    assert!(writer.create_player(vid));
    writer.set_lvl(vid, 99);
    
    assert_eq!(writer.get_stats(vid).unwrap().lvl, 99);
}
