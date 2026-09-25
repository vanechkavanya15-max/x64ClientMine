// Serializator pakietu tworzenia postaci (HEADER_CG_CHARACTER_CREATE)
// Zgodny z TPacketCGCreateCharacter w C++

pub struct CharacterCreatePacket {
    pub header: u8,
    pub index: u8,
    pub name: [u8; 25],
    pub job: u16,
    pub shape: u8,
    pub con: u8,
    pub int: u8,
    pub str: u8,
    pub dex: u8,
}

impl CharacterCreatePacket {
    // Serializuje strukture do wektora bajtow (Little-Endian)
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(34); // 1+1+25+2+1+1+1+1+1 = 34 bajtow
        buf.push(self.header);
        buf.push(self.index);
        buf.extend_from_slice(&self.name);
        buf.extend_from_slice(&self.job.to_le_bytes());
        buf.push(self.shape);
        buf.push(self.con);
        buf.push(self.int);
        buf.push(self.str);
        buf.push(self.dex);
        buf
    }
}
