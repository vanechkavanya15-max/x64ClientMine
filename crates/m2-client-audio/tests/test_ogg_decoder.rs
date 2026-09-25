use m2_client_audio::ogg_decoder::OggDecoder;

#[test]
fn test_integration_ogg_decoder() {
    let instance = OggDecoder::new();
    assert!(instance.is_ready());
}
