// Struktura pakietu strzalu CG na podstawie Packet.h: packet_shoot
// Komentarze wylacznie po polsku bez polskich znakow diakrytycznych.

use crate::net::constants::cg::SHOOT;
use crate::ffi_guard::safe_ffi_call;
use crate::net::send_queue::enqueue_outgoing_packet;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGShoot {
    pub header: u16,
    pub length: u16,
    pub b_type: u8,
}

impl TPacketCGShoot {
    pub const HEADER: u16 = SHOOT;
    pub const SIZE: u16 = std::mem::size_of::<Self>() as u16;

    // Tworzy nowy pakiet strzalu dla danego typu.
    pub fn new(b_type: u8) -> Self {
        Self {
            header: Self::HEADER,
            length: Self::SIZE,
            b_type,
        }
    }

    // Serializuje pakiet do wektora bajtow (Little-Endian).
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::SIZE as usize);
        buf.extend_from_slice(&self.header.to_le_bytes());
        buf.extend_from_slice(&self.length.to_le_bytes());
        buf.push(self.b_type);
        buf
    }
}

// Funkcja C FFI wysylajaca pakiet strzalu.
#[no_mangle]
pub unsafe extern "C" fn m2_net_send_shoot(b_type: u8) -> bool {
    safe_ffi_call(false, || {
        let pkt = TPacketCGShoot::new(b_type);
        let bytes = pkt.to_bytes();
        enqueue_outgoing_packet(bytes)
    })
}
