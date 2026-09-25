#[derive(Debug, PartialEq)]
pub struct GcCharStatusPacket {
    pub vid: u32,
    pub hp: u32,
    pub max_hp: u32,
}

impl GcCharStatusPacket {
    // Parser z bufora bajtow
    pub fn from_bytes(buf: &[u8]) -> Result<Self, String> {
        if buf.len() < 12 {
            return Err("Bufor jest za krotki".to_string());
        }

        let vid = u32::from_le_bytes(buf[0..4].try_into().unwrap());
        let hp = u32::from_le_bytes(buf[4..8].try_into().unwrap());
        let max_hp = u32::from_le_bytes(buf[8..12].try_into().unwrap());

        Ok(GcCharStatusPacket {
            vid,
            hp,
            max_hp,
        })
    }
}
