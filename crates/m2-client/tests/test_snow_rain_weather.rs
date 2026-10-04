use glam::Vec3;
use m2_client::wgpu_snow_rain_weather_environment::{SnowParticle, WeatherEnvironment, SNOW_DISTANCE};
use rand::{rngs::StdRng, SeedableRng};

#[test]
fn test_snow_particle_init() {
    let mut rng = StdRng::seed_from_u64(42);
    let mut particle = SnowParticle::new();
    let center_pos = Vec3::new(100.0, 200.0, 500.0);
    
    particle.init(center_pos, &mut rng);
    
    assert!(particle.activate);
    assert_eq!(particle.velocity.x, 0.0);
    assert_eq!(particle.velocity.y, 0.0);
    assert!(particle.velocity.z >= -200.0 && particle.velocity.z <= -50.0);
    
    assert!(particle.position.z >= center_pos.z + 1500.0 && particle.position.z <= center_pos.z + 2000.0);
    assert!(particle.half_width >= 2.0 && particle.half_width <= 7.0);
    assert_eq!(particle.half_width, particle.half_height);
}

#[test]
fn test_snow_environment_deform() {
    let mut env = WeatherEnvironment::new();
    let mut rng = StdRng::seed_from_u64(123);
    
    env.update_position(Vec3::new(0.0, 0.0, 0.0));
    env.enable();
    
    // Test that particles spawn up to 10 at a time
    env.deform(0.1, Vec3::new(0.0, 1.0, 0.0), &mut rng);
    assert_eq!(env.particles.len(), 10);
    
    env.deform(0.1, Vec3::new(0.0, 1.0, 0.0), &mut rng);
    assert_eq!(env.particles.len(), 20);
    
    // Simulate falling completely below center_pos - 500.0 to test removal
    for particle in &mut env.particles {
        particle.position.z = -600.0; // center is 0, so < -500
    }
    
    // Deform should remove those 20 particles and spawn 10 new ones
    env.deform(0.1, Vec3::new(0.0, 1.0, 0.0), &mut rng);
    assert_eq!(env.particles.len(), 10);
}

#[test]
fn test_snow_particle_update_wind() {
    let mut particle = SnowParticle::new();
    particle.position = Vec3::new(0.0, 0.0, 1000.0);
    particle.velocity = Vec3::new(0.0, 0.0, -100.0);
    particle.cur_radian = 0.5;
    particle.period = 2.0;
    particle.cross = Vec3::new(1.0, 0.0, 0.0);
    particle.activate = true;
    
    let center_pos = Vec3::new(0.0, 0.0, 0.0);
    let elapsed = 0.5;
    
    // Predict wind offset
    let expected_x_offset = particle.cross.x * f32::sin(0.5) / 10.0;
    
    particle.update(elapsed, center_pos);
    
    assert_eq!(particle.position.z, 1000.0 - 100.0 * elapsed);
    assert_eq!(particle.position.x, expected_x_offset);
    assert_eq!(particle.cur_radian, 0.5 + 2.0 * elapsed);
}

#[test]
fn test_weather_instance_buffer_generation() {
    let mut env = WeatherEnvironment::new();
    let mut rng = StdRng::seed_from_u64(456);
    env.enable();
    env.deform(0.1, Vec3::new(1.0, 0.0, 0.0), &mut rng);
    
    assert_eq!(env.particles.len(), 10);
    
    let up = Vec3::new(0.0, 0.0, 1.0);
    let cross = Vec3::new(1.0, 0.0, 0.0);
    
    let instances = env.generate_instance_buffer(up, cross);
    assert_eq!(instances.len(), 10);
    
    // Check that local up/cross matches expected scaling
    let p = &env.particles[0];
    let inst = &instances[0];
    
    assert_eq!(inst.position, [p.position.x, p.position.y, p.position.z]);
    assert_eq!(inst.up, [(up * p.half_width).x, (up * p.half_width).y, (up * p.half_width).z]);
    assert_eq!(inst.cross, [(cross * p.half_height).x, (cross * p.half_height).y, (cross * p.half_height).z]);
}
