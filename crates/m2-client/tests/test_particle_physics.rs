#[path = "../src/particle_physics.rs"]
mod particle_physics;

use particle_physics::{Particle, ParticlePhysicsEngine, Vector3};

#[test]
fn test_linear_movement_no_gravity_no_resistance() {
    let mut particles = vec![Particle::new(
        Vector3 { x: 0.0, y: 0.0, z: 0.0 },
        Vector3 { x: 10.0, y: 0.0, z: 0.0 },
        0.0,
        0.0,
    )];

    ParticlePhysicsEngine::update(&mut particles, 1.0, 0.0, 0.0);

    assert_eq!(particles.len(), 1);
    assert_eq!(particles[0].position.x, 10.0);
    assert_eq!(particles[0].position.y, 0.0);
    assert_eq!(particles[0].position.z, 0.0);
    assert_eq!(particles[0].velocity.x, 10.0);
    assert_eq!(particles[0].velocity.y, 0.0);
    assert_eq!(particles[0].velocity.z, 0.0);
}

#[test]
fn test_gravity_effect_on_z_axis() {
    let mut particles = vec![Particle::new(
        Vector3 { x: 0.0, y: 0.0, z: 100.0 },
        Vector3 { x: 0.0, y: 0.0, z: 0.0 },
        0.0,
        0.0,
    )];

    ParticlePhysicsEngine::update(&mut particles, 1.0, 9.81, 0.0);

    assert_eq!(particles.len(), 1);
    // Velocity drops by gravity * dt
    assert_eq!(particles[0].velocity.z, -9.81);
    // Position drops by velocity * dt
    assert_eq!(particles[0].position.z, 100.0 - 9.81);
}

#[test]
fn test_air_resistance_slowing_velocity() {
    let mut particles = vec![Particle::new(
        Vector3 { x: 0.0, y: 0.0, z: 0.0 },
        Vector3 { x: 100.0, y: 100.0, z: 100.0 },
        0.0,
        0.0,
    )];

    let dt = 1.0;
    let air_res = 0.5;

    ParticlePhysicsEngine::update(&mut particles, dt, 0.0, air_res);

    assert_eq!(particles.len(), 1);
    let expected_vel = 100.0 * (1.0 - air_res * dt).clamp(0.0, 1.0); // 50.0

    assert_eq!(particles[0].velocity.x, expected_vel);
    assert_eq!(particles[0].velocity.y, expected_vel);
    assert_eq!(particles[0].velocity.z, expected_vel);

    assert_eq!(particles[0].position.x, expected_vel);
    assert_eq!(particles[0].position.y, expected_vel);
    assert_eq!(particles[0].position.z, expected_vel);
}

#[test]
fn test_air_resistance_clamp_to_zero() {
    let mut particles = vec![Particle::new(
        Vector3 { x: 0.0, y: 0.0, z: 0.0 },
        Vector3 { x: 100.0, y: 0.0, z: 0.0 },
        0.0,
        0.0,
    )];

    // dt * air_res > 1.0
    let dt = 2.0;
    let air_res = 1.0;

    ParticlePhysicsEngine::update(&mut particles, dt, 0.0, air_res);

    assert_eq!(particles.len(), 1);
    assert_eq!(particles[0].velocity.x, 0.0); // Should be clamped to 0
    assert_eq!(particles[0].position.x, 0.0); // No movement
}

#[test]
fn test_efficient_removal_of_dead_particles() {
    let mut particles = vec![
        Particle::new(
            Vector3 { x: 1.0, y: 0.0, z: 0.0 },
            Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            0.0,
            0.0,
        ),
        Particle {
            is_dead: true,
            ..Particle::new(
                Vector3 { x: 2.0, y: 0.0, z: 0.0 },
                Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                0.0,
                0.0,
            )
        },
        Particle::new(
            Vector3 { x: 3.0, y: 0.0, z: 0.0 },
            Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            0.0,
            0.0,
        ),
    ];

    ParticlePhysicsEngine::update(&mut particles, 1.0, 0.0, 0.0);

    assert_eq!(particles.len(), 2);
    // Because swap_remove is used, the last element should replace the dead one.
    // The first is unchanged, the second was dead so it's removed and replaced by the last one (x: 3.0).
    assert_eq!(particles[0].position.x, 1.0);
    assert_eq!(particles[1].position.x, 3.0);
}

#[test]
fn test_rotation_update() {
    let mut particles = vec![Particle::new(
        Vector3 { x: 0.0, y: 0.0, z: 0.0 },
        Vector3 { x: 0.0, y: 0.0, z: 0.0 },
        10.0,
        5.0, // rotation_speed
    )];

    ParticlePhysicsEngine::update(&mut particles, 2.0, 0.0, 0.0);

    assert_eq!(particles.len(), 1);
    // rotation += rotation_speed * dt => 10.0 + 5.0 * 2.0 = 20.0
    assert_eq!(particles[0].rotation, 20.0);
}

#[test]
fn test_multiple_dead_particles() {
    let mut particles = vec![
        Particle {
            is_dead: true,
            ..Particle::new(
                Vector3 { x: 1.0, y: 0.0, z: 0.0 },
                Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                0.0,
                0.0,
            )
        },
        Particle {
            is_dead: true,
            ..Particle::new(
                Vector3 { x: 2.0, y: 0.0, z: 0.0 },
                Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                0.0,
                0.0,
            )
        },
    ];

    ParticlePhysicsEngine::update(&mut particles, 1.0, 0.0, 0.0);

    assert_eq!(particles.len(), 0);
}
