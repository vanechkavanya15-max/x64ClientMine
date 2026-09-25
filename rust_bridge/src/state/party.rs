use std::sync::RwLock;
use once_cell::sync::Lazy;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartyMember {
    pub vid: u32,
    pub name: String,
    pub hp_percent: u8,
    pub is_leader: bool,
}

#[derive(Debug, Default)]
pub struct PartyManager {
    pub members: Vec<PartyMember>,
}

impl PartyManager {
    // Tworzy nowego menedzera grupy
    pub fn new() -> Self {
        Self { members: Vec::new() }
    }

    // Dodaje czlonka do grupy, jesli jeszcze nie istnieje
    pub fn add_member(&mut self, m: PartyMember) -> bool {
        if self.get_member_by_vid(m.vid).is_some() {
            return false;
        }
        self.members.push(m);
        true
    }

    // Usuwa czlonka z grupy po VID
    pub fn remove_member(&mut self, vid: u32) -> bool {
        if let Some(pos) = self.members.iter().position(|m| m.vid == vid) {
            self.members.remove(pos);
            true
        } else {
            false
        }
    }

    // Zmienia procent punktow zdrowia czlonka grupy
    pub fn update_member_hp(&mut self, vid: u32, hp_percent: u8) -> bool {
        if let Some(member) = self.members.iter_mut().find(|m| m.vid == vid) {
            member.hp_percent = hp_percent;
            true
        } else {
            false
        }
    }

    // Pobiera czlonka grupy po indeksie (bezpieczne dla wyjsc poza zakres)
    pub fn get_member_by_index(&self, index: usize) -> Option<&PartyMember> {
        self.members.get(index)
    }

    // Pobiera czlonka grupy po VID
    pub fn get_member_by_vid(&self, vid: u32) -> Option<&PartyMember> {
        self.members.iter().find(|m| m.vid == vid)
    }
    
    // Czysci grupe
    pub fn clear(&mut self) {
        self.members.clear();
    }
}

// Globalna instancja menedzera grupy
pub static PARTY_MANAGER: Lazy<RwLock<PartyManager>> = Lazy::new(|| RwLock::new(PartyManager::new()));

use crate::ffi_guard::safe_ffi_call;
use std::os::raw::c_char;
use std::ffi::CString;

// FFI dla pobrania ilosci czlonkow grupy
#[no_mangle]
pub unsafe extern "C" fn m2_party_get_count() -> u32 {
    safe_ffi_call(0, || {
        if let Ok(manager) = PARTY_MANAGER.read() {
            manager.members.len() as u32
        } else {
            0
        }
    })
}

// FFI do zwalniania pamieci ciagow znakow (string)
#[no_mangle]
pub unsafe extern "C" fn m2_party_free_string(ptr: *mut c_char) {
    safe_ffi_call((), || {
        if !ptr.is_null() {
            let _ = unsafe { CString::from_raw(ptr) };
        }
    })
}
