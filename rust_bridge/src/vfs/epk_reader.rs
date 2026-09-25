use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::os::raw::c_char;
use std::ffi::CStr;
use std::ptr;
use std::slice;

use crate::ffi_guard::safe_ffi_call;

/// Struktura reprezentujaca czytnik archiwum .epk.
/// Zapewnia dostep do pliku z uzyciem Rusta i mozliwosc odczytu blokowego.
pub struct EpkReader {
    file: File,
}

impl EpkReader {
    /// Otwiera plik .epk.
    pub fn open(path: &str) -> Result<Self, std::io::Error> {
        let file = File::open(path)?;
        Ok(Self { file })
    }

    /// Odczytuje blok danych o podanym rozmiarze z zadanego offsetu w pliku.
    /// Zwraca liczbe faktycznie odczytanych bajtow.
    pub fn read_block(&mut self, offset: u64, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        self.file.seek(SeekFrom::Start(offset))?;
        let bytes_read = self.file.read(buf)?;
        Ok(bytes_read)
    }
}

// ============================================================================
// C FFI Exports - EpkReader
// ============================================================================

/// Tworzy nowa instancje EpkReader.
/// Zwraca wskaznik do zaalokowanego obiektu lub null pointer w razie bledu.
#[no_mangle]
pub extern "C" fn m2_epk_reader_open(path: *const c_char) -> *mut EpkReader {
    safe_ffi_call(ptr::null_mut(), || {
        if path.is_null() {
            return ptr::null_mut();
        }

        let c_str = unsafe { CStr::from_ptr(path) };
        let str_slice = match c_str.to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut(),
        };

        match EpkReader::open(str_slice) {
            Ok(reader) => Box::into_raw(Box::new(reader)),
            Err(_) => ptr::null_mut(),
        }
    })
}

/// Odczytuje dane za pomoca instancji EpkReader.
/// Zwraca liczbe odczytanych bajtow.
#[no_mangle]
pub extern "C" fn m2_epk_reader_read(
    reader: *mut EpkReader,
    offset: u64,
    buf: *mut u8,
    len: usize,
) -> usize {
    safe_ffi_call(0, || {
        if reader.is_null() || buf.is_null() || len == 0 {
            return 0;
        }

        let reader_ref = unsafe { &mut *reader };
        let buf_slice = unsafe { slice::from_raw_parts_mut(buf, len) };

        match reader_ref.read_block(offset, buf_slice) {
            Ok(bytes_read) => bytes_read,
            Err(_) => 0,
        }
    })
}

/// Zamyka i zwalnia z pamieci instancje EpkReader.
#[no_mangle]
pub extern "C" fn m2_epk_reader_close(reader: *mut EpkReader) {
    safe_ffi_call((), || {
        if !reader.is_null() {
            unsafe {
                let _ = Box::from_raw(reader);
            }
        }
    });
}
