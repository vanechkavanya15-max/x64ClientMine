use m2_client_render::shader_manager::ShaderManager;

#[test]
fn test_integration_shader_manager() {
    let instance = ShaderManager::new();
    assert!(instance.is_ready());
}
