use m2_client::background_mgr::BackgroundMgr;

#[test]
fn test_integration_background_mgr() {
    let instance = BackgroundMgr::new();
    assert!(instance.is_ready());
}
