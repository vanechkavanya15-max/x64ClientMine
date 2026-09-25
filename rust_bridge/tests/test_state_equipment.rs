use m2_client_bridge::state::equipment::{Equipment, WearSlot, m2_equipment_new, m2_equipment_free, m2_equipment_get_vnum};
use m2_client_bridge::state::inventory_item::InventoryItem;

#[test]
fn test_wear_slot_from_u8() {
    assert_eq!(WearSlot::from_u8(0), Some(WearSlot::Body));
    assert_eq!(WearSlot::from_u8(4), Some(WearSlot::Weapon));
    assert_eq!(WearSlot::from_u8(10), Some(WearSlot::CostumeHair));
    assert_eq!(WearSlot::from_u8(11), None);
    assert_eq!(WearSlot::from_u8(255), None);
}

#[test]
fn test_equipment_initialization() {
    let eq = Equipment::new();
    
    // Upewniamy sie ze wszystkie sloty sa poczatkowo puste
    assert_eq!(eq.get_weapon_vnum(), 0);
    assert_eq!(eq.get_item(WearSlot::Body), None);
}

#[test]
fn test_wear_and_unwear_item() {
    let mut eq = Equipment::new();
    let armor = InventoryItem::new(1, 1010, 1);
    
    // Zakladamy pancerz
    eq.wear(WearSlot::Body, armor.clone());
    assert_eq!(eq.get_item(WearSlot::Body), Some(&armor));
    
    // Zdejmujemy pancerz
    let unweared = eq.unwear(WearSlot::Body);
    assert_eq!(unweared, Some(armor));
    assert_eq!(eq.get_item(WearSlot::Body), None); // Slot znowu pusty
}

#[test]
fn test_get_weapon_vnum() {
    let mut eq = Equipment::new();
    let weapon = InventoryItem::new(2, 2020, 1);
    
    // Brak broni
    assert_eq!(eq.get_weapon_vnum(), 0);
    
    // Zakladamy bron
    eq.wear(WearSlot::Weapon, weapon.clone());
    assert_eq!(eq.get_weapon_vnum(), 2020);
    
    // Zdejmujemy bron
    eq.unwear(WearSlot::Weapon);
    assert_eq!(eq.get_weapon_vnum(), 0);
}

#[test]
fn test_simulation_of_gameplay() {
    let mut eq = Equipment::new();
    
    let armor = InventoryItem::new(1, 100, 1);
    let helmet = InventoryItem::new(2, 101, 1);
    let weapon = InventoryItem::new(3, 104, 1);
    
    // Postac loguje sie do gry i zaklada wyposazenie
    eq.wear(WearSlot::Body, armor.clone());
    eq.wear(WearSlot::Head, helmet.clone());
    eq.wear(WearSlot::Weapon, weapon.clone());
    
    // Weryfikacja zalozonego wyposazenia
    assert_eq!(eq.get_item(WearSlot::Body), Some(&armor));
    assert_eq!(eq.get_item(WearSlot::Head), Some(&helmet));
    assert_eq!(eq.get_item(WearSlot::Weapon), Some(&weapon));
    assert_eq!(eq.get_weapon_vnum(), 104);
    
    // Zmiana broni (zdejmujemy stara i zakladamy nowa)
    let old_weapon = eq.unwear(WearSlot::Weapon);
    assert_eq!(old_weapon, Some(weapon));
    
    let new_weapon = InventoryItem::new(4, 204, 1);
    eq.wear(WearSlot::Weapon, new_weapon.clone());
    
    assert_eq!(eq.get_item(WearSlot::Weapon), Some(&new_weapon));
    assert_eq!(eq.get_weapon_vnum(), 204);
    
    // Pozostale przedmioty bez zmian
    assert_eq!(eq.get_item(WearSlot::Body), Some(&armor));
    assert_eq!(eq.get_item(WearSlot::Head), Some(&helmet));
}

#[test]
fn test_c_ffi_functions() {
    unsafe {
        let ptr = m2_equipment_new();
        assert!(!ptr.is_null());
        
        let weapon = InventoryItem::new(1, 1234, 1);
        (*ptr).wear(WearSlot::Weapon, weapon);
        
        assert_eq!(m2_equipment_get_vnum(WearSlot::Weapon as u8), 0);
        assert_eq!(m2_equipment_get_vnum(WearSlot::Body as u8), 0);
        
        m2_equipment_free(ptr);
        
        // Null checks
        assert_eq!(m2_equipment_get_vnum(WearSlot::Weapon as u8), 0);
    }
}
