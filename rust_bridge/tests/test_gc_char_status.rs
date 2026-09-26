use m2_client_bridge::net::gc::char_status::{
    m2_net_parse_point_change, PacketError, TPacketGCPointChange, HEADER_GC_PLAYER_POINT_CHANGE,
};

#[test]
fn test_parse_valid_buffer() {
    let mut buf = Vec::new();
    buf.extend_from_slice(&HEADER_GC_PLAYER_POINT_CHANGE.to_le_bytes()); // header: u16
    buf.extend_from_slice(&17u16.to_le_bytes());                         // length: u16
    buf.extend_from_slice(&1234u32.to_le_bytes());                       // dwVID: u32
    buf.push(3);                                                         // Type: u8, np. POINT_GOLD
    buf.extend_from_slice(&100_000i32.to_le_bytes());                    // amount: i32
    buf.extend_from_slice(&300_000i32.to_le_bytes());                    // value: i32

    let packet = TPacketGCPointChange::from_bytes(&buf)
        .expect("Parsowanie poprawnego bufora powinno sie udac");

    assert_eq!({ packet.header }, HEADER_GC_PLAYER_POINT_CHANGE);
    assert_eq!({ packet.length }, 17);
    assert_eq!(packet.b_type(), 3);
    assert_eq!(packet.get_amount(), 100_000);
    assert_eq!(packet.get_value(), 300_000);
}

#[test]
fn test_parse_truncated_buffer() {
    let mut buf = Vec::new();
    buf.extend_from_slice(&HEADER_GC_PLAYER_POINT_CHANGE.to_le_bytes());
    buf.extend_from_slice(&17u16.to_le_bytes());
    buf.extend_from_slice(&1234u32.to_le_bytes());
    buf.push(3);
    // Brak pol amount i value (krotszy niz 17 bajtow)

    let result = TPacketGCPointChange::from_bytes(&buf);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), PacketError::BufferTooShort);
}

#[test]
fn test_parse_invalid_header() {
    let mut buf = Vec::new();
    buf.extend_from_slice(&0x9999u16.to_le_bytes()); // invalid header
    buf.extend_from_slice(&17u16.to_le_bytes());
    buf.extend_from_slice(&1234u32.to_le_bytes());
    buf.push(3);
    buf.extend_from_slice(&100_000i32.to_le_bytes());
    buf.extend_from_slice(&300_000i32.to_le_bytes());

    let result = TPacketGCPointChange::from_bytes(&buf);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), PacketError::InvalidHeader);
}

#[test]
fn test_ffi_valid() {
    let mut buf = Vec::new();
    buf.extend_from_slice(&HEADER_GC_PLAYER_POINT_CHANGE.to_le_bytes());
    buf.extend_from_slice(&17u16.to_le_bytes());
    buf.extend_from_slice(&1234u32.to_le_bytes());
    buf.push(4);
    buf.extend_from_slice(&200_000i32.to_le_bytes());
    buf.extend_from_slice(&400_000i32.to_le_bytes());

    let mut out_type = 0u8;
    let mut out_val = 0i64;

    let res = unsafe {
        m2_net_parse_point_change(
            buf.as_ptr(),
            buf.len() as u32,
            &mut out_type,
            &mut out_val,
        )
    };

    assert!(res);
    assert_eq!(out_type, 4);
    assert_eq!(out_val, 400_000);
}

#[test]
fn test_ffi_null_pointers() {
    let mut buf = Vec::new();
    buf.extend_from_slice(&HEADER_GC_PLAYER_POINT_CHANGE.to_le_bytes());
    buf.extend_from_slice(&17u16.to_le_bytes());
    buf.extend_from_slice(&1234u32.to_le_bytes());
    buf.push(4);
    buf.extend_from_slice(&200_000i32.to_le_bytes());
    buf.extend_from_slice(&400_000i32.to_le_bytes());

    let mut out_type = 0u8;
    let mut out_val = 0i64;

    let res = unsafe {
        m2_net_parse_point_change(
            std::ptr::null(),
            buf.len() as u32,
            &mut out_type,
            &mut out_val,
        )
    };
    assert!(!res);

    let res = unsafe {
        m2_net_parse_point_change(
            buf.as_ptr(),
            buf.len() as u32,
            std::ptr::null_mut(),
            &mut out_val,
        )
    };
    assert!(!res);

    let res = unsafe {
        m2_net_parse_point_change(
            buf.as_ptr(),
            buf.len() as u32,
            &mut out_type,
            std::ptr::null_mut(),
        )
    };
    assert!(!res);
}
