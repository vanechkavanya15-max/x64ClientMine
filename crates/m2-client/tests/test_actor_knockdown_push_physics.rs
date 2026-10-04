use glam::Vec3;
use m2_client::actor_knockdown_push_physics::{EaseOutInterpolation, PhysicsObject};

#[test]
fn test_ease_out_interpolation() {
    let mut ease_out = EaseOutInterpolation::new();
    assert!(!ease_out.is_playing());
    
    let setup_ok = ease_out.setup(0.0, 10.0, 2.0);
    assert!(setup_ok);
    assert!(ease_out.is_playing());
    
    // speed = 2 * (10 - 0) / 2.0 = 10.0
    // accel = 2 * 10 / 4.0 - 2 * 10.0 / 2.0 = 5.0 - 10.0 = -5.0
    assert_eq!(ease_out.speed, 10.0);
    assert_eq!(ease_out.acceleration, -5.0);
    
    // interpolate 1 second
    ease_out.interpolate(1.0);
    // remaining = 1.0
    // speed = 10.0 + (-5.0 * 1.0) = 5.0
    // value = 0.0 + (5.0 * 1.0) = 5.0
    assert_eq!(ease_out.remaining_time, 1.0);
    assert_eq!(ease_out.speed, 5.0);
    assert_eq!(ease_out.get_value(), 5.0);
    assert_eq!(ease_out.get_changing_value(), 5.0); // 5.0 - 0.0
    
    // interpolate another 1 second
    ease_out.interpolate(1.0);
    // remaining = 0.0
    // speed = 5.0 + (-5.0 * 1.0) = 0.0
    // value = 5.0 + (0.0 * 1.0) = 5.0  (Wait, speed is applied before value addition? Let's check formula)
    // Actually:
    // speed = 5.0 - 5.0 = 0.0
    // value = 5.0 + (0.0 * 1.0) = 5.0. BUT mathematically, if we integrate:
    // This is a naive discrete Euler step.
    
    // Check it stops playing
    assert!(!ease_out.is_playing());
    assert_eq!(ease_out.get_value(), 0.0); // reset to 0.0 when not playing
}

#[test]
fn test_physics_accumulation() {
    let mut physics = PhysicsObject::new();
    physics.mass = 1.0;
    physics.friction = 0.3;
    physics.direction = Vec3::new(1.0, 0.0, 0.0);
    
    // Simulate setting initial velocity (which accumulate will decay)
    physics.velocity = Vec3::new(10.0, 0.0, 0.0); 
    
    let base_position = Vec3::ZERO;
    let base_position = Vec3::ZERO;
    // Apply 0 force, but the loop relies on initial state
    // We can just manually call accumulate in a loop to test it better, but since it's private
    // let's just let increase_external_force do its job. We need to set acceleration initially since 
    // increase_external_force overrides velocity with acceleration (force/mass). So our initial velocity 
    // setup above is overwritten!
    // Let's pass a force so it calculates an initial velocity.
    
    physics.increase_external_force(base_position, 10.0);
    
    let final_velocity = physics.velocity;
    
    // It should decay completely due to the loop handling epsilon check and breaking
    assert!(final_velocity.x.abs() < 0.001);
    
    // Position should have accumulated
    let last_pos = physics.get_last_position();
    assert!(last_pos.x > 0.0);
}

#[test]
fn test_increase_external_force() {
    let mut physics = PhysicsObject::new();
    physics.mass = 1.0;
    physics.direction = Vec3::new(1.0, 0.0, 0.0);
    
    let base_position = Vec3::ZERO;
    let force = 10.0;
    
    physics.increase_external_force(base_position, force);
    
    // Force is 10.0, mass is 1.0 -> accel = 10.0
    // The loop simulates 100 frames. 
    // It should have calculated movement and set last position.
    
    let last_pos = physics.get_last_position();
    assert!(last_pos.x > 0.0);
    assert_eq!(last_pos.y, 0.0);
    assert_eq!(last_pos.z, 0.0);
    
    assert!(physics.x_pushing_position.is_playing());
}

#[test]
fn test_is_blending() {
    let mut physics = PhysicsObject::new();
    assert!(!physics.is_blending());
    
    physics.velocity = Vec3::new(1.0, 0.0, 0.0);
    assert!(physics.is_blending());
    
    physics.velocity = Vec3::ZERO;
    physics.x_pushing_position.setup(0.0, 10.0, 1.0);
    assert!(physics.is_blending());
}
