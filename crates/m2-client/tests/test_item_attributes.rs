use m2_client::item_attributes::*;

#[test]
fn test_set_and_get_attribute() {
    let mut item = ItemInstance::new();
    
    // Test setting and getting HP bonus
    assert!(item.set_attribute(0, APPLY_MAX_HP, 2000));
    let attr = item.get_attribute(0).expect("Should have attribute at index 0");
    assert_eq!(attr.attr_type, APPLY_MAX_HP);
    assert_eq!(attr.value, 2000);
}

#[test]
fn test_attribute_slot_limit() {
    let mut item = ItemInstance::new();
    
    // Should be able to set attribute at the last valid index
    assert!(item.set_attribute(ITEM_ATTRIBUTE_SLOT_MAX_NUM - 1, APPLY_STR, 12));
    
    // Out of bounds should fail safely
    assert!(!item.set_attribute(ITEM_ATTRIBUTE_SLOT_MAX_NUM, APPLY_INT, 12));
    assert!(item.get_attribute(ITEM_ATTRIBUTE_SLOT_MAX_NUM).is_none());
}

#[test]
fn test_calculate_total_attributes_multiple_items() {
    let mut item1 = ItemInstance::new();
    item1.set_attribute(0, APPLY_MAX_HP, 1500);
    item1.set_attribute(1, APPLY_STR, 10);
    
    let mut item2 = ItemInstance::new();
    item2.set_attribute(0, APPLY_MAX_HP, 2000);
    item2.set_attribute(1, APPLY_ATT_SPEED, 8);

    let items = vec![&item1, &item2];
    let totals = calculate_total_attributes(&items);
    
    // HP should sum up: 1500 + 2000 = 3500
    assert_eq!(totals.get(&APPLY_MAX_HP), Some(&3500));
    
    // STR should be 10
    assert_eq!(totals.get(&APPLY_STR), Some(&10));
    
    // Attack speed should be 8
    assert_eq!(totals.get(&APPLY_ATT_SPEED), Some(&8));
    
    // Other attributes should not be present
    assert_eq!(totals.get(&APPLY_INT), None);
}

#[test]
fn test_calculate_total_attributes_ignores_none() {
    let mut item = ItemInstance::new();
    item.set_attribute(0, APPLY_NONE, 100);
    item.set_attribute(1, APPLY_MAX_SP, 500);
    
    let items = vec![&item];
    let totals = calculate_total_attributes(&items);
    
    // APPLY_NONE should be ignored
    assert!(!totals.contains_key(&APPLY_NONE));
    
    // APPLY_MAX_SP should be included
    assert_eq!(totals.get(&APPLY_MAX_SP), Some(&500));
}

#[test]
fn test_calculate_total_attributes_negative_values() {
    let mut item1 = ItemInstance::new();
    item1.set_attribute(0, APPLY_MOV_SPEED, 10);
    
    let mut item2 = ItemInstance::new();
    item2.set_attribute(0, APPLY_MOV_SPEED, -5);
    
    let items = vec![&item1, &item2];
    let totals = calculate_total_attributes(&items);
    
    // Values can be negative, 10 - 5 = 5
    assert_eq!(totals.get(&APPLY_MOV_SPEED), Some(&5));
}
