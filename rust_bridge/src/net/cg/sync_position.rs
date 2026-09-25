use crate::ffi_guard::safe_ffi_call;

pub const HEADER_CG_SYNC_POSITION: u8 = 8;

/// Struktura elementu synchronizacji pozycji.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TPacketCGSyncPositionElement {
    pub dw_vid: u32,
    pub l_x: i32,
    pub l_y: i32,
}

impl TPacketCGSyncPositionElement {
    /// Serializuje strukture do wektora bajtow (Little-Endian).
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(12);
        buf.extend_from_slice(&self.dw_vid.to_le_bytes());
        buf.extend_from_slice(&self.l_x.to_le_bytes());
        buf.extend_from_slice(&self.l_y.to_le_bytes());
        buf
    }
}

/// Struktura pakietu synchronizacji pozycji.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TPacketCGSyncPosition {
    pub header: u8,
    pub w_size: u16,
    pub count: u16,
    pub elements: Vec<TPacketCGSyncPositionElement>,
}

impl TPacketCGSyncPosition {
    /// Tworzy nowy pakiet synchronizacji pozycji.
    pub fn new(elements: Vec<TPacketCGSyncPositionElement>) -> Self {
        let count = elements.len() as u16;
        let w_size = 5 + count * 12; // header (1) + w_size (2) + count (2) + elements (count * 12)
        Self {
            header: HEADER_CG_SYNC_POSITION,
            w_size,
            count,
            elements,
        }
    }

    /// Serializuje pakiet do wektora bajtow.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.w_size as usize);
        buf.push(self.header);
        buf.extend_from_slice(&self.w_size.to_le_bytes());
        buf.extend_from_slice(&self.count.to_le_bytes());
        
        for el in &self.elements {
            buf.extend_from_slice(&el.to_bytes());
        }
        buf
    }
}

/// Eksport C-FFI
#[no_mangle]
pub unsafe extern "C" fn m2_net_send_sync_position(vid: u32, x: i32, y: i32) -> bool {
    safe_ffi_call(false, || {
        let element = TPacketCGSyncPositionElement {
            dw_vid: vid,
            l_x: x,
            l_y: y,
        };
        
        let _packet = TPacketCGSyncPosition::new(vec![element]);
        
        // Brak logiki wysylania przez siec, wiec po prostu zwalniamy zasoby Rusta
        // poprzez opuszczenie scope (Vec elements zostanie automatycznie wyczyszczony)

        true
    })
}
