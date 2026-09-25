use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::ffi::CStr;
use std::os::raw::c_char;
use crate::ffi_guard::safe_ffi_call;

#[derive(Clone, Debug, PartialEq)]
struct Pack {
    name: String,
    priority: i32,
}

struct PackManager {
    packs: Vec<Pack>,
}

impl PackManager {
    fn new() -> Self {
        Self { packs: Vec::new() }
    }

    fn clear(&mut self) {
        self.packs.clear();
    }

    fn register_pack(&mut self, name: String, priority: i32) {
        self.packs.push(Pack { name, priority });
        // Sortowanie malejaco po priorytecie
        self.packs.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    fn get_count(&self) -> usize {
        self.packs.len()
    }

    fn get_pack_name_by_index(&self, index: usize) -> Option<&String> {
        self.packs.get(index).map(|p| &p.name)
    }
}

// Globalny stan (wzorzec Singletonu dla FFI)
static PACK_MANAGER: Lazy<Mutex<PackManager>> = Lazy::new(|| Mutex::new(PackManager::new()));

// ============================================================================
// C FFI Exports - VFS Pack Manager
// ============================================================================

#[no_mangle]
pub extern "C" fn m2_vfs_pack_manager_clear() {
    safe_ffi_call((), || {
        let mut manager = PACK_MANAGER.lock().unwrap();
        manager.clear();
    });
}

#[no_mangle]
pub extern "C" fn m2_vfs_pack_manager_register_pack(
    name_ptr: *const c_char,
    priority: i32,
) -> bool {
    safe_ffi_call(false, || {
        if name_ptr.is_null() {
            return false;
        }
        
        let c_str = unsafe { CStr::from_ptr(name_ptr) };
        let name = match c_str.to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return false,
        };
        
        let mut manager = PACK_MANAGER.lock().unwrap();
        manager.register_pack(name, priority);
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_vfs_pack_manager_get_count() -> usize {
    safe_ffi_call(0, || {
        let manager = PACK_MANAGER.lock().unwrap();
        manager.get_count()
    })
}

/// Zwraca nazwe pakietu o zadanym indeksie. Wynik jest wazny tylko w obrebie wywolania
/// i nie powinien byc przetrzymywany, wiec zwracamy kopie jako wyciek pamieci lub przez bufor.
/// Ale standardowo czesto sie kopiuje do bufora. Zrobmy to bezpiecznie.
#[no_mangle]
pub extern "C" fn m2_vfs_pack_manager_get_pack_name_by_index(
    index: usize,
    buffer: *mut c_char,
    buffer_len: usize,
) -> bool {
    safe_ffi_call(false, || {
        if buffer.is_null() || buffer_len == 0 {
            return false;
        }

        let manager = PACK_MANAGER.lock().unwrap();
        let name = match manager.get_pack_name_by_index(index) {
            Some(n) => n,
            None => return false,
        };

        // Kopiowanie do bufora w sposob bezpieczny dla C
        let name_bytes = name.as_bytes();
        let max_len = std::cmp::min(name_bytes.len(), buffer_len - 1); // Zostaw miejsce na '\0'

        unsafe {
            let buffer_slice = std::slice::from_raw_parts_mut(buffer as *mut u8, buffer_len);
            buffer_slice[..max_len].copy_from_slice(&name_bytes[..max_len]);
            buffer_slice[max_len] = 0; // Null terminator
        }
        
        true
    })
}
