/// Struktura reprezentujaca pakiet usuniecia postaci z widoku (HEADER_GC_CHARACTER_DEL)
#[derive(Debug, PartialEq, Eq)]
pub struct PacketGcCharacterDelete {
    pub header: u8,
    pub vid: u32,
}

impl PacketGcCharacterDelete {
    /// Deserializuje pakiet z bufora bajtow
    ///
    /// Rozmiar pakietu to 5 bajtow (1 bajt na header + 4 bajty na vid)
    pub fn from_bytes(buf: &[u8]) -> Result<Self, String> {
        if buf.len() < 5 {
            return Err("Bufor jest za krotki".to_string());
        }

        let header = buf[0];
        
        // Odczyt VID jako 32-bitowej liczby calkowitej bez znaku, w formacie little-endian
        let vid_bytes = [buf[1], buf[2], buf[3], buf[4]];
        let vid = u32::from_le_bytes(vid_bytes);

        Ok(Self { header, vid })
    }
}
