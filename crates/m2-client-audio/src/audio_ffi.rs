//! Modul audio_ffi dla skrzynki m2-client-audio
//! Mostek C-FFI dla podsystemu audio
//! Eksporty C-FFI m2_audio_init, m2_audio_play_sound, m2_audio_play_bgm, m2_audio_set_volume

use std::ffi::CStr;
use std::os::raw::{c_char, c_float, c_int};
use std::panic::catch_unwind;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Globalny stan inicjalizacji podsystemu audio
static AUDIO_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Globalna glosnosc systemu (0-100)
static AUDIO_VOLUME: AtomicU32 = AtomicU32::new(100);

/// Inicjalizuje podsystem audio.
/// Zwraca 0 w przypadku sukcesu, -1 w przypadku bledu.
#[no_mangle]
pub extern "C" fn m2_audio_init() -> c_int {
    let result = catch_unwind(|| {
        AUDIO_INITIALIZED.store(true, Ordering::SeqCst);
        0
    });
    result.unwrap_or(-1)
}

/// Odtwarza dzwiek 3D w podanej lokalizacji.
/// Zwraca 0 w przypadku sukcesu, -1 w przypadku bledu (np. nieprawidlowy wskaznik).
#[no_mangle]
pub extern "C" fn m2_audio_play_sound(
    path: *const c_char,
    _x: c_float,
    _y: c_float,
    _z: c_float,
) -> c_int {
    let result = catch_unwind(|| {
        if path.is_null() {
            return -1;
        }
        if !AUDIO_INITIALIZED.load(Ordering::SeqCst) {
            return -1;
        }

        let c_str = unsafe { CStr::from_ptr(path) };
        if c_str.to_str().is_err() {
            return -1;
        }

        // Miejsce na wlasciwa logike odtwarzania dzwieku
        0
    });
    result.unwrap_or(-1)
}

/// Odtwarza muzyke w tle.
/// Zwraca 0 w przypadku sukcesu, -1 w przypadku bledu.
#[no_mangle]
pub extern "C" fn m2_audio_play_bgm(path: *const c_char, volume: c_float) -> c_int {
    let result = catch_unwind(|| {
        if path.is_null() {
            return -1;
        }
        if !AUDIO_INITIALIZED.load(Ordering::SeqCst) {
            return -1;
        }
        if !(0.0..=1.0).contains(&volume) {
            return -1;
        }

        let c_str = unsafe { CStr::from_ptr(path) };
        if c_str.to_str().is_err() {
            return -1;
        }

        // Miejsce na wlasciwa logike odtwarzania BGM
        0
    });
    result.unwrap_or(-1)
}

/// Ustawia glowna glosnosc.
/// Zwraca 0 w przypadku sukcesu, -1 w przypadku bledu.
#[no_mangle]
pub extern "C" fn m2_audio_set_volume(volume: c_float) -> c_int {
    let result = catch_unwind(|| {
        if !AUDIO_INITIALIZED.load(Ordering::SeqCst) {
            return -1;
        }
        if !(0.0..=1.0).contains(&volume) {
            return -1;
        }

        // Skalujemy od 0.0 do 1.0 na 0 do 100
        let vol_int = (volume * 100.0) as u32;
        AUDIO_VOLUME.store(vol_int, Ordering::SeqCst);
        0
    });
    result.unwrap_or(-1)
}

/// Pomocnicza funkcja do pobierania obecnej glosnosci (dla testow).
#[doc(hidden)]
pub fn get_current_volume() -> u32 {
    AUDIO_VOLUME.load(Ordering::SeqCst)
}

/// Pomocnicza funkcja do resetowania stanu (dla testow).
#[doc(hidden)]
pub fn reset_audio_state() {
    AUDIO_INITIALIZED.store(false, Ordering::SeqCst);
    AUDIO_VOLUME.store(100, Ordering::SeqCst);
}
