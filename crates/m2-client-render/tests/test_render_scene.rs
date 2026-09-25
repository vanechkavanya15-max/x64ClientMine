use m2_client_render::render_scene::RenderScene;

#[test]
fn test_integration_render_scene() {
    let instance = RenderScene::new();
    assert!(instance.is_ready());
}
