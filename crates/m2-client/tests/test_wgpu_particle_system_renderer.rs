use m2_client::wgpu_particle_system_renderer::{WgpuParticleSystemRenderer, ParticleProperty};
use m2_client::mse_script_parser::{MseEmitterProperty, MseTimeEventFloat};

#[test]
fn test_particle_euler_integration() {
    let mut emitter_prop = MseEmitterProperty::default();
    emitter_prop.max_emission_count = 10;
    emitter_prop.cycle_loop_enable = false;
    emitter_prop.time_event_emission_count_per_second.push(MseTimeEventFloat { time: 0.0, value: 100.0 });
    emitter_prop.time_event_life_time.push(MseTimeEventFloat { time: 0.0, value: 5.0 });
    
    emitter_prop.time_event_emitting_velocity.push(MseTimeEventFloat { time: 0.0, value: 10.0 });
    emitter_prop.time_event_emitting_direction_z.push(MseTimeEventFloat { time: 0.0, value: 1.0 });
    emitter_prop.time_event_emitting_direction_x.push(MseTimeEventFloat { time: 0.0, value: 0.0 });
    emitter_prop.time_event_emitting_direction_y.push(MseTimeEventFloat { time: 0.0, value: 0.0 });

    let mut particle_prop = ParticleProperty::default();
    particle_prop.time_event_gravity.push(MseTimeEventFloat { time: 0.0, value: 5.0 });
    particle_prop.time_event_air_resistance.push(MseTimeEventFloat { time: 0.0, value: 0.0 });

    let mut renderer = WgpuParticleSystemRenderer::new(particle_prop, emitter_prop);
    
    // First update spawns the particle and initializes it
    // Wait slightly more than cycle length (0.05 default) to trigger spawn
    // For local_time = 0.1, we evaluate emission_count = 100 * 0.1 = 10 particles
    renderer.update(0.1);
    
    assert_eq!(renderer.particles.len(), 10);
    
    let particle = &renderer.particles[0];
    
    // According to our create_particles logic:
    // velocity = dir * vel => 1.0 * 10.0 = 10.0
    // But since it just spawned, its age is 0.0, position is 0.0
    assert_eq!(particle.velocity.2, 10.0);
    
    // Update by dt = 1.0
    // Particles were created at the END of the first update, so they didn't get physics processed.
    renderer.update(1.0);
    
    let particle = &renderer.particles[0];
    
    // New velocity = 10.0 - gravity * 1.0 = 5.0
    assert_eq!(particle.velocity.2, 5.0);
    // New position = 0.0 + velocity * dt = 0.0 + 5.0 * 1.0 = 5.0
    assert_eq!(particle.position.2, 5.0);
}
