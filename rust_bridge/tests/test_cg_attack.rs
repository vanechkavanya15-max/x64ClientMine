use m2_client_bridge::net::cg::attack::TPacketCGAttack;

#[test]
fn test_cg_attack_serialization() {
    let packet = TPacketCGAttack::new(
        1,              // b_type
        0x12345678,     // dw_victim_vid
        0xAB,           // b_crc_magic_cube_proc_piece
        0xCD,           // b_crc_magic_cube_file_piece
    );

    let bytes = packet.to_bytes();

    // Sprawdzenie oczekiwanego rozmiaru pakietu (2 + 2 + 1 + 4 + 1 + 1 = 11 bajtow)
    assert_eq!(bytes.len(), 11);

    // Sprawdzenie dokladnego ukladu bajtow (header u16: 0x0401, length u16: 11)
    let header = u16::from_le_bytes([bytes[0], bytes[1]]);
    assert_eq!(header, 0x0401);
    let length = u16::from_le_bytes([bytes[2], bytes[3]]);
    assert_eq!(length, 11);
    assert_eq!(bytes[4], 1);    // b_type
    
    // dw_victim_vid w formacie Little-Endian (0x12345678 -> 0x78, 0x56, 0x34, 0x12)
    assert_eq!(bytes[5], 0x78);
    assert_eq!(bytes[6], 0x56);
    assert_eq!(bytes[7], 0x34);
    assert_eq!(bytes[8], 0x12);

    assert_eq!(bytes[9], 0xAB); // b_crc_magic_cube_proc_piece
    assert_eq!(bytes[10], 0xCD); // b_crc_magic_cube_file_piece
}

#[test]
fn test_m2_net_build_cg_attack_success() {
    let mut buf = [0u8; 11];
    let result = unsafe {
        m2_client_bridge::net::cg::attack::m2_net_build_cg_attack(
            0x12345678,
            1,
            buf.as_mut_ptr(),
        )
    };
    
    assert!(result);
    let header = u16::from_le_bytes([buf[0], buf[1]]);
    assert_eq!(header, 0x0401);
    let length = u16::from_le_bytes([buf[2], buf[3]]);
    assert_eq!(length, 11);
    assert_eq!(buf[4], 1);    // b_type
    assert_eq!(buf[5], 0x78); // dw_victim_vid le
    assert_eq!(buf[6], 0x56);
    assert_eq!(buf[7], 0x34);
    assert_eq!(buf[8], 0x12);
    assert_eq!(buf[9], 0);    // crc proc
    assert_eq!(buf[10], 0);   // crc file
}

#[test]
fn test_m2_net_build_cg_attack_null_ptr() {
    let result = unsafe {
        m2_client_bridge::net::cg::attack::m2_net_build_cg_attack(
            0x12345678,
            1,
            std::ptr::null_mut(),
        )
    };
    
    assert!(!result);
}
