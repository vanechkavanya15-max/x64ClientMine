#![allow(non_snake_case)]

// Struktura pakietu usuniecia przedmiotu z ekwipunku (Game -> Client)
// Zgodna z E:\m2dev-client-src-main\src\UserInterface\Packet.h (TPacketGCItemDel)

use crate::ffi_guard::safe_ffi_call;
use std::mem::size_of;

pub const HEADER_GC_ITEM_DEL: u16 = 0x0510;

#[derive(Debug, PartialEq, Eq)]
pub enum PacketError {
    TooShort,
    InvalidHeader,
}

#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TItemPos {
    pub window_type: u8,
    pub cell: u16,
}

// Struktura pakietu usuniecia przedmiotu
// Rozmiar: header(2) + length(2) + pos(3) = 7 bajtow
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TPacketGCItemDel {
    pub header: u16,
    pub length: u16,
    pub pos: TItemPos,
}

impl TPacketGCItemDel {
    pub const PACKET_SIZE: usize = size_of::<Self>();

    pub fn from_bytes(data: &[u8]) -> Result<Self, PacketError> {
        if data.len() < Self::PACKET_SIZE {
            return Err(PacketError::TooShort);
        }

        let header = u16::from_le_bytes(data[0..2].try_into().unwrap());
        let length = u16::from_le_bytes(data[2..4].try_into().unwrap());
        let window_type = data[4];
        let cell = u16::from_le_bytes(data[5..7].try_into().unwrap());

        Ok(Self {
            header,
            length,
            pos: TItemPos {
                window_type,
                cell,
            },
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_parse_item_del(
    data: *const u8,
    len: u32,
    out_win: *mut u8,
    out_cell: *mut u16,
) -> bool {
    safe_ffi_call(false, || {
        if data.is_null() || out_win.is_null() || out_cell.is_null() {
            return false;
        }

        let slice = std::slice::from_raw_parts(data, len as usize);
        match TPacketGCItemDel::from_bytes(slice) {
            Ok(packet) => {
                *out_win = packet.pos.window_type;
                *out_cell = packet.pos.cell;
                true
            }
            Err(_) => false,
        }
    })
}
