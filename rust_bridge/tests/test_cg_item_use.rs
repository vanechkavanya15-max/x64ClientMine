use m2_client_bridge::net::cg::item_use::{TPacketCGItemUse, m2_net_send_item_use};

#[test]
fn test_cg_item_use_serialization() {
    let packet = TPacketCGItemUse::new(1, 12);
    let bytes = packet.to_bytes();
    
    assert_eq!(bytes.len(), 7, "Pakiet musi miec dokladnie 7 bajtow");
    assert_eq!(u16::from_le_bytes([bytes[0], bytes[1]]), 0x0501);
    assert_eq!(u16::from_le_bytes([bytes[2], bytes[3]]), 7);
    assert_eq!(bytes[4], 1);
    assert_eq!(bytes[5], 12);
    assert_eq!(bytes[6], 0);
    
    let packet_edge = TPacketCGItemUse::new(255, 0xFFFF);
    let bytes_edge = packet_edge.to_bytes();
    assert_eq!(bytes_edge.len(), 7);
    assert_eq!(u16::from_le_bytes([bytes_edge[0], bytes_edge[1]]), 0x0501);
    assert_eq!(u16::from_le_bytes([bytes_edge[2], bytes_edge[3]]), 7);
    assert_eq!(bytes_edge[4], 255);
    assert_eq!(bytes_edge[5], 255);
    assert_eq!(bytes_edge[6], 255);
}

#[test]
fn test_ffi_m2_net_send_item_use() {
    unsafe {
        let result = m2_net_send_item_use(1, 12);
        assert!(result, "Funkcja FFI powinna zwrocic true");
    }
}
