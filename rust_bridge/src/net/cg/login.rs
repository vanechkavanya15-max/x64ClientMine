const HEADER_CG_LOGIN3: u8 = 111;
const ID_MAX_NUM: usize = 30;
const PASS_MAX_NUM: usize = 16;

/// Struktura pakietu logowania (CG)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CgLogin3 {
    pub name: String,
    pub pwd: String,
    pub client_key: [u32; 4],
    pub crc_piece: u32,
}

impl CgLogin3 {
    /// Serializuje pakiet do bufora bajtow (Little-Endian)
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(1 + ID_MAX_NUM + 1 + PASS_MAX_NUM + 1 + 16 + 4);
        
        // Naglowek
        buf.push(HEADER_CG_LOGIN3);
        
        // Pole name (ID_MAX_NUM + 1), dopelniane zerami
        let mut name_bytes = [0u8; ID_MAX_NUM + 1];
        let name_len = self.name.len().min(ID_MAX_NUM);
        name_bytes[..name_len].copy_from_slice(&self.name.as_bytes()[..name_len]);
        buf.extend_from_slice(&name_bytes);

        // Pole pwd (PASS_MAX_NUM + 1), dopelniane zerami
        let mut pwd_bytes = [0u8; PASS_MAX_NUM + 1];
        let pwd_len = self.pwd.len().min(PASS_MAX_NUM);
        pwd_bytes[..pwd_len].copy_from_slice(&self.pwd.as_bytes()[..pwd_len]);
        buf.extend_from_slice(&pwd_bytes);

        // Pole client_key (4 * 4 bajty w formacie Little-Endian)
        for &k in &self.client_key {
            buf.extend_from_slice(&k.to_le_bytes());
        }

        // Pole crc_piece (4 bajty w formacie Little-Endian)
        buf.extend_from_slice(&self.crc_piece.to_le_bytes());

        buf
    }
}
