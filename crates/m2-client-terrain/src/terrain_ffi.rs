//! Modul terrain_ffi dla skrzynki m2-client-terrain
//! Mostek C-FFI dla operacji na terenie
//! Eksporty C-FFI m2_terrain_load, m2_terrain_get_height, m2_terrain_is_blocked, m2_terrain_free

use std::ffi::CStr;
use std::os::raw::c_char;
use std::panic::{catch_unwind, AssertUnwindSafe};

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct TerrainFfi {
    pub initialized: bool,
}

impl TerrainFfi {
    pub fn new() -> Self {
        Self { initialized: true }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }
    
    pub fn get_height(&self, _x: f32, _y: f32) -> f32 {
        0.0 // Domyslna wartosc
    }
    
    pub fn is_blocked(&self, _x: f32, _y: f32) -> bool {
        false // Domyslna wartosc
    }
}

#[no_mangle]
pub extern "C" fn m2_terrain_load(path: *const c_char) -> *mut TerrainFfi {
    let result = catch_unwind(AssertUnwindSafe(|| {
        if path.is_null() {
            return std::ptr::null_mut();
        }
        
        // Zabezpieczenie przed nieprawidlowym stringiem
        let _c_str = unsafe { CStr::from_ptr(path) };
        
        let terrain = Box::new(TerrainFfi::new());
        Box::into_raw(terrain)
    }));

    result.unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn m2_terrain_get_height(terrain: *mut TerrainFfi, x: f32, y: f32) -> f32 {
    let result = catch_unwind(AssertUnwindSafe(|| {
        if terrain.is_null() {
            return 0.0;
        }
        
        let terrain_ref = unsafe { &*terrain };
        terrain_ref.get_height(x, y)
    }));

    result.unwrap_or(0.0)
}

#[no_mangle]
pub extern "C" fn m2_terrain_is_blocked(terrain: *mut TerrainFfi, x: f32, y: f32) -> bool {
    let result = catch_unwind(AssertUnwindSafe(|| {
        if terrain.is_null() {
            return true; // Bezpieczna domyslna wartosc
        }
        
        let terrain_ref = unsafe { &*terrain };
        terrain_ref.is_blocked(x, y)
    }));

    result.unwrap_or(true)
}

#[no_mangle]
pub extern "C" fn m2_terrain_free(terrain: *mut TerrainFfi) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        if !terrain.is_null() {
            unsafe {
                let _ = Box::from_raw(terrain);
            }
        }
    }));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_terrain_ffi() {
        let obj = TerrainFfi::new();
        assert!(obj.is_ready());
    }
}
