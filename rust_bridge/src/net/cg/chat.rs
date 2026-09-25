const HEADER_CG_CHAT: u8 = 3;

/// Struktura reprezentujaca pakiet czatu od klienta do serwera (CG)
#[derive(Debug, Clone, PartialEq)]
pub struct CgChatPacket {
    pub chat_type: u8,
    pub message: String,
}

impl CgChatPacket {
    /// Serializuje pakiet do tablicy bajtow (Little-Endian).
    /// Format bajtowy zgodny z C++:
    /// 1 bajt - header (HEADER_CG_CHAT = 3)
    /// 2 bajty - length (dlugosc calkowita: rozmiar naglowka + dlugosc tekstu + 1 na znak '\0')
    /// 1 bajt - type (typ czatu)
    /// n bajtow - string zakonczeni zerem ('\0')
    pub fn to_bytes(&self) -> Vec<u8> {
        let text_len = self.message.len() + 1; // +1 dla null terminusa
        // Calkowita dlugosc to rozmiar TPacketCGChat (4 bajty: header 1 + length 2 + type 1) 
        // oraz dlugosc tekstu (razem ze znakiem '\0')
        let total_len = 4 + text_len;

        let mut buf = Vec::with_capacity(total_len);
        
        // 1 bajt - header
        buf.push(HEADER_CG_CHAT);
        
        // 2 bajty - length (little-endian)
        buf.extend_from_slice(&(total_len as u16).to_le_bytes());
        
        // 1 bajt - typ
        buf.push(self.chat_type);
        
        // n bajtow - wiadomosc string
        buf.extend_from_slice(self.message.as_bytes());
        
        // 1 bajt - null terminator ('\0')
        buf.push(0);

        buf
    }
}
