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
    // Important: must set calculate_push_direction first!
    let attacker_pos = Vec3::new(90.0, 100.0, 0.0);
    engine.calculate_push_direction(attacker_pos, engine.position);
    
    engine.apply_external_force(10.0, 0.0, 0.0, engine.position);

    assert!(engine.target_position.x > 100.0);
    assert_eq!(engine.target_position.y, 100.0);
    assert_eq!(engine.target_position.z, 0.0);

    assert!(engine.blending_time > 0.0);
}

#[test]
fn test_knockdown_rotation_states() {
    let mut engine = KnockbackPhysicsEngine::new(Vec3::ZERO);
    let attacker_pos = Vec3::new(-10.0, 0.0, 0.0);
    engine.calculate_push_direction(attacker_pos, engine.position);

    // Front attack (facing each other) - damage flying
    // If victim faces attacker, attacker faces victim
    engine.apply_external_force(10.0, 0.0, 180.0, engine.position);
    assert_eq!(engine.motion_state, KnockbackMotionState::DamageFlying);
    assert_eq!(engine.knockdown_rotation, 180.0);

    // Back attack (facing same direction) - damage flying back
    engine.apply_external_force(10.0, 0.0, 0.0, engine.position);
    assert_eq!(engine.motion_state, KnockbackMotionState::DamageFlyingBack);
    assert_eq!(engine.knockdown_rotation, 0.0);
}

#[test]
fn test_smooth_stand_up_transition() {
    let mut engine = KnockbackPhysicsEngine::new(Vec3::ZERO);
    let attacker_pos = Vec3::new(-10.0, 0.0, 0.0);
    engine.calculate_push_direction(attacker_pos, engine.position);
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

#[test]
fn test_gravity_arc_trajectory() {
    let mut engine = KnockbackPhysicsEngine::new(Vec3::ZERO);
    let attacker_pos = Vec3::new(-10.0, 0.0, 0.0);
    engine.calculate_push_direction(attacker_pos, engine.position);
    engine.apply_external_force(10.0, 0.0, 180.0, engine.position);

    assert_eq!(engine.motion_state, KnockbackMotionState::DamageFlying);
    
    // Halfway through the knockback
    engine.update(engine.blending_time / 2.0);

    // Z should be at its peak (arc_height = 150.0)
    assert!((engine.position.z - 150.0).abs() < 1.0);

    // Complete the knockback
    engine.update(engine.blending_time / 2.0);

    // Z should be back at ground level (0.0)
    assert_eq!(engine.position.z, 0.0);
}

#[test]
fn test_friction_deceleration_ticks() {
    let mut engine = KnockbackPhysicsEngine::new(Vec3::ZERO);
    
    // Manually step through the loop logic in rust
    let force = 10.0;
    let mass = 1.0;
    let friction = 0.3;
    let eps = 0.001;
    let direction = Vec3::new(1.0, 0.0, 0.0);
    
    let mut velocity = direction * (force / mass);
    let mut acceleration;
    let mut ticks = 0;
    
    // simulate tick by tick loop
    for _ in 0..100 {
        ticks += 1;
        let mut f_force = 0.0;
        if velocity.length() >= eps {
            f_force -= mass * friction;
        }
        
        acceleration = direction * (f_force / mass);
        velocity += acceleration;
        
        if velocity.x * direction.x <= 0.0 { velocity.x = 0.0; }
        
        if velocity.length() < eps {
            break;
        }
    }
    
    // Expected ticks should match the physics engine blending time calculations
    let expected_blending_time = (ticks as f32) * 0.02;

    let attacker_pos = Vec3::new(-10.0, 0.0, 0.0);
    engine.calculate_push_direction(attacker_pos, engine.position);
    engine.apply_external_force(10.0, 0.0, 180.0, engine.position);
    
    assert!((engine.blending_time - expected_blending_time).abs() < 0.001);
}
