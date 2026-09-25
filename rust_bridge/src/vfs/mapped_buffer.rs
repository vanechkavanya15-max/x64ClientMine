//! Modul vfs::mapped_buffer
//! Implementuje zarzadzanie buforem dla rozpakowanych plikow z mozliwoscia zwalniania z poziomu C++.

use crate::ffi_guard::safe_ffi_call;

/// Struktura opakowujaca wektor bajtow z gwarancja length == capacity.
pub struct MappedBuffer {
    data: Box<[u8]>,
}

impl MappedBuffer {
    /// Tworzy nowy bufor z istniejacego wektora upewniajac sie ze pojemnosc i dlugosc sa identyczne 
    /// poprzez konwersje do boxed slice.
    pub fn new(vec: Vec<u8>) -> Self {
        Self { data: vec.into_boxed_slice() }
    }

    /// Oddaje wlasnosc bufora jako surowy wskaznik dla FFI C/C++.
    /// C++ musi go pozniej zwolnic przez m2_vfs_free_buffer.
    pub fn into_raw(self) -> (*mut u8, usize) {
        let len = self.data.len();
        let ptr = Box::into_raw(self.data) as *mut u8;
        (ptr, len)
    }

    /// Odbudowuje wektor z surowego wskaznika aby Rust mogl go zwolnic.
    /// Wykorzystuje fakt ze layout pamieci dla boxed slice bazuje tylko na dlugosci.
    ///
    /// # Safety
    /// ptr musi byc wskaznikiem uzyskanym przez into_raw(), a size musi odpowiadac rozmiarowi bufora.
    pub unsafe fn from_raw(ptr: *mut u8, size: usize) -> Self {
        let slice_ptr = std::ptr::slice_from_raw_parts_mut(ptr, size);
        Self { data: Box::from_raw(slice_ptr) }
    }
}

// ============================================================================
// C FFI Exports - Mapped Buffer
// ============================================================================

/// Funkcja umozliwiajaca C++ zwolnienie bufora pamieci przydzielonego przez Rust.
#[no_mangle]
pub extern "C" fn m2_vfs_free_buffer(ptr: *mut u8, size: usize) {
    safe_ffi_call((), || {
        if ptr.is_null() || size == 0 {
            return;
        }
        unsafe {
            // Odtwarzamy bufor co spowoduje jego zwolnienie z pamieci przy wyjsciu z bloku.
            let _ = MappedBuffer::from_raw(ptr, size);
        }
    });
}
