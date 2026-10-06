#[path = "../src/wgpu_particle_effect_emitter_v2.rs"]
mod wgpu_particle_effect_emitter_v2;

use glam::{Vec2, Vec3};
use wgpu_particle_effect_emitter_v2::{
    BillboardType, EmitterProperty, ParticleProperty, WgpuParticleEffectEmitterV2, TimeEventFloat, TimeEventColor, ParticleInstance
};

#[test]
fn test_emitter_creation_and_basic_update() {
    let mut emitter_prop = EmitterProperty::default();
    emitter_prop.max_emission_count = 10;
    emitter_prop.time_event_emission_count_per_second.push(TimeEventFloat { time: 0.0, value: 5.0 }); // emit 5 per sec
    emitter_prop.time_event_life_time.push(TimeEventFloat { time: 0.0, value: 2.0 }); // live for 2 secs

    let particle_prop = ParticleProperty::default();

    let mut emitter = WgpuParticleEffectEmitterV2::new(emitter_prop, particle_prop);
    
    assert_eq!(emitter.particles.len(), 0);
    assert_eq!(emitter.current_emission_count, 0);

    // Update with 1 second elapsed time
    let is_alive = emitter.update(1.0);
    assert!(is_alive);
    
    // We should have emitted 5 particles
    assert_eq!(emitter.particles.len(), 5);
    assert_eq!(emitter.current_emission_count, 5);
}

#[test]
fn test_particle_lifetime_and_removal() {
    let mut emitter_prop = EmitterProperty::default();
    emitter_prop.max_emission_count = 5;
    emitter_prop.time_event_emission_count_per_second.push(TimeEventFloat { time: 0.0, value: 5.0 }); // emit 5 per sec
    emitter_prop.time_event_life_time.push(TimeEventFloat { time: 0.0, value: 1.0 }); // live for 1 sec

    let particle_prop = ParticleProperty::default();
    let mut emitter = WgpuParticleEffectEmitterV2::new(emitter_prop, particle_prop);
    
    // Update 1: emit particles
    emitter.update(1.0);
    assert_eq!(emitter.particles.len(), 5);

    // Update 2: particles age, but are still alive right at edge (age == lifetime depending on precision, let's go beyond)
    emitter.update(0.1); 
    // Wait, last_life_time goes from 1.0 -> 0.0, so at 1.1s it's -0.1 < 0 -> dead
    assert_eq!(emitter.particles.len(), 0);
}

#[test]
fn test_particle_physics_gravity_and_air_resistance() {
    let mut emitter_prop = EmitterProperty::default();
    emitter_prop.max_emission_count = 1;
    emitter_prop.time_event_emission_count_per_second.push(TimeEventFloat { time: 0.0, value: 1.0 });
    emitter_prop.time_event_life_time.push(TimeEventFloat { time: 0.0, value: 10.0 });
    emitter_prop.emitter_shape = 0; // Point
    emitter_prop.time_event_emitting_velocity.push(TimeEventFloat { time: 0.0, value: 1.0 });
    
    let mut particle_prop = ParticleProperty::default();
    particle_prop.time_event_gravity.push(TimeEventFloat { time: 0.0, value: 9.8 });
    particle_prop.time_event_air_resistance.push(TimeEventFloat { time: 0.0, value: 0.1 });

    let mut emitter = WgpuParticleEffectEmitterV2::new(emitter_prop, particle_prop);
    
    emitter.update(1.0);
    assert_eq!(emitter.particles.len(), 1);
    
    let p = &emitter.particles[0];
    
    // Original velocity was whatever was generated (mostly 0.0 except z drop), let's check z velocity decreases
    // Actually, vel.z -= 9.8 * 1.0 = -9.8
    // vel *= 1.0 - 0.1 = 0.9
    // so vel.z should be around -9.8 * 0.9 = -8.82
    assert!(p.velocity.z < 0.0);
    assert!(p.position.z < 0.0);
}

#[test]
fn test_billboard_vertex_generation() {
    let mut emitter_prop = EmitterProperty::default();
    emitter_prop.max_emission_count = 1;
    emitter_prop.time_event_emission_count_per_second.push(TimeEventFloat { time: 0.0, value: 10.0 });
    emitter_prop.time_event_life_time.push(TimeEventFloat { time: 0.0, value: 1.0 });
    
    let particle_prop = ParticleProperty {
        billboard_type: BillboardType::All,
        ..Default::default()
    };
    
    let mut emitter = WgpuParticleEffectEmitterV2::new(emitter_prop, particle_prop);
    emitter.update(0.1);
    assert_eq!(emitter.particles.len(), 1);
    
    let camera_up = Vec3::new(0.0, 1.0, 0.0);
    let camera_cross = Vec3::new(1.0, 0.0, 0.0);
    let camera_view = Vec3::new(0.0, 0.0, 1.0);
    
    let vertices = emitter.build_vertex_buffer(camera_up, camera_cross, camera_view);
    
    // For 1 particle with BillboardType::All, we expect 4 vertices
    assert_eq!(vertices.len(), 4);
}

#[test]
fn test_time_event_blending() {
    use wgpu_particle_effect_emitter_v2::get_time_event_blend_value_f32;
    
    let events = vec![
        TimeEventFloat { time: 0.0, value: 10.0 },
        TimeEventFloat { time: 1.0, value: 20.0 },
    ];
    
    assert_eq!(get_time_event_blend_value_f32(0.0, &events), 10.0);
    assert_eq!(get_time_event_blend_value_f32(0.5, &events), 15.0);
    assert_eq!(get_time_event_blend_value_f32(1.0, &events), 20.0);
    assert_eq!(get_time_event_blend_value_f32(2.0, &events), 20.0);
    assert_eq!(get_time_event_blend_value_f32(-1.0, &events), 10.0);
}

#[test]
fn test_time_event_color_blending() {
    use wgpu_particle_effect_emitter_v2::get_time_event_blend_value_color;
    
    let events = vec![
        TimeEventColor { time: 0.0, r: 1.0, g: 0.0, b: 0.0, a: 1.0 },
        TimeEventColor { time: 1.0, r: 0.0, g: 1.0, b: 0.0, a: 0.0 },
    ];
    
    let c_start = get_time_event_blend_value_color(0.0, &events);
    assert_eq!(c_start, [1.0, 0.0, 0.0, 1.0]);
    
    let c_mid = get_time_event_blend_value_color(0.5, &events);
    assert_eq!(c_mid, [0.5, 0.5, 0.0, 0.5]);
    
    let c_end = get_time_event_blend_value_color(1.0, &events);
    assert_eq!(c_end, [0.0, 1.0, 0.0, 0.0]);
}

