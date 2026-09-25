use m2_client_granny::gr2_mesh::Gr2Mesh;

#[test]
fn test_integration_gr2_mesh() {
    let instance = Gr2Mesh::new();
    assert!(instance.is_ready());
}
