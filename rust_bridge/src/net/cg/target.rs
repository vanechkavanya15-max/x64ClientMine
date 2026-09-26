// Struktura pakietu celu CG na podstawie Packet.h: command_target
// Komentarze wylacznie po polsku bez polskich znakow diakrytycznych.

use crate::net::constants::cg::TARGET;
use crate::ffi_guard::safe_ffi_call;
use crate::net::send_queue::enqueue_outgoing_packet;

pub const HEADER_CG_TARGET: u16 = TARGET;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGTarget {
    pub header: u16,
    pub length: u16,
    pub dw_vid: u32,
}

impl TPacketCGTarget {
    pub const HEADER: u16 = TARGET;
    pub const SIZE: u16 = std::mem::size_of::<Self>() as u16;

    // Tworzy nowy pakiet wyboru celu.
    pub fn new(dw_vid: u32) -> Self {
        Self {
            header: Self::HEADER,
            length: Self::SIZE,
            dw_vid,
        }
    }

    // Serializuje pakiet do wektora bajtow (Little-Endian).
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::SIZE as usize);
        buf.extend_from_slice(&self.header.to_le_bytes());
        buf.extend_from_slice(&self.length.to_le_bytes());
        buf.extend_from_slice(&self.dw_vid.to_le_bytes());
        buf
    }
}

// Funkcja C FFI wysylajaca pakiet wyboru celu.
#[no_mangle]
pub extern "C" fn m2_net_send_target(dw_vid: u32) -> bool {
    safe_ffi_call(false, || {
        let packet = TPacketCGTarget::new(dw_vid);
        let bytes = packet.to_bytes();
        enqueue_outgoing_packet(bytes)
    })
}
