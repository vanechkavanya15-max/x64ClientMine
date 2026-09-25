use m2_client_bridge::net::gc::char_status::{
    m2_net_parse_point_change, PacketError, TPacketGCPointChange, HEADER_GC_PLAYER_POINT_CHANGE,
};

#[test]
fn test_parse_valid_buffer() {
    let mut buf = Vec::new();
    buf.push(HEADER_GC_PLAYER_POINT_CHANGE);
    buf.push(3); // b_type, e.g. POINT_GOLD
    buf.extend_from_slice(&3_000_000_000i64.to_le_bytes()); // value > 2 billion
    buf.extend_from_slice(&100_000i64.to_le_bytes()); // amount

    let packet = TPacketGCPointChange::from_bytes(&buf)
        .expect("Parsowanie poprawnego bufora powinno sie udac");

    assert_eq!(packet.header, HEADER_GC_PLAYER_POINT_CHANGE);
    assert_eq!(packet.b_type, 3);
    
    // Copy out from packed struct to avoid unaligned reference errors in assert_eq!
    let val = packet.value;
    let amt = packet.amount;
    assert_eq!(val, 3_000_000_000);
    assert_eq!(amt, 100_000);
}

#[test]
fn test_parse_truncated_buffer() {
    let mut buf = Vec::new();
    buf.push(HEADER_GC_PLAYER_POINT_CHANGE);
    buf.push(3);
    buf.extend_from_slice(&3_000_000_000i64.to_le_bytes());
    // Missing amount completely

    let result = TPacketGCPointChange::from_bytes(&buf);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), PacketError::BufferTooShort);
}

#[test]
fn test_parse_invalid_header() {
    let mut buf = Vec::new();
    buf.push(99); // invalid header
    buf.push(3);
    buf.extend_from_slice(&3_000_000_000i64.to_le_bytes());
    buf.extend_from_slice(&100_000i64.to_le_bytes());

    let result = TPacketGCPointChange::from_bytes(&buf);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), PacketError::InvalidHeader);
}

#[test]
fn test_ffi_valid() {
    let mut buf = Vec::new();
    buf.push(HEADER_GC_PLAYER_POINT_CHANGE);
    buf.push(4);
    buf.extend_from_slice(&4_000_000_000i64.to_le_bytes());
    buf.extend_from_slice(&200_000i64.to_le_bytes());

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
    assert_eq!(out_val, 4_000_000_000);
}

#[test]
fn test_ffi_null_pointers() {
    let mut buf = Vec::new();
    buf.push(HEADER_GC_PLAYER_POINT_CHANGE);
    buf.push(4);
    buf.extend_from_slice(&4_000_000_000i64.to_le_bytes());
    buf.extend_from_slice(&200_000i64.to_le_bytes());

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
