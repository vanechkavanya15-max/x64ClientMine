use m2_client::effect_manager::{EffectManager, Vector3};
 
 #[test]
 fn test_integration_effect_manager() {
     let instance = EffectManager::new();
     assert!(instance.is_ready());
 }

#[test]
fn test_effect_manager_register_and_create() {
    let mut manager = EffectManager::new();
    
    // Register effect
    let is_registered = manager.register_effect("d:/ymir work/effect/test_effect.mse", false, true);
    assert!(is_registered);

    // Create effect by name
    let pos = Vector3 { x: 10.0, y: 20.0, z: 30.0 };
    let rot = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
    let instance_id = manager.create_effect_by_name("d:/ymir work/effect/test_effect.mse", pos, rot);
    
    assert!(instance_id.is_some());
    let id = instance_id.unwrap();
    assert!(manager.effect_instance_map.contains_key(&id));

    // Select and verify properties
    assert!(manager.select_effect_instance(id));
    
    let instance = manager.effect_instance_map.get(&id).unwrap();
    assert_eq!(instance.position.x, 10.0);
    assert_eq!(instance.position.y, 20.0);
    assert_eq!(instance.position.z, 30.0);
}

#[test]
fn test_effect_manager_lifecycle() {
    let mut manager = EffectManager::new();
    manager.register_effect("test_effect.mse", false, false);
    
    let instance_id = manager.create_effect_by_name("test_effect.mse", Vector3::default(), Vector3::default()).unwrap();
    
    // Initial state
    let instance = manager.effect_instance_map.get(&instance_id).unwrap();
    assert!(instance.is_alive);
    assert_eq!(instance.frame, 0);

    // Update 100 times, it should still be alive
    for _ in 0..100 {
        manager.update();
    }

    assert!(manager.effect_instance_map.contains_key(&instance_id));
    
    // One more update kills it based on mock logic
    manager.update();
    assert!(!manager.effect_instance_map.contains_key(&instance_id));
}

#[test]
fn test_effect_manager_visibility_and_transforms() {
    let mut manager = EffectManager::new();
    manager.register_effect("test_effect.mse", false, false);
    
    let instance_id = manager.create_effect_by_name("test_effect.mse", Vector3::default(), Vector3::default()).unwrap();
    manager.select_effect_instance(instance_id);

    manager.hide_effect();
    assert!(!manager.effect_instance_map.get(&instance_id).unwrap().is_visible);
    
    manager.show_effect();
    assert!(manager.effect_instance_map.get(&instance_id).unwrap().is_visible);

    manager.set_effect_instance_position(Vector3 { x: 5.0, y: 5.0, z: 5.0 });
    assert_eq!(manager.effect_instance_map.get(&instance_id).unwrap().position.x, 5.0);

    manager.set_effect_instance_rotation(Vector3 { x: 90.0, y: 0.0, z: 0.0 });
    assert_eq!(manager.effect_instance_map.get(&instance_id).unwrap().rotation.x, 90.0);
}

#[test]
fn test_effect_manager_deactive() {
    let mut manager = EffectManager::new();
    manager.register_effect("test_effect.mse", false, false);
    
    let instance_id = manager.create_effect_by_name("test_effect.mse", Vector3::default(), Vector3::default()).unwrap();
    
    manager.deactive_effect_instance(instance_id);
    assert!(!manager.effect_instance_map.get(&instance_id).unwrap().is_active);
    
    manager.update();
    // Frame should not increase if deactivated
    assert_eq!(manager.effect_instance_map.get(&instance_id).unwrap().frame, 0);
}

#[test]
fn test_effect_manager_destroy_and_clear() {
    let mut manager = EffectManager::new();
    manager.register_effect("test_effect.mse", false, false);
    
    let instance_id = manager.create_effect_by_name("test_effect.mse", Vector3::default(), Vector3::default()).unwrap();
    
    let destroyed = manager.destroy_effect_instance(instance_id);
    assert!(destroyed);
    assert!(!manager.effect_instance_map.contains_key(&instance_id));

    // Test delete all
    let instance_id2 = manager.create_effect_by_name("test_effect.mse", Vector3::default(), Vector3::default()).unwrap();
    manager.delete_all_instances();
    assert!(!manager.effect_instance_map.contains_key(&instance_id2));
    assert!(manager.selected_instance.is_none());
}
