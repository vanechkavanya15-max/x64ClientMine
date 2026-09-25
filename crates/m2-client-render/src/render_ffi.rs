//! Modul render_ffi dla skrzynki m2-client-render
//! Mostek C-FFI dla wywolan inicjalizacji renderera i rysowania klatki
//! Eksporty C-FFI m2_render_init, m2_render_begin_frame, m2_render_end_frame, m2_render_destroy

use std::panic::{catch_unwind, AssertUnwindSafe};

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct RenderFfi {
    pub initialized: bool,
    pub in_frame: bool,
}

impl RenderFfi {
    pub fn new() -> Self {
        Self { 
            initialized: true,
            in_frame: false,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn begin_frame(&mut self) -> bool {
        if !self.initialized || self.in_frame {
            return false;
        }
        self.in_frame = true;
        // Logika rozpoczecia klatki
        true
    }

    pub fn end_frame(&mut self) -> bool {
        if !self.initialized || !self.in_frame {
            return false;
        }
        self.in_frame = false;
        // Logika zakonczenia klatki
        true
    }
}

/// Inicjalizuje system renderowania i zwraca wskaznik do kontekstu.
#[no_mangle]
pub extern "C" fn m2_render_init() -> *mut RenderFfi {
    let result = catch_unwind(|| {
        let renderer = Box::new(RenderFfi::new());
        Box::into_raw(renderer)
    });
    
    match result {
        Ok(ptr) => ptr,
        Err(_) => std::ptr::null_mut(),
    }
}

/// Rozpoczyna nowa klatke renderowania.
#[no_mangle]
pub extern "C" fn m2_render_begin_frame(ctx: *mut RenderFfi) -> bool {
    if ctx.is_null() {
        return false;
    }
    
    let result = catch_unwind(AssertUnwindSafe(move || {
        let renderer = unsafe { &mut *ctx };
        renderer.begin_frame()
    }));
    
    match result {
        Ok(success) => success,
        Err(_) => false,
    }
}

/// Konczy klatke renderowania.
#[no_mangle]
pub extern "C" fn m2_render_end_frame(ctx: *mut RenderFfi) -> bool {
    if ctx.is_null() {
        return false;
    }
    
    let result = catch_unwind(AssertUnwindSafe(move || {
        let renderer = unsafe { &mut *ctx };
        renderer.end_frame()
    }));
    
    match result {
        Ok(success) => success,
        Err(_) => false,
    }
}

/// Niszczy kontekst renderowania i zwalnia pamiec.
#[no_mangle]
pub extern "C" fn m2_render_destroy(ctx: *mut RenderFfi) {
    if ctx.is_null() {
        return;
    }
    
    let _ = catch_unwind(AssertUnwindSafe(move || {
        let _ = unsafe { Box::from_raw(ctx) };
        // Pamiec jest zwalniana automatycznie na koncu bloku
    }));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_render_ffi() {
        let obj = RenderFfi::new();
        assert!(obj.is_ready());
    }
}
