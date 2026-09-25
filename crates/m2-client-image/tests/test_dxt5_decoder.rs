use m2_client_image::dxt5_decoder::Dxt5Decoder;

#[test]
fn test_integration_dxt5_decoder() {
    let instance = Dxt5Decoder::new();
    assert!(instance.is_ready());
}
