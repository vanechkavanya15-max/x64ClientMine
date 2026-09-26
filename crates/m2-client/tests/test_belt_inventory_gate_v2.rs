use m2_client::belt_inventory_gate_v2::*;

#[test]
fn test_belt_inventory_gate_invalid_slots_and_levels() {
    // Check out of bounds slot
    assert!(!m2_belt_can_store(0, 16, ITEM_TYPE_POTION));
    assert!(!m2_belt_can_store(9, 20, ITEM_TYPE_POTION));

    // Check out of bounds belt level
    assert!(!m2_belt_can_store(10, 0, ITEM_TYPE_POTION));
    assert!(!m2_belt_can_store(255, 15, ITEM_TYPE_POTION));
}

#[test]
fn test_belt_inventory_gate_invalid_item_types() {
    let invalid_types = [1, 5, 28, 30, 255];
    for &item_type in &invalid_types {
        assert!(!m2_belt_can_store(9, 0, item_type), "Should not allow item type {}", item_type);
    }
    // Also test that ITEM_TYPE_USE (3) is valid
    assert!(m2_belt_can_store(9, 0, ITEM_TYPE_USE));
}

#[test]
fn test_belt_inventory_gate_belt_level_0() {
    // Belt level 0 -> Grade 0
    // None of the rules are <= 0
    for slot in 0..16 {
        assert!(!m2_belt_can_store(0, slot, ITEM_TYPE_POTION));
        assert!(!m2_belt_can_store(0, slot, BELT_ACCEPT));
    }
}

#[test]
fn test_belt_inventory_gate_belt_level_1_and_2() {
    // Belt level 1 and 2 -> Grade 1
    // Available slots: slot 0 (rule 1)
    
    // Level 1
    assert!(m2_belt_can_store(1, 0, ITEM_TYPE_POTION));
    for slot in 1..16 {
        assert!(!m2_belt_can_store(1, slot, ITEM_TYPE_POTION));
    }

    // Level 2
    assert!(m2_belt_can_store(2, 0, BELT_ACCEPT));
    for slot in 1..16 {
        assert!(!m2_belt_can_store(2, slot, BELT_ACCEPT));
    }
}

#[test]
fn test_belt_inventory_gate_belt_level_3_and_4() {
    // Belt level 3 and 4 -> Grade 2
    // Available slots: 0 (rule 1), 1 (rule 2)
    for &level in &[3, 4] {
        assert!(m2_belt_can_store(level, 0, ITEM_TYPE_POTION));
        assert!(m2_belt_can_store(level, 1, ITEM_TYPE_POTION));
        for slot in 2..16 {
            assert!(!m2_belt_can_store(level, slot, ITEM_TYPE_POTION));
        }
    }
}

#[test]
fn test_belt_inventory_gate_belt_level_5() {
    // Belt level 5 -> Grade 3
    // Available slots:
    // rule 1: 0
    // rule 2: 1
    // rule 3: 4, 5
    let expected_available = [0, 1, 4, 5];
    
    for slot in 0..16 {
        let is_expected = expected_available.contains(&slot);
        assert_eq!(m2_belt_can_store(5, slot, ITEM_TYPE_POTION), is_expected, "Slot {} failed for level 5", slot);
    }
}

#[test]
fn test_belt_inventory_gate_belt_level_6() {
    // Belt level 6 -> Grade 4
    // Available slots:
    // rule 1: 0
    // rule 2: 1
    // rule 3: 4, 5
    // rule 4: 2, 6
    let expected_available = [0, 1, 2, 4, 5, 6];
    
    for slot in 0..16 {
        let is_expected = expected_available.contains(&slot);
        assert_eq!(m2_belt_can_store(6, slot, ITEM_TYPE_POTION), is_expected, "Slot {} failed for level 6", slot);
    }
}

#[test]
fn test_belt_inventory_gate_belt_level_7() {
    // Belt level 7 -> Grade 5
    // Available slots:
    // rules <= 5: 0 (r1), 1 (r2), 4,5 (r3), 2,6 (r4), 8,9,10 (r5)
    let expected_available = [0, 1, 2, 4, 5, 6, 8, 9, 10];
    
    for slot in 0..16 {
        let is_expected = expected_available.contains(&slot);
        assert_eq!(m2_belt_can_store(7, slot, ITEM_TYPE_POTION), is_expected, "Slot {} failed for level 7", slot);
    }
}

#[test]
fn test_belt_inventory_gate_belt_level_8() {
    // Belt level 8 -> Grade 6
    // Available slots:
    // rules <= 6: 0(r1), 1(r2), 4,5(r3), 2,6(r4), 8,9,10(r5), 3,7,11(r6)
    let expected_available = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    
    for slot in 0..16 {
        let is_expected = expected_available.contains(&slot);
        assert_eq!(m2_belt_can_store(8, slot, BELT_ACCEPT), is_expected, "Slot {} failed for level 8", slot);
    }
}

#[test]
fn test_belt_inventory_gate_belt_level_9() {
    // Belt level 9 -> Grade 7
    // All rules are <= 7, so all 16 slots are available
    for slot in 0..16 {
        assert!(m2_belt_can_store(9, slot, ITEM_TYPE_POTION));
        assert!(m2_belt_can_store(9, slot, BELT_ACCEPT));
    }
}
