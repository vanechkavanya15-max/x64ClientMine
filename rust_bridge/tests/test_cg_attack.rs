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

    // Sprawdzenie oczekiwanego rozmiaru pakietu (1 + 1 + 4 + 1 + 1 = 8 bajtow)
    assert_eq!(bytes.len(), 8);

    // Sprawdzenie dokladnego ukladu bajtow
    assert_eq!(bytes[0], 2);    // header (HEADER_CG_ATTACK = 2)
    assert_eq!(bytes[1], 1);    // b_type
    
    // dw_victim_vid w formacie Little-Endian (0x12345678 -> 0x78, 0x56, 0x34, 0x12)
    assert_eq!(bytes[2], 0x78);
    assert_eq!(bytes[3], 0x56);
    assert_eq!(bytes[4], 0x34);
    assert_eq!(bytes[5], 0x12);

    assert_eq!(bytes[6], 0xAB); // b_crc_magic_cube_proc_piece
    assert_eq!(bytes[7], 0xCD); // b_crc_magic_cube_file_piece
}

#[test]
fn test_m2_net_build_cg_attack_success() {
    let mut buf = [0u8; 8];
    let result = unsafe {
        m2_client_bridge::net::cg::attack::m2_net_build_cg_attack(
            0x12345678,
            1,
            buf.as_mut_ptr(),
        )
    };
    
    assert!(result);
    assert_eq!(buf[0], 2);    // header
    assert_eq!(buf[1], 1);    // b_type
    assert_eq!(buf[2], 0x78); // dw_victim_vid le
    assert_eq!(buf[3], 0x56);
    assert_eq!(buf[4], 0x34);
    assert_eq!(buf[5], 0x12);
    assert_eq!(buf[6], 0);    // crc proc
    assert_eq!(buf[7], 0);    // crc file
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
