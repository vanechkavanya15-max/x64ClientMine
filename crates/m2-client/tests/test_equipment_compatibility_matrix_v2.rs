#[path = "../src/equipment_compatibility_matrix_v2.rs"]
mod equipment_compatibility_matrix_v2;

use equipment_compatibility_matrix_v2::*;

#[test]
fn test_is_equip_cell_valid_equipment() {
    let pos = TItemPos::new(EWindows::Equipment as u8, C_EQUIPMENT_START + 5);
    assert!(pos.is_equip_cell());

    let pos2 = TItemPos::new(EWindows::Inventory as u8, C_EQUIPMENT_START);
    assert!(pos2.is_equip_cell());

    let pos3 = TItemPos::new(EWindows::Equipment as u8, C_EQUIPMENT_START + C_WEAR_MAX - 1);
    assert!(pos3.is_equip_cell());
}

#[test]
fn test_is_equip_cell_inventory_start() {
    // Inventory starts at 0, not an equipment cell
    let pos = TItemPos::new(EWindows::Inventory as u8, 0);
    assert!(!pos.is_equip_cell());
    
    // Boundary checks
    let pos_just_below = TItemPos::new(EWindows::Inventory as u8, C_EQUIPMENT_START - 1);
    assert!(!pos_just_below.is_equip_cell());

    let pos_just_above = TItemPos::new(EWindows::Equipment as u8, C_EQUIPMENT_START + C_WEAR_MAX);
    assert!(!pos_just_above.is_equip_cell());
}

#[test]
fn test_is_equip_cell_dragon_soul_invalid() {
    let pos = TItemPos::new(EWindows::DragonSoulInventory as u8, C_EQUIPMENT_START + 1);
    assert!(!pos.is_equip_cell());

    let pos2 = TItemPos::new(EWindows::BeltInventory as u8, C_EQUIPMENT_START);
    assert!(!pos2.is_equip_cell());
}

#[test]
fn test_is_equipment_types() {
    assert!(is_equipment(EItemType::Weapon as u8));
    assert!(is_equipment(EItemType::Armor as u8));
    
    assert!(!is_equipment(EItemType::None as u8));
    assert!(!is_equipment(EItemType::Use as u8));
    assert!(!is_equipment(EItemType::Material as u8));
    assert!(!is_equipment(EItemType::Belt as u8));
    assert!(!is_equipment(EItemType::Costume as u8));
}

#[test]
fn test_is_anti_flag_matches() {
    let flags = ITEM_ANTIFLAG_FEMALE | ITEM_ANTIFLAG_WARRIOR | ITEM_ANTIFLAG_SELL;
    
    assert!(is_anti_flag(flags, ITEM_ANTIFLAG_FEMALE));
    assert!(is_anti_flag(flags, ITEM_ANTIFLAG_WARRIOR));
    assert!(is_anti_flag(flags, ITEM_ANTIFLAG_SELL));
}

#[test]
fn test_is_anti_flag_not_matches() {
    let flags = ITEM_ANTIFLAG_FEMALE | ITEM_ANTIFLAG_WARRIOR | ITEM_ANTIFLAG_SELL;
    
    assert!(!is_anti_flag(flags, ITEM_ANTIFLAG_MALE));
    assert!(!is_anti_flag(flags, ITEM_ANTIFLAG_ASSASSIN));
    assert!(!is_anti_flag(flags, ITEM_ANTIFLAG_DROP));
    assert!(!is_anti_flag(flags, ITEM_ANTIFLAG_SAVE));
}

#[test]
fn test_check_item_level_limit() {
    let limits = vec![
        TItemLimit { b_type: ELimitTypes::LimitLevel as u8, l_value: 30 },
        TItemLimit { b_type: ELimitTypes::LimitNone as u8, l_value: 0 },
    ];

    assert!(check_item_level_limit(&limits, 30));
    assert!(check_item_level_limit(&limits, 45));
    assert!(!check_item_level_limit(&limits, 29));
    assert!(!check_item_level_limit(&limits, 1));

    let no_level_limit = vec![
        TItemLimit { b_type: ELimitTypes::LimitStr as u8, l_value: 10 },
    ];
    assert!(check_item_level_limit(&no_level_limit, 1));
}

#[test]
fn test_is_wear_position_valid() {
    // 1 << WearBody (0) = 1
    // 1 << WearWeapon (4) = 16
    let wear_flags = (1 << EWearPositions::WearBody as u32) | (1 << EWearPositions::WearWeapon as u32);
    
    assert!(is_wear_position_valid(wear_flags, EWearPositions::WearBody as u8));
    assert!(is_wear_position_valid(wear_flags, EWearPositions::WearWeapon as u8));
    
    assert!(!is_wear_position_valid(wear_flags, EWearPositions::WearHead as u8));
    assert!(!is_wear_position_valid(wear_flags, EWearPositions::WearShield as u8));
    
    // Out of bounds check
    assert!(!is_wear_position_valid(wear_flags, EWearPositions::WearMaxNum as u8));
    assert!(!is_wear_position_valid(wear_flags, 255));
}
