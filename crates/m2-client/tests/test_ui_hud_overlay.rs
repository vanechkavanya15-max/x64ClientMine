use m2_client::ui_hud_overlay::{PointType, UiHudOverlay};

#[test]
fn test_integration_ui_hud_overlay() {
    let instance = UiHudOverlay::new();
    assert!(instance.is_ready());
}

#[test]
fn test_set_get_status_xor() {
    let mut hud = UiHudOverlay::new();
    
    // Sprawdzenie wartosci domyslnych (point type 0)
    assert_eq!(hud.get_status(0), 0);
    
    hud.set_status(PointType::Hp as u32, 100);
    assert_eq!(hud.get_status(PointType::Hp as u32), 100);
    
    // Test wartosci krawedziowej z ujemna / bardzo duza
    hud.set_status(PointType::Sp as u32, -500);
    assert_eq!(hud.get_status(PointType::Sp as u32), -500);
    
    // Test bledu z za duzym id
    hud.set_status(999, 100);
    assert_eq!(hud.get_status(999), 0);
}

#[test]
fn test_quick_slot_pages() {
    let mut hud = UiHudOverlay::new();
    
    // Testowanie logicznych wartosci stron
    hud.set_quick_page(0);
    assert_eq!(hud.get_quick_page(), 0);
    
    hud.set_quick_page(3);
    assert_eq!(hud.get_quick_page(), 3);
    
    // Test zawijania > MAX_LINE
    hud.set_quick_page(5);
    assert_eq!(hud.get_quick_page(), 1); // 5 % 4 = 1
    
    // Test zawijania liczb ujemnych
    hud.set_quick_page(-1);
    assert_eq!(hud.get_quick_page(), 3); // (4 + (-1 % 4)) % 4 = 3
    
    hud.set_quick_page(-5);
    assert_eq!(hud.get_quick_page(), 3); // (4 + (-5 % 4)) % 4 = 3
}

#[test]
fn test_quick_slot_local_to_global() {
    let mut hud = UiHudOverlay::new();
    hud.set_quick_page(2);
    // page 2 * 8 = 16. Local 3 => 19
    assert_eq!(hud.local_to_global_quick_slot_index(3), 19);
}

#[test]
fn test_stamina_consumption() {
    let mut hud = UiHudOverlay::new();
    
    hud.start_stamina_consume(10, 100);
    assert_eq!(hud.get_status(PointType::Stamina as u32), 100);
    assert_eq!(hud.is_consuming_stamina, true);
    
    hud.update_stamina(1.5);
    // 100 - (1.5 * 10) = 85
    assert_eq!(hud.get_status(PointType::Stamina as u32), 85);
    assert_eq!(hud.current_stamina, 85.0);
    
    // Zuzyj ponizej zera
    hud.update_stamina(10.0);
    assert_eq!(hud.get_status(PointType::Stamina as u32), 0);
    
    hud.stop_stamina_consume(50);
    assert_eq!(hud.get_status(PointType::Stamina as u32), 50);
    assert_eq!(hud.is_consuming_stamina, false);
    
    // Skoro false, update nic nie zrrobi
    hud.update_stamina(1.0);
    assert_eq!(hud.get_status(PointType::Stamina as u32), 50);
}

#[test]
fn test_percentage_calculation() {
    let mut hud = UiHudOverlay::new();
    
    // Zabezpieczenie przed dzieleniem przez 0
    assert_eq!(hud.get_exp_percentage(), 0.0);
    assert_eq!(hud.get_hp_percentage(), 0.0);
    assert_eq!(hud.get_sp_percentage(), 0.0);
    
    hud.set_status(PointType::Hp as u32, 50);
    hud.set_status(PointType::MaxHp as u32, 200);
    assert_eq!(hud.get_hp_percentage(), 25.0);
    
    hud.set_status(PointType::Sp as u32, 30);
    hud.set_status(PointType::MaxSp as u32, 100);
    assert!((hud.get_sp_percentage() - 30.0).abs() < 0.001);
    
    hud.set_status(PointType::Exp as u32, 12345);
    hud.set_status(PointType::NextExp as u32, 100000);
    assert!((hud.get_exp_percentage() - 12.345).abs() < 0.001);
}

#[test]
fn test_target() {
    let mut hud = UiHudOverlay::new();
    assert_eq!(hud.get_target_vid(), 0);
    
    hud.set_target(1337);
    assert_eq!(hud.get_target_vid(), 1337);
}

#[test]
fn test_quick_slot_management() {
    let mut hud = UiHudOverlay::new();
    
    // add
    hud.add_quick_slot(5, 1, 10);
    assert_eq!(hud.player_status.quick_slots[5].slot_type, 1);
    assert_eq!(hud.player_status.quick_slots[5].position, 10);
    
    // add out of bounds
    hud.add_quick_slot(99, 2, 20); // should not panic
    
    // move
    hud.add_quick_slot(6, 2, 20);
    hud.move_quick_slot(5, 6);
    assert_eq!(hud.player_status.quick_slots[5].slot_type, 2);
    assert_eq!(hud.player_status.quick_slots[5].position, 20);
    assert_eq!(hud.player_status.quick_slots[6].slot_type, 1);
    assert_eq!(hud.player_status.quick_slots[6].position, 10);
    
    // move out of bounds
    hud.move_quick_slot(5, 99); // should not panic
    
    // delete
    hud.delete_quick_slot(5);
    assert_eq!(hud.player_status.quick_slots[5].slot_type, 0);
    assert_eq!(hud.player_status.quick_slots[5].position, 0);
    
    // delete out of bounds
    hud.delete_quick_slot(99); // should not panic
}
