use m2_client::mse_script_parser::{MseEffectData, MseParticleProperty, MseEmitterProperty, MseTimeEventFloat};
use m2_client::engine_mse_particle_emitter_wire::{create_particle_system_renderer, simulate_particle_system, spawn_particles_on_animation_event};
use m2_client::wgpu_particle_billboard_renderer::BillboardType;
use m2_client::event_effect_attach_v2::MotionEffectEventData;
use glam::Vec3;

#[test]
fn test_emitter_creation() {
    let mut effect_data = MseEffectData {
        emitter_property: MseEmitterProperty::default(),
        particle_property: MseParticleProperty::default(),
    };
    effect_data.particle_property.billboard_type = 1;
    effect_data.particle_property.rotation_speed = 5.0;

    let renderer = create_particle_system_renderer(&effect_data);

    assert_eq!(renderer.particle_property.billboard_type, BillboardType::All);
    assert_eq!(renderer.particle_property.rotation_speed, 5.0);
}

#[test]
fn test_spawning_logic() {
    let mut effect_data = MseEffectData {
        emitter_property: MseEmitterProperty::default(),
        particle_property: MseParticleProperty::default(),
    };
    effect_data.emitter_property.max_emission_count = 100;
    effect_data.emitter_property.cycle_length = 2.0;
    effect_data.emitter_property.cycle_loop_enable = true;
    effect_data.emitter_property.loop_count = 10;
    
    // Emit 10 particles per sec
    effect_data.emitter_property.time_event_emission_count_per_second.push(
        MseTimeEventFloat { time: 0.0, value: 10.0 }
    );
    // Lifetime 5 sec
    effect_data.emitter_property.time_event_life_time.push(
        MseTimeEventFloat { time: 0.0, value: 5.0 }
    );

    let mut renderer = create_particle_system_renderer(&effect_data);

    // Simulate for 1.0 second in small steps
    let steps = 10;
    let dt = 0.1;
    let count = simulate_particle_system(&mut renderer, dt, steps);

    // 10 particles/sec * 1.0 sec = ~10 particles
    assert!(count == 10 || count == 9 || count == 11);
}

#[test]
fn test_vertex_buffer_updates() {
    let mut effect_data = MseEffectData {
        emitter_property: MseEmitterProperty::default(),
        particle_property: MseParticleProperty::default(),
    };
    effect_data.emitter_property.max_emission_count = 100;
    effect_data.emitter_property.cycle_length = 2.0;
    effect_data.emitter_property.cycle_loop_enable = true;
    effect_data.emitter_property.loop_count = 10;
    
    // Emit 10 particles per sec
    effect_data.emitter_property.time_event_emission_count_per_second.push(
        MseTimeEventFloat { time: 0.0, value: 10.0 }
    );
    // Lifetime 5 sec
    effect_data.emitter_property.time_event_life_time.push(
        MseTimeEventFloat { time: 0.0, value: 5.0 }
    );
    effect_data.particle_property.billboard_type = 1;

    let mut renderer = create_particle_system_renderer(&effect_data);

    // Simulate for 0.5 second in small steps
    simulate_particle_system(&mut renderer, 0.1, 5);

    let up = Vec3::new(0.0, 1.0, 0.0);
    let cross = Vec3::new(1.0, 0.0, 0.0);
    let view = Vec3::new(0.0, 0.0, 1.0);

    let vertices = renderer.build_vertex_buffer(up, cross, view);

    // If 5 particles spawned, 5 * 4 = 20 vertices
    assert!(!vertices.is_empty());
    assert_eq!(vertices.len() % 4, 0);
    assert_eq!(vertices.len() / 4, renderer.particles.len());
}

#[test]
fn test_spawn_particles_on_animation_event() {
    let mut effect_data = MseEffectData {
        emitter_property: MseEmitterProperty::default(),
        particle_property: MseParticleProperty::default(),
    };
    effect_data.emitter_property.max_emission_count = 10;
    effect_data.emitter_property.cycle_length = 1.0;
    effect_data.emitter_property.cycle_loop_enable = false;
    
    // Emit highly immediately
    effect_data.emitter_property.time_event_emission_count_per_second.push(
        MseTimeEventFloat { time: 0.0, value: 100.0 }
    );
    // Lifetime 1 sec
    effect_data.emitter_property.time_event_life_time.push(
        MseTimeEventFloat { time: 0.0, value: 1.0 }
    );

    let event_data = MotionEffectEventData {
        is_independent: true,
        is_attaching: false,
        is_following: false,
        effect_index: 0,
        attaching_bone_name: String::new(),
        effect_position: (10.0, 20.0, 30.0),
    };

    let renderer = spawn_particles_on_animation_event(&effect_data, &event_data);

    assert!(renderer.particles.len() > 0);
    
    // Check initial position offset application
    for p in &renderer.particles {
        assert!(p.position.0 >= 10.0);
        assert!(p.position.1 >= 20.0);
        assert!(p.position.2 >= 30.0);
    }
}
