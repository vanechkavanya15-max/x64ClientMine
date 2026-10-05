use m2_client::game_auto_potion_manager::{AUTO_POTION_MANAGER, GameAutoPotionManager, m2_py_player_set_auto_potion_info, m2_py_player_get_auto_potion_info};
use std::time::{Instant, Duration};

#[test]
fn test_default_values() {
    let _a = &AUTO_POTION_MANAGER;
    let manager = GameAutoPotionManager::new();
    let hp_info = manager.get_info(0).unwrap();
    assert!(!hp_info.is_activated);
    assert_eq!(hp_info.current_amount, 0);
    
    let sp_info = manager.get_info(1).unwrap();
    assert!(!sp_info.is_activated);
    assert_eq!(sp_info.current_amount, 0);
}

#[test]
fn test_set_and_get_info() {
    // Check setting for HP (0)
    let res1 = m2_py_player_set_auto_potion_info(0, true, 1000, 2000, 10);
    assert!(res1);
    
    let mut is_act = false;
    let mut c_amt = 0;
    let mut t_amt = 0;
    let mut i_slot = 0;
    
    let res2 = m2_py_player_get_auto_potion_info(0, &mut is_act, &mut c_amt, &mut t_amt, &mut i_slot);
    assert!(res2);
    assert!(is_act);
    assert_eq!(c_amt, 1000);
    assert_eq!(t_amt, 2000);
    assert_eq!(i_slot, 10);
    
    // Check invalid type
    let res_invalid = m2_py_player_set_auto_potion_info(2, true, 0, 0, 0);
    assert!(!res_invalid);
}

#[test]
fn test_update_triggers_use() {
    let mut manager = GameAutoPotionManager::new();
    
    // Ustawienie progu na 50% = 0.5
    manager.hp_threshold_pct = 0.5;
    manager.set_info(0, true, 1000, 2000, 15);
    
    // HP jest na 60% - nie powinno odpalic mikstury
    let (hp_trigger, _) = manager.update(0.6, 1.0);
    assert_eq!(hp_trigger, None);
    
    // HP jest na 40% - powinno odpalic
    let (hp_trigger2, _) = manager.update(0.4, 1.0);
    assert_eq!(hp_trigger2, Some(15));
    
    // Zaraz po tym HP nadal 40% - nie powinno odpalic bo cooldown (250ms)
    let (hp_trigger3, _) = manager.update(0.4, 1.0);
    assert_eq!(hp_trigger3, None);
    
    // Sprawdzamy SP
    manager.sp_threshold_pct = 0.5;
    manager.set_info(1, true, 500, 1000, 20);
    
    // SP na 30% - powinno odpalic SP
    let (_, sp_trigger) = manager.update(0.9, 0.3);
    assert_eq!(sp_trigger, Some(20));
    
    // Test recznego przesuniecia czasu dla cooldownu
    manager.last_sp_use_time = Some(Instant::now() - Duration::from_millis(300));
    let (_, sp_trigger2) = manager.update(0.9, 0.3);
    assert_eq!(sp_trigger2, Some(20));
}
