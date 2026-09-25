use m2_client_image::texture_processor::TextureProcessor;

#[test]
fn test_integration_texture_processor() {
    let instance = TextureProcessor::new();
    assert!(instance.is_ready());
}
