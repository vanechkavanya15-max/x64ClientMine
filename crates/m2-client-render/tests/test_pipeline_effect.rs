use m2_client_render::pipeline_effect::PipelineEffect;

#[test]
fn test_integration_pipeline_effect() {
    let instance = PipelineEffect::new();
    assert!(instance.is_ready());
}
