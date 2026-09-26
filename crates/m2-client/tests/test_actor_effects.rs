use m2_client::actor_effects::*;

#[test]
fn test_actor_effects_new() {
    let effects = ActorEffects::new();
    assert!(!effects.has_effect(EFFECT_POISON));
}

#[test]
fn test_actor_effects_add_and_has() {
    let mut effects = ActorEffects::new();
    effects.add_effect(EFFECT_POISON, 5.0);
    
    assert!(effects.has_effect(EFFECT_POISON));
    assert_eq!(effects.get_remaining_time(EFFECT_POISON), Some(5.0));
}

#[test]
fn test_actor_effects_remove() {
    let mut effects = ActorEffects::new();
    effects.add_effect(EFFECT_FIRE, 10.0);
    assert!(effects.has_effect(EFFECT_FIRE));
    
    effects.remove_effect(EFFECT_FIRE);
    assert!(!effects.has_effect(EFFECT_FIRE));
    assert_eq!(effects.get_remaining_time(EFFECT_FIRE), None);
}

#[test]
fn test_actor_effects_update_expiration() {
    let mut effects = ActorEffects::new();
    effects.add_effect(EFFECT_STUN, 3.0);
    
    effects.update(2.0);
    assert!(effects.has_effect(EFFECT_STUN));
    assert_eq!(effects.get_remaining_time(EFFECT_STUN), Some(1.0));
    
    effects.update(1.0);
    assert!(!effects.has_effect(EFFECT_STUN));
    assert_eq!(effects.get_remaining_time(EFFECT_STUN), None);
}

#[test]
fn test_actor_effects_multiple_effects() {
    let mut effects = ActorEffects::new();
    effects.add_effect(EFFECT_POISON, 5.0);
    effects.add_effect(EFFECT_STUN, 2.0);
    effects.add_effect(EFFECT_FIRE, 10.0);
    
    assert!(effects.has_effect(EFFECT_POISON));
    assert!(effects.has_effect(EFFECT_STUN));
    assert!(effects.has_effect(EFFECT_FIRE));
    
    effects.update(3.0);
    
    // Poison should have 2.0 left
    assert!(effects.has_effect(EFFECT_POISON));
    assert_eq!(effects.get_remaining_time(EFFECT_POISON), Some(2.0));
    
    // Stun should be expired
    assert!(!effects.has_effect(EFFECT_STUN));
    
    // Fire should have 7.0 left
    assert!(effects.has_effect(EFFECT_FIRE));
    assert_eq!(effects.get_remaining_time(EFFECT_FIRE), Some(7.0));
}

#[test]
fn test_actor_effects_overwrite_with_longer_duration() {
    let mut effects = ActorEffects::new();
    effects.add_effect(EFFECT_LEVEL_UP, 5.0);
    assert_eq!(effects.get_remaining_time(EFFECT_LEVEL_UP), Some(5.0));
    
    // Add same effect with longer duration, it should overwrite
    effects.add_effect(EFFECT_LEVEL_UP, 10.0);
    assert_eq!(effects.get_remaining_time(EFFECT_LEVEL_UP), Some(10.0));
    
    // Add same effect with shorter duration, it should NOT overwrite
    effects.add_effect(EFFECT_LEVEL_UP, 3.0);
    assert_eq!(effects.get_remaining_time(EFFECT_LEVEL_UP), Some(10.0));
}

#[test]
fn test_actor_effects_negative_add_duration() {
    let mut effects = ActorEffects::new();
    effects.add_effect(EFFECT_REFINED_PLUS7, -5.0);
    
    // Negative duration should not add the effect
    assert!(!effects.has_effect(EFFECT_REFINED_PLUS7));
}

#[test]
fn test_actor_effects_zero_add_duration() {
    let mut effects = ActorEffects::new();
    effects.add_effect(EFFECT_REFINED_PLUS8, 0.0);
    
    // Zero duration should not add the effect
    assert!(!effects.has_effect(EFFECT_REFINED_PLUS8));
}

#[test]
fn test_actor_effects_negative_update_delta() {
    let mut effects = ActorEffects::new();
    effects.add_effect(EFFECT_REFINED_PLUS9, 5.0);
    
    // Update with negative delta should be ignored
    effects.update(-2.0);
    assert_eq!(effects.get_remaining_time(EFFECT_REFINED_PLUS9), Some(5.0));
}

#[test]
fn test_actor_effects_clear() {
    let mut effects = ActorEffects::new();
    effects.add_effect(EFFECT_POISON, 5.0);
    effects.add_effect(EFFECT_STUN, 2.0);
    
    assert!(effects.has_effect(EFFECT_POISON));
    assert!(effects.has_effect(EFFECT_STUN));
    
    effects.clear();
    
    assert!(!effects.has_effect(EFFECT_POISON));
    assert!(!effects.has_effect(EFFECT_STUN));
}
