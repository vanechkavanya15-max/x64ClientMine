// Struktura reprezentujaca pakiet GC znikniecia dropu z ziemi (HEADER_GC_ITEM_GROUND_DEL)
#[derive(Debug, PartialEq)]
pub struct GroundItemDelPacket {
    pub header: u8,
    pub vid: u32,
}

impl GroundItemDelPacket {
    // Deserializuje bufor bajtow do struktury GroundItemDelPacket.
    // Zwraca Err jesli bufor jest zbyt krotki.
    pub fn from_bytes(buf: &[u8]) -> Result<Self, String> {
        // Pakiet sklada sie z 1 bajtu naglowka i 4 bajtow identyfikatora VID (razem 5 bajtow)
        if buf.len() < 5 {
            return Err("Bufor jest zbyt krotki aby zdeserializowac GroundItemDelPacket".to_string());
        }

        let header = buf[0];
        
        // Kopiowanie 4 bajtow jako Little Endian (Metin2) i konwersja na u32
        let mut vid_bytes = [0u8; 4];
        vid_bytes.copy_from_slice(&buf[1..5]);
        let vid = u32::from_le_bytes(vid_bytes);

        Ok(GroundItemDelPacket {
            header,
            vid,
        })
    }
}
