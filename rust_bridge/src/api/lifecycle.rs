use crate::ffi_guard::safe_ffi_call;
use std::ptr;

/// Kontekst silnika reprezentujacy przydzielona pamiec
pub struct M2EngineContext {
    pub is_initialized: bool,
}

impl M2EngineContext {
    pub fn new() -> Self {
        Self {
            is_initialized: true,
        }
    }
}

/// Inicjalizuje silnik, przydzielajac pamiec na kontekst i zwraca wskaznik
#[no_mangle]
pub extern "C" fn m2_engine_init() -> *mut M2EngineContext {
    safe_ffi_call(ptr::null_mut(), || {
        let ctx = Box::new(M2EngineContext::new());
        Box::into_raw(ctx)
    })
}

/// Zamyka silnik, zwalniajac przydzielona pamiec
#[no_mangle]
pub extern "C" fn m2_engine_shutdown(ctx: *mut M2EngineContext) {
    safe_ffi_call((), || {
        if !ctx.is_null() {
            unsafe {
                let _ = Box::from_raw(ctx);
            }
        }
    });
}
