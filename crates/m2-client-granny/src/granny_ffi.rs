//! Modul granny_ffi dla skrzynki m2-client-granny
//! Mostek C-FFI dla ladowania modeli Granny i ewaluacji klatek
//! Eksporty C-FFI m2_granny_load_model, m2_granny_evaluate_pose, m2_granny_free

use std::ffi::CStr;
use std::os::raw::c_char;
use std::panic::catch_unwind;
use std::ptr;

/// Glowna struktura implementacyjna reprezentujaca zaladowany model Granny
#[derive(Debug, Clone, Default)]
pub struct GrannyFfi {
    pub initialized: bool,
    pub model_path: String,
    pub current_time: f32,
}

impl GrannyFfi {
    /// Tworzy nowa instancje z podana sciezka modelu
    pub fn new(path: String) -> Self {
        Self { 
            initialized: true,
            model_path: path,
            current_time: 0.0,
        }
    }

    /// Sprawdza czy model jest gotowy
    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Ewaluuje poze dla podanego czasu
    pub fn evaluate_pose(&mut self, time: f32) -> bool {
        if !self.initialized {
            return false;
        }
        self.current_time = time;
        true
    }
}

/// Laduje model Granny ze sciezki
/// Funkcja eksportowana przez C-FFI
#[no_mangle]
pub unsafe extern "C" fn m2_granny_load_model(path: *const c_char) -> *mut GrannyFfi {
    let result = catch_unwind(|| {
        if path.is_null() {
            return ptr::null_mut();
        }

        let c_str = unsafe { CStr::from_ptr(path) };
        let str_slice = match c_str.to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut(),
        };

        let model = GrannyFfi::new(str_slice.to_string());
        Box::into_raw(Box::new(model))
    });

    match result {
        Ok(ptr) => ptr,
        Err(_) => ptr::null_mut(),
    }
}

/// Ewaluuje poze modelu dla podanego czasu
/// Funkcja eksportowana przez C-FFI
#[no_mangle]
pub unsafe extern "C" fn m2_granny_evaluate_pose(model: *mut GrannyFfi, time: f32) -> bool {
    let result = catch_unwind(|| {
        if model.is_null() {
            return false;
        }

        let model_ref = unsafe { &mut *model };
        model_ref.evaluate_pose(time)
    });

    match result {
        Ok(res) => res,
        Err(_) => false,
    }
}

/// Zwalnia pamiec modelu zablokowana wczesniej przez Box::into_raw
/// Funkcja eksportowana przez C-FFI
#[no_mangle]
pub unsafe extern "C" fn m2_granny_free(model: *mut GrannyFfi) {
    let _ = catch_unwind(|| {
        if !model.is_null() {
            unsafe {
                let _ = Box::from_raw(model);
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_granny_ffi() {
        let obj = GrannyFfi::new("test".to_string());
        assert!(obj.is_ready());
    }
}
