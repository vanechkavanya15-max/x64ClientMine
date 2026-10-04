use m2_client::net_alch_refine_packets::{
    TItemPos, TPacketCGRefine, TPacketCGRefineCancel, TPacketCGDragonSoulRefine,
    HEADER_CG_REFINE, HEADER_CG_REFINE_CANCEL, HEADER_CG_DRAGON_SOUL_REFINE,
    DS_REFINE_WINDOW_MAX_NUM,
    serialize_refine_packet, serialize_refine_cancel_packet, serialize_dragon_soul_refine_packet,
};

#[test]
fn test_struct_sizes() {
    assert_eq!(std::mem::size_of::<TItemPos>(), 3);
    assert_eq!(std::mem::size_of::<TPacketCGRefine>(), 6);
    assert_eq!(std::mem::size_of::<TPacketCGRefineCancel>(), 4);
    assert_eq!(std::mem::size_of::<TPacketCGDragonSoulRefine>(), 50); // 4 + 1 + 15 * 3 = 50
}

#[test]
fn test_serialize_refine_packet() {
    let packet = serialize_refine_packet(10, 2);
    assert_eq!(packet.len(), 6);
    
    // Header
    let header = u16::from_le_bytes([packet[0], packet[1]]);
    assert_eq!(header, HEADER_CG_REFINE);
    
    // Length
    let length = u16::from_le_bytes([packet[2], packet[3]]);
    assert_eq!(length, 6);
    
    // Pos
    assert_eq!(packet[4], 10);
    
    // Type
    assert_eq!(packet[5], 2);
}

#[test]
fn test_serialize_refine_cancel_packet() {
    let packet = serialize_refine_cancel_packet();
    assert_eq!(packet.len(), 4);
    
    // Header
    let header = u16::from_le_bytes([packet[0], packet[1]]);
    assert_eq!(header, HEADER_CG_REFINE_CANCEL);
    
    // Length
    let length = u16::from_le_bytes([packet[2], packet[3]]);
    assert_eq!(length, 4);
}

#[test]
fn test_serialize_dragon_soul_refine_packet() {
    let mut grid = [TItemPos::default(); DS_REFINE_WINDOW_MAX_NUM];
    grid[0] = TItemPos::new(1, 100);
    grid[1] = TItemPos::new(2, 200);
    
    let packet = serialize_dragon_soul_refine_packet(1, &grid);
    assert_eq!(packet.len(), 50);
    
    // Header
    let header = u16::from_le_bytes([packet[0], packet[1]]);
    assert_eq!(header, HEADER_CG_DRAGON_SOUL_REFINE);
    
    // Length
    let length = u16::from_le_bytes([packet[2], packet[3]]);
    assert_eq!(length, 50);
    
    // SubType
    assert_eq!(packet[4], 1);
    
    // First Item
    assert_eq!(packet[5], 1); // window_type
    let cell1 = u16::from_le_bytes([packet[6], packet[7]]);
    assert_eq!(cell1, 100);
    
    // Second Item
    assert_eq!(packet[8], 2); // window_type
    let cell2 = u16::from_le_bytes([packet[9], packet[10]]);
    assert_eq!(cell2, 200);
    
    // Third Item should be default (0, 0)
    assert_eq!(packet[11], 0);
    let cell3 = u16::from_le_bytes([packet[12], packet[13]]);
    assert_eq!(cell3, 0);
}
