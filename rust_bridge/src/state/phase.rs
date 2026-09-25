use crate::ffi_guard::safe_ffi_call;
use once_cell::sync::Lazy;
use std::sync::Mutex;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClientPhase {
    Off = 0,
    HandShake = 1,
    Login = 2,
    Select = 3,
    Loading = 4,
    Game = 5,
}

pub struct PhaseManager {
    current_phase: ClientPhase,
}

impl PhaseManager {
    pub fn new() -> Self {
        Self {
            current_phase: ClientPhase::Off,
        }
    }

    pub fn set_phase(&mut self, phase: ClientPhase) -> Result<(), &'static str> {
        // Blokada przejscia bezposrednio z Game do Login
        if self.current_phase == ClientPhase::Game && phase == ClientPhase::Login {
            return Err("Nie mozna przejsc z fazy Game do Login bez uprzedniego rozlaczenia");
        }
        
        self.current_phase = phase;
        Ok(())
    }

    pub fn get_phase(&self) -> ClientPhase {
        self.current_phase
    }

    pub fn is_in_game(&self) -> bool {
        self.current_phase == ClientPhase::Game
    }
}

pub static GLOBAL_PHASE_MANAGER: Lazy<Mutex<PhaseManager>> = Lazy::new(|| {
    Mutex::new(PhaseManager::new())
});

#[no_mangle]
pub unsafe extern "C" fn m2_phase_get() -> u8 {
    safe_ffi_call(ClientPhase::Off as u8, || {
        let manager = GLOBAL_PHASE_MANAGER.lock().unwrap();
        manager.get_phase() as u8
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_phase_manager_free(ptr: *mut PhaseManager) {
    safe_ffi_call((), || {
        if !ptr.is_null() {
            let _ = Box::from_raw(ptr);
        }
    });
}
