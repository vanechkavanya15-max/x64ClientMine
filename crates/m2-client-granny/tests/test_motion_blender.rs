use m2_client_granny::motion_blender::MotionBlender;

#[test]
fn test_integration_motion_blender() {
    let instance = MotionBlender::new();
    assert!(instance.is_ready());
}

#[test]
fn test_set_state() {
    let mut blender = MotionBlender::new();
    blender.set_state("run");
    assert!(blender.current_state.is_some());
    let state = blender.current_state.as_ref().unwrap();
    assert_eq!(state.name, "run");
    assert_eq!(state.weight, 1.0);
    assert!(blender.transition.is_none());
}

#[test]
fn test_blend_to_state() {
    let mut blender = MotionBlender::new();
    blender.set_state("run");
    
    // Test poprawnego przejscia
    assert!(blender.blend_to_state("attack", 0.2).is_ok());
    assert!(blender.current_state.is_none());
    assert!(blender.transition.is_some());
    
    let transition = blender.transition.as_ref().unwrap();
    assert_eq!(transition.source.name, "run");
    assert_eq!(transition.source.weight, 1.0);
    assert_eq!(transition.target.name, "attack");
    assert_eq!(transition.target.weight, 0.0);
    assert_eq!(transition.duration, 0.2);

    // Test przejscia w polowie czasu
    blender.update(0.1);
    let transition_mid = blender.transition.as_ref().unwrap();
    assert_eq!(transition_mid.source.weight, 0.5);
    assert_eq!(transition_mid.target.weight, 0.5);

    // Test zakonczenia przejscia
    blender.update(0.1); // minelo juz lacznie 0.2, czyli koniec przejscia
    assert!(blender.transition.is_none());
    assert!(blender.current_state.is_some());
    
    let final_state = blender.current_state.as_ref().unwrap();
    assert_eq!(final_state.name, "attack");
    assert_eq!(final_state.weight, 1.0);
}

#[test]
fn test_blend_to_state_invalid_duration() {
    let mut blender = MotionBlender::new();
    blender.set_state("run");
    let result = blender.blend_to_state("attack", 0.0);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Czas przejscia musi byc wiekszy niz zero");
}
