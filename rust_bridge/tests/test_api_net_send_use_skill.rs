use m2_client_bridge::api::net_send_use_skill::m2_net_send_use_skill;

#[test]
fn test_m2_net_send_use_skill() {
    unsafe {
        let result = m2_net_send_use_skill(1, 100);
        assert!(result);
    }
}
