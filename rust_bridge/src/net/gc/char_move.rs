use crate::ffi_guard::safe_ffi_call;
use std::slice;

#[derive(Debug, PartialEq)]
pub enum PacketError {
    BufferTooShort,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TPacketGCCharacterMove {
    pub header: u8,
    pub b_func: u8,
    pub b_arg: u8,
    pub b_rot: u8,
    pub dw_vid: u32,
    pub l_x: i32,
    pub l_y: i32,
    pub dw_time: u32,
    pub dw_duration: u32,
}

impl TPacketGCCharacterMove {
    pub const PACKET_SIZE: usize = 24;

    pub fn from_bytes(data: &[u8]) -> Result<Self, PacketError> {
        if data.len() < Self::PACKET_SIZE {
            return Err(PacketError::BufferTooShort);
        }

        let header = data[0];
        let b_func = data[1];
        let b_arg = data[2];
        let b_rot = data[3];
        
        let dw_vid = u32::from_le_bytes(data[4..8].try_into().unwrap());
        let l_x = i32::from_le_bytes(data[8..12].try_into().unwrap());
        let l_y = i32::from_le_bytes(data[12..16].try_into().unwrap());
        let dw_time = u32::from_le_bytes(data[16..20].try_into().unwrap());
        let dw_duration = u32::from_le_bytes(data[20..24].try_into().unwrap());

        Ok(Self {
            header,
            b_func,
            b_arg,
            b_rot,
            dw_vid,
            l_x,
            l_y,
            dw_time,
            dw_duration,
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_parse_char_move(
    data: *const u8,
    len: usize,
    out_ptr: *mut *mut TPacketGCCharacterMove,
) -> bool {
    safe_ffi_call(false, || {
        if data.is_null() || out_ptr.is_null() {
            return false;
        }

        let slice = unsafe { slice::from_raw_parts(data, len) };
        match TPacketGCCharacterMove::from_bytes(slice) {
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
pub unsafe extern "C" fn m2_net_free_char_move(ptr: *mut TPacketGCCharacterMove) {
    safe_ffi_call((), || {
        if !ptr.is_null() {
            unsafe {
                let _ = Box::from_raw(ptr);
            }
        }
    })
}
