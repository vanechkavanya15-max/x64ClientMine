use m2_client_bridge::net::cg::character_select::{CharacterSelectPacket, HEADER_CG_PLAYER_SELECT};

#[test]
fn test_character_select_packet_serialization() {
    // Sprawdzamy sloty 0 do 3 (zgodnie z wymaganiami gry)
    for slot in 0..=3 {
        let packet = CharacterSelectPacket::new(slot);
        
        let header = { packet.header };
        let length = { packet.length };
        let player_index = { packet.player_index };

        assert_eq!(header, HEADER_CG_PLAYER_SELECT);
        assert_eq!(length, 5);
        assert_eq!(player_index, slot);
        
        let bytes = packet.to_bytes();
        
        // Oczekujemy 5 bajtow: header (2) + length (2) + player_index (1)
        assert_eq!(bytes.len(), 5);
        assert_eq!(u16::from_le_bytes([bytes[0], bytes[1]]), HEADER_CG_PLAYER_SELECT);
        assert_eq!(u16::from_le_bytes([bytes[2], bytes[3]]), 5);
        assert_eq!(bytes[4], slot);
    }
}
