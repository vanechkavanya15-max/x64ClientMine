#[path = "../src/stamina_system.rs"]
mod stamina_system;

use stamina_system::StaminaSystem;

#[test]
fn test_stamina_consumption_while_running() {
    let mut stamina = StaminaSystem::new();
    stamina.is_consuming_stamina = true;
    
    // Test for 2 seconds running
    let delta_time = 2.0;
    stamina.update(delta_time);
    
    let expected_stamina = 1000.0 - (delta_time * StaminaSystem::CONSUME_STAMINA_PER_SEC);
    assert_eq!(stamina.current_stamina, expected_stamina);
    assert!(stamina.is_consuming_stamina);
    assert_eq!(stamina.get_point_stamina(), expected_stamina as u32);
}

#[test]
fn test_stamina_automatic_switch_to_walking_when_empty() {
    let mut stamina = StaminaSystem::new();
    stamina.is_consuming_stamina = true;
    stamina.current_stamina = 40.0;
    
    // Running for 1 second will consume 50 stamina, which exceeds the current 40.
    let delta_time = 1.0;
    stamina.update(delta_time);
    
    // Stamina should be clamped to 0 and running state turned off (walking).
    assert_eq!(stamina.current_stamina, 0.0);
    assert!(!stamina.is_consuming_stamina);
    assert_eq!(stamina.get_point_stamina(), 0);
}

#[test]
fn test_stamina_regeneration_while_resting_or_walking() {
    let mut stamina = StaminaSystem::new();
    stamina.is_consuming_stamina = false;
    stamina.current_stamina = 500.0;
    
    // Test for 4 seconds walking/resting
    let delta_time = 4.0;
    stamina.update(delta_time);
    
    let expected_stamina = 500.0 + (delta_time * StaminaSystem::REGEN_STAMINA_PER_SEC);
    assert_eq!(stamina.current_stamina, expected_stamina);
    assert!(!stamina.is_consuming_stamina);
    assert_eq!(stamina.get_point_stamina(), expected_stamina as u32);
}

#[test]
fn test_stamina_regeneration_does_not_exceed_max() {
    let mut stamina = StaminaSystem::new();
    stamina.is_consuming_stamina = false;
    stamina.current_stamina = 980.0;
    
    // Regenerating for 2 seconds adds 50 stamina (980 + 50 = 1030), which is > max.
    let delta_time = 2.0;
    stamina.update(delta_time);
    
    // Stamina should be clamped to max_stamina
    assert_eq!(stamina.current_stamina, stamina.max_stamina);
    assert_eq!(stamina.get_point_stamina(), stamina.max_stamina as u32);
}

#[test]
fn test_start_and_stop_stamina_consume_c_api() {
    let mut stamina = StaminaSystem::new();
    
    // Server sends StartStaminaConsume(100, 800)
    stamina.start_stamina_consume(100.0, 800.0);
    assert!(stamina.is_consuming_stamina);
    assert_eq!(stamina.current_stamina, 800.0);
    assert_eq!(stamina.consume_per_sec, 100.0);

    // Update for 1.5 seconds
    stamina.update(1.5);
    // 800 - 150 = 650
    assert_eq!(stamina.current_stamina, 650.0);

    // Server sends StopStamina(650)
    stamina.stop_stamina(650.0);
    assert!(!stamina.is_consuming_stamina);
    assert_eq!(stamina.consume_per_sec, 0.0);
    assert_eq!(stamina.current_stamina, 650.0);
}

