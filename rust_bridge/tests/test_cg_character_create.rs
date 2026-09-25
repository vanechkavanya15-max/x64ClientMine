// Testy jednostkowe serializatora tworzenia postaci

use m2_client_bridge::net::cg::character_create::CharacterCreatePacket;

#[test]
fn test_character_create_packet_serialization() {
    let mut name = [0u8; 25];
    let name_str = b"TestName";
    name[..name_str.len()].copy_from_slice(name_str);

    let packet = CharacterCreatePacket {
        header: 4, // HEADER_CG_PLAYER_CREATE
        index: 1,
        name: name,
        job: 2,
        shape: 3,
        con: 10,
        int: 20,
        str: 30,
        dex: 40,
    };

    let bytes = packet.to_bytes();
    
    // Sprawdzenie ogolnego rozmiaru bufora (34 bajty)
    assert_eq!(bytes.len(), 34);
    
    // Sprawdzenie pol 1-bajtowych
    assert_eq!(bytes[0], 4);
    assert_eq!(bytes[1], 1);
    
    // Sprawdzenie tablicy znakow (nazwy postaci) zakonczonej zerami
    for i in 0..8 {
        assert_eq!(bytes[2 + i], name_str[i]);
    }
    for i in 8..25 {
        assert_eq!(bytes[2 + i], 0);
    }
    
    // Sprawdzenie pola 2-bajtowego w Little-Endian
    assert_eq!(bytes[27], 2);
    assert_eq!(bytes[28], 0);
    
    // Sprawdzenie pozostalych pol
    assert_eq!(bytes[29], 3);
    assert_eq!(bytes[30], 10);
    assert_eq!(bytes[31], 20);
    assert_eq!(bytes[32], 30);
    assert_eq!(bytes[33], 40);
}
