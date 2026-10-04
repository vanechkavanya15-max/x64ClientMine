use crate::mse_script_parser::MseEffectData;
use crate::wgpu_particle_system_renderer::{ParticleProperty, WgpuParticleSystemRenderer};
use crate::wgpu_particle_billboard_renderer::BillboardType;
use crate::mse_script_parser::{MseTimeEventFloat, MseTimeEventColor, MseEmitterProperty, MseParticleProperty};
use crate::event_effect_attach_v2::MotionEffectEventData;
use glam::Vec3;
use crate::wgpu_particle_pipeline_complete::{TPDTVertex, TPosition, TTextureCoordinate};

pub fn create_particle_system_renderer(effect_data: &MseEffectData) -> WgpuParticleSystemRenderer {
    let mse_prop = &effect_data.particle_property;
    
    let billboard_type = match mse_prop.billboard_type {
        0 => BillboardType::None,
        1 => BillboardType::All,
        2 => BillboardType::Y,
        3 => BillboardType::Lie,
        4 => BillboardType::TwoFace,
        5 => BillboardType::ThreeFace,
        _ => BillboardType::All,
    };

    let particle_prop = ParticleProperty {
        billboard_type,
        src_blend_type: mse_prop.src_blend_type,
        dest_blend_type: mse_prop.dest_blend_type,
        rotation_type: mse_prop.rotation_type,
        rotation_speed: mse_prop.rotation_speed,
        rotation_random_starting_begin: mse_prop.rotation_random_starting_begin as f32,
        rotation_random_starting_end: mse_prop.rotation_random_starting_end as f32,
        time_event_gravity: mse_prop.time_event_gravity.iter().map(|e| MseTimeEventFloat { time: e.time, value: e.value }).collect(),
        time_event_air_resistance: mse_prop.time_event_air_resistance.iter().map(|e| MseTimeEventFloat { time: e.time, value: e.value }).collect(),
        time_event_scale_x: mse_prop.time_event_scale_x.iter().map(|e| MseTimeEventFloat { time: e.time, value: e.value }).collect(),
        time_event_scale_y: mse_prop.time_event_scale_y.iter().map(|e| MseTimeEventFloat { time: e.time, value: e.value }).collect(),
        time_event_color: mse_prop.time_event_color.iter().map(|e| MseTimeEventColor { time: e.time, r: e.r, g: e.g, b: e.b, a: e.a }).collect(),
        time_event_rotation: mse_prop.time_event_rotation.iter().map(|e| MseTimeEventFloat { time: e.time, value: e.value }).collect(),
    };

    let em_prop = &effect_data.emitter_property;
    let emitter_property = MseEmitterProperty {
        max_emission_count: em_prop.max_emission_count,
        cycle_length: em_prop.cycle_length,
        cycle_loop_enable: em_prop.cycle_loop_enable,
        loop_count: em_prop.loop_count,
        emitter_shape: em_prop.emitter_shape,
        emitter_advanced_type: em_prop.emitter_advanced_type,
        emitting_size: em_prop.emitting_size,
        emitting_radius: em_prop.emitting_radius,
        emitting_direction: em_prop.emitting_direction,
        emit_from_edge_flag: em_prop.emit_from_edge_flag,
        
        time_event_emitting_size: em_prop.time_event_emitting_size.iter().map(|e| MseTimeEventFloat { time: e.time, value: e.value }).collect(),
        time_event_emitting_angular_velocity: em_prop.time_event_emitting_angular_velocity.iter().map(|e| MseTimeEventFloat { time: e.time, value: e.value }).collect(),
        time_event_emitting_direction_x: em_prop.time_event_emitting_direction_x.iter().map(|e| MseTimeEventFloat { time: e.time, value: e.value }).collect(),
        time_event_emitting_direction_y: em_prop.time_event_emitting_direction_y.iter().map(|e| MseTimeEventFloat { time: e.time, value: e.value }).collect(),
        time_event_emitting_direction_z: em_prop.time_event_emitting_direction_z.iter().map(|e| MseTimeEventFloat { time: e.time, value: e.value }).collect(),
        time_event_emitting_velocity: em_prop.time_event_emitting_velocity.iter().map(|e| MseTimeEventFloat { time: e.time, value: e.value }).collect(),
        time_event_emission_count_per_second: em_prop.time_event_emission_count_per_second.iter().map(|e| MseTimeEventFloat { time: e.time, value: e.value }).collect(),
        time_event_life_time: em_prop.time_event_life_time.iter().map(|e| MseTimeEventFloat { time: e.time, value: e.value }).collect(),
        time_event_size_x: em_prop.time_event_size_x.iter().map(|e| MseTimeEventFloat { time: e.time, value: e.value }).collect(),
        time_event_size_y: em_prop.time_event_size_y.iter().map(|e| MseTimeEventFloat { time: e.time, value: e.value }).collect(),
    };

    WgpuParticleSystemRenderer::new(particle_prop, emitter_property)
}

pub fn simulate_particle_system(renderer: &mut WgpuParticleSystemRenderer, dt: f32, steps: u32) -> u32 {
    for _ in 0..steps {
        renderer.update(dt);
    }
    renderer.current_emission_count
}

pub fn spawn_particles_on_animation_event(
    effect_data: &MseEffectData,
    event_data: &MotionEffectEventData,
) -> WgpuParticleSystemRenderer {
    let mut renderer = create_particle_system_renderer(effect_data);

    // Initial positioning based on animation event
    let (ex, ey, ez) = event_data.effect_position;

    // Typically, particle systems spawn some initial particles or
    // set their start position depending on where the effect was triggered (e.g. sword hitting ground)
    // We update all properties or simply run the update to spawn initial particles
    renderer.update(0.016); // force an initial spawn step

    for p in &mut renderer.particles {
        p.position.0 += ex;
        p.position.1 += ey;
        p.position.2 += ez;
    }

    renderer
}

pub fn build_wgpu_vertex_buffer(
    renderer: &WgpuParticleSystemRenderer,
    camera_up: Vec3,
    camera_cross: Vec3,
    camera_view: Vec3,
) -> Vec<TPDTVertex> {
    let base_vertices = renderer.build_vertex_buffer(camera_up, camera_cross, camera_view);
    
    let mut tpdt_vertices = Vec::with_capacity(base_vertices.len());
    
    let color_events: Vec<(f32, [f32; 4])> = renderer
        .particle_property
        .time_event_color
        .iter()
        .map(|e| (e.time, [e.r, e.g, e.b, e.a]))
        .collect();

    let mut v_idx = 0;
    for p in &renderer.particles {
        if !p.is_alive() {
            continue;
        }
            
        let color = p.evaluate_color(&color_events);
        
        let r_u8 = (color[0] * 255.0).clamp(0.0, 255.0) as u32;
        let g_u8 = (color[1] * 255.0).clamp(0.0, 255.0) as u32;
        let b_u8 = (color[2] * 255.0).clamp(0.0, 255.0) as u32;
        let a_u8 = (color[3] * 255.0).clamp(0.0, 255.0) as u32;
        
        let diffuse = (a_u8 << 24) | (r_u8 << 16) | (g_u8 << 8) | b_u8;
        
        for i in 0..4 {
            let base_v = &base_vertices[v_idx + i];
            tpdt_vertices.push(TPDTVertex {
                position: TPosition {
                    x: base_v.position[0],
                    y: base_v.position[1],
                    z: base_v.position[2],
                },
                diffuse,
                tex_coord: TTextureCoordinate {
                    u: base_v.tex_coord[0],
                    v: base_v.tex_coord[1],
                }
            });
        }
        
        v_idx += 4;
    }
    
    tpdt_vertices
}
