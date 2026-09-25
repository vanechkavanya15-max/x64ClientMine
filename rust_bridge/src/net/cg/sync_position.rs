pub const HEADER_CG_SYNC_POSITION: u8 = 8;

/// Struktura elementu synchronizacji pozycji.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CGSyncPositionElement {
    pub vid: u32,
    pub x: i32,
    pub y: i32,
}

impl CGSyncPositionElement {
    /// Serializuje strukture do wektora bajtow (Little-Endian).
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(12);
        buf.extend_from_slice(&self.vid.to_le_bytes());
        buf.extend_from_slice(&self.x.to_le_bytes());
        buf.extend_from_slice(&self.y.to_le_bytes());
        buf
    }
}

/// Struktura pakietu synchronizacji pozycji.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CGSyncPositionPacket {
    pub elements: Vec<CGSyncPositionElement>,
}

impl CGSyncPositionPacket {
    /// Serializuje pakiet do wektora bajtow.
    pub fn to_bytes(&self) -> Vec<u8> {
        let size = 3 + self.elements.len() * 12;
        let mut buf = Vec::with_capacity(size);
        buf.push(HEADER_CG_SYNC_POSITION);
        buf.extend_from_slice(&(size as u16).to_le_bytes());
        
        for el in &self.elements {
            buf.extend_from_slice(&el.to_bytes());
        }
        buf
    }
}
