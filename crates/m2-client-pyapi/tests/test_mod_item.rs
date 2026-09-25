use m2_client_pyapi::mod_item::ModItem;

#[test]
fn test_integration_mod_item() {
    let instance = ModItem::new();
    assert!(instance.is_ready());
}
