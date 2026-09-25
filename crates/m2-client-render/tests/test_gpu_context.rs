use m2_client_render::gpu_context::GpuContext;

#[test]
fn test_integration_gpu_context() {
    let instance = GpuContext::new();
    assert!(instance.is_ready());
}
