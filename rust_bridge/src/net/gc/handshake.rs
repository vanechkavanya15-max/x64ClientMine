// Struktura reprezentujaca pakiet HEADER_GC_HANDSHAKE
#[derive(Debug, PartialEq, Eq)]
pub struct HandshakePacket {
    pub header: u8,
    pub dw_handshake: u32,
    pub dw_time: u32,
    pub l_delta: i32,
}

impl HandshakePacket {
    // Rozmiar pakietu: 1 (header) + 4 (handshake) + 4 (time) + 4 (delta) = 13 bajtow.
    pub const PACKET_SIZE: usize = 13;

    // Deserializacja z bufora bajtow
    pub fn from_bytes(buf: &[u8]) -> Result<Self, String> {
        if buf.len() < Self::PACKET_SIZE {
            return Err(format!("Za krotki bufor. Oczekiwano {}, otrzymano {}", Self::PACKET_SIZE, buf.len()));
        }

        let header = buf[0];
        
        let mut dw_handshake_bytes = [0u8; 4];
        dw_handshake_bytes.copy_from_slice(&buf[1..5]);
        let dw_handshake = u32::from_le_bytes(dw_handshake_bytes);

        let mut dw_time_bytes = [0u8; 4];
        dw_time_bytes.copy_from_slice(&buf[5..9]);
        let dw_time = u32::from_le_bytes(dw_time_bytes);

        let mut l_delta_bytes = [0u8; 4];
        l_delta_bytes.copy_from_slice(&buf[9..13]);
        let l_delta = i32::from_le_bytes(l_delta_bytes);

        Ok(HandshakePacket {
            header,
            dw_handshake,
            dw_time,
            l_delta,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_bytes_success() {
        // Przygotowanie bufora z poprawnymi danymi
        let mut buf = [0u8; 13];
        buf[0] = 0xff; // header
        buf[1..5].copy_from_slice(&12345u32.to_le_bytes()); // dw_handshake
        buf[5..9].copy_from_slice(&98765u32.to_le_bytes()); // dw_time
        buf[9..13].copy_from_slice(&(-100i32).to_le_bytes()); // l_delta

        let packet = HandshakePacket::from_bytes(&buf).unwrap();
        assert_eq!(packet.header, 0xff);
        assert_eq!(packet.dw_handshake, 12345);
        assert_eq!(packet.dw_time, 98765);
        assert_eq!(packet.l_delta, -100);
    }

    #[test]
    fn test_from_bytes_too_short() {
        // Bufor o dlugosci 12 bajtow (za krotki)
        let buf = [0u8; 12];
        let result = HandshakePacket::from_bytes(&buf);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Za krotki bufor. Oczekiwano 13, otrzymano 12");
    }
}
