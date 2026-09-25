// Reprezentacja struktury pakietu ruchu postaci GC
// Odpowiada TPacketGCMove w kodzie C++
#[derive(Debug, PartialEq)]
pub struct PacketGcCharacterMove {
    pub header: u8,
    pub func: u8,
    pub arg: u8,
    pub rot: u8,
    pub vid: u32,
    pub x: i32,
    pub y: i32,
    pub time: u32,
    pub duration: u32,
}

impl PacketGcCharacterMove {
    // Rozmiar struktury wynosi 24 bajty
    pub const PACKET_SIZE: usize = 24;

    // Deserializuje strukture z bufora bajtow
    pub fn from_bytes(buf: &[u8]) -> Result<Self, String> {
        // Zabezpieczenie przed zbyt krotkim buforem
        if buf.len() < Self::PACKET_SIZE {
            return Err(format!("Bufor jest za krotki. Wymagane {}, otrzymano {}", Self::PACKET_SIZE, buf.len()));
        }

        // Odczyt poszczegolnych pol struktury
        let header = buf[0];
        let func = buf[1];
        let arg = buf[2];
        let rot = buf[3];
        
        let vid = u32::from_le_bytes(buf[4..8].try_into().unwrap());
        let x = i32::from_le_bytes(buf[8..12].try_into().unwrap());
        let y = i32::from_le_bytes(buf[12..16].try_into().unwrap());
        let time = u32::from_le_bytes(buf[16..20].try_into().unwrap());
        let duration = u32::from_le_bytes(buf[20..24].try_into().unwrap());

        Ok(PacketGcCharacterMove {
            header,
            func,
            arg,
            rot,
            vid,
            x,
            y,
            time,
            duration,
        })
    }
}
