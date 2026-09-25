use m2_client_render::render_target::RenderTarget;

#[test]
fn test_integration_render_target() {
    let instance = RenderTarget::new();
    assert!(instance.is_ready());
}
