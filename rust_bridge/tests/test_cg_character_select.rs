use m2_client_bridge::net::cg::character_select::{CharacterSelectPacket, HEADER_CG_PLAYER_SELECT};

#[test]
fn test_character_select_packet_serialization() {
    // Sprawdzamy sloty 0 do 3 (zgodnie z wymaganiami gry)
    for slot in 0..=3 {
        let packet = CharacterSelectPacket::new(slot);
        
        assert_eq!(packet.header, HEADER_CG_PLAYER_SELECT);
        assert_eq!(packet.player_index, slot);
        
        let bytes = packet.to_bytes();
        
        // Oczekujemy dokladnie 2 bajtow (Little-Endian to tutaj domyslny format, bo mamy tylko u8)
        assert_eq!(bytes.len(), 2);
        assert_eq!(bytes[0], HEADER_CG_PLAYER_SELECT); // header (BYTE)
        assert_eq!(bytes[1], slot); // player_index (BYTE)
    }
}
