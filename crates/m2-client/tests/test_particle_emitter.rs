use m2_client::particle_emitter::{EmitterConfig, ParticleEmitter, Shape};

#[test]
fn test_spawn_point() {
    let config = EmitterConfig {
        shape: Shape::Point,
        emission_rate: 10.0,
        max_emission_count: 100,
        cycle_length: 0.0,
        cycle_loop_flag: false,
        emitting_direction: (0.0, 1.0, 0.0),
        emitting_force: 5.0,
        emitting_angle_variance: 0.0,
    };
    let emitter = ParticleEmitter::new(config);

    for i in 0..10 {
        let pos = emitter.spawn_particle_position(i as u64);
        assert_eq!(pos, (0.0, 0.0, 0.0));
    }
}

#[test]
fn test_spawn_sphere() {
    let radius = 5.0;
    let config = EmitterConfig {
        shape: Shape::Sphere {
            emitting_radius: radius,
        },
        emission_rate: 10.0,
        max_emission_count: 100,
        cycle_length: 0.0,
        cycle_loop_flag: false,
        emitting_direction: (0.0, 1.0, 0.0),
        emitting_force: 5.0,
        emitting_angle_variance: 0.0,
    };
    let emitter = ParticleEmitter::new(config);

    for i in 0..100 {
        let (x, y, z) = emitter.spawn_particle_position(i as u64);
        let dist_sq = x * x + y * y + z * z;
        assert!(
            dist_sq <= radius * radius + 0.001,
            "Point {:?} outside sphere of radius {}",
            (x, y, z),
            radius
        );
    }
}

#[test]
fn test_spawn_box() {
    let size = (10.0, 20.0, 30.0);
    let config = EmitterConfig {
        shape: Shape::Box { emitting_size: size },
        emission_rate: 10.0,
        max_emission_count: 100,
        cycle_length: 0.0,
        cycle_loop_flag: false,
        emitting_direction: (0.0, 1.0, 0.0),
        emitting_force: 5.0,
        emitting_angle_variance: 0.0,
    };
    let emitter = ParticleEmitter::new(config);

    for i in 0..100 {
        let (x, y, z) = emitter.spawn_particle_position(i as u64);
        assert!(
            x >= -5.0 && x <= 5.0,
            "x {} out of bounds [-5, 5]",
            x
        );
        assert!(
            y >= -10.0 && y <= 10.0,
            "y {} out of bounds [-10, 10]",
            y
        );
        assert!(
            z >= -15.0 && z <= 15.0,
            "z {} out of bounds [-15, 15]",
            z
        );
    }
}

#[test]
fn test_emission_rate_and_max_count() {
    let config = EmitterConfig {
        shape: Shape::Point,
        emission_rate: 10.0, // 10 particles per second
        max_emission_count: 15,
        cycle_length: 0.0,
        cycle_loop_flag: false,
        emitting_direction: (0.0, 1.0, 0.0),
        emitting_force: 5.0,
        emitting_angle_variance: 0.0,
    };
    let mut emitter = ParticleEmitter::new(config);

    // 1.0 second -> emits 10
    let emitted = emitter.update(1.0);
    assert_eq!(emitted, 10);

    // 0.5 second -> emits 5, total 15, should hit max
    let emitted = emitter.update(0.5);
    assert_eq!(emitted, 5);

    // Any further time -> emits 0 because max is reached and no looping
    let emitted = emitter.update(1.0);
    assert_eq!(emitted, 0);
}

#[test]
fn test_fractional_emission() {
    let config = EmitterConfig {
        shape: Shape::Point,
        emission_rate: 5.0, // 5 per second
        max_emission_count: 0,
        cycle_length: 0.0,
        cycle_loop_flag: false,
        emitting_direction: (0.0, 1.0, 0.0),
        emitting_force: 5.0,
        emitting_angle_variance: 0.0,
    };
    let mut emitter = ParticleEmitter::new(config);

    // 0.1s -> 0.5 particles -> 0 emitted, 0.5 stored
    assert_eq!(emitter.update(0.1), 0);
    
    // another 0.1s -> 1.0 particles total -> 1 emitted
    assert_eq!(emitter.update(0.1), 1);
    
    // 0.2s -> 1.0 particles -> 1 emitted
    assert_eq!(emitter.update(0.2), 1);
}

#[test]
fn test_cycle_looping() {
    let config = EmitterConfig {
        shape: Shape::Point,
        emission_rate: 10.0,
        max_emission_count: 5,
        cycle_length: 1.0,
        cycle_loop_flag: true,
        emitting_direction: (0.0, 1.0, 0.0),
        emitting_force: 5.0,
        emitting_angle_variance: 0.0,
    };
    let mut emitter = ParticleEmitter::new(config);

    // 0.5s -> 5 particles emitted (hits max)
    assert_eq!(emitter.update(0.5), 5);
    
    // next 0.4s -> 0 particles (max hit for this cycle, total time 0.9s)
    assert_eq!(emitter.update(0.4), 0);
    
    // next 0.1s -> hits cycle length 1.0, loop restarts.
    assert_eq!(emitter.update(0.1), 0);
    
    // next 0.1s -> 1 particle in new cycle
    assert_eq!(emitter.update(0.1), 1);
    
    // next 0.4s -> 4 particles (hits max again)
    assert_eq!(emitter.update(0.4), 4);
    
    // next 0.4s -> 0 particles
    assert_eq!(emitter.update(0.4), 0);
}

#[test]
fn test_no_loop_deactivation() {
    let config = EmitterConfig {
        shape: Shape::Point,
        emission_rate: 10.0,
        max_emission_count: 0, // unlimited count
        cycle_length: 1.0,     // 1 second cycle
        cycle_loop_flag: false,
        emitting_direction: (0.0, 1.0, 0.0),
        emitting_force: 5.0,
        emitting_angle_variance: 0.0,
    };
    let mut emitter = ParticleEmitter::new(config);

    // 0.5s -> 5 particles
    assert_eq!(emitter.update(0.5), 5);
    
    // 0.6s -> crosses 1.0s cycle. Emits 5 particles (up to 1.0s) then deactivates.
    assert_eq!(emitter.update(0.6), 5);
    
    // Now inactive
    assert_eq!(emitter.update(0.5), 0);
}

#[test]
fn test_spawn_velocity() {
    let config = EmitterConfig {
        shape: Shape::Point,
        emission_rate: 10.0,
        max_emission_count: 10,
        cycle_length: 0.0,
        cycle_loop_flag: false,
        emitting_direction: (1.0, 0.0, 0.0),
        emitting_force: 10.0,
        emitting_angle_variance: 0.0, // No variance, should be exactly force * direction
    };
    let emitter = ParticleEmitter::new(config);

    let (vx, vy, vz) = emitter.spawn_particle_velocity(0);
    assert_eq!(vx, 10.0);
    assert_eq!(vy, 0.0);
    assert_eq!(vz, 0.0);

    let config_var = EmitterConfig {
        shape: Shape::Point,
        emission_rate: 10.0,
        max_emission_count: 10,
        cycle_length: 0.0,
        cycle_loop_flag: false,
        emitting_direction: (0.0, 1.0, 0.0),
        emitting_force: 5.0,
        emitting_angle_variance: 0.5,
    };
    let emitter_var = ParticleEmitter::new(config_var);
    let (vx, vy, vz) = emitter_var.spawn_particle_velocity(12345);
    
    // Total magnitude should be approximately 5.0 after normalization * force
    let mag = (vx * vx + vy * vy + vz * vz).sqrt();
    assert!((mag - 5.0).abs() < 0.001, "Magnitude was {}", mag);
}
