use m2_client_audio::wav_decoder::WavDecoder;

#[test]
fn test_integration_wav_decoder() {
    let instance = WavDecoder::new();
    assert!(instance.is_ready());
}
