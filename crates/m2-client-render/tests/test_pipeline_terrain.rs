use m2_client_render::pipeline_terrain::PipelineTerrain;

#[test]
fn test_integration_pipeline_terrain() {
    let instance = PipelineTerrain::new();
    assert!(instance.is_ready());
}
