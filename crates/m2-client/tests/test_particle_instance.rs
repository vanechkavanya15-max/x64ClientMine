use m2_client::particle_instance::ParticleInstance;

#[test]
fn test_particle_initialization_and_lifecycle() {
    let mut particle = ParticleInstance::new(5.0);
    
    assert_eq!(particle.position, (0.0, 0.0, 0.0));
    assert_eq!(particle.velocity, (0.0, 0.0, 0.0));
    assert_eq!(particle.age, 0.0);
    assert_eq!(particle.lifetime, 5.0);
    assert_eq!(particle.rotation, 0.0);
    assert_eq!(particle.rotation_speed, 0.0);
    assert_eq!(particle.base_scale, (1.0, 1.0));
    
    assert!(particle.is_alive());
    
    particle.age = 2.5;
    assert!(particle.is_alive());
    
    particle.age = 5.0;
    assert!(!particle.is_alive());
    
    particle.age = 6.0;
    assert!(!particle.is_alive());
}

#[test]
fn test_evaluate_scale_empty_events() {
    let mut particle = ParticleInstance::new(1.0);
    particle.base_scale = (2.0, 3.0);
    particle.age = 0.5; // progress = 0.5

    let events: Vec<(f32, f32)> = vec![];
    assert_eq!(particle.evaluate_scale(&events), (2.0, 3.0));
}

#[test]
fn test_evaluate_scale_single_event() {
    let mut particle = ParticleInstance::new(1.0);
    particle.base_scale = (2.0, 3.0);
    particle.age = 0.5; // progress = 0.5

    let events: Vec<(f32, f32)> = vec![(0.2, 1.5)];
    assert_eq!(particle.evaluate_scale(&events), (3.0, 4.5));
}

#[test]
fn test_evaluate_scale_out_of_bounds() {
    let mut particle = ParticleInstance::new(1.0);
    particle.base_scale = (2.0, 3.0);

    let events = vec![
        (0.2, 1.0),
        (0.8, 2.0),
    ];

    particle.age = 0.1; // progress = 0.1 (< 0.2)
    assert_eq!(particle.evaluate_scale(&events), (2.0, 3.0));

    particle.age = 0.9; // progress = 0.9 (> 0.8)
    assert_eq!(particle.evaluate_scale(&events), (4.0, 6.0));
}

#[test]
fn test_evaluate_scale_interpolation() {
    let mut particle = ParticleInstance::new(1.0);
    particle.base_scale = (2.0, 3.0);

    let events = vec![
        (0.2, 1.0),
        (0.8, 2.0),
    ];

    particle.age = 0.5; // progress = 0.5, exactly halfway between 0.2 and 0.8
    // Interpolated multiplier should be 1.5
    assert_eq!(particle.evaluate_scale(&events), (3.0, 4.5));
    
    particle.age = 0.35; // progress = 0.35, 25% between 0.2 and 0.8
    // Interpolated multiplier should be 1.25
    assert_eq!(particle.evaluate_scale(&events), (2.5, 3.75));
}

#[test]
fn test_evaluate_scale_invalid_lifetime() {
    let mut particle = ParticleInstance::new(0.0);
    particle.base_scale = (2.0, 3.0);

    let events = vec![
        (0.0, 0.0),
        (1.0, 5.0),
    ];
    
    assert_eq!(particle.evaluate_scale(&events), (2.0, 3.0));
}

#[test]
fn test_evaluate_color_empty_events() {
    let mut particle = ParticleInstance::new(1.0);
    particle.age = 0.5; // progress = 0.5

    let events: Vec<(f32, [f32; 4])> = vec![];
    assert_eq!(particle.evaluate_color(&events), [1.0, 1.0, 1.0, 1.0]);
}

#[test]
fn test_evaluate_color_single_event() {
    let mut particle = ParticleInstance::new(1.0);
    particle.age = 0.5; // progress = 0.5

    let events = vec![(0.2, [0.5, 0.6, 0.7, 0.8])];
    assert_eq!(particle.evaluate_color(&events), [0.5, 0.6, 0.7, 0.8]);
}

#[test]
fn test_evaluate_color_out_of_bounds() {
    let mut particle = ParticleInstance::new(1.0);
    let events = vec![
        (0.2, [1.0, 0.0, 0.0, 1.0]),
        (0.8, [0.0, 0.0, 1.0, 0.0]),
    ];

    particle.age = 0.1; // progress = 0.1 (< 0.2)
    assert_eq!(particle.evaluate_color(&events), [1.0, 0.0, 0.0, 1.0]);

    particle.age = 0.9; // progress = 0.9 (> 0.8)
    assert_eq!(particle.evaluate_color(&events), [0.0, 0.0, 1.0, 0.0]);
}

#[test]
fn test_evaluate_color_interpolation() {
    let mut particle = ParticleInstance::new(1.0);
    let events = vec![
        (0.2, [1.0, 0.0, 0.0, 1.0]),
        (0.8, [0.0, 1.0, 0.0, 0.0]),
    ];

    particle.age = 0.5; // progress = 0.5, halfway between 0.2 and 0.8
    // Result should be [0.5, 0.5, 0.0, 0.5]
    let color = particle.evaluate_color(&events);
    assert!((color[0] - 0.5).abs() < 1e-6);
    assert!((color[1] - 0.5).abs() < 1e-6);
    assert!((color[2] - 0.0).abs() < 1e-6);
    assert!((color[3] - 0.5).abs() < 1e-6);
}

#[test]
fn test_evaluate_color_exact_hit() {
    let mut particle = ParticleInstance::new(1.0);
    let events = vec![
        (0.2, [1.0, 0.0, 0.0, 1.0]),
        (0.5, [0.0, 1.0, 0.0, 0.5]),
        (0.8, [0.0, 0.0, 1.0, 0.0]),
    ];

    particle.age = 0.5;
    let color = particle.evaluate_color(&events);
    assert_eq!(color, [0.0, 1.0, 0.0, 0.5]);
}

#[test]
fn test_evaluate_color_invalid_lifetime() {
    let particle = ParticleInstance::new(0.0);
    let events = vec![
        (0.0, [0.0, 0.0, 0.0, 0.0]),
        (1.0, [0.0, 0.0, 0.0, 0.0]),
    ];
    
    assert_eq!(particle.evaluate_color(&events), [1.0, 1.0, 1.0, 1.0]);
}
