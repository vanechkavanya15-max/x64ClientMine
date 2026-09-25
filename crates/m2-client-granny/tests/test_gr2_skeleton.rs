use m2_client_granny::gr2_skeleton::Gr2Skeleton;

#[test]
fn test_integration_gr2_skeleton() {
    let instance = Gr2Skeleton::new();
    assert!(instance.is_ready());
}
