use m2_client_image::dxt1_decoder::Dxt1Decoder;

#[test]
fn test_integration_dxt1_decoder() {
    let instance = Dxt1Decoder::new();
    assert!(instance.is_ready());
}
