use m2_client_terrain::shadow_map::ShadowMap;

#[test]
fn test_integration_shadow_map() {
    let instance = ShadowMap::new();
    assert!(instance.is_ready());
}
