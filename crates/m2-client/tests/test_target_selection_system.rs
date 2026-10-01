// crates/m2-client/tests/test_target_selection_system.rs

use m2_client::target_selection_system::{
    TargetSelectionSystem, TPacketCGTarget, HEADER_CG_TARGET
};
use std::ptr;

#[test]
fn test_pick_target_valid() {
    let system = TargetSelectionSystem::new();
    // Bounding sphere = 50.0. Actor na (100, 100, 100), klikniecie (110, 100, 100) -> Dystans = 10 (mieszczący sie w 50)
    let is_picked = system.pick_target(
        110.0, 100.0, 100.0,
        100.0, 100.0, 100.0,
        50.0
    );
    assert!(is_picked, "Should pick the target within bounding sphere");
}

#[test]
fn test_pick_target_miss() {
    let system = TargetSelectionSystem::new();
    // Bounding sphere = 50.0. Actor na (100, 100, 100), klikniecie (200, 100, 100) -> Dystans = 100 (> 50)
    let is_picked = system.pick_target(
        200.0, 100.0, 100.0,
        100.0, 100.0, 100.0,
        50.0
    );
    assert!(!is_picked, "Should miss the target outside bounding sphere");
}

#[test]
fn test_select_target_sends_packet() {
    let mut system = TargetSelectionSystem::new();
    let mut buffer: [u8; 128] = [0; 128];
    
    let bytes_written = system.select_target(
        42,
        "Dziki Pies".to_string(),
        50,
        100,
        1,
        1,
        0,
        &mut buffer,
    ).unwrap();
    
    assert_eq!(bytes_written, std::mem::size_of::<TPacketCGTarget>());
    assert_eq!(system.selected_target_vid, Some(42));
    
    let info = system.target_board_info.unwrap();
    assert_eq!(info.name, "Dziki Pies");
    assert_eq!(info.hp_percentage, 50);
    assert_eq!(info.level, 1);
    assert_eq!(info.empire_id, 1);
    
    // Check packet contents
    let packet_ptr = buffer.as_ptr() as *const TPacketCGTarget;
    let packet = unsafe { ptr::read_unaligned(packet_ptr) };
    
    // Copy fields to unaligned local variables to avoid E0793
    let header = { packet.header };
    let dw_vid = { packet.dw_vid };
    
    assert_eq!(header, HEADER_CG_TARGET);
    assert_eq!(dw_vid, 42);
}

#[test]
fn test_update_hp_percentage() {
    let mut system = TargetSelectionSystem::new();
    let mut buffer: [u8; 128] = [0; 128];
    system.select_target(
        42,
        "Kowal".to_string(),
        1000,
        1000,
        1,
        1,
        0,
        &mut buffer,
    ).unwrap();
    
    assert_eq!(system.target_board_info.as_ref().unwrap().hp_percentage, 100);
    
    system.update_target_hp(250, 1000);
    assert_eq!(system.target_board_info.as_ref().unwrap().hp_percentage, 25);
    
    // Test clamp (e.g., healing above max)
    system.update_target_hp(1200, 1000);
    assert_eq!(system.target_board_info.as_ref().unwrap().hp_percentage, 100);
    
    // Test invalid max_hp -> 0% without crash
    system.update_target_hp(50, 0);
    assert_eq!(system.target_board_info.as_ref().unwrap().hp_percentage, 0);
}

#[test]
fn test_clear_target_on_escape() {
    let mut system = TargetSelectionSystem::new();
    let mut buffer: [u8; 128] = [0; 128];
    
    let mut temp_buffer: [u8; 128] = [0; 128];
    system.select_target(
        42,
        "Test".to_string(),
        100,
        100,
        1,
        1,
        0,
        &mut temp_buffer,
    ).unwrap();
    
    assert!(system.selected_target_vid.is_some());
    assert!(system.target_board_info.is_some());
    
    let bytes_written = system.clear_target(Some(&mut buffer)).unwrap();
    
    assert_eq!(bytes_written, std::mem::size_of::<TPacketCGTarget>());
    assert!(system.selected_target_vid.is_none());
    assert!(system.target_board_info.is_none());
    
    // Check unselect packet (vid = 0)
    let packet_ptr = buffer.as_ptr() as *const TPacketCGTarget;
    let packet = unsafe { ptr::read_unaligned(packet_ptr) };
    
    // Copy fields to unaligned local variables to avoid E0793
    let header = { packet.header };
    let dw_vid = { packet.dw_vid };
    
    assert_eq!(header, HEADER_CG_TARGET);
    assert_eq!(dw_vid, 0);
}
