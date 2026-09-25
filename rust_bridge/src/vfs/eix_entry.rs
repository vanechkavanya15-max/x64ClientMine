//! Modul reprezentujacy wpis w pliku indeksu (.eix) VFS EterPack.

use crate::ffi_guard::safe_ffi_call;
use std::slice;
use std::ptr;

pub const EIX_ENTRY: u32 = 0;

#[derive(Debug, PartialEq, Eq)]
pub enum VfsError {
    BufferTooSmall,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Default)]
pub struct TEixEntry {
    pub index: u32,
    pub filename_crc: u32,
    pub real_data_size: u32,
    pub compressed_data_size: u32,
    pub data_crc: u32,
    pub data_offset: u32,
}

impl TEixEntry {
    pub fn from_bytes(data: &[u8]) -> Result<Self, VfsError> {
        let size = std::mem::size_of::<Self>();
        if data.len() < size {
            return Err(VfsError::BufferTooSmall);
        }
        
        let index = u32::from_le_bytes(data[0..4].try_into().unwrap());
        let filename_crc = u32::from_le_bytes(data[4..8].try_into().unwrap());
        let real_data_size = u32::from_le_bytes(data[8..12].try_into().unwrap());
        let compressed_data_size = u32::from_le_bytes(data[12..16].try_into().unwrap());
        let data_crc = u32::from_le_bytes(data[16..20].try_into().unwrap());
        let data_offset = u32::from_le_bytes(data[20..24].try_into().unwrap());
        
        Ok(Self {
            index,
            filename_crc,
            real_data_size,
            compressed_data_size,
            data_crc,
            data_offset,
        })
    }
}

// C-FFI
#[no_mangle]
pub unsafe extern "C" fn m2_vfs_parse_entry(
    data: *const u8,
    len: usize,
    out_entry: *mut TEixEntry,
) -> bool {
    safe_ffi_call(false, || {
        if data.is_null() || out_entry.is_null() {
            return false;
        }
        let slice = unsafe { slice::from_raw_parts(data, len) };
        match TEixEntry::from_bytes(slice) {
            Ok(entry) => {
                unsafe { *out_entry = entry; }
                true
            }
            Err(_) => false,
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_vfs_alloc_eix_entry() -> *mut TEixEntry {
    safe_ffi_call(ptr::null_mut(), || {
        let entry = Box::new(TEixEntry::default());
        Box::into_raw(entry)
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_vfs_free_eix_entry(entry: *mut TEixEntry) {
    safe_ffi_call((), || {
        if !entry.is_null() {
            unsafe {
                let _ = Box::from_raw(entry);
            }
        }
    })
}
