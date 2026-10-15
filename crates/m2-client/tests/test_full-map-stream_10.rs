use m2_client::map_safe_zone_evaluator::is_in_safe_zone;

#[test]
fn test_full_map_stream_10_alias() {
    assert!(!is_in_safe_zone(0.0, 0.0));
}

