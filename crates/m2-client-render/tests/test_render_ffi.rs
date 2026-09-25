use m2_client_render::render_ffi::RenderFfi;

#[test]
fn test_integration_render_ffi() {
    let instance = RenderFfi::new();
    assert!(instance.is_ready());
}
