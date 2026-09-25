use m2_client::area_loader::AreaLoader;

#[test]
fn test_integration_area_loader() {
    let instance = AreaLoader::new();
    assert!(instance.is_ready());
}
