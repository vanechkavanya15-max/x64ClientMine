use std::os::raw::c_char;
use crate::ffi_guard::safe_ffi_call;

pub const ITEM_NONE: u8 = 0;
pub const ITEM_WEAPON: u8 = 1;
pub const ITEM_ARMOR: u8 = 2;
pub const ITEM_USE: u8 = 3;

/// Zwraca typ przedmiotu na podstawie podanego vnum.
#[no_mangle]
pub unsafe extern "C" fn m2_py_item_get_type(vnum: u32) -> u8 {
    safe_ffi_call(ITEM_NONE, || {
        match vnum {
            10..=299 => ITEM_WEAPON,
            11200..=12999 => ITEM_ARMOR,
            27000..=27999 => ITEM_USE,
            _ => ITEM_NONE,
        }
    })
}

/// Zwraca cene przedmiotu na podstawie podanego vnum.
#[no_mangle]
pub unsafe extern "C" fn m2_py_item_get_price(vnum: u32) -> u32 {
    safe_ffi_call(0, || {
        match vnum {
            10..=299 => 1000,
            11200..=12999 => 5000,
            27000..=27999 => 50,
            _ => 0,
        }
    })
}

/// Zwraca nazwe przedmiotu na podstawie podanego vnum poprzez przekazany bufor.
#[no_mangle]
pub unsafe extern "C" fn m2_py_item_get_name(vnum: u32, out_buf: *mut c_char, max_len: usize) -> bool {
    safe_ffi_call(false, || {
        if out_buf.is_null() || max_len == 0 {
            return false;
        }

        let name = match vnum {
            10 => "Miecz +0",
            11200 => "Mnisia Zbroja Plytowa +0",
            27000 => "Czerwona Mikstura",
            _ => "Nieznany Przedmiot",
        };

        let bytes = name.as_bytes();
        let copy_len = std::cmp::min(bytes.len(), max_len - 1);

        std::ptr::copy_nonoverlapping(bytes.as_ptr() as *const c_char, out_buf, copy_len);
        *out_buf.add(copy_len) = 0; // null terminator

        true
    })
}
