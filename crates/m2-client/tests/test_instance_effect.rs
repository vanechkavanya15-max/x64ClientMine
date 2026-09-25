use m2_client::instance_effect::*;

fn setup_instance() -> InstanceEffect {
    let mut instance = InstanceEffect::new();
    
    // Zarejestrujmy efekty uzywane w testach
    instance.register_effect(EFFECT_AFFECT + AFFECT_POISON, None, 100);
    instance.register_effect(EFFECT_AFFECT + AFFECT_STUN, None, 101);
    instance.register_effect(EFFECT_LEVELUP, None, 102);
    instance.register_effect(EFFECT_TARGET, None, 103);
    instance.register_effect(EFFECT_SWORD_REFINED9, Some("PART_WEAPON"), 104);
    instance.register_effect(EFFECT_BODYARMOR_REFINED7, None, 105);
    
    // Geomgyeong
    instance.register_effect(EFFECT_WEAPON + 1, Some("PART_WEAPON"), 106);
    
    instance
}

#[test]
fn test_integration_instance_effect() {
    let instance = setup_instance();
    assert!(instance.is_ready());
}

#[test]
fn test_set_affect_poison() {
    let mut instance = setup_instance();
    let effect_key = EFFECT_AFFECT + AFFECT_POISON;
    
    // Ustaw zatrucie
    instance.set_affect(AFFECT_POISON, true);
    
    // Sprawdz czy instancja efektu powstala i jest zapisana
    assert!(instance.active_effects.contains_key(&effect_key));
    let handle = *instance.active_effects.get(&effect_key).unwrap();
    assert!(handle > 0);
    
    // Zdejmij zatrucie
    instance.set_affect(AFFECT_POISON, false);
    
    // Sprawdz czy efekt zostal odpiety
    assert!(!instance.active_effects.contains_key(&effect_key));
}

#[test]
fn test_set_affect_geomgyeong() {
    let mut instance = setup_instance();
    
    // Ustaw Geomgyeong
    instance.set_affect(AFFECT_GEOMGYEONG, true);
    
    // Specyficzna obsluga broni
    let geom_handle = instance.active_effects.get(&(EFFECT_WEAPON + 1)).copied();
    assert!(geom_handle.is_some());
    
    // Zdejmij Geomgyeong
    instance.set_affect(AFFECT_GEOMGYEONG, false);
    
    let geom_handle_after = instance.active_effects.get(&(EFFECT_WEAPON + 1));
    assert!(geom_handle_after.is_none());
}

#[test]
fn test_set_refined_effect() {
    let mut instance = setup_instance();
    
    // Ustaw Miecz +9
    instance.set_refined_effect(EFFECT_SWORD_REFINED9);
    assert!(instance.active_effects.contains_key(&EFFECT_SWORD_REFINED9));
    
    // Zamien na zbroje +7, powinno usunac miecz +9 i wstawic zbroje
    instance.set_refined_effect(EFFECT_BODYARMOR_REFINED7);
    assert!(!instance.active_effects.contains_key(&EFFECT_SWORD_REFINED9));
    assert!(instance.active_effects.contains_key(&EFFECT_BODYARMOR_REFINED7));
    
    // Zdejmij swiecenie calkowicie
    instance.set_refined_effect(EFFECT_REFINED_NONE);
    assert!(!instance.active_effects.contains_key(&EFFECT_BODYARMOR_REFINED7));
}

#[test]
fn test_trigger_level_up() {
    let mut instance = setup_instance();
    
    assert!(!instance.active_effects.contains_key(&EFFECT_LEVELUP));
    
    instance.trigger_level_up();
    assert!(instance.active_effects.contains_key(&EFFECT_LEVELUP));
}

#[test]
fn test_set_target() {
    let mut instance = setup_instance();
    
    assert!(!instance.active_effects.contains_key(&EFFECT_TARGET));
    
    instance.set_target(true);
    assert!(instance.active_effects.contains_key(&EFFECT_TARGET));
    
    instance.set_target(false);
    assert!(!instance.active_effects.contains_key(&EFFECT_TARGET));
}

#[test]
fn test_detach_by_handle() {
    let mut instance = setup_instance();
    let handle1 = instance.attach_effect(EFFECT_TARGET);
    let handle2 = instance.attach_effect(EFFECT_LEVELUP);
    
    assert!(instance.active_effects.contains_key(&EFFECT_TARGET));
    assert!(instance.active_effects.contains_key(&EFFECT_LEVELUP));
    
    instance.detach_effect(handle1);
    
    assert!(!instance.active_effects.contains_key(&EFFECT_TARGET)); // Target powinien zniknac
    assert!(instance.active_effects.contains_key(&EFFECT_LEVELUP)); // Levelup powinien zostac
}
