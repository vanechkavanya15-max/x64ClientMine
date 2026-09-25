use m2_client_audio::sound_emitter::{SoundEmitter, Vec3, AttenuationConfig};

#[test]
fn test_sound_emitter_initialization() {
    let instance = SoundEmitter::new();
    assert!(instance.is_ready());
    assert_eq!(instance.get_position(), Vec3::new(0.0, 0.0, 0.0));
    assert_eq!(instance.get_base_volume(), 1.0);
    assert_eq!(
        instance.get_attenuation(),
        AttenuationConfig {
            min_distance: 1.0,
            max_distance: 100.0,
            rolloff_factor: 1.0,
        }
    );
}

#[test]
fn test_sound_emitter_set_position() {
    let mut instance = SoundEmitter::new();
    let pos = Vec3::new(10.0, 20.0, 30.0);
    instance.set_position(pos);
    assert_eq!(instance.get_position(), pos);
}

#[test]
fn test_sound_emitter_set_base_volume() {
    let mut instance = SoundEmitter::new();
    
    // Normal volume
    instance.set_base_volume(0.5);
    assert_eq!(instance.get_base_volume(), 0.5);

    // Clamped below 0
    instance.set_base_volume(-0.5);
    assert_eq!(instance.get_base_volume(), 0.0);

    // Clamped above 1
    instance.set_base_volume(1.5);
    assert_eq!(instance.get_base_volume(), 1.0);
}

#[test]
fn test_sound_emitter_set_attenuation() {
    let mut instance = SoundEmitter::new();
    let config = AttenuationConfig {
        min_distance: 5.0,
        max_distance: 50.0,
        rolloff_factor: 2.0,
    };
    instance.set_attenuation(config);
    assert_eq!(instance.get_attenuation(), config);
}

#[test]
fn test_sound_emitter_calculate_volume_close() {
    let mut instance = SoundEmitter::new();
    instance.set_base_volume(1.0);
    instance.set_position(Vec3::new(0.0, 0.0, 0.0));
    
    // Listener is within min_distance
    let listener_pos = Vec3::new(0.5, 0.0, 0.0);
    assert_eq!(instance.calculate_volume(&listener_pos), 1.0);
}

#[test]
fn test_sound_emitter_calculate_volume_far() {
    let mut instance = SoundEmitter::new();
    instance.set_base_volume(1.0);
    instance.set_position(Vec3::new(0.0, 0.0, 0.0));
    
    // Listener is beyond max_distance
    let listener_pos = Vec3::new(150.0, 0.0, 0.0);
    assert_eq!(instance.calculate_volume(&listener_pos), 0.0);
}

#[test]
fn test_sound_emitter_calculate_volume_mid() {
    let mut instance = SoundEmitter::new();
    instance.set_base_volume(1.0);
    instance.set_position(Vec3::new(0.0, 0.0, 0.0));
    instance.set_attenuation(AttenuationConfig {
        min_distance: 0.0,
        max_distance: 100.0,
        rolloff_factor: 1.0,
    });
    
    // Listener is exactly halfway
    let listener_pos = Vec3::new(50.0, 0.0, 0.0);
    assert_eq!(instance.calculate_volume(&listener_pos), 0.5);
}

#[test]
fn test_sound_emitter_calculate_volume_custom_rolloff() {
    let mut instance = SoundEmitter::new();
    instance.set_base_volume(1.0);
    instance.set_position(Vec3::new(0.0, 0.0, 0.0));
    instance.set_attenuation(AttenuationConfig {
        min_distance: 0.0,
        max_distance: 100.0,
        rolloff_factor: 2.0, // Dwa razy szybszy spadek
    });
    
    // Listener is exactly halfway
    // dist_factor = 50 / 100 = 0.5
    // attenuation_val = 0.5 * 2.0 = 1.0
    // vol = 1.0 * (1.0 - 1.0) = 0.0
    let listener_pos = Vec3::new(50.0, 0.0, 0.0);
    assert_eq!(instance.calculate_volume(&listener_pos), 0.0);
}

#[test]
fn test_vec3_distance() {
    let v1 = Vec3::new(0.0, 0.0, 0.0);
    let v2 = Vec3::new(3.0, 4.0, 0.0); // 3-4-5 triangle
    assert_eq!(v1.distance(&v2), 5.0);
    assert_eq!(v2.distance(&v1), 5.0);
}
