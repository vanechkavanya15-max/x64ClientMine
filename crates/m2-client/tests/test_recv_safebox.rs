use m2_client::recv_safebox::{
    SafeboxManager, TPacketGCItemSet, TPacketGCItemDel, TPacketGCSafeboxMoneyChange, TPacketGCMallOpen,
    HEADER_GC_SAFEBOX_SET, HEADER_GC_SAFEBOX_DEL, HEADER_GC_SAFEBOX_MONEY_CHANGE, HEADER_GC_MALL_OPEN,
    PacketError
};

#[test]
fn test_recv_safebox_set_packet() {
    let mut manager = SafeboxManager::new();
    let mut data = vec![0u8; TPacketGCItemSet::PACKET_SIZE];
    
    // Fill test data
    data[0..2].copy_from_slice(&HEADER_GC_SAFEBOX_SET.to_le_bytes()); // header
    data[2..4].copy_from_slice(&(TPacketGCItemSet::PACKET_SIZE as u16).to_le_bytes()); // length
    data[4] = 2; // window_type
    data[5..7].copy_from_slice(&15u16.to_le_bytes()); // cell
    data[7..11].copy_from_slice(&12345u32.to_le_bytes()); // vnum
    data[11] = 5; // count
    data[12..16].copy_from_slice(&10u32.to_le_bytes()); // flags
    data[16..20].copy_from_slice(&20u32.to_le_bytes()); // anti_flags
    data[20] = 1; // highlight
    
    // Set socket 0
    data[21..25].copy_from_slice(&99i32.to_le_bytes());
    // Set attr 0
    data[33] = 7; // bType
    data[34..36].copy_from_slice(&50i16.to_le_bytes()); // sValue
    
    let result = manager.recv_safebox_set_packet(&data);
    assert_eq!(result, Ok(true));
    
    // Verify State
    assert_eq!(manager.safebox_items.len(), 1);
    let item = manager.safebox_items.get(&15).unwrap();
    assert_eq!(item.vnum, 12345);
    assert_eq!(item.count, 5);
    assert_eq!(item.flags, 10);
    assert_eq!(item.anti_flags, 20);
    
    let sock = { item.alSockets[0] };
    assert_eq!(sock, 99);
    
    let attr_type = { item.aAttr[0].bType };
    let attr_val = { item.aAttr[0].sValue };
    assert_eq!(attr_type, 7);
    assert_eq!(attr_val, 50);
}

#[test]
fn test_recv_safebox_del_packet() {
    let mut manager = SafeboxManager::new();
    
    // Pre-populate
    manager.recv_safebox_set_packet(&{
        let mut data = vec![0u8; TPacketGCItemSet::PACKET_SIZE];
        data[0..2].copy_from_slice(&HEADER_GC_SAFEBOX_SET.to_le_bytes());
        data[2..4].copy_from_slice(&(TPacketGCItemSet::PACKET_SIZE as u16).to_le_bytes());
        data[5..7].copy_from_slice(&42u16.to_le_bytes()); // cell
        data[7..11].copy_from_slice(&9999u32.to_le_bytes()); // vnum
        data
    }).unwrap();
    assert_eq!(manager.safebox_items.len(), 1);
    
    let mut data = vec![0u8; TPacketGCItemDel::PACKET_SIZE];
    data[0..2].copy_from_slice(&HEADER_GC_SAFEBOX_DEL.to_le_bytes()); // header
    data[2..4].copy_from_slice(&(TPacketGCItemDel::PACKET_SIZE as u16).to_le_bytes()); // length
    data[4] = 2; // window_type
    data[5..7].copy_from_slice(&42u16.to_le_bytes()); // cell
    
    let result = manager.recv_safebox_del_packet(&data);
    assert_eq!(result, Ok(true));
    
    // Verify State
    assert_eq!(manager.safebox_items.len(), 0);
}

#[test]
fn test_recv_safebox_money_change_packet() {
    let mut manager = SafeboxManager::new();
    
    let mut data = vec![0u8; TPacketGCSafeboxMoneyChange::PACKET_SIZE];
    data[0..2].copy_from_slice(&HEADER_GC_SAFEBOX_MONEY_CHANGE.to_le_bytes()); // header
    data[2..4].copy_from_slice(&(TPacketGCSafeboxMoneyChange::PACKET_SIZE as u16).to_le_bytes()); // length
    data[4..8].copy_from_slice(&100500i32.to_le_bytes()); // lMoney
    
    let result = manager.recv_safebox_money_change_packet(&data);
    assert_eq!(result, Ok(true));
    
    // Verify State
    assert_eq!(manager.safebox_money, 100500);
}

#[test]
fn test_recv_mall_open_packet() {
    let mut manager = SafeboxManager::new();
    
    let mut data = vec![0u8; TPacketGCMallOpen::PACKET_SIZE];
    data[0..2].copy_from_slice(&HEADER_GC_MALL_OPEN.to_le_bytes()); // header
    data[2..4].copy_from_slice(&(TPacketGCMallOpen::PACKET_SIZE as u16).to_le_bytes()); // length
    data[4] = 3; // bSize
    
    let result = manager.recv_mall_open_packet(&data);
    assert_eq!(result, Ok(true));
    
    // Verify State
    assert_eq!(manager.mall_size, 3);
}

#[test]
fn test_packet_buffer_too_small() {
    let mut manager = SafeboxManager::new();
    
    let data = vec![0u8; TPacketGCItemSet::PACKET_SIZE - 1];
    let result = manager.recv_safebox_set_packet(&data);
    assert_eq!(result, Err(PacketError::BufferTooSmall));
    
    let data = vec![0u8; TPacketGCItemDel::PACKET_SIZE - 1];
    let result = manager.recv_safebox_del_packet(&data);
    assert_eq!(result, Err(PacketError::BufferTooSmall));
    
    let data = vec![0u8; TPacketGCSafeboxMoneyChange::PACKET_SIZE - 1];
    let result = manager.recv_safebox_money_change_packet(&data);
    assert_eq!(result, Err(PacketError::BufferTooSmall));
    
    let data = vec![0u8; TPacketGCMallOpen::PACKET_SIZE - 1];
    let result = manager.recv_mall_open_packet(&data);
    assert_eq!(result, Err(PacketError::BufferTooSmall));
}
