use crate::ffi_guard::safe_ffi_call;
use std::sync::atomic::{AtomicBool, Ordering};

pub const ENGINE_LIFECYCLE: u32 = 0;

static INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Inicjalizuje silnik
#[no_mangle]
pub extern "C" fn m2_engine_init() -> bool {
    safe_ffi_call(false, || {
        INITIALIZED.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_ok()
    })
}

/// Zamyka silnik
#[no_mangle]
pub extern "C" fn m2_engine_shutdown() {
    safe_ffi_call((), || {
        INITIALIZED.store(false, Ordering::SeqCst);
    });
}

/// Zwraca status inicjalizacji
#[no_mangle]
pub extern "C" fn is_initialized() -> bool {
    safe_ffi_call(false, || {
        INITIALIZED.load(Ordering::SeqCst)
    })
}
