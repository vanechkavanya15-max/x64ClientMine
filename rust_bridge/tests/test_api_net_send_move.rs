use m2_client_bridge::api::net_send_move::m2_net_send_move;

// Test poprawnego wykonania funkcji FFI.
#[test]
fn test_m2_net_send_move_success() {
    let result = m2_net_send_move(10.0, 20.0, 90.0, 1);
    assert!(result, "Funkcja powinna zwrocic prawde przy pomyslnym wykonaniu");
}
