use m2_client_render::pipeline_shadow::PipelineShadow;

#[test]
fn test_integration_pipeline_shadow() {
    let instance = PipelineShadow::new();
    assert!(instance.is_ready());
}
