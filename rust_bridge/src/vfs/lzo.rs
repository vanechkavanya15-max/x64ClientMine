use lzo::decompress_into;

/// Struktura odpowiedzialna za dekompresje danych w formacie LZO
pub struct LzoDecompressor;

#[derive(Debug, PartialEq, Eq)]
pub enum VfsError {
    InvalidData,
    SizeMismatch,
}

impl LzoDecompressor {
    /// Dekompresuje dane zrodlowe, oczekujac konkretnego rozmiaru wyjsciowego.
    pub fn decompress(src: &[u8], expected_size: usize) -> Result<Vec<u8>, VfsError> {
        let mut dst = vec![0u8; expected_size];
        match decompress_into(src, &mut dst) {
            Ok(size) if size == expected_size => Ok(dst),
            Ok(_) => Err(VfsError::SizeMismatch),
            Err(_) => Err(VfsError::InvalidData),
        }
    }
}

use crate::ffi_guard::safe_ffi_call;

/// Eksportowana funkcja C-FFI do dekompresji LZO1X.
/// Zwraca true w przypadku sukcesu, w przeciwnym razie false.
#[no_mangle]
pub unsafe extern "C" fn m2_vfs_decompress_lzo(
    src: *const u8,
    src_len: u32,
    dst: *mut u8,
    dst_len: u32,
) -> bool {
    safe_ffi_call(false, || {
        if src.is_null() || dst.is_null() || src_len == 0 || dst_len == 0 {
            return false;
        }

        let src_slice = std::slice::from_raw_parts(src, src_len as usize);
        let dst_slice = std::slice::from_raw_parts_mut(dst, dst_len as usize);

        match decompress_into(src_slice, dst_slice) {
            Ok(size) => size == dst_len as usize,
            Err(_) => false,
        }
    })
}
