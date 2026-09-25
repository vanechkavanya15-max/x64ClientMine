use m2_client_bridge::api::lifecycle::{is_initialized, m2_engine_init, m2_engine_shutdown};

#[test]
fn test_m2_engine_init_and_shutdown() {
    // Upewnij sie, ze na poczatku nie jest zainicjalizowane (choc inne testy mogly zmienic stan)
    // Zresetuj stan przed testem.
    m2_engine_shutdown();

    assert_eq!(is_initialized(), false);

    // Inicjalizacja
    let init_success = m2_engine_init();
    assert_eq!(init_success, true);
    assert_eq!(is_initialized(), true);

    // Podwojna inicjalizacja (powinna zwrocic false)
    let double_init = m2_engine_init();
    assert_eq!(double_init, false);
    assert_eq!(is_initialized(), true);

    // Zamkniecie
    m2_engine_shutdown();
    assert_eq!(is_initialized(), false);

    // Zamkniecie kiedy juz zamkniete
    m2_engine_shutdown();
    assert_eq!(is_initialized(), false);
}
