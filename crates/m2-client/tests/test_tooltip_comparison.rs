use m2_client::equipment_slots;
use m2_client::ui_tooltip_item_comparison::{calculate_stat_deltas, find_equipped_slot, StatDelta};

#[test]
fn test_stat_delta_calculation_basic() {
    let equipped = vec![(1, 100), (5, 10)]; // e.g. Max HP +100, STR +10
    let hovered = vec![(1, 150), (5, 8), (8, 5)]; // e.g. Max HP +150, STR +8, Move Speed +5

    let deltas = calculate_stat_deltas(&equipped, &hovered);

    // Expected:
    // Max HP (1) -> 150 - 100 = 50
    // STR (5) -> 8 - 10 = -2
    // Move Speed (8) -> 5 - 0 = 5
    assert_eq!(deltas.len(), 3);

    // Sorted by apply_type: 1, 5, 8
    assert_eq!(deltas[0], StatDelta { apply_type: 1, delta: 50 });
    assert_eq!(deltas[1], StatDelta { apply_type: 5, delta: -2 });
    assert_eq!(deltas[2], StatDelta { apply_type: 8, delta: 5 });
}

#[test]
fn test_stat_delta_missing_in_hovered() {
    let equipped = vec![(3, 20), (4, 15)]; // e.g. VIT +20, INT +15
    let hovered = vec![(4, 15)]; // e.g. INT +15 (unchanged)

    let deltas = calculate_stat_deltas(&equipped, &hovered);

    // Expected:
    // VIT (3) -> 0 - 20 = -20
    // INT (4) -> 15 - 15 = 0 (filtered out)
    assert_eq!(deltas.len(), 1);
    assert_eq!(deltas[0], StatDelta { apply_type: 3, delta: -20 });
}

#[test]
fn test_stat_delta_no_zero_apply_type() {
    // apply_type 0 means no bonus, should be ignored
    let equipped = vec![(0, 100), (5, 10)];
    let hovered = vec![(0, 150), (5, 20)];

    let deltas = calculate_stat_deltas(&equipped, &hovered);

    assert_eq!(deltas.len(), 1);
    assert_eq!(deltas[0], StatDelta { apply_type: 5, delta: 10 });
}

#[test]
fn test_find_equipped_slot() {
    // Test mapping of standard item wearable flags to slot indices
    assert_eq!(find_equipped_slot(equipment_slots::WEARABLE_BODY), Some(equipment_slots::SLOT_ARMOR));
    assert_eq!(find_equipped_slot(equipment_slots::WEARABLE_HEAD), Some(equipment_slots::SLOT_HEAD));
    assert_eq!(find_equipped_slot(equipment_slots::WEARABLE_FOOTS), Some(equipment_slots::SLOT_FOOTS));
    assert_eq!(find_equipped_slot(equipment_slots::WEARABLE_WRIST), Some(equipment_slots::SLOT_WRIST));
    assert_eq!(find_equipped_slot(equipment_slots::WEARABLE_WEAPON), Some(equipment_slots::SLOT_WEAPON));
    assert_eq!(find_equipped_slot(equipment_slots::WEARABLE_NECK), Some(equipment_slots::SLOT_NECK));
    assert_eq!(find_equipped_slot(equipment_slots::WEARABLE_EAR), Some(equipment_slots::SLOT_EAR));
    
    // Unique slot 1
    assert_eq!(find_equipped_slot(equipment_slots::WEARABLE_UNIQUE), Some(equipment_slots::SLOT_UNIQUE1));
    
    assert_eq!(find_equipped_slot(equipment_slots::WEARABLE_ARROW), Some(equipment_slots::SLOT_ARROW));
    assert_eq!(find_equipped_slot(equipment_slots::WEARABLE_SHIELD), Some(equipment_slots::SLOT_SHIELD));
    
    // Test invalid flag
    assert_eq!(find_equipped_slot(1 << 30), None);
}
