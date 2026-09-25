use m2_client_render::pipeline_ui_2d::PipelineUi2d;

#[test]
fn test_integration_pipeline_ui_2d() {
    let instance = PipelineUi2d::new();
    assert!(instance.is_ready());
}
