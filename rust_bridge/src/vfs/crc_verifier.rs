use crate::crc32::get_crc32;
use crate::ffi_guard::safe_ffi_call;
use std::slice;

/// Weryfikuje sume kontrolna CRC32 rozpakowanego bufora z wartoscia z indeksu.
/// Zwraca 1 jesli suma kontrolna sie zgadza, 0 w przeciwnym wypadku.
#[no_mangle]
pub extern "C" fn m2_vfs_verify_crc32(
    buf: *const u8,
    len: usize,
    expected_crc: u32,
) -> u8 {
    safe_ffi_call(0, || {
        if buf.is_null() {
            return 0; // pusty wskaznik oznacza blad weryfikacji
        }
        
        if len == 0 {
            // Dla pustego bufora, crc32 jest 0
            if expected_crc == 0 {
                return 1;
            } else {
                return 0;
            }
        }

        let slice = unsafe { slice::from_raw_parts(buf, len) };
        let actual_crc = get_crc32(slice);

        if actual_crc == expected_crc {
            1
        } else {
            0
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_crc32_valid() {
        let data = b"123456789";
        // get_crc32(b"123456789") == 0xCBF43926
        let res = m2_vfs_verify_crc32(data.as_ptr(), data.len(), 0xCBF43926);
        assert_eq!(res, 1, "Prawidlowe CRC musi zwrocic 1");
    }

    #[test]
    fn test_verify_crc32_invalid() {
        let data = b"123456789";
        let res = m2_vfs_verify_crc32(data.as_ptr(), data.len(), 0x00000000);
        assert_eq!(res, 0, "Bledne CRC musi zwrocic 0");
    }

    #[test]
    fn test_verify_crc32_null_ptr() {
        let res = m2_vfs_verify_crc32(std::ptr::null(), 10, 0x00000000);
        assert_eq!(res, 0, "Null pointer musi zwrocic 0");
    }

    #[test]
    fn test_verify_crc32_empty_len() {
        let data = b"";
        let res = m2_vfs_verify_crc32(data.as_ptr(), 0, 0);
        assert_eq!(res, 1, "Pusty bufor powinien miec CRC = 0");

        let res2 = m2_vfs_verify_crc32(data.as_ptr(), 0, 1234);
        assert_eq!(res2, 0, "Pusty bufor nie moze zgadzac sie z niezerowym CRC");
    }
}
