use m2_client_granny::gr2_header::Gr2Header;

#[test]
fn test_integration_gr2_header() {
    let instance = Gr2Header::new();
    assert!(instance.is_ready());
}
