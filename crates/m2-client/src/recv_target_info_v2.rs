use m2_client_bridge::ffi_guard::safe_ffi_call;
use std::sync::{Mutex, OnceLock};

/// Struktura pakietu dla celu, zgodna w 100% z C++ Packet.h (rozmiar: 9 bajtow)
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct TPacketGCTarget {
    pub header: u16,
    pub length: u16,
    pub dwVID: u32,
    pub bHPPercent: u8,
}

/// Stan przechowujacy informacje o aktualnym celu
#[derive(Default, Clone, Copy, Debug)]
pub struct TargetState {
    pub target_vid: u32,
    pub hp_percent: u8,
}

static TARGET_STATE: OnceLock<Mutex<TargetState>> = OnceLock::new();

/// Zwraca referencje do globalnego stanu celu
pub fn get_target_state() -> &'static Mutex<TargetState> {
    TARGET_STATE.get_or_init(|| Mutex::new(TargetState::default()))
}

/// Parsuje pakiet i aktualizuje globalny stan celu
pub fn parse_target_packet(packet: &TPacketGCTarget) {
    let mut state = get_target_state().lock().unwrap();
    
    // Bezpieczny odczyt pol ze struktury packed
    let vid = u32::from_le_bytes(packet.dwVID.to_le_bytes());
    let hp_pct = packet.bHPPercent;
    
    state.target_vid = vid;
    state.hp_percent = hp_pct;
}

/// Czysci aktualny cel w stanie
pub fn clear_target() {
    let mut state = get_target_state().lock().unwrap();
    state.target_vid = 0;
    state.hp_percent = 0;
}

/// Funkcja FFI do pobierania informacji o celu
#[no_mangle]
pub unsafe extern "C" fn m2_net_get_target_hp(out_vid: *mut u32, out_hp_pct: *mut u8) -> bool {
    safe_ffi_call(false, || {
        if out_vid.is_null() || out_hp_pct.is_null() {
            return false;
        }

        let state = get_target_state().lock().unwrap();
        if state.target_vid == 0 {
            return false;
        }

        unsafe {
            *out_vid = state.target_vid;
            *out_hp_pct = state.hp_percent;
        }

        true
    })
}
