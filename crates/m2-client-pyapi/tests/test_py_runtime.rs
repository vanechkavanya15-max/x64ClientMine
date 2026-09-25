use m2_client_pyapi::py_runtime::PyRuntime;

#[test]
fn test_integration_py_runtime() {
    let instance = PyRuntime::new();
    assert!(instance.is_ready());
}
