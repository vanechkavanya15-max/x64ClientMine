use m2_client::race_data::RaceData;

#[test]
fn test_integration_race_data() {
    let instance = RaceData::new();
    assert!(instance.is_ready());
}
