use m2_client_audio::sound_buffer::SoundBuffer;

#[test]
fn test_integration_sound_buffer() {
    let instance = SoundBuffer::new();
    assert!(instance.is_ready());
}
