use m2_client_bridge::net::cg::attack::TPacketCgAttack;

#[test]
fn test_cg_attack_serialization() {
    let packet = TPacketCgAttack::new(
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
