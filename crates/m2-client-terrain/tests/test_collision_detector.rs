use m2_client_terrain::collision_detector::CollisionDetector;

#[test]
fn test_integration_collision_detector() {
    let instance = CollisionDetector::new();
    assert!(instance.is_ready());
}
