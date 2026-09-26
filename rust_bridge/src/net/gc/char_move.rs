#![allow(non_snake_case)]

// Struktura pakietu ruchu postaci (Game -> Client)
// Zgodna z E:\m2dev-client-src-main\src\UserInterface\Packet.h (TPacketGCMove)

use crate::ffi_guard::safe_ffi_call;
use std::slice;

pub const HEADER_GC_MOVE: u16 = 0x0302;

#[derive(Debug, PartialEq, Eq)]
pub enum PacketError {
    BufferTooShort,
    InvalidHeader,
}

// Struktura pakietu ruchu postaci
// Rozmiar: header(2) + length(2) + bFunc(1) + bArg(1) + bRot(1)
// + dwVID(4) + lX(4) + lY(4) + dwTime(4) + dwDuration(4) = 27 bajtow
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_snake_case)]
pub struct TPacketGCMove {
    pub header: u16,
    pub length: u16,
    pub bFunc: u8,
    pub bArg: u8,
    pub bRot: u8,
    pub dwVID: u32,
    pub lX: i32,
    pub lY: i32,
    pub dwTime: u32,
    pub dwDuration: u32,
}

pub type TPacketGCCharacterMove = TPacketGCMove;

impl TPacketGCMove {
    pub const PACKET_SIZE: usize = 27;

    pub fn from_bytes(data: &[u8]) -> Result<Self, PacketError> {
        if data.len() < Self::PACKET_SIZE {
            return Err(PacketError::BufferTooShort);
        }

        let header = u16::from_le_bytes(data[0..2].try_into().unwrap());
        let length = u16::from_le_bytes(data[2..4].try_into().unwrap());
        let bFunc = data[4];
        let bArg = data[5];
        let bRot = data[6];
        let dwVID = u32::from_le_bytes(data[7..11].try_into().unwrap());
        let lX = i32::from_le_bytes(data[11..15].try_into().unwrap());
        let lY = i32::from_le_bytes(data[15..19].try_into().unwrap());
        let dwTime = u32::from_le_bytes(data[19..23].try_into().unwrap());
        let dwDuration = u32::from_le_bytes(data[23..27].try_into().unwrap());

        Ok(Self {
            header,
            length,
            bFunc,
            bArg,
            bRot,
            dwVID,
            lX,
            lY,
            dwTime,
            dwDuration,
        })
    }

    // Akcesory pomocnicze
    pub fn b_func(&self) -> u8 {
        self.bFunc
    }

    pub fn b_arg(&self) -> u8 {
        self.bArg
    }

    pub fn b_rot(&self) -> u8 {
        self.bRot
    }

    pub fn dw_vid(&self) -> u32 {
        unsafe { std::ptr::addr_of!(self.dwVID).read_unaligned() }
    }

    pub fn l_x(&self) -> i32 {
        unsafe { std::ptr::addr_of!(self.lX).read_unaligned() }
    }

    pub fn l_y(&self) -> i32 {
        unsafe { std::ptr::addr_of!(self.lY).read_unaligned() }
    }

    pub fn dw_time(&self) -> u32 {
        unsafe { std::ptr::addr_of!(self.dwTime).read_unaligned() }
    }

    pub fn dw_duration(&self) -> u32 {
        unsafe { std::ptr::addr_of!(self.dwDuration).read_unaligned() }
    }
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_parse_char_move(
    data: *const u8,
    len: usize,
    out_ptr: *mut *mut TPacketGCMove,
) -> bool {
    safe_ffi_call(false, || {
        if data.is_null() || out_ptr.is_null() {
            return false;
        }

        let slice = unsafe { slice::from_raw_parts(data, len) };
        match TPacketGCMove::from_bytes(slice) {
            Ok(packet) => {
                let boxed = Box::new(packet);
                unsafe {
                    *out_ptr = Box::into_raw(boxed);
                }
                true
            }
            Err(_) => false,
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_free_char_move(ptr: *mut TPacketGCMove) {
    safe_ffi_call((), || {
        if !ptr.is_null() {
            unsafe {
                let _ = Box::from_raw(ptr);
            }
        }
    })
}
