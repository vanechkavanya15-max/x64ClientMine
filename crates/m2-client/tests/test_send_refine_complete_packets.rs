#[path = "../src/send_refine_complete_packets.rs"]
mod send_refine_complete_packets;

use send_refine_complete_packets::*;
use std::mem;

#[test]
fn test_titempos_layout() {
    assert_eq!(
        mem::size_of::<TItemPos>(),
        3,
        "TItemPos size must be 3 bytes (1 byte window_type + 2 bytes cell)"
    );
    assert_eq!(
        mem::align_of::<TItemPos>(),
        1,
        "TItemPos alignment must be 1 byte due to #[repr(C, packed)]"
    );
}

#[test]
fn test_cg_refine_layout() {
    assert_eq!(
        mem::size_of::<TPacketCGRefine>(),
        6,
        "TPacketCGRefine size must be exactly 6 bytes (2 + 2 + 1 + 1)"
    );
    assert_eq!(
        mem::align_of::<TPacketCGRefine>(),
        1,
        "TPacketCGRefine alignment must be 1 byte due to #[repr(C, packed)]"
    );
}

#[test]
fn test_cg_dragon_soul_refine_layout() {
    let expected_size = 2 + 2 + 1 + (3 * DS_REFINE_WINDOW_MAX_NUM); // 2(header) + 2(length) + 1(bSubType) + 15 * 3(TItemPos) = 5 + 45 = 50
    assert_eq!(
        mem::size_of::<TPacketCGDragonSoulRefine>(),
        expected_size,
        "TPacketCGDragonSoulRefine size must be 50 bytes"
    );
    assert_eq!(
        mem::align_of::<TPacketCGDragonSoulRefine>(),
        1,
        "TPacketCGDragonSoulRefine alignment must be 1 byte due to #[repr(C, packed)]"
    );
}

#[test]
fn test_cg_refine_serialization() {
    let pos = 42;
    let type_ = 1;
    let serialized = format_cg_refine_packet(pos, type_).expect("Failed to serialize refine packet");

    assert_eq!(serialized.len(), 6);
    
    // Header (little endian)
    assert_eq!(serialized[0], (HEADER_CG_REFINE & 0xFF) as u8);
    assert_eq!(serialized[1], (HEADER_CG_REFINE >> 8) as u8);
    
    // Length (little endian)
    let length = 6u16;
    assert_eq!(serialized[2], (length & 0xFF) as u8);
    assert_eq!(serialized[3], (length >> 8) as u8);
    
    // Pos
    assert_eq!(serialized[4], pos);
    
    // Type
    assert_eq!(serialized[5], type_);
}

#[test]
fn test_dragon_soul_refine_serialization() {
    let sub_type = EDragonSoulRefineWindowRefineType::DragonSoulRefineWindow_UPGRADE as u32 as u8;
    let mut grid = [TItemPos::default(); DS_REFINE_WINDOW_MAX_NUM];
    grid[0] = TItemPos::new(1, 100);
    grid[14] = TItemPos::new(2, 200);

    let serialized = format_cg_dragon_soul_refine_packet(sub_type, &grid).expect("Failed to serialize dragon soul packet");

    let expected_size = 50;
    assert_eq!(serialized.len(), expected_size);
    
    // Header (little endian)
    assert_eq!(serialized[0], (HEADER_CG_DRAGON_SOUL_REFINE & 0xFF) as u8);
    assert_eq!(serialized[1], (HEADER_CG_DRAGON_SOUL_REFINE >> 8) as u8);
    
    // Length (little endian)
    let length = expected_size as u16;
    assert_eq!(serialized[2], (length & 0xFF) as u8);
    assert_eq!(serialized[3], (length >> 8) as u8);
    
    // bSubType
    assert_eq!(serialized[4], sub_type);
    
    // Grid 0
    assert_eq!(serialized[5], 1); // window_type
    assert_eq!(serialized[6], 100); // cell lower
    assert_eq!(serialized[7], 0); // cell upper

    // Grid 1 (Empty default)
    assert_eq!(serialized[8], 0);
    assert_eq!(serialized[9], 0);
    assert_eq!(serialized[10], 0);

    // Grid 14
    let offset = 5 + (14 * 3);
    assert_eq!(serialized[offset], 2); // window_type
    assert_eq!(serialized[offset + 1], 200); // cell lower
    assert_eq!(serialized[offset + 2], 0); // cell upper
}

#[test]
fn test_ds_refine_max_num_limit() {
    assert_eq!(DS_REFINE_WINDOW_MAX_NUM, 15, "DS_REFINE_WINDOW_MAX_NUM must match C++ 15");
}
