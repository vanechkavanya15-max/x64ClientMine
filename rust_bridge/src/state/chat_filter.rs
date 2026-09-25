use std::collections::HashSet;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::RwLock;
use once_cell::sync::Lazy;

use crate::ffi_guard::safe_ffi_call;

/// Lokalny filtr spamu i czarna lista graczy (ignorowanie wiadomosci).
pub struct ChatFilter {
    blocklist: HashSet<String>,
}

impl ChatFilter {
    /// Tworzy nowa, pusta instancje filtra.
    pub fn new() -> Self {
        Self {
            blocklist: HashSet::new(),
        }
    }

    /// Blokuje gracza o podanym nicku (ignoruje wielkosc liter).
    pub fn block_user(&mut self, name: &str) {
        self.blocklist.insert(name.to_lowercase());
    }

    /// Odblokowuje gracza o podanym nicku (ignoruje wielkosc liter).
    pub fn unblock_user(&mut self, name: &str) {
        self.blocklist.remove(&name.to_lowercase());
    }

    /// Sprawdza czy gracz o podanym nicku jest zablokowany (ignoruje wielkosc liter).
    pub fn is_blocked(&self, name: &str) -> bool {
        self.blocklist.contains(&name.to_lowercase())
    }
    
    /// Czysci liste zablokowanych graczy.
    pub fn clear(&mut self) {
        self.blocklist.clear();
    }
}

/// Globalna instancja filtra uzywana przez C FFI.
static GLOBAL_CHAT_FILTER: Lazy<RwLock<ChatFilter>> = Lazy::new(|| RwLock::new(ChatFilter::new()));

// ============================================================================
// C FFI Exports
// ============================================================================

/// Zwraca true, jezeli uzytkownik jest na liscie zablokowanych.
#[no_mangle]
pub unsafe extern "C" fn m2_chat_is_user_blocked(name: *const c_char) -> bool {
    safe_ffi_call(false, || {
        if name.is_null() {
            return false;
        }
        let c_str = unsafe { CStr::from_ptr(name) };
        if let Ok(str_slice) = c_str.to_str() {
            if let Ok(filter) = GLOBAL_CHAT_FILTER.read() {
                return filter.is_blocked(str_slice);
            }
        }
        false
    })
}

/// Dodaje uzytkownika do listy zablokowanych.
#[no_mangle]
pub unsafe extern "C" fn m2_chat_block_user(name: *const c_char) {
    safe_ffi_call((), || {
        if name.is_null() {
            return;
        }
        let c_str = unsafe { CStr::from_ptr(name) };
        if let Ok(str_slice) = c_str.to_str() {
            if let Ok(mut filter) = GLOBAL_CHAT_FILTER.write() {
                filter.block_user(str_slice);
            }
        }
    });
}

/// Usuwa uzytkownika z listy zablokowanych.
#[no_mangle]
pub unsafe extern "C" fn m2_chat_unblock_user(name: *const c_char) {
    safe_ffi_call((), || {
        if name.is_null() {
            return;
        }
        let c_str = unsafe { CStr::from_ptr(name) };
        if let Ok(str_slice) = c_str.to_str() {
            if let Ok(mut filter) = GLOBAL_CHAT_FILTER.write() {
                filter.unblock_user(str_slice);
            }
        }
    });
}

/// Zwalnia zasoby uzywane przez globalny filtr (czysci blockliste).
#[no_mangle]
pub unsafe extern "C" fn m2_chat_filter_clear() {
    safe_ffi_call((), || {
        if let Ok(mut filter) = GLOBAL_CHAT_FILTER.write() {
            filter.clear();
        }
    });
}
