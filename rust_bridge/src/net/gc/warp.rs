#[derive(Debug, PartialEq, Eq)]
pub struct WarpPacket {
    pub header: u8,
    pub l_x: i32,
    pub l_y: i32,
    pub l_addr: i32,
    pub w_port: u16,
}

impl WarpPacket {
    /// Oczekiwany rozmiar pakietu wynikajacy ze struktury C++ i #pragma pack(1)
    pub const PACKET_SIZE: usize = 15;

    /// Deserializacja pakietu z bufora bajtow
    pub fn from_bytes(buf: &[u8]) -> Result<Self, String> {
        if buf.len() < Self::PACKET_SIZE {
            return Err(format!("Bufor jest za krotki. Oczekiwano {}, otrzymano {}", Self::PACKET_SIZE, buf.len()));
        }

        let header = buf[0];
        
        let l_x = i32::from_le_bytes(buf[1..5].try_into().unwrap());
        let l_y = i32::from_le_bytes(buf[5..9].try_into().unwrap());
        let l_addr = i32::from_le_bytes(buf[9..13].try_into().unwrap());
        let w_port = u16::from_le_bytes(buf[13..15].try_into().unwrap());

        Ok(WarpPacket {
            header,
            l_x,
            l_y,
            l_addr,
            w_port,
        })
    }
}
