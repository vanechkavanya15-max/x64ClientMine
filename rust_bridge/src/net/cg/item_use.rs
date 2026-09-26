// Struktura pakietu uzycia przedmiotu CG na podstawie Packet.h: command_item_use
// Komentarze wylacznie po polsku bez polskich znakow diakrytycznych.

use crate::net::constants::cg::ITEM_USE;
use crate::ffi_guard::safe_ffi_call;
use crate::net::send_queue::enqueue_outgoing_packet;

// Pozycja przedmiotu w ekwipunku / oknach gry (GameType.h SItemPos)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TItemPos {
    pub window_type: u8,
    pub cell: u16,
}

impl TItemPos {
    pub fn new(window_type: u8, cell: u16) -> Self {
        Self { window_type, cell }
    }
}

// Pakiet CG uzycia przedmiotu
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGItemUse {
    pub header: u16,
    pub length: u16,
    pub pos: TItemPos,
}

impl TPacketCGItemUse {
    pub const HEADER: u16 = ITEM_USE;
    pub const SIZE: u16 = std::mem::size_of::<Self>() as u16;

    // Tworzy nowy pakiet uzycia przedmiotu z podanymi parametrami.
    pub fn new(window_type: u8, cell: u16) -> Self {
        Self {
            header: Self::HEADER,
            length: Self::SIZE,
            pos: TItemPos {
                window_type,
                cell,
            },
        }
    }

    // Tworzy nowy pakiet na podstawie istniejacej struktury TItemPos.
    pub fn new_with_pos(pos: TItemPos) -> Self {
        Self {
            header: Self::HEADER,
            length: Self::SIZE,
            pos,
        }
    }

    // Serializuje strukture do wektora bajtow (Little-Endian).
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::SIZE as usize);
        buf.extend_from_slice(&self.header.to_le_bytes());
        buf.extend_from_slice(&self.length.to_le_bytes());
        buf.push(self.pos.window_type);
        buf.extend_from_slice(&self.pos.cell.to_le_bytes());
        buf
    }
}

// Funkcja C FFI wysylajaca zadanie uzycia przedmiotu.
#[no_mangle]
pub unsafe extern "C" fn m2_net_send_item_use(window_type: u8, slot: u16) -> bool {
    safe_ffi_call(false, || {
        let packet = TPacketCGItemUse::new(window_type, slot);
        let bytes = packet.to_bytes();
        enqueue_outgoing_packet(bytes)
    })
}
