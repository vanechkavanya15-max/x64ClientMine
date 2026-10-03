#[path = "../src/py_bridge_item_flags.rs"]
mod py_bridge_item_flags;

use py_bridge_item_flags::*;

#[test]
fn test_item_is_metin() {
    assert_eq!(m2_py_item_is_metin(ITEM_TYPE_METIN), 1);
    assert_eq!(m2_py_item_is_metin(ITEM_TYPE_WEAPON), 0);
    assert_eq!(m2_py_item_is_metin(ITEM_TYPE_USE), 0);
}

#[test]
fn test_item_is_refine_scroll() {
    assert_eq!(m2_py_item_is_refine_scroll(ITEM_TYPE_USE, USE_TUNING), 1);
    
    // Boundary cases
    assert_eq!(m2_py_item_is_refine_scroll(ITEM_TYPE_USE, USE_POTION), 0);
    assert_eq!(m2_py_item_is_refine_scroll(ITEM_TYPE_WEAPON, USE_TUNING), 0);
    assert_eq!(m2_py_item_is_refine_scroll(ITEM_TYPE_NONE, USE_TUNING), 0);
}

#[test]
fn test_item_is_detach_scroll() {
    assert_eq!(m2_py_item_is_detach_scroll(ITEM_TYPE_USE, USE_DETACHMENT), 1);
    
    // Boundary cases
    assert_eq!(m2_py_item_is_detach_scroll(ITEM_TYPE_USE, USE_TUNING), 0);
    assert_eq!(m2_py_item_is_detach_scroll(ITEM_TYPE_WEAPON, USE_DETACHMENT), 0);
    assert_eq!(m2_py_item_is_detach_scroll(ITEM_TYPE_NONE, USE_DETACHMENT), 0);
}

#[test]
fn test_item_is_key() {
    assert_eq!(m2_py_item_is_key(ITEM_TYPE_TREASURE_KEY), 1);
    assert_eq!(m2_py_item_is_key(ITEM_TYPE_TREASURE_BOX), 0);
    assert_eq!(m2_py_item_is_key(ITEM_TYPE_WEAPON), 0);
}

#[test]
fn test_item_is_1gold_item() {
    assert_eq!(m2_py_item_is_1gold_item(ITEM_FLAG_COUNT_PER_1GOLD), 1);
    
    // Multiple flags set
    let mixed_flags = ITEM_FLAG_COUNT_PER_1GOLD | ITEM_FLAG_STACKABLE;
    assert_eq!(m2_py_item_is_1gold_item(mixed_flags), 1);
    
    // Flag missing
    assert_eq!(m2_py_item_is_1gold_item(ITEM_FLAG_STACKABLE | ITEM_FLAG_SAVE), 0);
    assert_eq!(m2_py_item_is_1gold_item(0), 0);
}

#[test]
fn test_item_is_anti_flag() {
    assert_eq!(m2_py_item_is_anti_flag(ITEM_ANTIFLAG_WARRIOR, ITEM_ANTIFLAG_WARRIOR), 1);
    
    // Mixed anti flags
    let mixed_anti_flags = ITEM_ANTIFLAG_WARRIOR | ITEM_ANTIFLAG_SURA | ITEM_ANTIFLAG_DROP;
    assert_eq!(m2_py_item_is_anti_flag(mixed_anti_flags, ITEM_ANTIFLAG_WARRIOR), 1);
    assert_eq!(m2_py_item_is_anti_flag(mixed_anti_flags, ITEM_ANTIFLAG_DROP), 1);
    
    // Flag not in anti flags
    assert_eq!(m2_py_item_is_anti_flag(mixed_anti_flags, ITEM_ANTIFLAG_SELL), 0);
    assert_eq!(m2_py_item_is_anti_flag(0, ITEM_ANTIFLAG_WARRIOR), 0);
}

#[test]
fn test_item_is_flag() {
    assert_eq!(m2_py_item_is_flag(ITEM_FLAG_STACKABLE, ITEM_FLAG_STACKABLE), 1);
    
    // Mixed flags
    let mixed_flags = ITEM_FLAG_STACKABLE | ITEM_FLAG_SAVE | ITEM_FLAG_CONFIRM_WHEN_USE;
    assert_eq!(m2_py_item_is_flag(mixed_flags, ITEM_FLAG_STACKABLE), 1);
    assert_eq!(m2_py_item_is_flag(mixed_flags, ITEM_FLAG_CONFIRM_WHEN_USE), 1);
    
    // Flag not set
    assert_eq!(m2_py_item_is_flag(mixed_flags, ITEM_FLAG_UNIQUE), 0);
    assert_eq!(m2_py_item_is_flag(0, ITEM_FLAG_SAVE), 0);
}
