// Serializator pakietu tworzenia postaci (HEADER_CG_CHARACTER_CREATE)
// Zgodny z TPacketCGCreateCharacter w C++

use crate::ffi_guard::safe_ffi_call;
use std::ffi::CStr;
use std::os::raw::c_char;

#[repr(C, packed)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TPacketCGPlayerCreate {
    pub header: u8,
    pub index: u8,
    pub name: [u8; 25],
    pub job: u16,
    pub shape: u8,
    pub con: u8,
    pub int: u8,
    pub str: u8,
    pub dex: u8,
}

impl TPacketCGPlayerCreate {
    pub fn new(index: u8, name: &str, job: u16, shape: u8, con: u8, int: u8, str: u8, dex: u8) -> Self {
        let mut name_arr = [0u8; 25];
        let bytes = name.as_bytes();
        let len = bytes.len().min(24); // max 24 characters + 1 null terminator
        name_arr[..len].copy_from_slice(&bytes[..len]);
        
        Self {
            header: 4, // HEADER_CG_PLAYER_CREATE
            index,
            name: name_arr,
            job,
            shape,
            con,
            int,
            str,
            dex,
        }
    }

    // Serializuje strukture do wektora bajtow (Little-Endian)
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(34); // 1+1+25+2+1+1+1+1+1 = 34 bajtow
        buf.push(self.header);
        buf.push(self.index);
        buf.extend_from_slice(&self.name);
        buf.extend_from_slice(&self.job.to_le_bytes());
        buf.push(self.shape);
        buf.push(self.con);
        buf.push(self.int);
        buf.push(self.str);
        buf.push(self.dex);
        buf
    }
}

// Dla zgodnosci wstecznej z testami, ktore moga uzywac CharacterCreatePacket
pub type CharacterCreatePacket = TPacketCGPlayerCreate;

#[no_mangle]
pub unsafe extern "C" fn m2_net_build_cg_player_create(
    index: u8,
    name_ptr: *const c_char,
    job: u16,
    shape: u8,
    con: u8,
    int: u8,
    str_stat: u8,
    dex: u8,
    out_buf: *mut *mut u8,
    out_len: *mut usize,
) -> bool {
    safe_ffi_call(false, || {
        if name_ptr.is_null() || out_buf.is_null() || out_len.is_null() {
            return false;
        }

        let name_cstr = unsafe { CStr::from_ptr(name_ptr) };
        let name_str = match name_cstr.to_str() {
            Ok(s) => s,
            Err(_) => return false,
        };

        let packet = CharacterCreatePacket::new(index, name_str, job, shape, con, int, str_stat, dex);
        let bytes = packet.to_bytes();
        
        let len = bytes.len();
        let ptr = bytes.into_boxed_slice();
        let raw_ptr = Box::into_raw(ptr) as *mut u8;

        unsafe {
            *out_buf = raw_ptr;
            *out_len = len;
        }

        true
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_net_free_cg_player_create(ptr: *mut u8, len: usize) {
    safe_ffi_call((), || {
        if !ptr.is_null() && len > 0 {
            unsafe {
                let slice_ptr = std::ptr::slice_from_raw_parts_mut(ptr, len);
                let _ = Box::from_raw(slice_ptr);
            }
        }
    });
}
