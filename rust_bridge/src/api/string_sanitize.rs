//! Modul do czyszczenia i normalizacji tekstu (API FFI)

use crate::ffi_guard::safe_ffi_call;
use std::slice;

/// Funkcja pomocnicza do zapisu stringa do bufora C
fn write_to_c_buffer(dest: *mut u8, dest_len: usize, data: &[u8]) -> usize {
    if dest.is_null() || dest_len == 0 {
        return 0;
    }
    
    let copy_len = std::cmp::min(data.len(), dest_len - 1);
    unsafe {
        std::ptr::copy_nonoverlapping(data.as_ptr(), dest, copy_len);
        *dest.add(copy_len) = 0; // null terminator
    }
    copy_len
}

/// Usuwa z tekstu niedrukowalne znaki sterujace (z wyjatkiem spacji/tabulacji i znakow nowej linii).
/// Zwraca ilosc zapisanych bajtow (bez znaku null).
#[no_mangle]
pub extern "C" fn m2_string_sanitize(
    src: *const u8,
    src_len: usize,
    dest: *mut u8,
    dest_len: usize,
) -> usize {
    safe_ffi_call(0, || {
        if src.is_null() || dest.is_null() || src_len == 0 || dest_len == 0 {
            return 0;
        }

        let src_slice = unsafe { slice::from_raw_parts(src, src_len) };
        let mut result = Vec::with_capacity(src_len);

        for &b in src_slice {
            // Zostawiamy widoczne ASCII (32-126) oraz tabulacje (9), nowa linia (10, 13)
            // Zostawiamy tez znaki ASCII > 127
            if (b >= 32 && b != 127) || b == 9 || b == 10 || b == 13 {
                result.push(b);
            }
        }

        write_to_c_buffer(dest, dest_len, &result)
    })
}

/// Usuwa z tekstu tagi kolorow i linkow charakterystyczne dla Metin2.
/// Zwraca ilosc zapisanych bajtow.
#[no_mangle]
pub extern "C" fn m2_string_strip_colors(
    src: *const u8,
    src_len: usize,
    dest: *mut u8,
    dest_len: usize,
) -> usize {
    safe_ffi_call(0, || {
        if src.is_null() || dest.is_null() || src_len == 0 || dest_len == 0 {
            return 0;
        }

        let src_slice = unsafe { slice::from_raw_parts(src, src_len) };
        let mut result = Vec::with_capacity(src_len);
        
        let mut i = 0;
        let mut in_hyperlink = false;

        while i < src_len {
            let b = src_slice[i];
            
            if b == b'|' {
                if i + 1 < src_len {
                    let next = src_slice[i + 1];
                    match next {
                        b'c' => {
                            if i + 9 < src_len {
                                i += 10;
                            } else {
                                result.push(b'|');
                                i += 1;
                            }
                            continue;
                        }
                        b'r' => {
                            i += 2;
                            continue;
                        }
                        b'H' => {
                            in_hyperlink = true;
                            i += 2;
                            continue;
                        }
                        b'h' => {
                            in_hyperlink = false;
                            i += 2;
                            continue;
                        }
                        b'|' => {
                            if !in_hyperlink {
                                result.push(b'|');
                            }
                            i += 2;
                            continue;
                        }
                        _ => {
                            if !in_hyperlink {
                                result.push(b'|');
                            }
                            i += 1;
                            continue;
                        }
                    }
                } else {
                    if !in_hyperlink {
                        result.push(b'|');
                    }
                    i += 1;
                }
            } else {
                if !in_hyperlink {
                    result.push(b);
                }
                i += 1;
            }
        }

        write_to_c_buffer(dest, dest_len, &result)
    })
}

/// Normalizuje znaki ASCII.
/// Zwraca ilosc zapisanych bajtow.
#[no_mangle]
pub extern "C" fn m2_string_normalize(
    src: *const u8,
    src_len: usize,
    dest: *mut u8,
    dest_len: usize,
) -> usize {
    safe_ffi_call(0, || {
        if src.is_null() || dest.is_null() || src_len == 0 || dest_len == 0 {
            return 0;
        }

        let src_slice = unsafe { slice::from_raw_parts(src, src_len) };
        let mut result = Vec::with_capacity(src_len);

        for &b in src_slice {
            // Zamiana znakow extended ASCII na ich odpowiedniki bazowe (uproszczona)
            // Uzywamy czesto spotykanych znakow w ISO-8859-2 / Windows-1250 / CP-1252 
            let norm = match b {
                0xA1 | 0xB9 | 0xE1 | 0xE2 | 0xE3 | 0xE4 | 0xE5 | 0xC1 | 0xC2 | 0xC3 | 0xC4 | 0xC5 => b'a',
                0xA5 | 0xC0 => b'A',
                0xE6 | 0xE7 | 0xC6 | 0xC7 => b'c',
                0xCA | 0xEA | 0xE8 | 0xE9 | 0xEB | 0xC8 | 0xC9 | 0xCB => b'e',
                0xA3 | 0xB3 => b'l',
                0xD1 | 0xF1 => b'n',
                0xD3 | 0xF3 | 0xF2 | 0xF4 | 0xF5 | 0xF6 | 0xD2 | 0xD4 | 0xD5 | 0xD6 => b'o',
                0x8C | 0x9C | 0xA6 | 0xB6 => b's',
                0x8F | 0x9F | 0xAC | 0xBC | 0xAF | 0xBF => b'z',
                _ => b,
            };
            result.push(norm);
        }

        write_to_c_buffer(dest, dest_len, &result)
    })
}
