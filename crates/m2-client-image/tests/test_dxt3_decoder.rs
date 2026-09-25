use m2_client_image::dxt3_decoder::Dxt3Decoder;

#[test]
fn test_integration_dxt3_decoder() {
    let instance = Dxt3Decoder::new();
    assert!(instance.is_ready());
}
