use m2_client_bridge::net::cg::move_packet::TPacketCGMove;

#[test]
fn test_cg_move_packet_serialization() {
    let packet = TPacketCGMove::new(
        1,          // func
        2,          // arg
        10,         // rot
        0x12345678, // x
        0x23456789, // y
        0x3456789A, // time
    );

    let serialized = packet.to_bytes();

    // Dlugosc: header(2) + length(2) + func(1) + arg(1) + rot(1) + x(4) + y(4) + time(4) = 19 bajtow
    assert_eq!(serialized.len(), 19);

    assert_eq!(u16::from_le_bytes([serialized[0], serialized[1]]), 0x0301); // CG::MOVE
    assert_eq!(u16::from_le_bytes([serialized[2], serialized[3]]), 19);

    assert_eq!(serialized[4], 1);
    assert_eq!(serialized[5], 2);
    assert_eq!(serialized[6], 10);
    
    // x = 0x12345678 -> [0x78, 0x56, 0x34, 0x12]
    assert_eq!(&serialized[7..11], &[0x78, 0x56, 0x34, 0x12]);

    // y = 0x23456789 -> [0x89, 0x67, 0x45, 0x23]
    assert_eq!(&serialized[11..15], &[0x89, 0x67, 0x45, 0x23]);

    // time = 0x3456789A -> [0x9A, 0x78, 0x56, 0x34]
    assert_eq!(&serialized[15..19], &[0x9A, 0x78, 0x56, 0x34]);
}
