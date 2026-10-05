use glam::Vec3;
use m2_client::wgpu_snow_rain_weather_environment::{RainParticle, SnowParticle, WeatherEnvironment, WEATHER_CUBE_RADIUS_XY, WEATHER_CUBE_RADIUS_Z};
use rand::{rngs::StdRng, SeedableRng};

#[test]
fn test_snow_particle_init() {
    let mut rng = StdRng::seed_from_u64(42);
    let mut particle = SnowParticle::new();
    let center_pos = Vec3::new(100.0, 200.0, 500.0);
    
    particle.init(center_pos, &mut rng);
    
    assert_eq!(particle.velocity.x, 0.0);
    assert_eq!(particle.velocity.y, 0.0);
    assert!(particle.velocity.z >= -200.0 && particle.velocity.z <= -50.0);
    
    // Bounds check based on the cubic wrap around distances
    assert!(particle.position.x >= center_pos.x - WEATHER_CUBE_RADIUS_XY && particle.position.x <= center_pos.x + WEATHER_CUBE_RADIUS_XY);
    assert!(particle.position.y >= center_pos.y - WEATHER_CUBE_RADIUS_XY && particle.position.y <= center_pos.y + WEATHER_CUBE_RADIUS_XY);
    assert!(particle.position.z >= center_pos.z - WEATHER_CUBE_RADIUS_Z && particle.position.z <= center_pos.z + WEATHER_CUBE_RADIUS_Z);
    
    assert!(particle.half_width >= 2.0 && particle.half_width <= 7.0);
    assert_eq!(particle.half_width, particle.half_height);
}

#[test]
fn test_rain_particle_init() {
    let mut rng = StdRng::seed_from_u64(84);
    let mut particle = RainParticle::new();
    let center_pos = Vec3::new(100.0, 200.0, 500.0);
    
    particle.init(center_pos, &mut rng);
    
    assert_eq!(particle.velocity.x, 0.0);
    assert_eq!(particle.velocity.y, 0.0);
    assert!(particle.velocity.z >= -1500.0 && particle.velocity.z <= -800.0);
    
    // Bounds check
    assert!(particle.position.x >= center_pos.x - WEATHER_CUBE_RADIUS_XY && particle.position.x <= center_pos.x + WEATHER_CUBE_RADIUS_XY);
    assert!(particle.position.y >= center_pos.y - WEATHER_CUBE_RADIUS_XY && particle.position.y <= center_pos.y + WEATHER_CUBE_RADIUS_XY);
    assert!(particle.position.z >= center_pos.z - WEATHER_CUBE_RADIUS_Z && particle.position.z <= center_pos.z + WEATHER_CUBE_RADIUS_Z);
    
    assert!(particle.half_width >= 0.5 && particle.half_width <= 1.5);
    assert!(particle.half_height >= 10.0 && particle.half_height <= 25.0);
}

#[test]
fn test_weather_environment_deform_with_wind() {
    let mut env = WeatherEnvironment::new();
    let mut rng = StdRng::seed_from_u64(123);
    
    env.update_position(Vec3::new(0.0, 0.0, 0.0));
    env.enable_snow();
    env.enable_rain();
    env.set_wind(Vec3::new(50.0, 0.0, 0.0));
    
    // Test that particles spawn up to 50 at a time
    env.deform(0.1, Vec3::new(0.0, 1.0, 0.0), &mut rng);
    assert_eq!(env.snow_particles.len(), 50);
    assert_eq!(env.rain_particles.len(), 50);
    
    env.deform(0.1, Vec3::new(0.0, 1.0, 0.0), &mut rng);
    assert_eq!(env.snow_particles.len(), 100);
    assert_eq!(env.rain_particles.len(), 100);

    // Take note of position and velocities to verify wind was applied
    let p_snow = env.snow_particles[0];
    let p_rain = env.rain_particles[0];

    env.deform(1.0, Vec3::new(0.0, 1.0, 0.0), &mut rng);

    // Verify Snow update (wind + internal oscillation)
    // The exact position depends on the sin wave, but we can verify it moved
    assert_ne!(env.snow_particles[0].position.x, p_snow.position.x);
    assert_ne!(env.snow_particles[0].position.z, p_snow.position.z);

    // Verify Rain update (wind + velocity)
    assert_eq!(env.rain_particles[0].position.x, p_rain.position.x + (p_rain.velocity.x + 50.0) * 1.0);
    assert_eq!(env.rain_particles[0].position.z, p_rain.position.z + p_rain.velocity.z * 1.0);
}

#[test]
fn test_snow_particle_update_wind_and_wrap_around() {
    let mut particle = SnowParticle::new();
    particle.position = Vec3::new(0.0, 0.0, -WEATHER_CUBE_RADIUS_Z - 100.0);
    particle.velocity = Vec3::new(0.0, 0.0, -100.0);
    particle.cur_radian = 0.5;
    particle.period = 2.0;
    particle.cross = Vec3::new(1.0, 0.0, 0.0);
    
    let center_pos = Vec3::new(0.0, 0.0, 0.0);
    let wind = Vec3::new(200.0, 0.0, 0.0);
    let elapsed = 0.5;
    
    // Predict wind offset
    let expected_x_offset = particle.position.x + wind.x * elapsed + particle.cross.x * f32::sin(0.5) / 10.0 * elapsed;
    
    particle.update(elapsed, center_pos, wind);
    
    // Check that it wrapped around on Z axis
    // Original pos = -2600
    // Moved = -2600 + (-100 * 0.5) = -2650
    // Wrapped = -2650 + 2 * 2500 = 2350
    assert_eq!(particle.position.z, -WEATHER_CUBE_RADIUS_Z - 100.0 - 50.0 + 2.0 * WEATHER_CUBE_RADIUS_Z);
    assert_eq!(particle.position.x, expected_x_offset);
    assert_eq!(particle.cur_radian, 0.5 + 2.0 * elapsed);
}

#[test]
fn test_weather_instance_buffer_generation() {
    let mut env = WeatherEnvironment::new();
    let mut rng = StdRng::seed_from_u64(456);
    env.enable_snow();
    env.enable_rain();
    env.set_wind(Vec3::new(10.0, 0.0, 0.0));
    env.deform(0.1, Vec3::new(1.0, 0.0, 0.0), &mut rng);
    
    assert_eq!(env.snow_particles.len(), 50);
    assert_eq!(env.rain_particles.len(), 50);
    
    let up = Vec3::new(0.0, 0.0, 1.0);
    let cross = Vec3::new(1.0, 0.0, 0.0);
    
    let snow_instances = env.generate_snow_instance_buffer(up, cross);
    assert_eq!(snow_instances.len(), 50);
    
    let rain_instances = env.generate_rain_instance_buffer(up, cross);
    assert_eq!(rain_instances.len(), 50);

    // Check that snow local up/cross matches expected scaling
    let p_snow = &env.snow_particles[0];
    let inst_snow = &snow_instances[0];
    
    assert_eq!(inst_snow.position, [p_snow.position.x, p_snow.position.y, p_snow.position.z]);
    assert_eq!(inst_snow.up, [(up * p_snow.half_width).x, (up * p_snow.half_width).y, (up * p_snow.half_width).z]);
    assert_eq!(inst_snow.cross, [(cross * p_snow.half_height).x, (cross * p_snow.half_height).y, (cross * p_snow.half_height).z]);

    // Check that rain local up/cross matches expected scaling with wind tilt
    let p_rain = &env.rain_particles[0];
    let inst_rain = &rain_instances[0];
    let dir = (p_rain.velocity + env.wind).normalize_or_zero();

    assert_eq!(inst_rain.position, [p_rain.position.x, p_rain.position.y, p_rain.position.z]);
    assert_eq!(inst_rain.up, [(-dir * p_rain.half_height).x, (-dir * p_rain.half_height).y, (-dir * p_rain.half_height).z]);
    assert_eq!(inst_rain.cross, [(cross * p_rain.half_width).x, (cross * p_rain.half_width).y, (cross * p_rain.half_width).z]);
}
