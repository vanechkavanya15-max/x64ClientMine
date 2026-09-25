use m2_client_bridge::net::cg::chat::CgChatPacket;

#[test]
fn test_cg_chat_packet_serialization() {
    let packet = CgChatPacket {
        chat_type: 1,
        message: "hello".to_string(),
    };

    let serialized = packet.to_bytes();

    // Oczekiwana dlugosc to 4 (header TPacketCGChat) + 5 ("hello") + 1 ('\0') = 10 bajtow
    // [3, 10, 0, 1, 104, 101, 108, 108, 111, 0]
    let expected: Vec<u8> = vec![
        3,          // HEADER_CG_CHAT
        10, 0,      // length (little-endian u16)
        1,          // chat_type
        104, 101, 108, 108, 111, // "hello" w ASCII
        0           // null terminator
    ];

    assert_eq!(serialized, expected, "Nieprawidlowy uklad bajtow dla CgChatPacket");
}

#[test]
fn test_cg_chat_packet_serialization_empty_string() {
    let packet = CgChatPacket {
        chat_type: 2,
        message: "".to_string(),
    };

    let serialized = packet.to_bytes();

    // Oczekiwana dlugosc to 4 (header) + 0 ("") + 1 ('\0') = 5 bajtow
    // [3, 5, 0, 2, 0]
    let expected: Vec<u8> = vec![
        3,          // HEADER_CG_CHAT
        5, 0,       // length (little-endian u16)
        2,          // chat_type
        0           // null terminator
    ];

    assert_eq!(serialized, expected, "Nieprawidlowy uklad bajtow dla CgChatPacket (pusty string)");
}
