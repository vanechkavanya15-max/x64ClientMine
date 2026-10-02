use glam::{Vec2, Vec3};
use wgpu;
use crate::mse_script_parser::{MseEmitterProperty, MseTimeEventFloat, MseTimeEventColor};
use crate::wgpu_particle_billboard_renderer::{ParticleVertex, BillboardType, ParticleBillboardRenderer};
use crate::particle_instance::ParticleInstance;
use std::f32::consts::PI;

fn get_time_event_blend_value_f32(time: f32, events: &[MseTimeEventFloat]) -> f32 {
    if events.is_empty() {
        return 1.0; 
    }
    if events.len() == 1 {
        return events[0].value;
    }
    if time <= events[0].time {
        return events[0].value;
    }
    if time >= events.last().unwrap().time {
        return events.last().unwrap().value;
    }
    let idx = match events.binary_search_by(|e| e.time.partial_cmp(&time).unwrap()) {
        Ok(index) => return events[index].value,
        Err(index) => index,
    };
    let low = &events[idx - 1];
    let high = &events[idx];
    let time_diff = high.time - low.time;
    let perc = (time - low.time) / time_diff;
    let val_diff = high.value - low.value;
    low.value + perc * val_diff
}

fn get_time_event_blend_value_color(time: f32, events: &[MseTimeEventColor]) -> [f32; 4] {
    if events.is_empty() {
        return [1.0, 1.0, 1.0, 1.0];
    }
    if events.len() == 1 {
        return [events[0].r, events[0].g, events[0].b, events[0].a];
    }
    if time <= events[0].time {
        return [events[0].r, events[0].g, events[0].b, events[0].a];
    }
    if time >= events.last().unwrap().time {
        let e = events.last().unwrap();
        return [e.r, e.g, e.b, e.a];
    }
    let idx = match events.binary_search_by(|e| e.time.partial_cmp(&time).unwrap()) {
        Ok(index) => return [events[index].r, events[index].g, events[index].b, events[index].a],
        Err(index) => index,
    };
    let low = &events[idx - 1];
    let high = &events[idx];
    let time_diff = high.time - low.time;
    let perc = (time - low.time) / time_diff;
    
    let mut result = [0.0; 4];
    result[0] = low.r + perc * (high.r - low.r);
    result[1] = low.g + perc * (high.g - low.g);
    result[2] = low.b + perc * (high.b - low.b);
    result[3] = low.a + perc * (high.a - low.a);
    result
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParticleProperty {
    pub billboard_type: BillboardType,
    pub src_blend_type: u8,
    pub dest_blend_type: u8,
    pub rotation_type: u8,
    pub rotation_speed: f32,
    pub rotation_random_starting_begin: f32,
    pub rotation_random_starting_end: f32,
    pub time_event_gravity: Vec<MseTimeEventFloat>,
    pub time_event_air_resistance: Vec<MseTimeEventFloat>,
    pub time_event_scale_x: Vec<MseTimeEventFloat>,
    pub time_event_scale_y: Vec<MseTimeEventFloat>,
    pub time_event_color: Vec<MseTimeEventColor>,
    pub time_event_rotation: Vec<MseTimeEventFloat>,
}

impl Default for ParticleProperty {
    fn default() -> Self {
        Self {
            billboard_type: BillboardType::All,
            src_blend_type: 2,
            dest_blend_type: 2,
            rotation_type: 0,
            rotation_speed: 0.0,
            rotation_random_starting_begin: 0.0,
            rotation_random_starting_end: 0.0,
            time_event_gravity: vec![],
            time_event_air_resistance: vec![],
            time_event_scale_x: vec![],
            time_event_scale_y: vec![],
            time_event_color: vec![],
            time_event_rotation: vec![],
        }
    }
}

pub struct WgpuParticleSystemRenderer {
    pub particle_property: ParticleProperty,
    pub emitter_property: MseEmitterProperty,
    pub particles: Vec<ParticleInstance>,
    pub local_time: f32,
    pub loop_count: i32,
    pub active: bool,
    pub emission_residue: f32,
    pub current_emission_count: u32,
}

impl WgpuParticleSystemRenderer {
    pub fn new(particle_property: ParticleProperty, emitter_property: MseEmitterProperty) -> Self {
        let loop_count = emitter_property.loop_count;
        Self {
            particle_property,
            emitter_property,
            particles: Vec::new(),
            local_time: 0.0,
            loop_count,
            active: true,
            emission_residue: 0.0,
            current_emission_count: 0,
        }
    }

    pub fn update(&mut self, elapsed_time: f32) {
        if !self.active {
            return;
        }

        self.local_time += elapsed_time;

        let mut make_particle = true;
        if self.local_time >= self.emitter_property.cycle_length {
            if self.emitter_property.cycle_loop_enable {
                self.loop_count -= 1;
                if self.loop_count != 0 {
                    if self.loop_count < 0 {
                        self.loop_count = 0;
                    }
                    self.local_time -= self.emitter_property.cycle_length;
                } else {
                    make_particle = false;
                    self.loop_count = 1;
                    if self.current_emission_count == 0 {
                        self.active = false;
                        return;
                    }
                }
            } else {
                make_particle = false;
                self.loop_count = 1;
                if self.current_emission_count == 0 {
                    self.active = false;
                    return;
                }
            }
        }

        for p in &mut self.particles {
            if p.is_alive() {
                p.age += elapsed_time;
                
                // Gravity
                let gravity = get_time_event_blend_value_f32(p.age, &self.particle_property.time_event_gravity);
                p.velocity.2 -= gravity * elapsed_time;
                
                // Air Resistance
                let air_res = get_time_event_blend_value_f32(p.age, &self.particle_property.time_event_air_resistance);
                p.velocity.0 *= 1.0 - air_res;
                p.velocity.1 *= 1.0 - air_res;
                p.velocity.2 *= 1.0 - air_res;
                
                // Euler integration
                p.position.0 += p.velocity.0 * elapsed_time;
                p.position.1 += p.velocity.1 * elapsed_time;
                p.position.2 += p.velocity.2 * elapsed_time;
            }
        }

        self.particles.retain(|p| p.is_alive());
        self.current_emission_count = self.particles.len() as u32;

        if self.active && make_particle {
            self.create_particles(elapsed_time);
        }
    }

    fn create_particles(&mut self, elapsed_time: f32) {
        let emission_rate = get_time_event_blend_value_f32(self.local_time, &self.emitter_property.time_event_emission_count_per_second);
        let emission_count_f = emission_rate * elapsed_time + self.emission_residue;
        let mut emit_count = emission_count_f.floor() as u32;
        self.emission_residue = emission_count_f - emit_count as f32;

        if emit_count == 0 {
            return;
        }

        if self.current_emission_count + emit_count > self.emitter_property.max_emission_count {
            if self.emitter_property.max_emission_count > self.current_emission_count {
                emit_count = self.emitter_property.max_emission_count - self.current_emission_count;
            } else {
                emit_count = 0;
            }
        }

        for _ in 0..emit_count {
            let lifetime = get_time_event_blend_value_f32(self.local_time, &self.emitter_property.time_event_life_time);
            let mut p = ParticleInstance::new(lifetime);
            
            p.position = (0.0, 0.0, 0.0);
            
            // Velocity
            let velocity = get_time_event_blend_value_f32(self.local_time, &self.emitter_property.time_event_emitting_velocity);
            let dir_x = get_time_event_blend_value_f32(self.local_time, &self.emitter_property.time_event_emitting_direction_x);
            let dir_y = get_time_event_blend_value_f32(self.local_time, &self.emitter_property.time_event_emitting_direction_y);
            let dir_z = get_time_event_blend_value_f32(self.local_time, &self.emitter_property.time_event_emitting_direction_z);
            
            p.velocity.0 = dir_x * velocity;
            p.velocity.1 = dir_y * velocity;
            p.velocity.2 = dir_z * velocity;
            
            // Initial Rotation (for Billboard)
            p.rotation = self.particle_property.rotation_random_starting_begin;
            
            p.age = 0.0;
            self.particles.push(p);
            self.current_emission_count += 1;
        }
    }
}

impl WgpuParticleSystemRenderer {
    pub fn build_vertex_buffer(
        &self,
        camera_up: Vec3,
        camera_cross: Vec3,
        camera_view: Vec3,
    ) -> Vec<ParticleVertex> {
        let mut vertices = Vec::with_capacity(self.particles.len() * 4);

        for p in &self.particles {
            if !p.is_alive() {
                continue;
            }

            let mut scale_x = get_time_event_blend_value_f32(p.age, &self.particle_property.time_event_scale_x);
            let mut scale_y = get_time_event_blend_value_f32(p.age, &self.particle_property.time_event_scale_y);

            if self.particle_property.time_event_scale_x.is_empty() {
                scale_x = p.base_scale.0;
            }
            if self.particle_property.time_event_scale_y.is_empty() {
                scale_y = p.base_scale.1;
            }

            let scale = Vec2::new(scale_x, scale_y);
            // hardcoded half_size to match CParticleInstance defaults if needed, 
            // but we'll use 1.0 for standard quad mapping (or rely on scale completely)
            let half_size = Vec2::new(1.0, 1.0); 
            
            let pos = Vec3::new(p.position.0, p.position.1, p.position.2);
            
            let mut rotation = p.rotation;
            if self.particle_property.rotation_type != 0 {
                if !self.particle_property.time_event_rotation.is_empty() {
                    rotation += get_time_event_blend_value_f32(p.age, &self.particle_property.time_event_rotation);
                } else {
                    rotation += self.particle_property.rotation_speed * p.age;
                }
            }

            let quad = ParticleBillboardRenderer::calculate_billboard_vertices(
                pos,
                scale,
                half_size,
                rotation,
                self.particle_property.billboard_type,
                camera_up,
                camera_cross,
                camera_view,
            );

            vertices.extend_from_slice(&quad);
        }

        vertices
    }

    pub fn get_blend_state(&self) -> wgpu::BlendState {
        // C++:
        // STATEMANAGER.SetRenderState(D3DRS_SRCBLEND, m_pParticleProperty->m_bySrcBlendType);
        // STATEMANAGER.SetRenderState(D3DRS_DESTBLEND, m_pParticleProperty->m_byDestBlendType);

        let src = match self.particle_property.src_blend_type {
            1 => wgpu::BlendFactor::Zero,
            2 => wgpu::BlendFactor::One,
            5 => wgpu::BlendFactor::SrcAlpha,
            _ => wgpu::BlendFactor::SrcAlpha, // default fallback
        };

        let dst = match self.particle_property.dest_blend_type {
            1 => wgpu::BlendFactor::Zero,
            2 => wgpu::BlendFactor::One,
            6 => wgpu::BlendFactor::OneMinusSrcAlpha,
            _ => wgpu::BlendFactor::One,
        };

        wgpu::BlendState {
            color: wgpu::BlendComponent {
                src_factor: src,
                dst_factor: dst,
                operation: wgpu::BlendOperation::Add,
            },
            alpha: wgpu::BlendComponent {
                src_factor: src,
                dst_factor: dst,
                operation: wgpu::BlendOperation::Add,
            },
        }
    }
}
