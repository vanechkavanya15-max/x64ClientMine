use m2_client::network_actor_mgr::NetworkActorMgr;

#[test]
fn test_integration_network_actor_mgr() {
    let instance = NetworkActorMgr::new();
    assert!(instance.is_ready());
}
