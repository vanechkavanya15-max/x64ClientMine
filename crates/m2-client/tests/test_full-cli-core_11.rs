#[path = "../src/full-cli-core_11.rs"]
mod full_cli_core;

use full_cli_core::game_state_payload_validator::GameStatePayloadValidator;

#[test]
fn test_yang_valid_amount() {
    assert!(GameStatePayloadValidator::validate_yang(0));
    assert!(GameStatePayloadValidator::validate_yang(1000));
    assert!(GameStatePayloadValidator::validate_yang(2_000_000_000));
}

#[test]
fn test_yang_invalid_amount() {
    assert!(!GameStatePayloadValidator::validate_yang(-1));
    assert!(!GameStatePayloadValidator::validate_yang(2_000_000_001));
}

#[test]
fn test_inventory_valid_slot() {
    // INVENTORY ma 180 slotow (od 0 do 179)
    assert!(GameStatePayloadValidator::validate_inventory_slot(1, 0));
    assert!(GameStatePayloadValidator::validate_inventory_slot(1, 179));
    
    // EQUIPMENT ma 32 sloty
    assert!(GameStatePayloadValidator::validate_inventory_slot(2, 31));
    
    // SAFEBOX ma 135
    assert!(GameStatePayloadValidator::validate_inventory_slot(3, 134));
    
    // MALL ma 45
    assert!(GameStatePayloadValidator::validate_inventory_slot(4, 44));
    
    // DRAGON_SOUL_INVENTORY ma 960
    assert!(GameStatePayloadValidator::validate_inventory_slot(5, 959));
    
    // BELT_INVENTORY ma 16
    assert!(GameStatePayloadValidator::validate_inventory_slot(7, 15));
}

#[test]
fn test_inventory_invalid_slot() {
    assert!(!GameStatePayloadValidator::validate_inventory_slot(1, 180));
    assert!(!GameStatePayloadValidator::validate_inventory_slot(2, 32));
    assert!(!GameStatePayloadValidator::validate_inventory_slot(3, 135));
    assert!(!GameStatePayloadValidator::validate_inventory_slot(4, 45));
    assert!(!GameStatePayloadValidator::validate_inventory_slot(5, 960));
    assert!(!GameStatePayloadValidator::validate_inventory_slot(7, 16));
}

#[test]
fn test_invalid_window_type() {
    assert!(!GameStatePayloadValidator::validate_inventory_slot(0, 0));
    assert!(!GameStatePayloadValidator::validate_inventory_slot(6, 0));
    assert!(!GameStatePayloadValidator::validate_inventory_slot(8, 0));
    assert!(!GameStatePayloadValidator::validate_inventory_slot(255, 0));
}
