use m2_client_terrain::map_setting::MapSetting;

#[test]
fn test_integration_map_setting() {
    let instance = MapSetting::new();
    assert!(instance.is_ready());
}
