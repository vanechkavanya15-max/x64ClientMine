use m2_client_bridge::state::equipment::{EquipmentManager, EquipmentSlot};

#[test]
fn test_equipment_slot_from_index() {
    assert_eq!(EquipmentSlot::from_index(0), Some(EquipmentSlot::Armor));
    assert_eq!(EquipmentSlot::from_index(7), Some(EquipmentSlot::Belt));
    assert_eq!(EquipmentSlot::from_index(8), None);
    assert_eq!(EquipmentSlot::from_index(999), None);
}

#[test]
fn test_manager_initialization() {
    let manager = EquipmentManager::new();
    
    // Upewniamy sie ze wszystkie sloty sa poczatkowo puste
    for i in 0..8 {
        assert_eq!(manager.get_item(i), 0);
    }
}

#[test]
fn test_equip_and_unequip_item() {
    let mut manager = EquipmentManager::new();
    let armor_vid = 1010;
    
    // Zakladamy pancerz (Armor to indeks 0)
    assert!(manager.equip_item(0, armor_vid));
    assert_eq!(manager.get_item(0), armor_vid);
    
    // Zdejmujemy pancerz
    let unequipped_vid = manager.unequip_item(0);
    assert_eq!(unequipped_vid, armor_vid);
    assert_eq!(manager.get_item(0), 0); // Slot znowu pusty
}

#[test]
fn test_out_of_bounds_indices() {
    let mut manager = EquipmentManager::new();
    let vid = 2020;
    
    // Proba zalozenia, pobrania i zdjecia przedmiotu na nieistniejacym slocie (indeks 8)
    assert_eq!(manager.equip_item(8, vid), false);
    assert_eq!(manager.get_item(8), 0);
    assert_eq!(manager.unequip_item(8), 0);
}

#[test]
fn test_invalid_vid() {
    let mut manager = EquipmentManager::new();
    
    // Proba zalozenia przedmiotu o VID rownym 0 (zbroja)
    assert_eq!(manager.equip_item(0, 0), false);
    
    // Slot nadal powinien byc pusty
    assert_eq!(manager.get_item(0), 0);
}

#[test]
fn test_simulation_of_gameplay() {
    let mut manager = EquipmentManager::new();
    
    // Postac loguje sie do gry i zaklada wyposazenie
    manager.equip_item(0, 100); // Armor
    manager.equip_item(1, 101); // Helmet
    manager.equip_item(4, 104); // Weapon
    
    // Weryfikacja zalozonego wyposazenia
    assert_eq!(manager.get_item(0), 100);
    assert_eq!(manager.get_item(1), 101);
    assert_eq!(manager.get_item(4), 104);
    
    // Zmiana broni (zdejmujemy stara i zakladamy nowa)
    let old_weapon = manager.unequip_item(4);
    assert_eq!(old_weapon, 104);
    assert!(manager.equip_item(4, 204)); // Nowa bron
    
    assert_eq!(manager.get_item(4), 204);
    
    // Pozostale przedmioty bez zmian
    assert_eq!(manager.get_item(0), 100);
    assert_eq!(manager.get_item(1), 101);
}
