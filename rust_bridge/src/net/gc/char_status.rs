#![allow(non_snake_case)]

// Modul pakietu zmiany punktow postaci (Game -> Client)
// Zgodny z E:\m2dev-client-src-main\src\UserInterface\Packet.h (TPacketGCPointChange)

use std::mem::size_of;
use std::ptr;

#[derive(Debug, PartialEq, Eq)]
pub enum PacketError {
    BufferTooShort,
    InvalidHeader,
}

pub const HEADER_GC_PLAYER_POINT_CHANGE: u16 = 0x0215;
pub const HEADER_GC_PLAYER_POINTS: u16 = 0x0214;
pub const POINT_MAX_NUM: usize = 255;

// Struktura pakietu TPacketGCPointChange
// Rozmiar: header(2) + length(2) + dwVID(4) + Type(1) + amount(4) + value(4) = 17 bajtow
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_snake_case)]
pub struct TPacketGCPointChange {
    pub header: u16,
    pub length: u16,
    pub dwVID: u32,
    pub Type: u8,
    pub amount: i32,
    pub value: i32,
}

impl TPacketGCPointChange {
    pub const PACKET_SIZE: usize = size_of::<Self>();

    pub fn from_bytes(data: &[u8]) -> Result<Self, PacketError> {
        if data.len() < Self::PACKET_SIZE {
            return Err(PacketError::BufferTooShort);
        }

        let header = u16::from_le_bytes(data[0..2].try_into().unwrap());
        if header != HEADER_GC_PLAYER_POINT_CHANGE {
            return Err(PacketError::InvalidHeader);
        }

        let length = u16::from_le_bytes(data[2..4].try_into().unwrap());
        let dwVID = u32::from_le_bytes(data[4..8].try_into().unwrap());
        let Type = data[8];
        let amount = i32::from_le_bytes(data[9..13].try_into().unwrap());
        let value = i32::from_le_bytes(data[13..17].try_into().unwrap());

        Ok(Self {
            header,
            length,
            dwVID,
            Type,
            amount,
            value,
        })
    }

    // Akcesory dla kompatybilnosci wstecznej
    pub fn b_type(&self) -> u8 {
        self.Type
    }

    pub fn get_value(&self) -> i32 {
        unsafe { ptr::addr_of!(self.value).read_unaligned() }
    }

    pub fn get_amount(&self) -> i32 {
        unsafe { ptr::addr_of!(self.amount).read_unaligned() }
    }
}

// Struktura pakietu wszystkich punktow postaci
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketGCPoints {
    pub header: u16,
    pub length: u16,
    pub points: [i32; POINT_MAX_NUM],
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_parse_point_change(
    data: *const u8,
    len: u32,
    out_type: *mut u8,
    out_val: *mut i64,
) -> bool {
    if data.is_null() || out_type.is_null() || out_val.is_null() {
        return false;
    }

    let result = std::panic::catch_unwind(|| {
        let slice = std::slice::from_raw_parts(data, len as usize);
        match TPacketGCPointChange::from_bytes(slice) {
            Ok(packet) => {
                *out_type = packet.Type;
                let val_i32 = ptr::addr_of!(packet.value).read_unaligned();
                *out_val = val_i32 as i64;
                true
            }
            Err(_) => false,
        }
    });

    result.unwrap_or(false)
}
