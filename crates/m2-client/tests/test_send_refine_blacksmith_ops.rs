#[path = "../src/send_refine_blacksmith_ops.rs"]
mod send_refine_blacksmith_ops;

use send_refine_blacksmith_ops::{
    format_cg_item_use_to_item_packet, format_cg_refine_packet, TItemPos,
    TPacketCGItemUseToItem, TPacketCGRefine, HEADER_CG_ITEM_USE_TO_ITEM, HEADER_CG_REFINE,
};
use std::mem;

#[test]
fn test_tpacketcgrefine_layout() {
    assert_eq!(
        mem::size_of::<TPacketCGRefine>(),
        6,
        "TPacketCGRefine must be exactly 6 bytes"
    );
    assert_eq!(
        mem::align_of::<TPacketCGRefine>(),
        1,
        "TPacketCGRefine must have an alignment of 1"
    );

    let offset_header = mem::offset_of!(TPacketCGRefine, header);
    let offset_length = mem::offset_of!(TPacketCGRefine, length);
    let offset_pos = mem::offset_of!(TPacketCGRefine, pos);
    let offset_type = mem::offset_of!(TPacketCGRefine, type_);

    assert_eq!(offset_header, 0);
    assert_eq!(offset_length, 2);
    assert_eq!(offset_pos, 4);
    assert_eq!(offset_type, 5);
}

#[test]
fn test_tpacketitemusetositem_layout() {
    assert_eq!(
        mem::size_of::<TPacketCGItemUseToItem>(),
        10,
        "TPacketCGItemUseToItem must be exactly 10 bytes"
    );
    assert_eq!(
        mem::align_of::<TPacketCGItemUseToItem>(),
        1,
        "TPacketCGItemUseToItem must have an alignment of 1"
    );

    let offset_header = mem::offset_of!(TPacketCGItemUseToItem, header);
    let offset_length = mem::offset_of!(TPacketCGItemUseToItem, length);
    let offset_source = mem::offset_of!(TPacketCGItemUseToItem, source_pos);
    let offset_target = mem::offset_of!(TPacketCGItemUseToItem, target_pos);

    assert_eq!(offset_header, 0);
    assert_eq!(offset_length, 2);
    assert_eq!(offset_source, 4);
    assert_eq!(offset_target, 7);
}

#[test]
fn test_titempos_layout() {
    assert_eq!(
        mem::size_of::<TItemPos>(),
        3,
        "TItemPos must be exactly 3 bytes"
    );
    assert_eq!(
        mem::align_of::<TItemPos>(),
        1,
        "TItemPos must have an alignment of 1"
    );

    let offset_window_type = mem::offset_of!(TItemPos, window_type);
    let offset_cell = mem::offset_of!(TItemPos, cell);

    assert_eq!(offset_window_type, 0);
    assert_eq!(offset_cell, 1);
}

#[test]
fn test_format_cg_refine_packet() {
    let buffer = format_cg_refine_packet(15, 2).expect("Failed to format refine packet");
    
    assert_eq!(buffer.len(), 6);
    
    let expected_header = HEADER_CG_REFINE.to_ne_bytes();
    let expected_length = (6u16).to_ne_bytes();
    
    assert_eq!(buffer[0..2], expected_header);
    assert_eq!(buffer[2..4], expected_length);
    assert_eq!(buffer[4], 15);
    assert_eq!(buffer[5], 2);
}

#[test]
fn test_format_cg_item_use_to_item_packet() {
    let source_pos = TItemPos::new(1, 10);
    let target_pos = TItemPos::new(1, 20);
    
    let buffer = format_cg_item_use_to_item_packet(source_pos, target_pos)
        .expect("Failed to format item use to item packet");
        
    assert_eq!(buffer.len(), 10);
    
    let expected_header = HEADER_CG_ITEM_USE_TO_ITEM.to_ne_bytes();
    let expected_length = (10u16).to_ne_bytes();
    let expected_src_cell = (10u16).to_ne_bytes();
    let expected_tgt_cell = (20u16).to_ne_bytes();
    
    assert_eq!(buffer[0..2], expected_header);
    assert_eq!(buffer[2..4], expected_length);
    assert_eq!(buffer[4], 1); // src window_type
    assert_eq!(buffer[5..7], expected_src_cell);
    assert_eq!(buffer[7], 1); // tgt window_type
    assert_eq!(buffer[8..10], expected_tgt_cell);
}

#[test]
fn test_handle_refine_dialog() {
    use send_refine_blacksmith_ops::{handle_refine_dialog, TRefineTable, TMaterial};
    
    let table = TRefineTable {
        src_vnum: 10,
        result_vnum: 20,
        material_count: 1,
        cost: 100,
        prob: 50,
        materials: [TMaterial::default(); 5],
    };
    
    assert_eq!(handle_refine_dialog(&table, true, true), true);
    assert_eq!(handle_refine_dialog(&table, false, true), false);
    assert_eq!(handle_refine_dialog(&table, true, false), false);
    
    let bad_table = TRefineTable {
        src_vnum: 10,
        result_vnum: 20,
        material_count: 6, // too many materials
        cost: 100,
        prob: 50,
        materials: [TMaterial::default(); 5],
    };
    assert_eq!(handle_refine_dialog(&bad_table, true, true), false);
}
