use crate::ffi_guard::safe_ffi_call;
use std::mem::size_of;

#[derive(Debug, PartialEq)]
pub enum PacketError {
    TooShort,
    InvalidHeader,
}

#[repr(C, packed)]
#[derive(Debug)]
pub struct TPacketGCAuthSuccess {
    pub header: u8,
    pub dw_login_key: u32,
    pub b_result: u8,
}

impl TPacketGCAuthSuccess {
    pub const HEADER_GC_AUTH_SUCCESS: u8 = 150;
    pub const PACKET_SIZE: usize = size_of::<Self>(); // 1 + 4 + 1 = 6 bajtow

    pub fn from_bytes(data: &[u8]) -> Result<Self, PacketError> {
        if data.len() < Self::PACKET_SIZE {
            return Err(PacketError::TooShort);
        }

        let header = data[0];
        if header != Self::HEADER_GC_AUTH_SUCCESS {
            return Err(PacketError::InvalidHeader);
        }

        let dw_login_key = u32::from_le_bytes([data[1], data[2], data[3], data[4]]);
        let b_result = data[5];

        Ok(Self {
            header,
            dw_login_key,
            b_result,
        })
    }
}

/// Eksport do C++ FFI: Parsuje pakiet i zapisuje w `out_key` otrzymany klucz logowania, jesli b_result to sukces.
/// Zwraca true jesli parsowanie zakonczylo sie sukcesem.
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
                *out_key = packet.dw_login_key;
                true
            }
            Err(_) => false,
        }
    })
}
