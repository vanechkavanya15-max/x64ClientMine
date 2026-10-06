#[path = "../src/send_item_interaction_pack.rs"]
mod send_item_interaction_pack;

use send_item_interaction_pack::*;
use std::mem::size_of;

#[test]
fn test_titempos_size() {
    assert_eq!(size_of::<TItemPos>(), 3);
}

#[test]
fn test_packet_item_use() {
    let packet = TPacketCGItemUse::new(1, 10);
    
    let header = { packet.header };
    let length = { packet.length };
    
    assert_eq!(header, HEADER_CG_ITEM_USE);
    assert_eq!(length, 7); // 2 + 2 + 3
    
    assert_eq!(packet.pos.window_type, 1);
    
    let cell = { packet.pos.cell };
    assert_eq!(cell, 10);
}

#[test]
fn test_packet_item_move() {
    let packet = TPacketCGItemMove::new(1, 10, 2, 20, 5);
    
    let header = { packet.header };
    let length = { packet.length };
    
    assert_eq!(header, HEADER_CG_ITEM_MOVE);
    assert_eq!(length, 11); // 2 + 2 + 3 + 3 + 1
    
    assert_eq!(packet.pos.window_type, 1);
    let cell = { packet.pos.cell };
    assert_eq!(cell, 10);
    
    assert_eq!(packet.change_pos.window_type, 2);
    let cell2 = { packet.change_pos.cell };
    assert_eq!(cell2, 20);
    
    assert_eq!(packet.num, 5);
}

#[test]
fn test_packet_item_drop() {
    let packet = TPacketCGItemDrop::new(1, 10, 5000);
    
    let header = { packet.header };
    let length = { packet.length };
    
    assert_eq!(header, HEADER_CG_ITEM_DROP);
    assert_eq!(length, 11); // 2 + 2 + 3 + 4(u32)
    
    assert_eq!(packet.pos.window_type, 1);
    
    let cell = { packet.pos.cell };
    assert_eq!(cell, 10);
    
    let elk = { packet.elk };
    assert_eq!(elk, 5000);
}

#[test]
fn test_packet_item_drop2() {
    let packet = TPacketCGItemDrop2::new(1, 10, 5000, 5);
    
    let header = { packet.header };
    let length = { packet.length };
    
    assert_eq!(header, HEADER_CG_ITEM_DROP2);
    assert_eq!(length, 12); // 2 + 2 + 3 + 4 + 1
    
    assert_eq!(packet.pos.window_type, 1);
    
    let cell = { packet.pos.cell };
    assert_eq!(cell, 10);
    
    let gold = { packet.gold };
    assert_eq!(gold, 5000);

    assert_eq!(packet.count, 5);
}

#[test]
fn test_ffi_send_item_use() {
    let mut buf = vec![0u8; 128];
    let written = unsafe { m2_ffi_send_item_use(1, 10, buf.as_mut_ptr()) };
    assert_eq!(written, size_of::<TPacketCGItemUse>());
    
    let written_null = unsafe { m2_ffi_send_item_use(1, 10, std::ptr::null_mut()) };
    assert_eq!(written_null, 0);
}

#[test]
fn test_ffi_send_item_move() {
    let mut buf = vec![0u8; 128];
    let written = unsafe { m2_ffi_send_item_move(1, 10, 2, 20, 5, buf.as_mut_ptr()) };
    assert_eq!(written, size_of::<TPacketCGItemMove>());
    
    let written_null = unsafe { m2_ffi_send_item_move(1, 10, 2, 20, 5, std::ptr::null_mut()) };
    assert_eq!(written_null, 0);
}

#[test]
fn test_ffi_send_item_drop() {
    let mut buf = vec![0u8; 128];
    let written = unsafe { m2_ffi_send_item_drop(1, 10, 5000, buf.as_mut_ptr()) };
    assert_eq!(written, size_of::<TPacketCGItemDrop>());
    
    let written_null = unsafe { m2_ffi_send_item_drop(1, 10, 5000, std::ptr::null_mut()) };
    assert_eq!(written_null, 0);
}

#[test]
fn test_ffi_send_item_drop2() {
    let mut buf = vec![0u8; 128];
    let written = unsafe { m2_ffi_send_item_drop2(1, 10, 5000, 2, buf.as_mut_ptr()) };
    assert_eq!(written, size_of::<TPacketCGItemDrop2>());
    
    let written_null = unsafe { m2_ffi_send_item_drop2(1, 10, 5000, 2, std::ptr::null_mut()) };
    assert_eq!(written_null, 0);
}

#[test]
fn test_ffi_send_item_destroy() {
    let mut buf = vec![0u8; 128];
    let written = unsafe { m2_ffi_send_item_destroy(1, 10, buf.as_mut_ptr()) };
    assert_eq!(written, size_of::<TPacketCGItemDrop>());
    
    let written_null = unsafe { m2_ffi_send_item_destroy(1, 10, std::ptr::null_mut()) };
    assert_eq!(written_null, 0);
}
