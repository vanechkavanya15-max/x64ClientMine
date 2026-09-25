use m2_client_render::pipeline_model::PipelineModel;

#[test]
fn test_integration_pipeline_model() {
    let instance = PipelineModel::new();
    assert!(instance.is_ready());
}
