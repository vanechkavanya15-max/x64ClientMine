use m2_client_terrain::water_map::WaterMap;

#[test]
fn test_integration_water_map() {
    let instance = WaterMap::new();
    assert!(instance.is_ready());
}
