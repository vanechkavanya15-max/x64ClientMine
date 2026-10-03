#[path = "../src/floating_damage_animator.rs"]
mod floating_damage_animator;

use floating_damage_animator::{AnimatorConfig, DamageType, FloatingDamageAnimator};

#[test]
fn test_initialization_and_adding_damage() {
    let mut animator = FloatingDamageAnimator::new(AnimatorConfig::default());
    assert_eq!(animator.get_instances().len(), 0);

    let id = animator.add_damage(1500, DamageType::Normal, 0.0);
    assert_eq!(id, 1);

    let instances = animator.get_instances();
    assert_eq!(instances.len(), 1);
    assert_eq!(instances[0].amount, 1500);
    assert_eq!(instances[0].damage_type, DamageType::Normal);
    assert_eq!(instances[0].scale, 0.0);
    assert_eq!(instances[0].alpha, 1.0);
}

#[test]
fn test_vertical_floating_computation() {
    let config = AnimatorConfig {
        float_speed_z: 100.0,
        ..AnimatorConfig::default()
    };
    let mut animator = FloatingDamageAnimator::new(config);

    animator.add_damage(200, DamageType::Critical, 0.0);

    // Update at time 0.5s
    animator.update(0.5);

    let instances = animator.get_instances();
    assert_eq!(instances.len(), 1);
    // elapsed time = 0.5, speed = 100.0, expected offset = 50.0
    assert!((instances[0].offset_z - 50.0).abs() < 1e-5);

    // Update at time 1.0s
    animator.update(1.0);
    let instances = animator.get_instances();
    assert!((instances[0].offset_z - 100.0).abs() < 1e-5);
}

#[test]
fn test_bouncing_scale_behavior() {
    let config = AnimatorConfig {
        bounce_duration: 0.4,
        max_bounce_scale: 2.0,
        base_scale: 1.0,
        ..AnimatorConfig::default()
    };
    let mut animator = FloatingDamageAnimator::new(config);

    animator.add_damage(100, DamageType::Normal, 0.0);

    // Middle of first half of bounce (0.0 -> max_bounce_scale)
    // t = 0.1 / 0.4 = 0.25 -> t_half = 0.5. Smooth interp(0, 2, 0.5) = 1.0
    animator.update(0.1);
    let mut instances = animator.get_instances();
    assert_eq!(instances.len(), 1);
    assert!(instances[0].scale > 0.0 && instances[0].scale <= 2.0);

    // Exactly half of bounce duration (t_half = 1.0)
    animator.update(0.2);
    instances = animator.get_instances();
    assert!((instances[0].scale - 2.0).abs() < 1e-5);

    // After bounce duration
    animator.update(0.5);
    instances = animator.get_instances();
    assert!((instances[0].scale - 1.0).abs() < 1e-5);
}

#[test]
fn test_alpha_fading_logic() {
    let config = AnimatorConfig {
        lifetime: 2.0,
        fade_duration: 0.5,
        ..AnimatorConfig::default()
    };
    let mut animator = FloatingDamageAnimator::new(config);

    animator.add_damage(500, DamageType::Poison, 0.0);

    // At t = 1.0s, remaining = 1.0s, greater than fade_duration, alpha should be 1.0
    animator.update(1.0);
    let mut instances = animator.get_instances();
    assert_eq!(instances.len(), 1);
    assert!((instances[0].alpha - 1.0).abs() < 1e-5);

    // At t = 1.75s, remaining = 0.25s, less than fade_duration, alpha should be 0.25 / 0.5 = 0.5
    animator.update(1.75);
    instances = animator.get_instances();
    assert!((instances[0].alpha - 0.5).abs() < 1e-5);

    // At t = 1.9s, remaining = 0.1s, alpha should be 0.1 / 0.5 = 0.2
    animator.update(1.9);
    instances = animator.get_instances();
    assert!((instances[0].alpha - 0.2).abs() < 1e-5);
}

#[test]
fn test_expiration_and_cleanup() {
    let config = AnimatorConfig {
        lifetime: 2.0,
        ..AnimatorConfig::default()
    };
    let mut animator = FloatingDamageAnimator::new(config);

    animator.add_damage(100, DamageType::Miss, 0.0); // ID 1
    animator.add_damage(200, DamageType::Normal, 1.0); // ID 2

    assert_eq!(animator.get_instances().len(), 2);

    // At t = 2.0s, ID 1 has elapsed 2.0s (should expire), ID 2 has elapsed 1.0s (active)
    animator.update(2.0);
    let instances = animator.get_instances();
    assert_eq!(instances.len(), 1);
    assert_eq!(instances[0].amount, 200);

    // At t = 3.0s, ID 2 has elapsed 2.0s (should expire)
    animator.update(3.0);
    let instances = animator.get_instances();
    assert_eq!(instances.len(), 0);
}
