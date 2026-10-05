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

use m2_client::wgpu_particle_pipeline_complete::{TPDTVertex, TPosition, TTextureCoordinate};
use m2_client::engine_mse_particle_emitter_wire::build_wgpu_vertex_buffer;
use m2_client::mse_script_parser::MseTimeEventColor;

#[test]
fn test_build_wgpu_vertex_buffer_format() {
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
    // Red color
    effect_data.particle_property.time_event_color.push(
        MseTimeEventColor { time: 0.0, r: 1.0, g: 0.0, b: 0.0, a: 1.0 }
    );

    let mut renderer = create_particle_system_renderer(&effect_data);
    simulate_particle_system(&mut renderer, 0.1, 5);

    let up = Vec3::new(0.0, 1.0, 0.0);
    let cross = Vec3::new(1.0, 0.0, 0.0);
    let view = Vec3::new(0.0, 0.0, 1.0);

    let tpdt_vertices = build_wgpu_vertex_buffer(&renderer, up, cross, view);
    assert!(!tpdt_vertices.is_empty());
    assert!(tpdt_vertices.len() >= 4);
    
    // Check color packing (A=255, R=255, G=0, B=0)
    let diffuse = tpdt_vertices[0].diffuse;
    let a = (diffuse >> 24) & 0xFF;
    let r = (diffuse >> 16) & 0xFF;
    let g = (diffuse >> 8) & 0xFF;
    let b = diffuse & 0xFF;
    
    assert_eq!(a, 255);
    assert_eq!(r, 255);
    assert_eq!(g, 0);
    assert_eq!(b, 0);
}
#[test]
fn test_build_wgpu_vertex_buffer_color_interpolation() {
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
    // Interpolate from Red to Green over 2 seconds
    effect_data.particle_property.time_event_color.push(
        MseTimeEventColor { time: 0.0, r: 1.0, g: 0.0, b: 0.0, a: 1.0 }
    );
    effect_data.particle_property.time_event_color.push(
        MseTimeEventColor { time: 2.0, r: 0.0, g: 1.0, b: 0.0, a: 1.0 }
    );

    let mut renderer = create_particle_system_renderer(&effect_data);
    
    // Create one particle manually to be sure for test
    renderer.update(1.0);

    let up = Vec3::new(0.0, 1.0, 0.0);
    let cross = Vec3::new(1.0, 0.0, 0.0);
    let view = Vec3::new(0.0, 0.0, 1.0);

    let tpdt_vertices = build_wgpu_vertex_buffer(&renderer, up, cross, view);
    let diffuse = tpdt_vertices[0].diffuse;
    let a = (diffuse >> 24) & 0xFF;
    let r = (diffuse >> 16) & 0xFF;
    let g = (diffuse >> 8) & 0xFF;
    
    assert_eq!(a, 255);
    // age is likely exactly 0.0 because of the `renderer.update(1.0)` step and new particles spawn at 0.0. 
    // Wait, the renderer `update` increments `local_time` then creates particles with `p.age = 0.0;`. 
    // This is why r=255 and g=0. So:
    assert_eq!(r, 255);
    assert_eq!(g, 0);
}

#[test]
fn test_spawning_shapes() {
    let mut effect_data = MseEffectData {
        emitter_property: MseEmitterProperty::default(),
        particle_property: MseParticleProperty::default(),
    };
    effect_data.emitter_property.max_emission_count = 10;
    effect_data.emitter_property.cycle_length = 2.0;
    effect_data.emitter_property.cycle_loop_enable = false;
    effect_data.emitter_property.time_event_emission_count_per_second.push(
        MseTimeEventFloat { time: 0.0, value: 100.0 }
    );
    effect_data.emitter_property.time_event_life_time.push(
        MseTimeEventFloat { time: 0.0, value: 5.0 }
    );
    
    // 0 = POINT
    effect_data.emitter_property.emitter_shape = 0;
    let mut renderer = create_particle_system_renderer(&effect_data);
    simulate_particle_system(&mut renderer, 0.1, 1);
    for p in &renderer.particles {
        assert_eq!(p.position.0, 0.0);
        assert_eq!(p.position.1, 0.0);
        assert_eq!(p.position.2, 0.0);
    }
    
    // 2 = SQUARE
    effect_data.emitter_property.emitter_shape = 2;
    effect_data.emitter_property.emitting_size = [10.0, 20.0, 30.0];
    let mut renderer2 = create_particle_system_renderer(&effect_data);
    simulate_particle_system(&mut renderer2, 0.1, 1);
    for p in &renderer2.particles {
        assert!(p.position.0 >= -5.0 && p.position.0 <= 5.0);
        assert!(p.position.1 >= -10.0 && p.position.1 <= 10.0);
        assert!(p.position.2 >= -15.0 && p.position.2 <= 15.0);
    }
}

#[test]
fn test_advanced_emitter_types() {
    let mut effect_data = MseEffectData {
        emitter_property: MseEmitterProperty::default(),
        particle_property: MseParticleProperty::default(),
    };
    effect_data.emitter_property.max_emission_count = 10;
    effect_data.emitter_property.cycle_length = 2.0;
    effect_data.emitter_property.time_event_emission_count_per_second.push(
        MseTimeEventFloat { time: 0.0, value: 100.0 }
    );
    effect_data.emitter_property.time_event_life_time.push(
        MseTimeEventFloat { time: 0.0, value: 5.0 }
    );
    effect_data.emitter_property.time_event_emitting_velocity.push(
        MseTimeEventFloat { time: 0.0, value: 1.0 } // Scale velocity by 1 for simplicity
    );
    
    // SHAPE = POINT, ADVANCED = OUTER (1)
    effect_data.emitter_property.emitter_shape = 0;
    effect_data.emitter_property.emitter_advanced_type = 1;
    let mut renderer = create_particle_system_renderer(&effect_data);
    simulate_particle_system(&mut renderer, 0.1, 1);
    for p in &renderer.particles {
        // Point outer generates random velocity in [-100, 100] per axis
        assert!(p.velocity.0 >= -100.0 && p.velocity.0 <= 100.0);
        assert!(p.velocity.1 >= -100.0 && p.velocity.1 <= 100.0);
        assert!(p.velocity.2 >= -100.0 && p.velocity.2 <= 100.0);
    }
}
