use m2_client::item_proto_loader::ItemProtoLoader;

#[test]
fn test_integration_item_proto_loader() {
    let instance = ItemProtoLoader::new();
    assert!(instance.is_ready());
}
