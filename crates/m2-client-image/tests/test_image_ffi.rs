use m2_client_image::image_ffi::ImageFfi;

#[test]
fn test_integration_image_ffi() {
    let instance = ImageFfi::new();
    assert!(instance.is_ready());
}
