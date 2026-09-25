use m2_client::net_stream_core::NetStreamCore;

#[test]
fn test_integration_net_stream_core() {
    let instance = NetStreamCore::new();
    assert!(instance.is_ready());
}
