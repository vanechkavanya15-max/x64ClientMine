use m2_client_bridge::net::gc::warp::TPacketGCWarp;
use std::net::Ipv4Addr;

#[test]
fn test_gc_warp_from_bytes_success() {
    let mut buf = Vec::new();
    
    // Header (HEADER_GC_WARP = 65)
    buf.push(65);
    
    // l_x = 1000 (0x000003E8) -> le: E8 03 00 00
    buf.extend_from_slice(&1000i32.to_le_bytes());
    
    // l_y = 2000 (0x000007D0) -> le: D0 07 00 00
    buf.extend_from_slice(&2000i32.to_le_bytes());
    
    // l_addr = 2130706433 (127.0.0.1) -> le: 01 00 00 7F
    buf.extend_from_slice(&2130706433u32.to_le_bytes());
    
    // w_port = 13000 (0x32C8) -> le: C8 32
    buf.extend_from_slice(&13000u16.to_le_bytes());

    assert_eq!(buf.len(), 15);

    let packet = TPacketGCWarp::from_bytes(&buf).expect("Oczekiwano poprawnego parsowania pakietu");

    assert_eq!(packet.header, 65);
    let l_x = packet.l_x;
    assert_eq!(l_x, 1000);
    let l_y = packet.l_y;
    assert_eq!(l_y, 2000);
    let l_addr = packet.l_addr;
    assert_eq!(l_addr, 2130706433);
    let w_port = packet.w_port;
    assert_eq!(w_port, 13000);
    
    let ip = Ipv4Addr::from(l_addr);
    assert_eq!(ip.to_string(), "127.0.0.1");
}

#[test]
fn test_gc_warp_from_bytes_too_short() {
    let buf = vec![65, 0xE8, 0x03, 0x00, 0x00]; // Tylko header i l_x

    let result = TPacketGCWarp::from_bytes(&buf);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Bufor jest za krotki. Oczekiwano 15, otrzymano 5");
}

#[test]
fn test_gc_warp_from_bytes_with_extra_data() {
    let mut buf = Vec::new();
    buf.push(65);
    buf.extend_from_slice(&1000i32.to_le_bytes());
    buf.extend_from_slice(&2000i32.to_le_bytes());
    buf.extend_from_slice(&2130706433u32.to_le_bytes());
    buf.extend_from_slice(&13000u16.to_le_bytes());
    
    // Dodatkowe dane
    buf.extend_from_slice(&[0xFF, 0xAA]);

    let packet = TPacketGCWarp::from_bytes(&buf).expect("Oczekiwano poprawnego parsowania pomimo dodatkowych danych");

    assert_eq!(packet.header, 65);
    let l_x = packet.l_x;
    assert_eq!(l_x, 1000);
}
