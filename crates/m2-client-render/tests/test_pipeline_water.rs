use m2_client_render::pipeline_water::PipelineWater;

#[test]
fn test_integration_pipeline_water() {
    let instance = PipelineWater::new();
    assert!(instance.is_ready());
}
