// Struktury pakietow wyrzucenia przedmiotu CG na podstawie Packet.h: command_item_drop oraz command_item_drop2
// Komentarze wylacznie po polsku bez polskich znakow diakrytycznych.

use crate::net::constants::cg::{ITEM_DROP, ITEM_DROP2};
use crate::net::cg::item_use::TItemPos;
use crate::ffi_guard::safe_ffi_call;
use crate::net::send_queue::enqueue_outgoing_packet;

// Pakiet upuszczenia przedmiotu / jang
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGItemDrop {
    pub header: u16,
    pub length: u16,
    pub pos: TItemPos,
    pub elk: u32,
}

impl TPacketCGItemDrop {
    pub const HEADER: u16 = ITEM_DROP;
    pub const SIZE: u16 = std::mem::size_of::<Self>() as u16;

    pub fn new(window_type: u8, cell: u16, elk: u32) -> Self {
        Self {
            header: Self::HEADER,
            length: Self::SIZE,
            pos: TItemPos { window_type, cell },
            elk,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::SIZE as usize);
        buf.extend_from_slice(&self.header.to_le_bytes());
        buf.extend_from_slice(&self.length.to_le_bytes());
        buf.push(self.pos.window_type);
        buf.extend_from_slice(&self.pos.cell.to_le_bytes());
        buf.extend_from_slice(&self.elk.to_le_bytes());
        buf
    }
}

// Pakiet upuszczenia przedmiotu z iloscia (ITEM_DROP2)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGItemDrop2 {
    pub header: u16,
    pub length: u16,
    pub pos: TItemPos,
    pub gold: u32,
    pub count: u8,
}

impl TPacketCGItemDrop2 {
    pub const HEADER: u16 = ITEM_DROP2;
    pub const SIZE: u16 = std::mem::size_of::<Self>() as u16;

    pub fn new(window_type: u8, cell: u16, gold: u32, count: u8) -> Self {
        Self {
            header: Self::HEADER,
            length: Self::SIZE,
            pos: TItemPos { window_type, cell },
            gold,
            count,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::SIZE as usize);
        buf.extend_from_slice(&self.header.to_le_bytes());
        buf.extend_from_slice(&self.length.to_le_bytes());
        buf.push(self.pos.window_type);
        buf.extend_from_slice(&self.pos.cell.to_le_bytes());
        buf.extend_from_slice(&self.gold.to_le_bytes());
        buf.push(self.count);
        buf
    }
}

// Funkcja C FFI wysylajaca pakiet upuszczenia przedmiotu.
#[no_mangle]
pub unsafe extern "C" fn m2_net_send_item_drop(window_type: u8, slot: u16, count: u32) -> bool {
    safe_ffi_call(false, || {
        let packet = TPacketCGItemDrop::new(window_type, slot, count);
        let bytes = packet.to_bytes();
        enqueue_outgoing_packet(bytes)
    })
}
