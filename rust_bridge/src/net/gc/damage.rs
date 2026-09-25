// Struktura pakietu GCDamageInfo odpowiadajaca TPacketGCDamageInfo w C++.
// W zwiazku z `#pragma pack(1)` wszystkie pola sa upakowane jedno po drugim bez paddingu.
#[derive(Debug, PartialEq)]
pub struct GcDamageInfoPacket {
    pub header: u8,
    pub victim_vid: u32,
    pub flags: u8,
    pub damage: i32,
}

impl GcDamageInfoPacket {
    // Rozmiar struktury wynikajacy z: 1 (header) + 4 (vid) + 1 (flag) + 4 (damage) = 10 bajtow.
    pub const PACKET_SIZE: usize = 10;

    /// Deserializuje pakiet z bufora bajtow.
    pub fn from_bytes(buf: &[u8]) -> Result<Self, String> {
        // Zabezpieczenie przed za krotkim buforem. Zwracamy Err, zeby uniknac paniki (np. przy uzyciu indeksowania na out of bounds).
        if buf.len() < Self::PACKET_SIZE {
            return Err(format!(
                "Bufor jest za krotki. Wymagane {}, otrzymano {}",
                Self::PACKET_SIZE,
                buf.len()
            ));
        }

        // Odczyt wartosci uzywajac little-endian (typowy dla x86, architektury Metin2).
        let header = buf[0];
        
        // Zczytanie 4 bajtow dla victim_vid i przekonwertowanie na u32.
        let mut vid_bytes = [0u8; 4];
        vid_bytes.copy_from_slice(&buf[1..5]);
        let victim_vid = u32::from_le_bytes(vid_bytes);

        // Odczyt flagi (pojedynczy bajt).
        let flags = buf[5];

        // Zczytanie 4 bajtow dla damage i przekonwertowanie na i32.
        let mut damage_bytes = [0u8; 4];
        damage_bytes.copy_from_slice(&buf[6..10]);
        let damage = i32::from_le_bytes(damage_bytes);

        Ok(Self {
            header,
            victim_vid,
            flags,
            damage,
        })
    }
}
