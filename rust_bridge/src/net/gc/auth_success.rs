#![allow(non_snake_case)]

// Struktura pakietu sukcesu autoryzacji (Game -> Client)
// Zgodna z E:\m2dev-client-src-main\src\UserInterface\Packet.h (TPacketGCAuthSuccess)

use crate::ffi_guard::safe_ffi_call;
use std::mem::size_of;
use std::ptr;

pub const HEADER_GC_AUTH_SUCCESS: u16 = 0x0108;

#[derive(Debug, PartialEq, Eq)]
pub enum PacketError {
    TooShort,
    InvalidHeader,
}

// Struktura pakietu sukcesu autoryzacji
// Rozmiar: header(2) + length(2) + dwLoginKey(4) + bResult(1) = 9 bajtow
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_snake_case)]
pub struct TPacketGCAuthSuccess {
    pub header: u16,
    pub length: u16,
    pub dwLoginKey: u32,
    pub bResult: u8,
}

impl TPacketGCAuthSuccess {
    pub const PACKET_SIZE: usize = size_of::<Self>();

    pub fn from_bytes(data: &[u8]) -> Result<Self, PacketError> {
        if data.len() < Self::PACKET_SIZE {
            return Err(PacketError::TooShort);
        }

        let header = u16::from_le_bytes(data[0..2].try_into().unwrap());
        if header != HEADER_GC_AUTH_SUCCESS {
            return Err(PacketError::InvalidHeader);
        }

        let length = u16::from_le_bytes(data[2..4].try_into().unwrap());
        let dwLoginKey = u32::from_le_bytes(data[4..8].try_into().unwrap());
        let bResult = data[8];

        Ok(Self {
            header,
            length,
            dwLoginKey,
            bResult,
        })
    }

    // Akcesory dla kompatybilnosci wstecznej
    pub fn dw_login_key(&self) -> u32 {
        unsafe { ptr::addr_of!(self.dwLoginKey).read_unaligned() }
    }

    pub fn b_result(&self) -> u8 {
        self.bResult
    }
}

// Eksport do C++ FFI: Parsuje pakiet i zapisuje w `out_key` otrzymany klucz logowania, jesli bResult to sukces
#[no_mangle]
pub unsafe extern "C" fn m2_net_parse_auth_success(
    data: *const u8,
    len: u32,
    out_key: *mut u32,
) -> bool {
    safe_ffi_call(false, || {
        if data.is_null() || out_key.is_null() {
            return false;
        }

        let slice = std::slice::from_raw_parts(data, len as usize);
        match TPacketGCAuthSuccess::from_bytes(slice) {
            Ok(packet) => {
                *out_key = packet.dw_login_key();
                true
            }
            Err(_) => false,
        }
    })
}
