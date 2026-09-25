//! Modul ffi_guard - gwarantuje brak paniki przez granice FFI C/C++.
//!
//! Zgodnie z zasadami inzynierskimi, panika Rusta wywijajaca sie przez ramke C++
//! powoduje natychmiastowe Undefined Behavior.
//! Kazda funkcja extern "C" musi byc chroniona przez catch_unwind.

use std::panic::{catch_unwind, AssertUnwindSafe};

/// Wykonuje blok kodu z ochrona przed panika.
/// W razie paniki zwraca wartosc domyslna (default_val).
pub fn safe_ffi_call<F, R>(default_val: R, f: F) -> R
where
    F: FnOnce() -> R,
{
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(res) => res,
        Err(_) => {
            eprintln!("[m2_client_bridge] KRYTYCZNY BLAD: Zlapano panike na granicy FFI!");
            default_val
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_call_success() {
        let val = safe_ffi_call(0u32, || 42u32);
        assert_eq!(val, 42);
    }

    #[test]
    fn test_safe_call_catches_panic() {
        let val = safe_ffi_call(999u32, || {
            panic!("Testowa panika!");
        });
        assert_eq!(val, 999);
    }
}
