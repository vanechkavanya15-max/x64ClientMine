#![allow(non_snake_case)]

// Struktura pakietu ustawienia przedmiotu w ekwipunku (Game -> Client)
// Zgodna z E:\m2dev-client-src-main\src\UserInterface\Packet.h (TPacketGCItemSet)

use crate::ffi_guard::safe_ffi_call;
use std::slice;
use std::ptr;

pub const HEADER_GC_ITEM_SET: u16 = 0x0511;
pub const ITEM_SOCKET_SLOT_MAX_NUM: usize = 3;
pub const ITEM_ATTRIBUTE_SLOT_MAX_NUM: usize = 7;

#[derive(Debug, PartialEq, Eq)]
pub enum PacketError {
    BufferTooSmall,
    InvalidHeader,
}

#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TItemPos {
    pub window_type: u8,
    pub cell: u16,
}

#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(non_snake_case)]
pub struct TPlayerItemAttribute {
    pub bType: u8,
    pub sValue: i16,
}

// Struktura pakietu TPacketGCItemSet
// Rozmiar: header(2) + length(2) + pos(3) + vnum(4) + count(1) + flags(4)
// + anti_flags(4) + highlight(1) + alSockets(12) + aAttr(21) = 54 bajty
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(non_snake_case)]
pub struct TPacketGCItemSet {
    pub header: u16,
    pub length: u16,
    pub pos: TItemPos,
    pub vnum: u32,
    pub count: u8,
    pub flags: u32,
    pub anti_flags: u32,
    pub highlight: u8,
    pub alSockets: [i32; ITEM_SOCKET_SLOT_MAX_NUM],
    pub aAttr: [TPlayerItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM],
}

impl TPacketGCItemSet {
    pub const EXPECTED_SIZE: usize = 54;
    pub const PACKET_SIZE: usize = 54;

    pub fn from_bytes(data: &[u8]) -> Result<Self, PacketError> {
        if data.len() < Self::PACKET_SIZE {
            return Err(PacketError::BufferTooSmall);
        }

        let header = u16::from_le_bytes(data[0..2].try_into().unwrap());
        let length = u16::from_le_bytes(data[2..4].try_into().unwrap());

        let window_type = data[4];
        let cell = u16::from_le_bytes(data[5..7].try_into().unwrap());
        let pos = TItemPos { window_type, cell };

        let vnum = u32::from_le_bytes(data[7..11].try_into().unwrap());
        let count = data[11];
        let flags = u32::from_le_bytes(data[12..16].try_into().unwrap());
        let anti_flags = u32::from_le_bytes(data[16..20].try_into().unwrap());
        let highlight = data[20];

        let mut alSockets = [0i32; ITEM_SOCKET_SLOT_MAX_NUM];
        for i in 0..ITEM_SOCKET_SLOT_MAX_NUM {
            let offset = 21 + i * 4;
            alSockets[i] = i32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
        }

        let mut aAttr = [TPlayerItemAttribute { bType: 0, sValue: 0 }; ITEM_ATTRIBUTE_SLOT_MAX_NUM];
        for i in 0..ITEM_ATTRIBUTE_SLOT_MAX_NUM {
            let offset = 33 + i * 3;
            let bType = data[offset];
            let sValue = i16::from_le_bytes(data[offset + 1..offset + 3].try_into().unwrap());
            aAttr[i] = TPlayerItemAttribute { bType, sValue };
        }

        Ok(Self {
            header,
            length,
            pos,
            vnum,
            count,
            flags,
            anti_flags,
            highlight,
            alSockets,
            aAttr,
        })
    }

    // Akcesory dla kompatybilnosci
    pub fn cell(&self) -> TItemPos {
        unsafe { ptr::addr_of!(self.pos).read_unaligned() }
    }

    pub fn al_sockets(&self) -> [i32; ITEM_SOCKET_SLOT_MAX_NUM] {
        unsafe { ptr::addr_of!(self.alSockets).read_unaligned() }
    }

    pub fn a_attrs(&self) -> [TPlayerItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM] {
        unsafe { ptr::addr_of!(self.aAttr).read_unaligned() }
    }
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_parse_item_set(
    data: *const u8,
    len: usize,
    out_packet: *mut *mut TPacketGCItemSet,
) -> bool {
    safe_ffi_call(false, || {
        if data.is_null() || out_packet.is_null() || len == 0 {
            return false;
        }

        let slice = unsafe { slice::from_raw_parts(data, len) };

        match TPacketGCItemSet::from_bytes(slice) {
            Ok(packet) => {
                let boxed = Box::new(packet);
                unsafe {
                    *out_packet = Box::into_raw(boxed);
                }
                true
            }
            Err(_) => false,
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_free_item_set(packet: *mut TPacketGCItemSet) {
    safe_ffi_call((), || {
        if !packet.is_null() {
            unsafe {
                let _ = Box::from_raw(packet);
            }
        }
    });
}
