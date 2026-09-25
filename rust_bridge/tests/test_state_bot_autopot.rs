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

use m2_client_bridge::state::bot_autopot::{AutoPotEngine, POTION_COOLDOWN_MS};

#[test]
fn test_autopot_engine_hp_threshold_logic() {
    let mut engine = AutoPotEngine::new();
    engine.is_enabled = true;
    engine.hp_threshold_pct = 70;

    // HP=71% -> false
    assert_eq!(engine.should_pot_hp(71, 1000), false);
    
    // HP=70% -> true
    assert_eq!(engine.should_pot_hp(70, 1000), true);

    // HP=50% -> true (Wyzwalanie pota przy HP=50% gdy prog to 70%)
    assert_eq!(engine.should_pot_hp(50, 1000), true);
}

#[test]
fn test_autopot_engine_cooldown_logic() {
    let mut engine = AutoPotEngine::new();
    engine.is_enabled = true;
    engine.hp_threshold_pct = 70;

    // Poczatkowe uzycie ok
    let current_time = 1000;
    assert_eq!(engine.should_pot_hp(50, current_time), true);
    engine.register_pot_used(current_time);

    // Zaraz po uzyciu, czas ten sam - zablokowane
    assert_eq!(engine.should_pot_hp(50, current_time), false);

    // Czas tuz przed uplywem cooldownu - zablokowane
    assert_eq!(engine.should_pot_hp(50, current_time + POTION_COOLDOWN_MS - 1), false);

    // Czas po uplywie cooldownu - mozna znow uzyc
    assert_eq!(engine.should_pot_hp(50, current_time + POTION_COOLDOWN_MS), true);
    
    // Przewinienie czasu (wrap-around u32)
    let wrap_around_time = u32::MAX - 500;
    engine.register_pot_used(wrap_around_time);
    
    // Przeszlo 500ms
    assert_eq!(engine.should_pot_hp(50, u32::MAX), false);
    
    // Przeszlo 500ms do max, potem 500ms od 0 -> 1000ms
    assert_eq!(engine.should_pot_hp(50, 500), true); // 500 ms + 500 ms = 1000 ms -> >= cooldown
}

#[test]
fn test_ffi_functions() {
    // Sprawdzenie alokacji i dealokacji nie powinnna crashowac
    unsafe {
        let ptr = m2_client_bridge::state::bot_autopot::m2_autopot_create();
        assert!(!ptr.is_null());
        m2_client_bridge::state::bot_autopot::m2_autopot_free(ptr);
        
        // Zwalnianie null_mut nie crashuje
        m2_client_bridge::state::bot_autopot::m2_autopot_free(std::ptr::null_mut());
    }

    // Sprawdzenie globalnego ffi check
    unsafe {
        m2_client_bridge::state::bot_autopot::m2_autopot_set_config(70, 70, true);
        
        // Powinno uzyc bo HP=50 < 70
        let result1 = m2_client_bridge::state::bot_autopot::m2_autopot_check(50);
        assert_eq!(result1, true);
        
        // Zaraz po powinno byc zablokowane przez cooldown (zakladajac ze czas systemowy nie skoczyl o > 1000ms w trakcie wywolania)
        let result2 = m2_client_bridge::state::bot_autopot::m2_autopot_check(50);
        assert_eq!(result2, false);
    }
}
