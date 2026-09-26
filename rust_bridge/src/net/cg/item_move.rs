// Struktura pakietu przeniesienia przedmiotu CG na podstawie Packet.h: command_item_move
// Komentarze wylacznie po polsku bez polskich znakow diakrytycznych.

use crate::net::constants::cg::ITEM_MOVE;
use crate::net::cg::item_use::TItemPos;
use crate::ffi_guard::safe_ffi_call;
use crate::net::send_queue::enqueue_outgoing_packet;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGItemMove {
    pub header: u16,
    pub length: u16,
    pub pos: TItemPos,
    pub change_pos: TItemPos,
    pub num: u8,
}

impl TPacketCGItemMove {
    pub const HEADER: u16 = ITEM_MOVE;
    pub const SIZE: u16 = std::mem::size_of::<Self>() as u16;

    // Tworzy nowy pakiet przesuniecia przedmiotu.
    pub fn new(src_win: u8, src_cell: u16, dst_win: u8, dst_cell: u16, num: u8) -> Self {
        Self {
            header: Self::HEADER,
            length: Self::SIZE,
            pos: TItemPos {
                window_type: src_win,
                cell: src_cell,
            },
            change_pos: TItemPos {
                window_type: dst_win,
                cell: dst_cell,
            },
            num,
        }
    }

    // Serializuje strukture do wektora bajtow (Little-Endian).
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::SIZE as usize);
        buf.extend_from_slice(&self.header.to_le_bytes());
        buf.extend_from_slice(&self.length.to_le_bytes());
        buf.push(self.pos.window_type);
        buf.extend_from_slice(&self.pos.cell.to_le_bytes());
        buf.push(self.change_pos.window_type);
        buf.extend_from_slice(&self.change_pos.cell.to_le_bytes());
        buf.push(self.num);
        buf
    }
}

// Funkcja C FFI obslugujaca przeniesienie przedmiotu.
#[no_mangle]
pub unsafe extern "C" fn m2_net_send_item_move(
    src_win: u8,
    src_cell: u16,
    dst_win: u8,
    dst_cell: u16,
    count: u8,
) -> bool {
    safe_ffi_call(false, || {
        let pkt = TPacketCGItemMove::new(src_win, src_cell, dst_win, dst_cell, count);
        let bytes = pkt.to_bytes();
        enqueue_outgoing_packet(bytes)
    })
}
