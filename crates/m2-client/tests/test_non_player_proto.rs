use m2_client::non_player_proto::NonPlayerProto;

#[test]
fn test_integration_non_player_proto() {
    let instance = NonPlayerProto::new();
    assert!(instance.is_ready());
}
