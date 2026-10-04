use glam::Vec3;
use m2_client::game_combat_knockback_physics_engine::{KnockbackMotionState, KnockbackPhysicsEngine};

#[test]
fn test_push_direction() {
    let mut engine = KnockbackPhysicsEngine::new(Vec3::new(100.0, 100.0, 0.0));
    let attacker_pos = Vec3::new(0.0, 0.0, 0.0);
    let victim_pos = Vec3::new(10.0, 0.0, 0.0);

    let dir = engine.calculate_push_direction(attacker_pos, victim_pos);
    
    assert_eq!(dir.x, 1.0);
    assert_eq!(dir.y, 0.0);
    assert_eq!(dir.z, 0.0);
    assert_eq!(engine.direction, Vec3::new(1.0, 0.0, 0.0));
}

#[test]
fn test_knockback_trajectory_and_friction() {
    let mut engine = KnockbackPhysicsEngine::new(Vec3::new(100.0, 100.0, 0.0));
    engine.direction = Vec3::new(1.0, 0.0, 0.0);

    engine.apply_external_force(10.0, 0.0, 0.0, engine.position);

    assert!(engine.target_position.x > 100.0);
    assert_eq!(engine.target_position.y, 100.0);
    assert_eq!(engine.target_position.z, 0.0);

    assert!(engine.blending_time > 0.0);
}

#[test]
fn test_knockdown_rotation_states() {
    let mut engine = KnockbackPhysicsEngine::new(Vec3::ZERO);
    engine.direction = Vec3::new(1.0, 0.0, 0.0);

    // Front attack (facing each other) - damage flying
    // If victim faces attacker, attacker faces victim
    engine.apply_external_force(10.0, 0.0, 180.0, engine.position);
    assert_eq!(engine.motion_state, KnockbackMotionState::DamageFlying);

    // Back attack (facing same direction) - damage flying back
    engine.apply_external_force(10.0, 0.0, 0.0, engine.position);
    assert_eq!(engine.motion_state, KnockbackMotionState::DamageFlyingBack);
}

#[test]
fn test_smooth_stand_up_transition() {
    let mut engine = KnockbackPhysicsEngine::new(Vec3::ZERO);
    engine.direction = Vec3::new(1.0, 0.0, 0.0);
    engine.apply_external_force(10.0, 0.0, 180.0, engine.position);

    assert_eq!(engine.motion_state, KnockbackMotionState::DamageFlying);

    // Skip the knockback duration
    engine.update(engine.blending_time);

    // We are at target position
    assert_eq!(engine.position, engine.target_position);

    // We still have 1.0s before standing up starts
    engine.update(1.0);

    assert_eq!(engine.motion_state, KnockbackMotionState::StandUp);

    // Finish stand up duration
    engine.update(1.0);

    assert_eq!(engine.motion_state, KnockbackMotionState::Wait);
}
