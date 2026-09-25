use crate::ffi_guard::safe_ffi_call;
use std::slice;

pub const EPKD_MAGIC: u32 = 0x444b5045; // 'EPKD'

#[derive(Debug, PartialEq, Eq)]
pub enum VfsError {
    BufferTooShort,
    InvalidMagic,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TEixHeader {
    pub magic: u32,       // EPKD_MAGIC
    pub version: u32,     // 2
    pub file_count: u32,  // ilosc wpisow
}

impl TEixHeader {
    pub const HEADER_SIZE: usize = 12;

    pub fn from_bytes(data: &[u8]) -> Result<Self, VfsError> {
        if data.len() < Self::HEADER_SIZE {
            return Err(VfsError::BufferTooShort);
        }

        let magic = u32::from_le_bytes(data[0..4].try_into().unwrap());
        let version = u32::from_le_bytes(data[4..8].try_into().unwrap());
        let file_count = u32::from_le_bytes(data[8..12].try_into().unwrap());

        let header = TEixHeader {
            magic,
            version,
            file_count,
        };

        if !header.is_valid() {
            return Err(VfsError::InvalidMagic);
        }

        Ok(header)
    }

    pub fn is_valid(&self) -> bool {
        self.magic == EPKD_MAGIC
    }
}

#[no_mangle]
pub unsafe extern "C" fn m2_vfs_parse_eix_header(
    buf: *const u8,
    len: u32,
    out_count: *mut u32,
) -> bool {
    safe_ffi_call(false, || {
        if buf.is_null() || out_count.is_null() {
            return false;
        }

        let slice = unsafe { slice::from_raw_parts(buf, len as usize) };
        
        match TEixHeader::from_bytes(slice) {
            Ok(header) => {
                unsafe { *out_count = header.file_count; }
                true
            },
            Err(_) => false,
        }
    })
}
