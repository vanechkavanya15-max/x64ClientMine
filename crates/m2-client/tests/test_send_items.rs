use m2_client::send_items::{
    format_item_move_packet, format_item_pickup_packet, format_item_use_packet, ItemPacketError,
};
use m2_client::bridge::net::cg::item_use::{TItemPos, TPacketCGItemUse};
use m2_client::bridge::net::cg::item_move::TPacketCGItemMove;
use m2_client::bridge::net::cg::item_pickup::TPacketCGItemPickUp;

#[test]
fn test_send_item_use_packet_success() {
    let pos = TItemPos::new(1, 42); // np. INVENTORY, slot 42
    let result = format_item_use_packet(pos);
    assert!(result.is_ok(), "Pakiet powinien zostac zbudowany poprawnie");

    let bytes = result.unwrap();
    assert_eq!(bytes.len(), TPacketCGItemUse::SIZE as usize);
    
    // Sprawdzenie naglowka
    let header = u16::from_le_bytes([bytes[0], bytes[1]]);
    assert_eq!(header, TPacketCGItemUse::HEADER);

    // Sprawdzenie pol
    assert_eq!(bytes[4], 1); // window_type
    let cell = u16::from_le_bytes([bytes[5], bytes[6]]);
    assert_eq!(cell, 42);
}

#[test]
fn test_send_item_use_packet_invalid_window() {
    let pos = TItemPos::new(0, 42); // RESERVED_WINDOW = 0
    let result = format_item_use_packet(pos);
    assert_eq!(result, Err(ItemPacketError::InvalidWindowType));

    let pos2 = TItemPos::new(99, 42); // Nieistniejace okno
    let result2 = format_item_use_packet(pos2);
    assert_eq!(result2, Err(ItemPacketError::InvalidWindowType));

    // Dragon Soul (5) i Belt (7) musza byc prawidlowe
    let pos_ds = TItemPos::new(5, 10);
    assert!(format_item_use_packet(pos_ds).is_ok());

    let pos_belt = TItemPos::new(7, 2);
    assert!(format_item_use_packet(pos_belt).is_ok());
}

#[test]
fn test_send_item_move_packet_success() {
    let src_pos = TItemPos::new(1, 10);
    let dst_pos = TItemPos::new(1, 20);
    let count = 5;

    let result = format_item_move_packet(src_pos, dst_pos, count);
    assert!(result.is_ok(), "Pakiet przeniesienia powinien zostac zbudowany");

    let bytes = result.unwrap();
    assert_eq!(bytes.len(), TPacketCGItemMove::SIZE as usize);

    // Sprawdzenie naglowka
    let header = u16::from_le_bytes([bytes[0], bytes[1]]);
    assert_eq!(header, TPacketCGItemMove::HEADER);

    // Sprawdzenie zrodel
    assert_eq!(bytes[4], 1); // src window_type
    assert_eq!(u16::from_le_bytes([bytes[5], bytes[6]]), 10); // src cell

    // Sprawdzenie celu
    assert_eq!(bytes[7], 1); // dst window_type
    assert_eq!(u16::from_le_bytes([bytes[8], bytes[9]]), 20); // dst cell

    // Sprawdzenie ilosci
    assert_eq!(bytes[10], 5);
}

#[test]
fn test_send_item_move_packet_invalid_count() {
    let src_pos = TItemPos::new(1, 10);
    let dst_pos = TItemPos::new(1, 20);
    
    let result = format_item_move_packet(src_pos, dst_pos, 0);
    assert_eq!(result, Err(ItemPacketError::InvalidItemCount));
}

#[test]
fn test_send_item_move_packet_invalid_windows() {
    let src_pos = TItemPos::new(0, 10);
    let dst_pos = TItemPos::new(1, 20);
    assert_eq!(format_item_move_packet(src_pos, dst_pos, 1), Err(ItemPacketError::InvalidWindowType));

    let src_pos2 = TItemPos::new(1, 10);
    let dst_pos2 = TItemPos::new(99, 20);
    assert_eq!(format_item_move_packet(src_pos2, dst_pos2, 1), Err(ItemPacketError::InvalidTargetWindowType));
}

#[test]
fn test_send_item_pickup_packet_success() {
    let vid = 123456;
    let result = format_item_pickup_packet(vid);
    assert!(result.is_ok());

    let bytes = result.unwrap();
    assert_eq!(bytes.len(), TPacketCGItemPickUp::SIZE as usize);

    // Sprawdzenie naglowka
    let header = u16::from_le_bytes([bytes[0], bytes[1]]);
    assert_eq!(header, TPacketCGItemPickUp::HEADER);

    // Sprawdzenie vid
    let parsed_vid = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
    assert_eq!(parsed_vid, 123456);
}

#[test]
fn test_send_item_pickup_packet_invalid_vid() {
    let result = format_item_pickup_packet(0);
    assert_eq!(result, Err(ItemPacketError::InvalidEntityId));
}
