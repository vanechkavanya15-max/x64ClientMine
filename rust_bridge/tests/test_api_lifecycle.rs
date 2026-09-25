use m2_client_bridge::api::lifecycle::{m2_engine_init, m2_engine_shutdown};
use std::ptr;

#[test]
fn test_m2_engine_init_and_shutdown() {
    // Inicjalizacja, powinna zwrocic poprawny wskaznik
    let ctx_ptr = m2_engine_init();
    assert!(!ctx_ptr.is_null());

    // Sprawdzenie danych pod wskaznikiem, aby sie upewnic ze to nie smieci
    unsafe {
        assert_eq!((*ctx_ptr).is_initialized, true);
    }

    // Zamkniecie, zwalnia pamiec
    m2_engine_shutdown(ctx_ptr);
}

#[test]
fn test_m2_engine_shutdown_null() {
    // Proba zamkniecia z null pointerem nie powinna wywolac bledu/paniki
    m2_engine_shutdown(ptr::null_mut());
}
