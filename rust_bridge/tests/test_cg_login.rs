use m2_client_bridge::net::cg::login::{TPacketCGLogin3, TPacketCGLogin2};

#[test]
fn test_cg_login3_serialization() {
    let packet = TPacketCGLogin3::new("testuser", "password123");
    let bytes = packet.to_bytes();
    
    // header(2) + length(2) + name(31) + pwd(17) = 52 bajty
    assert_eq!(bytes.len(), 52);
    
    assert_eq!(u16::from_le_bytes([bytes[0], bytes[1]]), 0x0102); // LOGIN3
    assert_eq!(u16::from_le_bytes([bytes[2], bytes[3]]), 52);

    // Sprawdzenie pola name (wypelnione zerami)
    let mut expected_name = [0u8; 31];
    expected_name[..8].copy_from_slice(b"testuser");
    assert_eq!(&bytes[4..35], &expected_name);

    // Sprawdzenie pola pwd (wypelnione zerami)
    let mut expected_pwd = [0u8; 17];
    expected_pwd[..11].copy_from_slice(b"password123");
    assert_eq!(&bytes[35..52], &expected_pwd);
}

#[test]
fn test_cg_login2_serialization() {
    let packet = TPacketCGLogin2::new("testuser", 0x12345678);
    let bytes = packet.to_bytes();

    // header(2) + length(2) + name(31) + login_key(4) = 39 bajtow
    assert_eq!(bytes.len(), 39);
    assert_eq!(u16::from_le_bytes([bytes[0], bytes[1]]), 0x0101); // LOGIN2
    assert_eq!(u16::from_le_bytes([bytes[2], bytes[3]]), 39);
    assert_eq!(u32::from_le_bytes([bytes[35], bytes[36], bytes[37], bytes[38]]), 0x12345678);
}
