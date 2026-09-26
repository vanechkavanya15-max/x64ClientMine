use m2_client::weather_particle_sim_v2::*;
use glam::Vec3;

#[test]
fn test_weather_sim_initialization() {
    let mut sim = WeatherSimulation::new();
    assert_eq!(sim.particles.len(), 0);
    sim.init_particles();
    assert_eq!(sim.particles.len(), 2000);
}

#[test]
fn test_particle_spawning_and_gravity() {
    let mut sim = WeatherSimulation::new();
    sim.init_particles();
    
    // Test that they are spawned within bounds
    sim.set_camera(Vec3::new(100.0, 200.0, 300.0));
    sim.spawn_particle(0);
    
    let p = sim.particles[0];
    assert!(p.active);
    assert_eq!(p.velocity.z, -150.0);
    
    // Check bounding box logic
    assert!(p.position.x >= 100.0 - 2000.0 && p.position.x <= 100.0 + 2000.0);
    assert!(p.position.y >= 200.0 - 2000.0 && p.position.y <= 200.0 + 2000.0);
    assert!(p.position.z >= 300.0 - 1000.0 && p.position.z <= 300.0 + 1000.0);
}

#[test]
fn test_wind_drift() {
    let mut sim = WeatherSimulation::new();
    sim.init_particles();
    sim.set_wind(Vec3::new(10.0, -20.0, 0.0));
    
    sim.spawn_particle(0);
    sim.particles[0].position = Vec3::new(0.0, 0.0, 0.0);
    let initial_pos = sim.particles[0].position;
    
    sim.update(1.0);
    
    let updated_pos = sim.particles[0].position;
    
    assert!((updated_pos.x - (initial_pos.x + 10.0)).abs() < 0.001);
    assert!((updated_pos.y - (initial_pos.y - 20.0)).abs() < 0.001);
}

#[test]
fn test_wrap_around_z() {
    let mut sim = WeatherSimulation::new();
    sim.init_particles();
    sim.set_camera(Vec3::new(0.0, 0.0, 0.0));
    
    sim.spawn_particle(0);
    // Put particle right at the bottom edge
    sim.particles[0].position.z = -1001.0;
    
    sim.update(0.1); // Gravity will pull it further down
    
    // Should wrap around to top
    assert!(sim.particles[0].position.z > 0.0);
}

#[test]
fn test_wrap_around_x() {
    let mut sim = WeatherSimulation::new();
    sim.init_particles();
    sim.set_camera(Vec3::new(0.0, 0.0, 0.0));
    
    sim.spawn_particle(0);
    // Move particle past right edge
    sim.particles[0].position.x = 2001.0;
    
    sim.update(0.1); // No wind, just trigger wrap logic
    
    assert!(sim.particles[0].position.x < 0.0);
}

#[test]
fn test_ffi_functions() {
    m2_weather_reset();
    m2_weather_set_camera(10.0, 20.0, 30.0);
    m2_weather_set_wind(5.0, 5.0, 0.0);
    
    let count = m2_weather_update(0.1);
    assert_eq!(count, 2000);
}
