use m2_client::map_outdoor::MapOutdoor;

#[test]
fn test_integration_map_outdoor() {
    let instance = MapOutdoor::new();
    assert!(instance.is_ready());
}
