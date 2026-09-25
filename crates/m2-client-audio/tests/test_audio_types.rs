use m2_client_audio::audio_types::AudioTypes;

#[test]
fn test_integration_audio_types() {
    let instance = AudioTypes::new();
    assert!(instance.is_ready());
}
