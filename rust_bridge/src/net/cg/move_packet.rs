// Serializator ruchu postaci HEADER_CG_MOVE (x, y, rot, move_type, time)

pub struct MovePacket {
    pub header: u8,
    pub func: u8,
    pub arg: u8,
    pub rot: u8,
    pub x: i32,
    pub y: i32,
    pub time: u32,
}

impl MovePacket {
    // Serializuje pakiet do formatu Little-Endian.
    // Zwraca wektor bajtow zserializowanej struktury MovePacket.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(16);
        buffer.push(self.header);
        buffer.push(self.func);
        buffer.push(self.arg);
        buffer.push(self.rot);
        buffer.extend_from_slice(&self.x.to_le_bytes());
        buffer.extend_from_slice(&self.y.to_le_bytes());
        buffer.extend_from_slice(&self.time.to_le_bytes());
        buffer
    }
}
