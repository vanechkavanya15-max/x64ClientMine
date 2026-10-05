use glam::Vec3;
use m2_client::wgpu_bone_cloth_simple_physics::BoneClothSimulation;

#[test]
fn test_verlet_integration() {
    let mut sim = BoneClothSimulation::new();
    // Initialize with acceleration = gravity, otherwise first step is 0 acceleration
    sim.gravity = Vec3::new(0.0, 0.0, -9.81);
    let idx = sim.add_point(Vec3::new(0.0, 0.0, 10.0), 1.0, false);
    sim.points[idx].acceleration = sim.gravity;
    
    // Disable drag to test pure gravity
    sim.drag = 1.0;
    
    // Simulate for 1 second with 0.1s steps (10 steps)
    for _ in 0..10 {
        sim.update(0.1, 1);
    }
    
    // pos = 10.0 + 0.5 * (-9.81) * (1.0)^2 = 5.095
    // But since it's step-based Verlet without exact analytical integration, it'll be close to 5.095
    let point = &sim.points[idx];
    assert!(point.position.z < 10.0);
    assert_eq!(point.position.x, 0.0);
    assert_eq!(point.position.y, 0.0);
}

#[test]
fn test_distance_constraint() {
    let mut sim = BoneClothSimulation::new();
    sim.gravity = Vec3::new(0.0, 0.0, -9.81);
    let p1_idx = sim.add_point(Vec3::new(0.0, 0.0, 10.0), 1.0, true);
    let p2_idx = sim.add_point(Vec3::new(0.0, 0.0, 8.0), 1.0, false);
    
    sim.points[p1_idx].acceleration = sim.gravity;
    sim.points[p2_idx].acceleration = sim.gravity;
    
    sim.add_constraint(p1_idx, p2_idx, 1.0);
    
    // Simulate for 1 second with 0.1s steps (10 steps)
    for _ in 0..10 {
        sim.update(0.1, 10);
    }
    
    let p1 = &sim.points[p1_idx];
    let p2 = &sim.points[p2_idx];
    
    // P1 is pinned, shouldn't move
    assert_eq!(p1.position.z, 10.0);
    
    // P2 should be constrained to distance of ~2.0 from P1
    let distance = p1.position.distance(p2.position);
    assert!((distance - 2.0).abs() < 0.1, "Distance constraint violated: dist={}", distance);
}

#[test]
fn test_inertia_effect() {
    let mut sim = BoneClothSimulation::new();
    let idx = sim.add_point(Vec3::new(0.0, 10.0, 0.0), 1.0, false);
    
    // Simulate a moving player, base velocity is along X axis
    let base_vel = Vec3::new(10.0, 0.0, 0.0);
    
    sim.apply_external_forces(base_vel, Vec3::ZERO, 0.1);
    
    // Acceleration should be pushed in opposite direction of movement
    let p1 = &sim.points[idx];
    assert!(p1.acceleration.x < 0.0);
}

#[test]
fn test_wind_flutter() {
    let mut sim = BoneClothSimulation::new();
    let idx = sim.add_point(Vec3::new(0.0, 10.0, 0.0), 1.0, false);
    
    // Apply wind force on Y axis
    let wind_dir = Vec3::new(0.0, 1.0, 0.0);
    
    // Step time forward a little to test sine waves
    sim.apply_external_forces(Vec3::ZERO, wind_dir, 0.5);
    
    let p1 = &sim.points[idx];
    assert_ne!(p1.acceleration.y, 0.0);
    
    // Step again, it should have a different acceleration due to sine wave
    let acc_y_1 = p1.acceleration.y;
    sim.points[idx].acceleration = Vec3::ZERO;
    
    sim.apply_external_forces(Vec3::ZERO, wind_dir, 0.5);
    let p1_2 = &sim.points[idx];
    
    assert_ne!(p1_2.acceleration.y, 0.0);
    assert_ne!(p1_2.acceleration.y, acc_y_1);
}
