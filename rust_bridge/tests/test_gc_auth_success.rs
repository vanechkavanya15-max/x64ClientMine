use m2_client_bridge::net::gc::auth_success::{
    m2_net_parse_auth_success, PacketError, TPacketGCAuthSuccess, HEADER_GC_AUTH_SUCCESS,
};

#[test]
fn test_auth_success_from_bytes_valid() {
    let mut buf = Vec::new();
    buf.extend_from_slice(&HEADER_GC_AUTH_SUCCESS.to_le_bytes()); // header: u16
    buf.extend_from_slice(&9u16.to_le_bytes());                  // length: u16
    buf.extend_from_slice(&0x12345678u32.to_le_bytes());         // dwLoginKey: u32
    buf.push(1);                                                 // bResult: u8

    let result = TPacketGCAuthSuccess::from_bytes(&buf);
    assert!(result.is_ok(), "Deserializacja powinna zakonczyc sie sukcesem");

    let packet = result.unwrap();
    assert_eq!({ packet.header }, HEADER_GC_AUTH_SUCCESS);
    assert_eq!({ packet.length }, 9);
    assert_eq!(packet.dw_login_key(), 0x12345678);
    assert_eq!(packet.b_result(), 1);
}

#[test]
fn test_auth_success_from_bytes_too_short() {
    let buf = vec![0u8; 8]; // brakuje b_result

    let result = TPacketGCAuthSuccess::from_bytes(&buf);
    assert!(
        result.is_err(),
        "Deserializacja powinna zwrocic blad dla za krotkiego bufora"
    );

    assert_eq!(result.unwrap_err(), PacketError::TooShort);
}

#[test]
fn test_auth_success_from_bytes_invalid_header() {
    let mut buf = Vec::new();
    buf.extend_from_slice(&0x9999u16.to_le_bytes()); // Zly naglowek
    buf.extend_from_slice(&9u16.to_le_bytes());
    buf.extend_from_slice(&0x12345678u32.to_le_bytes());
    buf.push(1);

    let result = TPacketGCAuthSuccess::from_bytes(&buf);
    assert!(
        result.is_err(),
        "Deserializacja powinna zwrocic blad dla niepoprawnego naglowka"
    );

    assert_eq!(result.unwrap_err(), PacketError::InvalidHeader);
}

#[test]
fn test_ffi_auth_success_valid() {
    let mut buf = Vec::new();
    buf.extend_from_slice(&HEADER_GC_AUTH_SUCCESS.to_le_bytes());
    buf.extend_from_slice(&9u16.to_le_bytes());
    buf.extend_from_slice(&0x12345678u32.to_le_bytes());
    buf.push(1);

    let mut out_key: u32 = 0;

    unsafe {
        let success = m2_net_parse_auth_success(buf.as_ptr(), buf.len() as u32, &mut out_key);
        assert!(success);
        assert_eq!(out_key, 0x12345678);
    }
}

#[test]
fn test_ffi_auth_success_null_ptr() {
    let mut buf = Vec::new();
    buf.extend_from_slice(&HEADER_GC_AUTH_SUCCESS.to_le_bytes());
    buf.extend_from_slice(&9u16.to_le_bytes());
    buf.extend_from_slice(&0x12345678u32.to_le_bytes());
    buf.push(1);

    let mut out_key: u32 = 0;

    unsafe {
        let success = m2_net_parse_auth_success(std::ptr::null(), buf.len() as u32, &mut out_key);
        assert!(!success);

        let success2 = m2_net_parse_auth_success(buf.as_ptr(), buf.len() as u32, std::ptr::null_mut());
        assert!(!success2);
    }
}
