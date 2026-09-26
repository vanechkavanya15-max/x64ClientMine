use crate::ffi_guard::safe_ffi_call;
use std::sync::atomic::{AtomicBool, Ordering};

pub const ENGINE_LIFECYCLE: u32 = 0;

static INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Inicjalizuje silnik
#[no_mangle]
pub extern "C" fn m2_engine_init() -> bool {
    safe_ffi_call(false, || {
        let ok = INITIALIZED.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_ok();
        if ok {
            use std::io::Write;
            if let Ok(mut f) = std::fs::File::create("rust_bridge_live.log") {
                let _ = writeln!(f, "============================================================");
                let _ = writeln!(f, "[RUST BRIDGE x64 NATIVE] Engine Initialized Successfully!");
                let _ = writeln!(f, "[RUST BRIDGE x64 NATIVE] Running inside Metin2_x64 process");
                let _ = writeln!(f, "[RUST BRIDGE x64 NATIVE] Target: x86_64-pc-windows-msvc");
                let _ = writeln!(f, "============================================================");
            }
        }
        ok
    })
}

/// Zamyka silnik
#[no_mangle]
pub extern "C" fn m2_engine_shutdown() {
    safe_ffi_call((), || {
        INITIALIZED.store(false, Ordering::SeqCst);
        use std::io::Write;
        if let Ok(mut f) = std::fs::OpenOptions::new().append(true).open("rust_bridge_live.log") {
            let _ = writeln!(f, "[RUST BRIDGE x64 NATIVE] Engine Shutdown called cleanly.");
        }
    });
}


/// Zwraca status inicjalizacji
#[no_mangle]
pub extern "C" fn is_initialized() -> bool {
    safe_ffi_call(false, || {
        INITIALIZED.load(Ordering::SeqCst)
    })
}
