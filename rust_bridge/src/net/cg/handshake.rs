//! Serializacja pakietu PONG oraz HANDSHAKE w kierunku Klient -> Serwer

#[derive(Debug, Clone, PartialEq)]
pub struct PacketCgPong {
    pub header: u8,
}

impl PacketCgPong {
    pub const HEADER: u8 = 0xfe;

    pub fn new() -> Self {
        Self {
            header: Self::HEADER,
        }
    }

    /// Serializuje pakiet do tablicy bajtow.
    pub fn to_bytes(&self) -> Vec<u8> {
        vec![self.header]
    }
}

impl Default for PacketCgPong {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PacketCgHandshake {
    pub header: u8,
    pub handshake: u32,
    pub time: u32,
    pub delta: i32,
}

impl PacketCgHandshake {
    // Pakiet od klienta na re-sync korzysta czesto z TIME_SYNC,
    // ale zachowujemy identyczny uklad dla obu wariantow w razie potrzeby.
    pub const HEADER: u8 = 0xff;
    pub const HEADER_TIME_SYNC: u8 = 0xfc;

    pub fn new(handshake: u32, time: u32, delta: i32) -> Self {
        Self {
            header: Self::HEADER,
            handshake,
            time,
            delta,
        }
    }

    pub fn new_time_sync(handshake: u32, time: u32, delta: i32) -> Self {
        Self {
            header: Self::HEADER_TIME_SYNC,
            handshake,
            time,
            delta,
        }
    }

    /// Serializuje pakiet do bufora z zachowaniem formatu Little-Endian
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(13);
        
        bytes.push(self.header);
        
        // Zgodnie z `#pragma pack(1)` i typami DWORD (u32), LONG (i32) w kodzie zrodlowym C++
        bytes.extend_from_slice(&self.handshake.to_le_bytes());
        bytes.extend_from_slice(&self.time.to_le_bytes());
        bytes.extend_from_slice(&self.delta.to_le_bytes());
        
        bytes
    }
}
