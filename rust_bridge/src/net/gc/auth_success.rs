#[derive(Debug, PartialEq)]
pub struct AuthSuccessPacket {
    pub header: u8,
    pub login_key: u32,
    pub encryption_keys: [u32; 4],
}

impl AuthSuccessPacket {
    // 1 bajt (header) + 4 bajty (login_key) + 16 bajtow (encryption_keys, 4 * u32) = 21 bajtow
    pub const PACKET_SIZE: usize = 21;

    pub fn from_bytes(buf: &[u8]) -> Result<Self, String> {
        if buf.len() < Self::PACKET_SIZE {
            return Err(format!(
                "Bufor jest za krotki: oczekiwano minimum {} bajtow, otrzymano {}",
                Self::PACKET_SIZE,
                buf.len()
            ));
        }

        let header = buf[0];
        let login_key = u32::from_le_bytes([buf[1], buf[2], buf[3], buf[4]]);
        
        let mut encryption_keys = [0u32; 4];
        for i in 0..4 {
            let start = 5 + i * 4;
            encryption_keys[i] = u32::from_le_bytes([
                buf[start], buf[start + 1], buf[start + 2], buf[start + 3]
            ]);
        }

        Ok(Self {
            header,
            login_key,
            encryption_keys,
        })
    }
}
