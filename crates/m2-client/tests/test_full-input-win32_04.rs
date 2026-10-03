#[path = "../src/input_keyboard_dik_mapper.rs"]
mod input_keyboard_dik_mapper;

use input_keyboard_dik_mapper::{vk_to_dik, dik_to_vk};

#[test]
fn test_letters_mapping() {
    // A
    assert_eq!(vk_to_dik(0x41), Some(0x1E));
    assert_eq!(dik_to_vk(0x1E), Some(0x41));
    
    // Z
    assert_eq!(vk_to_dik(0x5A), Some(0x2C));
    assert_eq!(dik_to_vk(0x2C), Some(0x5A));
    
    // M
    assert_eq!(vk_to_dik(0x4D), Some(0x32));
    assert_eq!(dik_to_vk(0x32), Some(0x4D));
}

#[test]
fn test_numbers_mapping() {
    // 0
    assert_eq!(vk_to_dik(0x30), Some(0x0B));
    assert_eq!(dik_to_vk(0x0B), Some(0x30));
    
    // 1
    assert_eq!(vk_to_dik(0x31), Some(0x02));
    assert_eq!(dik_to_vk(0x02), Some(0x31));
    
    // 9
    assert_eq!(vk_to_dik(0x39), Some(0x0A));
    assert_eq!(dik_to_vk(0x0A), Some(0x39));
}

#[test]
fn test_numpad_mapping() {
    // NUMPAD0
    assert_eq!(vk_to_dik(0x60), Some(0x52));
    assert_eq!(dik_to_vk(0x52), Some(0x60));
    
    // NUMPAD7
    assert_eq!(vk_to_dik(0x67), Some(0x47));
    assert_eq!(dik_to_vk(0x47), Some(0x67));
    
    // ADD
    assert_eq!(vk_to_dik(0x6B), Some(0x4E));
    assert_eq!(dik_to_vk(0x4E), Some(0x6B));
}

#[test]
fn test_f_keys_mapping() {
    // F1
    assert_eq!(vk_to_dik(0x70), Some(0x3B));
    assert_eq!(dik_to_vk(0x3B), Some(0x70));
    
    // F10
    assert_eq!(vk_to_dik(0x79), Some(0x44));
    assert_eq!(dik_to_vk(0x44), Some(0x79));
    
    // F12
    assert_eq!(vk_to_dik(0x7B), Some(0x58));
    assert_eq!(dik_to_vk(0x58), Some(0x7B));
}

#[test]
fn test_special_keys_mapping() {
    // ESCAPE
    assert_eq!(vk_to_dik(0x1B), Some(0x01));
    assert_eq!(dik_to_vk(0x01), Some(0x1B));
    
    // RETURN
    assert_eq!(vk_to_dik(0x0D), Some(0x1C));
    assert_eq!(dik_to_vk(0x1C), Some(0x0D));
    
    // SPACE
    assert_eq!(vk_to_dik(0x20), Some(0x39));
    assert_eq!(dik_to_vk(0x39), Some(0x20));
}

#[test]
fn test_unknown_keys() {
    assert_eq!(vk_to_dik(0xFF), None);
    assert_eq!(dik_to_vk(0xFF), None);
    assert_eq!(vk_to_dik(0x00), None);
    assert_eq!(dik_to_vk(0x00), None);
}
