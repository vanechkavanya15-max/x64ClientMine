use m2_client_granny::gr2_transform::Gr2Transform;

#[test]
fn test_integration_gr2_transform() {
    let instance = Gr2Transform::new();
    assert!(instance.is_ready());
}
