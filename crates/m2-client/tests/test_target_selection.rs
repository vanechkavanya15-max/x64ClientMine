// Testy jednostkowe modulu Target Selection
// ZAKAZ PISANIA ASTRAP - kazdy test sprawdza logike z implementacji

#[path = "../src/target_selection.rs"]
mod target_selection;

use target_selection::TargetManager;

#[test]
fn test_new_target_manager_is_empty() {
    let tm = TargetManager::new();
    assert_eq!(tm.selected_target_vid, None);
    assert_eq!(tm.target_hp_percentage, 0);
}

#[test]
fn test_select_target_valid_hp() {
    let mut tm = TargetManager::new();
    tm.select_target(101, 50, 100);
    assert_eq!(tm.selected_target_vid, Some(101));
    assert_eq!(tm.target_hp_percentage, 50);
}

#[test]
fn test_select_target_full_hp() {
    let mut tm = TargetManager::new();
    tm.select_target(102, 500, 500);
    assert_eq!(tm.selected_target_vid, Some(102));
    assert_eq!(tm.target_hp_percentage, 100);
}

#[test]
fn test_select_target_over_hp() {
    // Current hp > max hp
    let mut tm = TargetManager::new();
    tm.select_target(103, 600, 500);
    assert_eq!(tm.selected_target_vid, Some(103));
    assert_eq!(tm.target_hp_percentage, 100);
}

#[test]
fn test_select_target_dead_initially() {
    let mut tm = TargetManager::new();
    tm.select_target(104, 0, 100);
    // Powinien odznaczyc cel poniewaz current_hp == 0
    assert_eq!(tm.selected_target_vid, None);
    assert_eq!(tm.target_hp_percentage, 0);
    
    tm.select_target(105, -10, 100);
    assert_eq!(tm.selected_target_vid, None);
    assert_eq!(tm.target_hp_percentage, 0);
}

#[test]
fn test_select_target_zero_max_hp() {
    let mut tm = TargetManager::new();
    tm.select_target(106, 50, 0);
    // Błędne wejscie max_hp <= 0 -> hp_percentage = 0, target selected
    assert_eq!(tm.selected_target_vid, Some(106));
    assert_eq!(tm.target_hp_percentage, 0);
    
    tm.select_target(107, 50, -50);
    assert_eq!(tm.selected_target_vid, Some(107));
    assert_eq!(tm.target_hp_percentage, 0);
}

#[test]
fn test_update_target_hp_normal() {
    let mut tm = TargetManager::new();
    tm.select_target(201, 1000, 1000);
    assert_eq!(tm.target_hp_percentage, 100);
    
    tm.update_target_hp(250, 1000);
    assert_eq!(tm.target_hp_percentage, 25);
    
    tm.update_target_hp(333, 1000);
    assert_eq!(tm.target_hp_percentage, 33);
}

#[test]
fn test_update_target_hp_death_unselects() {
    let mut tm = TargetManager::new();
    tm.select_target(202, 100, 100);
    
    tm.update_target_hp(0, 100);
    assert_eq!(tm.selected_target_vid, None);
    assert_eq!(tm.target_hp_percentage, 0);
}

#[test]
fn test_update_target_hp_negative_hp_unselects() {
    let mut tm = TargetManager::new();
    tm.select_target(203, 100, 100);
    
    tm.update_target_hp(-50, 100);
    assert_eq!(tm.selected_target_vid, None);
    assert_eq!(tm.target_hp_percentage, 0);
}

#[test]
fn test_update_target_hp_without_target() {
    let mut tm = TargetManager::new();
    tm.update_target_hp(50, 100);
    // Nic nie powinno sie zmienic
    assert_eq!(tm.selected_target_vid, None);
    assert_eq!(tm.target_hp_percentage, 0);
}

#[test]
fn test_update_target_hp_zero_max_hp() {
    let mut tm = TargetManager::new();
    tm.select_target(204, 100, 100);
    
    tm.update_target_hp(50, 0);
    // max_hp = 0, invalid
    assert_eq!(tm.selected_target_vid, Some(204));
    assert_eq!(tm.target_hp_percentage, 0);
}

#[test]
fn test_clear_target() {
    let mut tm = TargetManager::new();
    tm.select_target(301, 50, 100);
    
    tm.clear_target();
    assert_eq!(tm.selected_target_vid, None);
    assert_eq!(tm.target_hp_percentage, 0);
}

#[test]
fn test_check_target_distance_in_range() {
    let mut tm = TargetManager::new();
    tm.select_target(401, 100, 100);
    
    tm.check_target_distance(2500.0, 5000.0);
    assert_eq!(tm.selected_target_vid, Some(401));
    assert_eq!(tm.target_hp_percentage, 100);
}

#[test]
fn test_check_target_distance_out_of_range() {
    let mut tm = TargetManager::new();
    tm.select_target(402, 100, 100);
    
    tm.check_target_distance(5001.0, 5000.0);
    // Poza zasiegiem, powinno odznaczyc
    assert_eq!(tm.selected_target_vid, None);
    assert_eq!(tm.target_hp_percentage, 0);
}

#[test]
fn test_change_target_vid() {
    let mut tm = TargetManager::new();
    tm.select_target(501, 50, 100);
    assert_eq!(tm.selected_target_vid, Some(501));
    
    // Przelaczenie na inny cel
    tm.select_target(502, 75, 100);
    assert_eq!(tm.selected_target_vid, Some(502));
    assert_eq!(tm.target_hp_percentage, 75);
}

#[test]
fn test_rounding_down_hp() {
    let mut tm = TargetManager::new();
    tm.select_target(601, 1, 1000);
    // 1 / 1000 = 0.1%, powinno zwrocic 0, ale vid zostaje wybrane poniewaz 1 > 0
    assert_eq!(tm.selected_target_vid, Some(601));
    assert_eq!(tm.target_hp_percentage, 0);
}

#[test]
fn test_rounding_up_hp() {
    let mut tm = TargetManager::new();
    tm.select_target(602, 999, 1000);
    // 999 / 1000 = 99.9%, powinno zwrocic 99
    assert_eq!(tm.selected_target_vid, Some(602));
    assert_eq!(tm.target_hp_percentage, 99);
}
