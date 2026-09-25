use m2_client_terrain::attr_map::AttrMap;

#[test]
fn test_integration_attr_map() {
    let instance = AttrMap::new();
    assert!(instance.is_ready());
}
