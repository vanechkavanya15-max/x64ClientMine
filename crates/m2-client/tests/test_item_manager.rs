use m2_client::item_manager::ItemManager;

#[test]
fn test_integration_item_manager() {
    let instance = ItemManager::new();
    assert!(instance.is_ready());
}
