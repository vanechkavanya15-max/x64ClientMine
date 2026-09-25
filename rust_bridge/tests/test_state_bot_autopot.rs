use m2_client_bridge::state::bot_autopot::AutopotManager;

#[test]
fn test_default_config_is_empty_and_safe() {
    let manager = AutopotManager::new();
    // Bledne VID powinno bezpiecznie zwrocic None
    assert_eq!(manager.get_config(999), None);
    
    // Proba uzycia potki na nieistniejacym VID
    assert_eq!(manager.should_use_hp(999, 100, 100), false);
    assert_eq!(manager.should_use_mp(999, 100, 100), false);
}

#[test]
fn test_set_and_get_config() {
    let mut manager = AutopotManager::new();
    let vid = 1;
    
    manager.set_hp_config(vid, true, 50);
    manager.set_mp_config(vid, true, 30);
    
    let config = manager.get_config(vid).unwrap();
    assert_eq!(config.hp.enabled, true);
    assert_eq!(config.hp.threshold_percent, 50);
    assert_eq!(config.mp.enabled, true);
    assert_eq!(config.mp.threshold_percent, 30);
}

#[test]
fn test_threshold_percent_capped_at_100() {
    let mut manager = AutopotManager::new();
    let vid = 2;
    
    // Probujemy ustawic prog > 100
    manager.set_hp_config(vid, true, 150);
    manager.set_mp_config(vid, true, 255);
    
    let config = manager.get_config(vid).unwrap();
    // Powinno byc obciete do 100
    assert_eq!(config.hp.threshold_percent, 100);
    assert_eq!(config.mp.threshold_percent, 100);
}

#[test]
fn test_remove_config() {
    let mut manager = AutopotManager::new();
    let vid = 3;
    
    manager.set_hp_config(vid, true, 50);
    assert!(manager.get_config(vid).is_some());
    
    manager.remove_config(vid);
    assert!(manager.get_config(vid).is_none());
}

#[test]
fn test_should_use_hp_logic() {
    let mut manager = AutopotManager::new();
    let vid = 4;
    
    // Prog to 50%
    manager.set_hp_config(vid, true, 50);
    
    // 60/100 to 60%, wieksze od 50% -> false
    assert_eq!(manager.should_use_hp(vid, 60, 100), false);
    
    // 50/100 to 50%, rowne progowi -> true
    assert_eq!(manager.should_use_hp(vid, 50, 100), true);
    
    // 40/100 to 40%, mniejsze od 50% -> true
    assert_eq!(manager.should_use_hp(vid, 40, 100), true);
}

#[test]
fn test_should_use_mp_logic() {
    let mut manager = AutopotManager::new();
    let vid = 5;
    
    // Prog to 80%
    manager.set_mp_config(vid, true, 80);
    
    // 90/100 to 90%, wieksze od 80% -> false
    assert_eq!(manager.should_use_mp(vid, 90, 100), false);
    
    // 80/100 to 80%, rowne progowi -> true
    assert_eq!(manager.should_use_mp(vid, 80, 100), true);
    
    // 20/100 to 20%, mniejsze od 80% -> true
    assert_eq!(manager.should_use_mp(vid, 20, 100), true);
}

#[test]
fn test_disabled_config_returns_false() {
    let mut manager = AutopotManager::new();
    let vid = 6;
    
    // Konfiguracja wylaczona, mimo ze prog wysoki
    manager.set_hp_config(vid, false, 99);
    
    // Nawet przy 10% zycia, powinnismy dostac false
    assert_eq!(manager.should_use_hp(vid, 10, 100), false);
}

#[test]
fn test_zero_max_hp_mp_avoid_division_by_zero() {
    let mut manager = AutopotManager::new();
    let vid = 7;
    
    manager.set_hp_config(vid, true, 50);
    manager.set_mp_config(vid, true, 50);
    
    // Max HP = 0, nie powinno crashowac ani rzucac bledu, powinno po prostu dac false
    assert_eq!(manager.should_use_hp(vid, 0, 0), false);
    assert_eq!(manager.should_use_hp(vid, 10, 0), false);
    
    // Max MP = 0
    assert_eq!(manager.should_use_mp(vid, 0, 0), false);
    assert_eq!(manager.should_use_mp(vid, 10, 0), false);
}
