use m2_client_image::tga_decoder::TgaDecoder;

#[test]
fn test_integration_tga_decoder() {
    let instance = TgaDecoder::new();
    assert!(instance.is_ready());
}
