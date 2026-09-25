use m2_client_bridge::net::cg::login::CgLogin3;

#[test]
fn test_cg_login3_serialization() {
    // Tworzenie pakietu z przykladowymi danymi
    let packet = CgLogin3 {
        name: "testuser".to_string(),
        pwd: "password123".to_string(),
        client_key: [0x11223344, 0x55667788, 0x99AABBCC, 0xDDEEFF00],
        crc_piece: 0x4A3B2C1D,
    };

    let bytes = packet.to_bytes();
    
    // Sprawdzenie rozmiaru: header(1) + name(31) + pwd(17) + client_key(16) + crc_piece(4) = 69
    assert_eq!(bytes.len(), 69);
    
    // Sprawdzenie naglowka
    assert_eq!(bytes[0], 111); // HEADER_CG_LOGIN3

    // Sprawdzenie pola name (wypelnione zerami)
    let mut expected_name = [0u8; 31];
    expected_name[..8].copy_from_slice(b"testuser");
    assert_eq!(&bytes[1..32], &expected_name);

    // Sprawdzenie pola pwd (wypelnione zerami)
    let mut expected_pwd = [0u8; 17];
    expected_pwd[..11].copy_from_slice(b"password123");
    assert_eq!(&bytes[32..49], &expected_pwd);

    // Sprawdzenie klucza w formacie Little-Endian
    assert_eq!(&bytes[49..53], &[0x44, 0x33, 0x22, 0x11]);
    assert_eq!(&bytes[53..57], &[0x88, 0x77, 0x66, 0x55]);
    assert_eq!(&bytes[57..61], &[0xCC, 0xBB, 0xAA, 0x99]);
    assert_eq!(&bytes[61..65], &[0x00, 0xFF, 0xEE, 0xDD]);

    // Sprawdzenie pola crc_piece w formacie Little-Endian
    assert_eq!(&bytes[65..69], &[0x1D, 0x2C, 0x3B, 0x4A]);
}
