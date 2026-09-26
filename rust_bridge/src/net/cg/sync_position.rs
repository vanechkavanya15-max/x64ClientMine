// Struktury synchronizacji pozycji CG na podstawie Packet.h: command_sync_position_element oraz command_sync_position
// Komentarze wylacznie po polsku bez polskich znakow diakrytycznych.

use crate::net::constants::cg::SYNC_POSITION;
use crate::ffi_guard::safe_ffi_call;
use crate::net::send_queue::enqueue_outgoing_packet;

pub const HEADER_CG_SYNC_POSITION: u16 = SYNC_POSITION;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGSyncPositionElement {
    pub dw_vid: u32,
    pub l_x: i32,
    pub l_y: i32,
}

impl TPacketCGSyncPositionElement {
    pub const SIZE: usize = std::mem::size_of::<Self>();

    pub fn new(dw_vid: u32, l_x: i32, l_y: i32) -> Self {
        Self {
            dw_vid,
            l_x,
            l_y,
        }
    }

    // Serializuje element synchronizacji pozycji do wektora bajtow (Little-Endian).
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::SIZE);
        buf.extend_from_slice(&self.dw_vid.to_le_bytes());
        buf.extend_from_slice(&self.l_x.to_le_bytes());
        buf.extend_from_slice(&self.l_y.to_le_bytes());
        buf
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGSyncPosition {
    pub header: u16,
    pub length: u16,
}

impl TPacketCGSyncPosition {
    pub const HEADER: u16 = SYNC_POSITION;
    pub const BASE_SIZE: u16 = std::mem::size_of::<Self>() as u16;

    // Tworzy naglowek pakietu synchronizacji pozycji dla zadanej liczby elementow.
    pub fn new(element_count: usize) -> Self {
        let length = Self::BASE_SIZE + (element_count * TPacketCGSyncPositionElement::SIZE) as u16;
        Self {
            header: Self::HEADER,
            length,
        }
    }

    // Serializuje sam naglowek do wektora bajtow (Little-Endian).
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::BASE_SIZE as usize);
        buf.extend_from_slice(&self.header.to_le_bytes());
        buf.extend_from_slice(&self.length.to_le_bytes());
        buf
    }

    // Serializuje naglowek wraz z lista elementow do kompletnej ramki sieciowej.
    pub fn serialize_with_elements(elements: &[TPacketCGSyncPositionElement]) -> Vec<u8> {
        let header = Self::new(elements.len());
        let mut buf = header.to_bytes();
        for el in elements {
            buf.extend_from_slice(&el.to_bytes());
        }
        buf
    }
}

// Funkcja C FFI obslugujaca wysylanie synchronizacji pozycji dla pojedynczej jednostki.
#[no_mangle]
pub unsafe extern "C" fn m2_net_send_sync_position(vid: u32, x: i32, y: i32) -> bool {
    safe_ffi_call(false, || {
        let element = TPacketCGSyncPositionElement::new(vid, x, y);
        let bytes = TPacketCGSyncPosition::serialize_with_elements(&[element]);
        enqueue_outgoing_packet(bytes)
    })
}
