use m2_client_pyapi::pyapi_ffi::PyapiFfi;

#[test]
fn test_integration_pyapi_ffi() {
    let instance = PyapiFfi::new();
    assert!(instance.is_ready());
}
