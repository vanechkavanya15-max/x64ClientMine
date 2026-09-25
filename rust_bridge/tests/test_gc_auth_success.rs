use m2_client_bridge::net::gc::auth_success::{
    m2_net_parse_auth_success, PacketError, TPacketGCAuthSuccess,
};

#[test]
fn test_auth_success_from_bytes_valid() {
    let buf = vec![150, 0x78, 0x56, 0x34, 0x12, 1]; // header, login_key, b_result

    let result = TPacketGCAuthSuccess::from_bytes(&buf);
    assert!(result.is_ok(), "Deserializacja powinna zakonczyc sie sukcesem");

    let packet = result.unwrap();
    assert_eq!(packet.header, 150);
    // Kopiowanie wartosci pola do zmiennej lokalnej aby zapobiec bledowi niewyrownanego dostepu (#repr(packed))
    let login_key = packet.dw_login_key;
    assert_eq!(login_key, 0x12345678);
    assert_eq!(packet.b_result, 1);
}

#[test]
fn test_auth_success_from_bytes_too_short() {
    let buf = vec![150, 0x78, 0x56, 0x34, 0x12]; // brakuje b_result

    let result = TPacketGCAuthSuccess::from_bytes(&buf);
    assert!(
        result.is_err(),
        "Deserializacja powinna zwrocic blad dla za krotkiego bufora"
    );

    assert_eq!(result.unwrap_err(), PacketError::TooShort);
}

#[test]
fn test_auth_success_from_bytes_invalid_header() {
    let buf = vec![151, 0x78, 0x56, 0x34, 0x12, 1]; // Zly naglowek

    let result = TPacketGCAuthSuccess::from_bytes(&buf);
    assert!(
        result.is_err(),
        "Deserializacja powinna zwrocic blad dla niepoprawnego naglowka"
    );

    assert_eq!(result.unwrap_err(), PacketError::InvalidHeader);
}

#[test]
fn test_ffi_auth_success_valid() {
    let buf = vec![150, 0x78, 0x56, 0x34, 0x12, 1];
    let mut out_key: u32 = 0;

    unsafe {
        let success = m2_net_parse_auth_success(buf.as_ptr(), buf.len() as u32, &mut out_key);
        assert!(success);
        assert_eq!(out_key, 0x12345678);
    }
}

#[test]
fn test_ffi_auth_success_null_ptr() {
    let buf = vec![150, 0x78, 0x56, 0x34, 0x12, 1];
    let mut out_key: u32 = 0;

    unsafe {
        let success = m2_net_parse_auth_success(std::ptr::null(), buf.len() as u32, &mut out_key);
        assert!(!success);

        let success2 = m2_net_parse_auth_success(buf.as_ptr(), buf.len() as u32, std::ptr::null_mut());
        assert!(!success2);
    }
}
