use m2_client_render::light_environment::LightEnvironment;

#[test]
fn test_integration_light_environment() {
    let instance = LightEnvironment::new();
    assert!(instance.is_ready());
}
