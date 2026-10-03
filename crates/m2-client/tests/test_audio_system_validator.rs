#[path = "../src/audio_system_validator.rs"]
mod audio_system_validator;

use audio_system_validator::AudioSystemValidator;

#[test]
fn test_validate_mixer_state() {
    let mut validator = AudioSystemValidator::new();
    
    validator.set_master_volume(0.5);
    assert!(validator.validate_mixer_state());
    
    validator.set_master_volume(1.5);
    assert!(validator.validate_mixer_state());
    
    validator.set_master_volume(-0.5);
    assert!(validator.validate_mixer_state());
}

#[test]
fn test_validate_3d_audio() {
    let mut validator = AudioSystemValidator::new();
    
    let pos = [1.0, 2.0, 3.0];
    validator.set_last_pos(pos);
    assert!(validator.validate_3d_audio(pos));
    
    assert!(!validator.validate_3d_audio([0.0, 0.0, 0.0]));
}

#[test]
fn test_validate_python_bindings() {
    let mut validator = AudioSystemValidator::new();
    assert!(!validator.validate_python_bindings()); // Initially false
    
    validator.init_python_bindings();
    assert!(validator.validate_python_bindings()); // True after init
}
