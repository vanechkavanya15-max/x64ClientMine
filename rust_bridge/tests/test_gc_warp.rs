use m2_client_bridge::net::gc::warp::{TPacketGCWarp, HEADER_GC_WARP};
use std::net::Ipv4Addr;

#[test]
fn test_gc_warp_from_bytes_success() {
    let mut buf = Vec::new();
    
    // Header (HEADER_GC_WARP = 0x0306)
    buf.extend_from_slice(&HEADER_GC_WARP.to_le_bytes());
    // Length = 18
    buf.extend_from_slice(&18u16.to_le_bytes());
    
    // lX = 1000
    buf.extend_from_slice(&1000i32.to_le_bytes());
    
    // lY = 2000
    buf.extend_from_slice(&2000i32.to_le_bytes());
    
    // lAddr = 2130706433 (127.0.0.1)
    buf.extend_from_slice(&2130706433i32.to_le_bytes());
    
    // wPort = 13000
    buf.extend_from_slice(&13000u16.to_le_bytes());

    assert_eq!(buf.len(), 18);

    let packet = TPacketGCWarp::from_bytes(&buf).expect("Oczekiwano poprawnego parsowania pakietu");

    assert_eq!({ packet.header }, HEADER_GC_WARP);
    assert_eq!({ packet.length }, 18);
    assert_eq!(packet.l_x(), 1000);
    assert_eq!(packet.l_y(), 2000);
    assert_eq!(packet.l_addr(), 2130706433);
    assert_eq!(packet.w_port(), 13000);
    
    let ip = Ipv4Addr::from(packet.l_addr() as u32);
    assert_eq!(ip.to_string(), "127.0.0.1");
}

#[test]
fn test_gc_warp_from_bytes_too_short() {
    let buf = vec![0u8; 17];

    let result = TPacketGCWarp::from_bytes(&buf);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Bufor jest za krotki. Oczekiwano 18, otrzymano 17");
}

#[test]
fn test_gc_warp_from_bytes_with_extra_data() {
    let mut buf = Vec::new();
    buf.extend_from_slice(&HEADER_GC_WARP.to_le_bytes());
    buf.extend_from_slice(&18u16.to_le_bytes());
    buf.extend_from_slice(&1000i32.to_le_bytes());
    buf.extend_from_slice(&2000i32.to_le_bytes());
    buf.extend_from_slice(&2130706433i32.to_le_bytes());
    buf.extend_from_slice(&13000u16.to_le_bytes());
    
    // Dodatkowe dane
    buf.extend_from_slice(&[0xFF, 0xAA]);

    let packet = TPacketGCWarp::from_bytes(&buf).expect("Oczekiwano poprawnego parsowania pomimo dodatkowych danych");

    assert_eq!({ packet.header }, HEADER_GC_WARP);
    assert_eq!(packet.l_x(), 1000);
}
