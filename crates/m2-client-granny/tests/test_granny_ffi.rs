use m2_client_granny::granny_ffi::GrannyFfi;

#[test]
fn test_integration_granny_ffi() {
    let instance = GrannyFfi::new();
    assert!(instance.is_ready());
}
