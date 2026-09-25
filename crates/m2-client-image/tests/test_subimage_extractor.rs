use m2_client_image::subimage_extractor::SubimageExtractor;

#[test]
fn test_integration_subimage_extractor() {
    let instance = SubimageExtractor::new();
    assert!(instance.is_ready());
}
