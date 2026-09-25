use m2_client::actor_sync::ActorSync;

#[test]
fn test_integration_actor_sync() {
    let instance = ActorSync::new();
    assert!(instance.is_ready());
}
