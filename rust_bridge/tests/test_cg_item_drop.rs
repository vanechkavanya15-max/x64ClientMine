use m2_client_bridge::net::cg::item_drop::TPacketCGItemDrop;

#[test]
fn test_serialize_cg_item_drop() {
    let packet = TPacketCGItemDrop::new(1, 10, 500);
    let bytes = packet.to_bytes();

    // header(2) + length(2) + window_type(1) + cell(2) + elk(4) = 11 bajtow
    assert_eq!(bytes.len(), 11, "Rozmiar pakietu powinien wynosic 11 bajtow");

    // header: 0x0502
    assert_eq!(u16::from_le_bytes([bytes[0], bytes[1]]), 0x0502);
    // length: 11
    assert_eq!(u16::from_le_bytes([bytes[2], bytes[3]]), 11);

    // window_type: 1
    assert_eq!(bytes[4], 1);

    // cell: 10
    assert_eq!(u16::from_le_bytes([bytes[5], bytes[6]]), 10);

    // elk: 500
    assert_eq!(u32::from_le_bytes([bytes[7], bytes[8], bytes[9], bytes[10]]), 500);
}

#[test]
fn test_serialize_cg_item_drop_count_1() {
    let packet = TPacketCGItemDrop::new(2, 5, 1);
    let bytes = packet.to_bytes();
    
    assert_eq!(bytes.len(), 11);
    assert_eq!(u16::from_le_bytes([bytes[0], bytes[1]]), 0x0502);
    assert_eq!(u16::from_le_bytes([bytes[2], bytes[3]]), 11);
    assert_eq!(bytes[4], 2);
    assert_eq!(u16::from_le_bytes([bytes[5], bytes[6]]), 5);
    assert_eq!(u32::from_le_bytes([bytes[7], bytes[8], bytes[9], bytes[10]]), 1);
}

#[test]
fn test_serialize_cg_item_drop_count_2_000_000_000() {
    let packet = TPacketCGItemDrop::new(3, 100, 2_000_000_000);
    let bytes = packet.to_bytes();
    
    assert_eq!(bytes.len(), 11);
    assert_eq!(u16::from_le_bytes([bytes[0], bytes[1]]), 0x0502);
    assert_eq!(u16::from_le_bytes([bytes[2], bytes[3]]), 11);
    assert_eq!(bytes[4], 3);
    assert_eq!(u16::from_le_bytes([bytes[5], bytes[6]]), 100);
    assert_eq!(u32::from_le_bytes([bytes[7], bytes[8], bytes[9], bytes[10]]), 2_000_000_000);
}
