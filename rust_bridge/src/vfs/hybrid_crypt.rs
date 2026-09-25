use std::collections::HashMap;
use std::slice;

/// Struktura przechowujaca informacje o bloku SDB
#[derive(Clone, Debug)]
pub struct SdbInfo {
    pub stream: Vec<u8>,
}

/// Glowna struktura dla HybridCrypt zarzadzajaca blokami SDB
pub struct HybridCrypt {
    sdb_map: HashMap<u32, SdbInfo>,
}

impl HybridCrypt {
    pub fn new() -> Self {
        Self {
            sdb_map: HashMap::new(),
        }
    }

    /// Wczytuje blok SDB ze strumienia danych i zwraca ilosc odczytanych bajtow
    pub fn read_sdb_from_stream(&mut self, stream: &[u8]) -> Option<usize> {
        let mut offset = 0;
        
        if stream.len() < offset + 4 {
            return None;
        }
        
        let hash = u32::from_ne_bytes(stream[offset..offset+4].try_into().unwrap());
        offset += 4;
        
        if stream.len() < offset + 1 {
            return None;
        }
        
        let size = stream[offset] as usize;
        offset += 1;
        
        if stream.len() < offset + size {
            return None;
        }
        
        let mut sdb_stream = vec![0u8; size];
        sdb_stream.copy_from_slice(&stream[offset..offset+size]);
        offset += size;
        
        self.sdb_map.insert(hash, SdbInfo { stream: sdb_stream });
        
        Some(offset)
    }

    /// Pobiera blok SDB na podstawie hasha nazwy pliku
    pub fn get_sdb(&self, hash: u32) -> Option<&[u8]> {
        self.sdb_map.get(&hash).map(|info| info.stream.as_slice())
    }
}

// ============================================================================
// C FFI Exports - HybridCrypt
// ============================================================================

use crate::ffi_guard::safe_ffi_call;

#[no_mangle]
pub extern "C" fn m2_hybrid_crypt_new() -> *mut HybridCrypt {
    safe_ffi_call(std::ptr::null_mut(), || {
        let crypt = Box::new(HybridCrypt::new());
        Box::into_raw(crypt)
    })
}

#[no_mangle]
pub extern "C" fn m2_hybrid_crypt_free(ptr: *mut HybridCrypt) {
    safe_ffi_call((), || {
        if !ptr.is_null() {
            unsafe {
                let _ = Box::from_raw(ptr);
            }
        }
    });
}

#[no_mangle]
pub extern "C" fn m2_hybrid_crypt_read_sdb(ptr: *mut HybridCrypt, stream: *const u8, len: usize) -> i32 {
    safe_ffi_call(-1, || {
        if ptr.is_null() || stream.is_null() || len == 0 {
            return -1;
        }
        
        let crypt = unsafe { &mut *ptr };
        let slice = unsafe { slice::from_raw_parts(stream, len) };
        
        match crypt.read_sdb_from_stream(slice) {
            Some(read_len) => read_len as i32,
            None => -1,
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_hybrid_crypt_get_sdb(
    ptr: *mut HybridCrypt, 
    hash: u32, 
    out_sdb: *mut *const u8, 
    out_size: *mut usize
) -> bool {
    safe_ffi_call(false, || {
        if ptr.is_null() || out_sdb.is_null() || out_size.is_null() {
            return false;
        }
        
        let crypt = unsafe { &*ptr };
        match crypt.get_sdb(hash) {
            Some(sdb) => unsafe {
                *out_sdb = sdb.as_ptr();
                *out_size = sdb.len();
                true
            },
            None => false,
        }
    })
}
