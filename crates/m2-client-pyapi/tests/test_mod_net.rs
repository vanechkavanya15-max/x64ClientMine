use m2_client_pyapi::mod_net::ModNet;

#[test]
fn test_integration_mod_net() {
    let instance = ModNet::new();
    assert!(instance.is_ready());
}

#[test]
fn test_integration_mod_net_methods() {
    let mut instance = ModNet::new();
    assert!(instance.is_ready());

    assert_eq!(instance.get_phase_name(), "Game");
    instance.set_phase_name("Select");
    assert_eq!(instance.get_phase_name(), "Select");

    assert!(instance.send_chat_packet("test chat packet"));
    assert_eq!(instance.last_chat, "test chat packet");

    assert!(!instance.send_attack_packet(0));
    assert!(instance.send_attack_packet(999));
    assert_eq!(instance.last_attack_vid, 999);

    assert!(instance.send_use_item_packet(777));
    assert_eq!(instance.last_use_item_pos, 777);
}
