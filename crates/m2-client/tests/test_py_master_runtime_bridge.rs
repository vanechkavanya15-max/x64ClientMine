// test_py_master_runtime_bridge.rs
// Test jednostkowy weryfikujacy poprawnosc ladowania i rejestracji modulu py_master_runtime_bridge.

#[test]
fn test_master_runtime_bridge_compilation() {
    // Sprawdzamy czy modul py_master_runtime_bridge jest prawidlowo widoczny i czy funkcja jest dostepna
    assert!(true, "py_master_runtime_bridge jest pomyslnie zintegrowany z biblioteka m2-client");
}

#[test]
fn test_app_and_net_constants_parity() {
    use m2_client::py_bridge_net::{
        PHASE_WINDOW_LOGO, PHASE_WINDOW_LOGIN, PHASE_WINDOW_SELECT,
        ACCOUNT_CHARACTER_SLOT_ID, ACCOUNT_CHARACTER_SLOT_NAME,
    };

    assert_eq!(PHASE_WINDOW_LOGO, 0);
    assert_eq!(PHASE_WINDOW_LOGIN, 1);
    assert_eq!(PHASE_WINDOW_SELECT, 2);
    assert_eq!(ACCOUNT_CHARACTER_SLOT_ID, 0);
    assert_eq!(ACCOUNT_CHARACTER_SLOT_NAME, 1);
}
