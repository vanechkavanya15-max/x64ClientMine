// Reprezentuje pakiet wyboru postaci (slot 0-3) z klienta do serwera.
// Odpowiada to HEADER_CG_PLAYER_SELECT (6) z C++.

pub const HEADER_CG_PLAYER_SELECT: u8 = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharacterSelectPacket {
    pub header: u8,
    pub player_index: u8,
}

impl CharacterSelectPacket {
    // Tworzy nowa instancje pakietu z podanym indeksem gracza
    pub fn new(player_index: u8) -> Self {
        Self {
            header: HEADER_CG_PLAYER_SELECT,
            player_index,
        }
    }

    // Serializuje pakiet do wektora bajtow (Little-Endian)
    // Struktura C++ TPacketCGSelectCharacter ma 2 bajty: header i player_index
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(2);
        buf.push(self.header);
        buf.push(self.player_index);
        buf
    }
}
