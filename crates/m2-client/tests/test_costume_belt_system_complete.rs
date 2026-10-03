#[path = "../src/costume_belt_system_complete.rs"]
mod costume_belt_system_complete;

use costume_belt_system_complete::*;

#[test]
fn test_costume_constants() {
    assert_eq!(ITEM_TYPE_COSTUME, 28);
    assert_eq!(COSTUME_BODY, 0);
    assert_eq!(COSTUME_HAIR, 1);
    assert_eq!(WEAR_COSTUME_BODY, 19);
    assert_eq!(WEARABLE_COSTUME_BODY, 1 << 12);
    assert_eq!(APPLY_COSTUME_ATTR_BONUS, 84);
}

#[test]
fn test_belt_constants() {
    assert_eq!(ITEM_TYPE_BELT, 34);
    assert_eq!(WEAR_BELT, 23);
    assert_eq!(WEARABLE_BELT, 1 << 14);
}

#[test]
fn test_equipment_slot_calculations() {
    assert_eq!(C_ITEM_SLOT_COUNT, 90);
    assert_eq!(C_EQUIPMENT_START, 90);
    
    assert_eq!(C_NEW_EQUIPMENT_START, 111);
    assert_eq!(C_EQUIPMENT_RING1, 111);
    assert_eq!(C_EQUIPMENT_RING2, 112);
    assert_eq!(C_EQUIPMENT_BELT, 113);
    
    assert_eq!(C_COSTUME_SLOT_START, 109);
    assert_eq!(C_COSTUME_SLOT_BODY, 109);
    assert_eq!(C_COSTUME_SLOT_HAIR, 110);
    assert_eq!(C_COSTUME_SLOT_END, 111);
}

#[test]
fn test_belt_inventory_slot_calculations() {
    assert_eq!(C_DRAGON_SOUL_EQUIP_START, 122);
    assert_eq!(C_DRAGON_SOUL_EQUIP_END, 134);
    assert_eq!(C_DRAGON_SOUL_EQUIP_RESERVED_COUNT, 18);
    assert_eq!(C_BELT_INVENTORY_SLOT_START, 152);
    assert_eq!(C_BELT_INVENTORY_SLOT_COUNT, 16);
    assert_eq!(C_BELT_INVENTORY_SLOT_END, 168);
    assert_eq!(C_INVENTORY_COUNT, 168);
}

#[test]
fn test_is_belt_inventory_cell() {
    assert!(!is_belt_inventory_cell(151));
    assert!(is_belt_inventory_cell(152));
    assert!(is_belt_inventory_cell(153));
    assert!(is_belt_inventory_cell(167));
    assert!(!is_belt_inventory_cell(168));
}

#[test]
fn test_is_costume_slot() {
    assert!(!is_costume_slot(108));
    assert!(is_costume_slot(109));
    assert!(is_costume_slot(110));
    assert!(!is_costume_slot(111));
}

#[test]
fn test_belt_grade_by_level() {
    assert_eq!(get_belt_grade_by_refine_level(-1), 0);
    assert_eq!(get_belt_grade_by_refine_level(0), 0);
    assert_eq!(get_belt_grade_by_refine_level(1), 1);
    assert_eq!(get_belt_grade_by_refine_level(2), 1);
    assert_eq!(get_belt_grade_by_refine_level(3), 2);
    assert_eq!(get_belt_grade_by_refine_level(4), 2);
    assert_eq!(get_belt_grade_by_refine_level(5), 3);
    assert_eq!(get_belt_grade_by_refine_level(9), 7);
    assert_eq!(get_belt_grade_by_refine_level(10), 7);
}

#[test]
fn test_is_available_belt_inventory_cell() {
    // 0 grade (0) -> nothing available
    assert!(!is_available_belt_inventory_cell(0, 0));
    
    // grade 1 (+1, +2)
    assert!(is_available_belt_inventory_cell(0, 1));
    assert!(!is_available_belt_inventory_cell(1, 1));
    
    // grade 7 (+9) -> all 16 slots available
    for i in 0..16 {
        assert!(is_available_belt_inventory_cell(i, 7));
    }
    
    // out of bounds check
    assert!(!is_available_belt_inventory_cell(16, 7));
}

#[test]
fn test_item_data_costume() {
    let costume = ItemDataCostume {
        vnum: 41001,
        item_type: ITEM_TYPE_COSTUME,
        sub_type: COSTUME_BODY,
        wearable_flag: WEARABLE_COSTUME_BODY,
        apply_type: [APPLY_COSTUME_ATTR_BONUS, 0, 0, 0, 0, 0, 0],
        apply_value: [10, 0, 0, 0, 0, 0, 0],
    };
    assert!(costume.is_costume());
    assert!(!costume.is_belt());
    
    let belt = ItemDataCostume {
        vnum: 18000,
        item_type: ITEM_TYPE_BELT,
        sub_type: 0,
        wearable_flag: WEARABLE_BELT,
        apply_type: [0, 0, 0, 0, 0, 0, 0],
        apply_value: [0, 0, 0, 0, 0, 0, 0],
    };
    assert!(!belt.is_costume());
    assert!(belt.is_belt());
}

#[test]
fn test_ffi_functions() {
    assert!(m2_costume_is_costume_slot(109));
    assert!(m2_belt_is_belt_inventory_cell(152));
    assert!(m2_belt_is_available_cell(0, 1));
    assert_eq!(m2_belt_get_grade_by_level(9), 7);
}
