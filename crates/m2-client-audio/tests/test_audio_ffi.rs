use m2_client_audio::audio_ffi::AudioFfi;

#[test]
fn test_integration_audio_ffi() {
    let instance = AudioFfi::new();
    assert!(instance.is_ready());
}
