use m2_client_bridge::api::net_send_pickup::m2_net_send_pickup;

#[test]
fn test_m2_net_send_pickup() {
    unsafe {
        let result = m2_net_send_pickup(42);
        assert!(result);
    }
}
