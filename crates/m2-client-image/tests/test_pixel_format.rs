use m2_client_image::pixel_format::PixelFormat;

#[test]
fn test_integration_pixel_format() {
    let instance = PixelFormat::new();
    assert!(instance.is_ready());
}
