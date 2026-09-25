use m2_client_audio::sound_emitter::SoundEmitter;

#[test]
fn test_integration_sound_emitter() {
    let instance = SoundEmitter::new();
    assert!(instance.is_ready());
}
