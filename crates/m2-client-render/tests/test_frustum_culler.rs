use m2_client_render::frustum_culler::FrustumCuller;

#[test]
fn test_integration_frustum_culler() {
    let instance = FrustumCuller::new();
    assert!(instance.is_ready());
}
