use m2_client_bridge::net::cg::move_packet::MovePacket;

#[test]
fn test_cg_move_packet_serialization() {
    // Przygotowanie testowego pakietu ruchu
    // Wartosci dobieramy tak, by sprawdzic endiannosc poszczegolnych pol.
    let packet = MovePacket {
        header: 7, // HEADER_CG_CHARACTER_MOVE
        func: 1,
        arg: 2,
        rot: 10, // Przykladowa rotacja
        x: 0x12345678,
        y: 0x23456789,
        time: 0x3456789A,
    };

    let serialized = packet.to_bytes();

    // Dlugosc zserializowanego pakietu to 16 bajtow (4 x 1 + 2 x 4 + 4)
    assert_eq!(serialized.len(), 16);

    // Weryfikacja poszczegolnych bajtow w Little-Endian
    assert_eq!(serialized[0], 7);
    assert_eq!(serialized[1], 1);
    assert_eq!(serialized[2], 2);
    assert_eq!(serialized[3], 10);
    
    // x = 0x12345678 -> [0x78, 0x56, 0x34, 0x12]
    assert_eq!(serialized[4], 0x78);
    assert_eq!(serialized[5], 0x56);
    assert_eq!(serialized[6], 0x34);
    assert_eq!(serialized[7], 0x12);

    // y = 0x23456789 -> [0x89, 0x67, 0x45, 0x23]
    assert_eq!(serialized[8], 0x89);
    assert_eq!(serialized[9], 0x67);
    assert_eq!(serialized[10], 0x45);
    assert_eq!(serialized[11], 0x23);

    // time = 0x3456789A -> [0x9A, 0x78, 0x56, 0x34]
    assert_eq!(serialized[12], 0x9A);
    assert_eq!(serialized[13], 0x78);
    assert_eq!(serialized[14], 0x56);
    assert_eq!(serialized[15], 0x34);
}
