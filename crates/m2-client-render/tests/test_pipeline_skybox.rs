use m2_client_render::pipeline_skybox::PipelineSkybox;

#[test]
fn test_integration_pipeline_skybox() {
    let instance = PipelineSkybox::new();
    assert!(instance.is_ready());
}
