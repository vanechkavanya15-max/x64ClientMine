use m2_client::send_item_packets_v2::*;
use std::mem::size_of;

#[test]
fn test_titempos_size_and_layout() {
    assert_eq!(size_of::<TItemPos>(), 3); // 1 byte for window_type, 2 bytes for cell
    let pos = TItemPos::new(1, 10);
    assert_eq!(pos.window_type, 1);
    
    // We cannot do assert_eq!(pos.cell, 10) directly due to unaligned reference on packed struct, 
    // but in Rust >= 1.62 or by destructuring/copying it's fine, let's copy it first.
    let cell = { pos.cell }; 
    assert_eq!(cell, 10);
}

#[test]
fn test_packet_item_use_creation() {
    assert_eq!(size_of::<TPacketCGItemUse>(), 7); // 2 header + 2 length + 3 TItemPos
    let pkt = TPacketCGItemUse::new(1, 5);
    
    let header = { pkt.header };
    let length = { pkt.length };
    assert_eq!(header, HEADER_CG_ITEM_USE);
    assert_eq!(length, 7);
    
    let w = pkt.pos.window_type;
    assert_eq!(w, 1);
}

#[test]
fn test_packet_item_move_creation() {
    assert_eq!(size_of::<TPacketCGItemMove>(), 11); // 2+2+3+3+1 = 11
    let pkt = TPacketCGItemMove::new(1, 2, 3, 4, 10);
    
    let header = { pkt.header };
    let length = { pkt.length };
    assert_eq!(header, HEADER_CG_ITEM_MOVE);
    assert_eq!(length, 11);
    
    let num = pkt.num;
    assert_eq!(num, 10);
}

#[test]
fn test_packet_item_pickup_creation() {
    assert_eq!(size_of::<TPacketCGItemPickUp>(), 8); // 2+2+4 = 8
    let pkt = TPacketCGItemPickUp::new(12345);
    
    let header = { pkt.header };
    let length = { pkt.length };
    assert_eq!(header, HEADER_CG_ITEM_PICKUP);
    assert_eq!(length, 8);
    
    let vid = { pkt.vid };
    assert_eq!(vid, 12345);
}

#[test]
fn test_packet_item_drop_creation() {
    assert_eq!(size_of::<TPacketCGItemDrop>(), 15); // 2+2+3+8(u64) = 15
    let pkt = TPacketCGItemDrop::new(2, 42, 9999);
    
    let header = { pkt.header };
    let length = { pkt.length };
    assert_eq!(header, HEADER_CG_ITEM_DROP);
    assert_eq!(length, 15);
    
    let gold = { pkt.gold };
    assert_eq!(gold, 9999);
}

#[test]
fn test_packet_item_drop2_creation() {
    assert_eq!(size_of::<TPacketCGItemDrop2>(), 12); // 2+2+3+4+1 = 12
    let pkt = TPacketCGItemDrop2::new(1, 20, 500, 5);
    
    let header = { pkt.header };
    let length = { pkt.length };
    assert_eq!(header, HEADER_CG_ITEM_DROP2);
    assert_eq!(length, 12);
    
    let count = pkt.count;
    assert_eq!(count, 5);
}

#[test]
fn test_m2_send_item_move_ffi_success() {
    let mut buf = vec![0u8; 128];
    let expected_size = size_of::<TPacketCGItemMove>();
    
    let written = unsafe {
        m2_send_item_move(1, 10, 1, 20, 5, buf.as_mut_ptr())
    };
    
    assert_eq!(written, expected_size);
    
    // Check if the first bytes match header and length
    let header_bytes = HEADER_CG_ITEM_MOVE.to_le_bytes();
    assert_eq!(buf[0], header_bytes[0]);
    assert_eq!(buf[1], header_bytes[1]);
    
    let length_bytes = (expected_size as u16).to_le_bytes();
    assert_eq!(buf[2], length_bytes[0]);
    assert_eq!(buf[3], length_bytes[1]);
    
    // window_type src
    assert_eq!(buf[4], 1);
}

#[test]
fn test_m2_send_item_move_ffi_null_ptr() {
    let written = unsafe {
        m2_send_item_move(1, 10, 1, 20, 5, std::ptr::null_mut())
    };
    
    assert_eq!(written, 0); // Should fail safely
}

