// Struktura pakietu tworzenia postaci CG na podstawie Packet.h: command_player_create
// Komentarze wylacznie po polsku bez polskich znakow diakrytycznych.

use crate::net::constants::cg::CHARACTER_CREATE;
use crate::ffi_guard::safe_ffi_call;
use std::ffi::CStr;
use std::os::raw::c_char;

pub const CHARACTER_NAME_MAX_LEN: usize = 64;

#[repr(C, packed)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TPacketCGCreateCharacter {
    pub header: u16,
    pub length: u16,
    pub index: u8,
    pub name: [u8; CHARACTER_NAME_MAX_LEN + 1],
    pub job: u16,
    pub shape: u8,
    pub con: u8,
    pub int_stat: u8,
    pub str_stat: u8,
    pub dex: u8,
}

// Aliases dla zachowania pelnej zgodnosci z kodem i testami
pub type TPacketCGPlayerCreate = TPacketCGCreateCharacter;
pub type CharacterCreatePacket = TPacketCGCreateCharacter;

impl TPacketCGCreateCharacter {
    pub const HEADER: u16 = CHARACTER_CREATE;
    pub const SIZE: u16 = std::mem::size_of::<Self>() as u16;

    // Tworzy nowy pakiet z podanymi atrybutami postaci.
    pub fn new(
        index: u8,
        name: &str,
        job: u16,
        shape: u8,
        con: u8,
        int_stat: u8,
        str_stat: u8,
        dex: u8,
    ) -> Self {
        let mut name_arr = [0u8; CHARACTER_NAME_MAX_LEN + 1];
        let bytes = name.as_bytes();
        let len = bytes.len().min(CHARACTER_NAME_MAX_LEN);
        name_arr[..len].copy_from_slice(&bytes[..len]);

        Self {
            header: Self::HEADER,
            length: Self::SIZE,
            index,
            name: name_arr,
            job,
            shape,
            con,
            int_stat,
            str_stat,
            dex,
        }
    }

    // Serializuje strukture do wektora bajtow (Little-Endian).
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::SIZE as usize);
        buf.extend_from_slice(&self.header.to_le_bytes());
        buf.extend_from_slice(&self.length.to_le_bytes());
        buf.push(self.index);
        buf.extend_from_slice(&self.name);
        buf.extend_from_slice(&self.job.to_le_bytes());
        buf.push(self.shape);
        buf.push(self.con);
        buf.push(self.int_stat);
        buf.push(self.str_stat);
        buf.push(self.dex);
        buf
    }
}

// Funkcja C FFI budujaca bufor pakietu tworzenia postaci.
#[no_mangle]
pub unsafe extern "C" fn m2_net_build_cg_player_create(
    index: u8,
    name_ptr: *const c_char,
    job: u16,
    shape: u8,
    con: u8,
    int_stat: u8,
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

        let packet = TPacketCGCreateCharacter::new(index, name_str, job, shape, con, int_stat, str_stat, dex);
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

// Funkcja zwalniajaca zaalokowany bufor C FFI.
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
