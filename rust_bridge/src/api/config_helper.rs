use crate::ffi_guard::safe_ffi_call;

// Struktura reprezentujaca ustawienia klienta, przekazywana przez wskaznik z C++
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ClientConfig {
    pub width: u32,
    pub height: u32,
    pub bpp: u8,
    pub shadow_quality: u8,
}

// Funkcja eksportowana do C (FFI), weryfikujaca poprawnosc ustawien
// Zwraca 1 jesli ustawienia sa poprawne, 0 w przeciwnym razie.
#[no_mangle]
pub extern "C" fn m2_config_is_valid(config_ptr: *const ClientConfig) -> u8 {
    safe_ffi_call(0, || {
        if config_ptr.is_null() {
            return 0; // Zwroc falsz jesli wskaznik jest pusty
        }

        let config = unsafe { &*config_ptr };

        // Sprawdzenie poprawnosci rozdzielczosci
        if config.width < 800 || config.height < 600 {
            return 0;
        }
        if config.width > 7680 || config.height > 4320 {
            return 0;
        }

        // Sprawdzenie poprawnosci bpp (bity na piksel)
        if config.bpp != 16 && config.bpp != 32 {
            return 0;
        }

        // Sprawdzenie poprawnosci jakosci cieni (np. 0-5)
        if config.shadow_quality > 5 {
            return 0;
        }

        1 // Ustawienia poprawne
    })
}
