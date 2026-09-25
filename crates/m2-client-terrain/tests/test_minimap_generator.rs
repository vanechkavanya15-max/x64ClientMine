use m2_client_terrain::minimap_generator::MinimapGenerator;

#[test]
fn test_integration_minimap_generator() {
    let instance = MinimapGenerator::new();
    assert!(instance.is_ready());
}
