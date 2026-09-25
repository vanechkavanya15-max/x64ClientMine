use m2_client_bridge::state::player_stats::{PlayerStatsManager, PLAYER_STATS};

#[test]
fn test_create_and_remove_player() {
    let mut manager = PlayerStatsManager::new();
    let vid = 100;

    assert!(manager.create_player(vid));
    assert!(!manager.create_player(vid));

    let stats = manager.get_stats(vid);
    assert!(stats.is_some());
    let stats = stats.unwrap();
    assert_eq!(stats.level, 0);

    assert!(manager.remove_player(vid));
    assert!(!manager.remove_player(vid));
    assert!(manager.get_stats(vid).is_none());
}

#[test]
fn test_set_lvl() {
    let mut manager = PlayerStatsManager::new();
    let vid = 200;

    assert!(!manager.set_lvl(vid, 10));
    manager.create_player(vid);
    assert!(manager.set_lvl(vid, 10));
    assert_eq!(manager.get_stats(vid).unwrap().level, 10);
}

#[test]
fn test_add_exp() {
    let mut manager = PlayerStatsManager::new();
    let vid = 300;

    manager.create_player(vid);
    manager.set_max_exp(vid, 1000);

    assert!(manager.add_exp(vid, 500));
    assert_eq!(manager.get_stats(vid).unwrap().exp, 500);

    assert!(manager.add_exp(vid, 600));
    assert_eq!(manager.get_stats(vid).unwrap().exp, 1000);

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
    
    assert!(manager.sub_hp(vid, 100));
    assert_eq!(manager.get_stats(vid).unwrap().hp, 0);

    assert!(manager.add_mp(vid, 50));
    assert_eq!(manager.get_stats(vid).unwrap().sp, 50);
    assert!(manager.sub_mp(vid, 60));
    assert_eq!(manager.get_stats(vid).unwrap().sp, 0);
    
    assert!(manager.add_hp(vid, i32::MAX - 10));
    assert!(manager.add_hp(vid, 20));
    assert_eq!(manager.get_stats(vid).unwrap().hp, i32::MAX);
}

#[test]
fn test_update_point() {
    let mut manager = PlayerStatsManager::new();
    let vid = 600;

    manager.create_player(vid);
    if let Some(mut player) = manager.get_stats(vid) {
        player.update_point(PLAYER_STATS, 1000);
        assert_eq!(player.gold, 1000);
    }
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
}

#[test]
fn test_singleton_instance() {
    let instance = PlayerStatsManager::instance();
    let mut writer = instance.write().unwrap();
    
    let vid = 700;
    assert!(writer.create_player(vid));
    writer.set_lvl(vid, 99);
    
    assert_eq!(writer.get_stats(vid).unwrap().level, 99);
}

#[test]
fn test_hp_percentage_and_is_alive() {
    let mut manager = PlayerStatsManager::new();
    let vid = 800;

    manager.create_player(vid);
    
    // Simulate updating player locally
    if let Some(mut player) = manager.get_stats(vid) {
        player.max_hp = 200;
        player.hp = 100;
        assert_eq!(player.hp_percentage(), 50.0);
        assert!(player.is_alive());
        
        player.hp = 0;
        assert_eq!(player.hp_percentage(), 0.0);
        assert!(!player.is_alive());
        
        player.max_hp = 0;
        assert_eq!(player.hp_percentage(), 0.0);
    }
}
