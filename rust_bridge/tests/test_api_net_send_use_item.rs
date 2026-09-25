use m2_client_bridge::api::net_send_use_item::m2_net_send_use_item;

#[test]
fn test_m2_net_send_use_item() {
    // Sprawdzamy czy funkcja FFI poprawnie zwraca true.
    let result = m2_net_send_use_item(1, 2);
    assert!(result, "m2_net_send_use_item powinno zwrocic true");
}
