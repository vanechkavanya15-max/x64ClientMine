use glam::{Mat4, Quat, Vec2, Vec3, Vec4};
use rand::Rng;
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BillboardType {
    None = 0,
    All = 1,
    Y = 2,
    Lie = 3,
    TwoFace = 4,
    ThreeFace = 5,
}

impl Default for BillboardType {
    fn default() -> Self {
        BillboardType::None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BlendType {
    Zero = 1,
    One = 2,
    SrcColor = 3,
    InvSrcColor = 4,
    SrcAlpha = 5,
    InvSrcAlpha = 6,
    DestAlpha = 7,
    InvDestAlpha = 8,
    DestColor = 9,
    InvDestColor = 10,
    SrcAlphaSat = 11,
    BothSrcAlpha = 12,
    InvBothSrcAlpha = 13,
}

impl Default for BlendType {
    fn default() -> Self {
        BlendType::One
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ParticleVertex {
    pub position: [f32; 3],
    pub diffuse: u32,
    pub tex_coord: [f32; 2],
}

#[derive(Debug, Clone, PartialEq)]
pub struct TimeEventFloat {
    pub time: f32,
    pub value: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TimeEventColor {
    pub time: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

pub fn get_time_event_blend_value_f32(time: f32, events: &[TimeEventFloat]) -> f32 {
    if events.is_empty() {
        return 0.0; 
    }
    if events.len() == 1 || time <= events[0].time {
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

pub fn get_time_event_blend_value_color(time: f32, events: &[TimeEventColor]) -> [f32; 4] {
    if events.is_empty() {
        return [1.0, 1.0, 1.0, 1.0];
    }
    if events.len() == 1 || time <= events[0].time {
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

fn pack_color(color: [f32; 4]) -> u32 {
    let r = (color[0].clamp(0.0, 1.0) * 255.0) as u32;
    let g = (color[1].clamp(0.0, 1.0) * 255.0) as u32;
    let b = (color[2].clamp(0.0, 1.0) * 255.0) as u32;
    let a = (color[3].clamp(0.0, 1.0) * 255.0) as u32;
    (a << 24) | (r << 16) | (g << 8) | b
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParticleInstance {
    pub position: Vec3,
    pub last_position: Vec3,
    pub velocity: Vec3,
    pub start_position: Vec3,
    pub half_size: Vec2,
    pub scale: Vec2,
    pub rotation: f32,
    pub color: [f32; 4],
    
    pub texture_animation_type: u8,
    pub last_frame_time: f32,
    pub frame_index: u8,
    pub frame_time: f32,
    
    pub life_time: f32,
    pub last_life_time: f32,
    
    pub rotation_type: u8,
    pub air_resistance: f32,
    pub rotation_speed: f32,
    pub gravity: f32,
}

impl ParticleInstance {
    pub fn new(life_time: f32) -> Self {
        Self {
            position: Vec3::ZERO,
            last_position: Vec3::ZERO,
            velocity: Vec3::ZERO,
            start_position: Vec3::ZERO,
            half_size: Vec2::new(1.0, 1.0),
            scale: Vec2::new(1.0, 1.0),
            rotation: 0.0,
            color: [1.0, 1.0, 1.0, 1.0],
            
            texture_animation_type: 0,
            last_frame_time: 0.0,
            frame_index: 0,
            frame_time: 0.0,
            
            life_time,
            last_life_time: life_time,
            
            rotation_type: 0,
            air_resistance: 0.0,
            rotation_speed: 0.0,
            gravity: 0.0,
        }
    }
    
    pub fn is_alive(&self) -> bool {
        self.last_life_time > 0.0
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ParticleProperty {
    pub billboard_type: BillboardType,
    pub src_blend_type: BlendType,
    pub dest_blend_type: BlendType,
    pub color_operation_type: u8,
    
    pub rotation_type: u8,
    pub rotation_speed: f32,
    pub rotation_random_starting_begin: u16,
    pub rotation_random_starting_end: u16,
    
    pub attach_flag: bool,
    pub stretch_flag: bool,
    
    pub texture_animation_type: u8,
    pub texture_animation_delay: f32,
    pub texture_animation_random_start_frame_flag: bool,
    pub texture_animation_frame_count: u32,
    
    pub time_event_gravity: Vec<TimeEventFloat>,
    pub time_event_air_resistance: Vec<TimeEventFloat>,
    pub time_event_scale_x: Vec<TimeEventFloat>,
    pub time_event_scale_y: Vec<TimeEventFloat>,
    pub time_event_color: Vec<TimeEventColor>,
    pub time_event_rotation: Vec<TimeEventFloat>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct EmitterProperty {
    pub max_emission_count: u32,
    pub cycle_length: f32,
    pub cycle_loop_flag: bool,
    pub loop_count: i32,
    
    pub emitter_shape: u8,
    pub emitter_advanced_type: u8,
    pub emit_from_edge_flag: bool,
    pub emitting_size: Vec3,
    pub emitting_radius: f32,
    pub emitting_direction: Vec3,
    
    pub time_event_emitting_size: Vec<TimeEventFloat>,
    pub time_event_emitting_angular_velocity: Vec<TimeEventFloat>,
    pub time_event_emitting_direction_x: Vec<TimeEventFloat>,
    pub time_event_emitting_direction_y: Vec<TimeEventFloat>,
    pub time_event_emitting_direction_z: Vec<TimeEventFloat>,
    pub time_event_emitting_velocity: Vec<TimeEventFloat>,
    pub time_event_emission_count_per_second: Vec<TimeEventFloat>,
    pub time_event_life_time: Vec<TimeEventFloat>,
    pub time_event_size_x: Vec<TimeEventFloat>,
    pub time_event_size_y: Vec<TimeEventFloat>,
}

pub struct WgpuParticleEffectEmitterV2 {
    pub particles: Vec<ParticleInstance>,
    pub emitter_property: EmitterProperty,
    pub particle_property: ParticleProperty,
    pub local_time: f32,
    pub emission_residue: f32,
    pub current_emission_count: u32,
    pub loop_count: i32,
    pub local_matrix: Option<Mat4>,
    pub active: bool,
}

impl WgpuParticleEffectEmitterV2 {
    pub fn new(emitter_property: EmitterProperty, particle_property: ParticleProperty) -> Self {
        Self {
            particles: Vec::new(),
            emitter_property,
            particle_property,
            local_time: 0.0,
            emission_residue: 0.0,
            current_emission_count: 0,
            loop_count: 0,
            local_matrix: None,
            active: true,
        }
    }

    pub fn update(&mut self, elapsed_time: f32) -> bool {
        if !self.active {
            return false;
        }

        self.local_time += elapsed_time;

        let mut is_alive = true;
        if self.emitter_property.cycle_length > 0.0 && self.local_time > self.emitter_property.cycle_length {
            if self.emitter_property.cycle_loop_flag {
                self.local_time = (self.local_time % self.emitter_property.cycle_length).max(0.0);
                self.loop_count += 1;
                if self.emitter_property.loop_count != 0 && self.loop_count >= self.emitter_property.loop_count {
                    is_alive = false;
                }
            } else {
                is_alive = false;
            }
        }

        self.create_particles(elapsed_time);

        let mut next_particles = Vec::with_capacity(self.particles.len());
        
        let mut current_particles = std::mem::take(&mut self.particles);
        for mut p in current_particles.drain(..) {
            if WgpuParticleEffectEmitterV2::update_particle_static(&self.particle_property, &mut p, elapsed_time) {
                next_particles.push(p);
            }
        }
        
        self.particles = next_particles;
        self.current_emission_count = self.particles.len() as u32;
        
        is_alive || !self.particles.is_empty()
    }

    fn update_particle_static(particle_property: &ParticleProperty, p: &mut ParticleInstance, elapsed_time: f32) -> bool {
        p.last_life_time -= elapsed_time;
        if p.last_life_time < 0.0 {
            return false;
        }

        let life_percentage = if p.life_time > 0.0 {
            (p.life_time - p.last_life_time) / p.life_time
        } else {
            0.0
        };

        // Rotation
        if p.rotation_type == 1 { // ROTATION_TYPE_TIME_EVENT
            p.rotation_speed = get_time_event_blend_value_f32(life_percentage, &particle_property.time_event_rotation);
        }
        if p.rotation_type != 0 {
            p.rotation += p.rotation_speed * elapsed_time;
        }

        // Texture Animation
        if particle_property.texture_animation_type != 0 {
            let delay = particle_property.texture_animation_delay;
            let count = particle_property.texture_animation_frame_count;
            if count > 0 && delay > 0.0 {
                p.frame_time += elapsed_time;
                let elapsed_frames = (p.frame_time / delay) as u32;
                if elapsed_frames > 0 {
                    p.frame_time -= elapsed_frames as f32 * delay;
                    
                    match particle_property.texture_animation_type {
                        1 => { // CW
                            p.frame_index = ((p.frame_index as u32 + elapsed_frames) % count) as u8;
                        }
                        2 => { // CCW
                            p.frame_index = p.frame_index.saturating_sub(elapsed_frames as u8).min((count - 1) as u8);
                        }
                        3 => { // RANDOM
                            let mut rng = rand::thread_rng();
                            p.frame_index = rng.gen_range(0..count) as u8;
                        }
                        _ => {}
                    }
                }
            }
        }

        // Scale
        if !particle_property.time_event_scale_x.is_empty() {
            p.scale.x = get_time_event_blend_value_f32(life_percentage, &particle_property.time_event_scale_x);
        }
        if !particle_property.time_event_scale_y.is_empty() {
            p.scale.y = get_time_event_blend_value_f32(life_percentage, &particle_property.time_event_scale_y);
        }

        // Color
        if !particle_property.time_event_color.is_empty() {
            p.color = get_time_event_blend_value_color(life_percentage, &particle_property.time_event_color);
        }

        // Physics
        let mut gravity = 0.0;
        if !particle_property.time_event_gravity.is_empty() {
            gravity = get_time_event_blend_value_f32(life_percentage, &particle_property.time_event_gravity);
        }
        let mut air_res = 0.0;
        if !particle_property.time_event_air_resistance.is_empty() {
            air_res = get_time_event_blend_value_f32(life_percentage, &particle_property.time_event_air_resistance);
        }

        p.last_position = p.position;
        
        // Physics integration
        p.velocity.z -= gravity * elapsed_time;
        if air_res > 0.0 {
            p.velocity *= 1.0 - air_res;
        }
        p.position += p.velocity * elapsed_time;

        true
    }

    fn create_particles(&mut self, elapsed_time: f32) {
        let emission_rate = get_time_event_blend_value_f32(self.local_time, &self.emitter_property.time_event_emission_count_per_second);
        let creating_value = emission_rate * elapsed_time + self.emission_residue;
        let mut i_creating_count = creating_value.floor() as i32;
        self.emission_residue = creating_value - i_creating_count as f32;

        let max_emission = self.emitter_property.max_emission_count as i32;
        let next_emission = self.current_emission_count as i32 + i_creating_count;
        if next_emission > max_emission {
            let overflow = next_emission - max_emission;
            i_creating_count = i_creating_count.saturating_sub(overflow).max(0);
        }

        if i_creating_count <= 0 {
            return;
        }

        let mut rng = rand::thread_rng();

        for _ in 0..i_creating_count {
            let life_time = get_time_event_blend_value_f32(self.local_time, &self.emitter_property.time_event_life_time);
            if life_time <= 0.0 {
                continue;
            }

            let emitting_size = get_time_event_blend_value_f32(self.local_time, &self.emitter_property.time_event_emitting_size);
            
            let mut pos = Vec3::ZERO;
            
            match self.emitter_property.emitter_shape {
                0 => {} // POINT
                1 => { // ELLIPSE
                    let mut x: f32 = rng.gen_range(-500.0..=500.0);
                    let mut y: f32 = rng.gen_range(-500.0..=500.0);
                    let mut p_pos = Vec3::new(x, y, 0.0);
                    if p_pos.length_squared() > 0.0 {
                        p_pos = p_pos.normalize();
                    }
                    if self.emitter_property.emit_from_edge_flag {
                        p_pos *= self.emitter_property.emitting_radius + emitting_size;
                    } else {
                        let r = rng.gen_range(0.0..=(self.emitter_property.emitting_radius.max(0.0001)));
                        p_pos *= r + emitting_size;
                    }
                    pos = p_pos;
                }
                2 => { // SQUARE
                    let hx = self.emitter_property.emitting_size.x / 2.0;
                    let hy = self.emitter_property.emitting_size.y / 2.0;
                    let hz = self.emitter_property.emitting_size.z / 2.0;
                    let rx = rng.gen_range(-hx..=hx) + emitting_size;
                    let ry = rng.gen_range(-hy..=hy) + emitting_size;
                    let rz = rng.gen_range(-hz..=hz) + emitting_size;
                    pos = Vec3::new(rx, ry, rz);
                }
                3 => { // SPHERE
                    let mut x: f32 = rng.gen_range(-500.0..=500.0);
                    let mut y: f32 = rng.gen_range(-500.0..=500.0);
                    let mut z: f32 = rng.gen_range(-500.0..=500.0);
                    let mut p_pos = Vec3::new(x, y, z);
                    if p_pos.length_squared() > 0.0 {
                        p_pos = p_pos.normalize();
                    }
                    if self.emitter_property.emit_from_edge_flag {
                        p_pos *= self.emitter_property.emitting_radius + emitting_size;
                    } else {
                        let r = rng.gen_range(0.0..=(self.emitter_property.emitting_radius.max(0.0001)));
                        p_pos *= r + emitting_size;
                    }
                    pos = p_pos;
                }
                _ => {}
            }

            let mut p = ParticleInstance::new(life_time);
            p.position = pos;
            p.start_position = pos;
            p.last_position = pos;

            let mut vel = Vec3::ZERO;
            match self.emitter_property.emitter_advanced_type {
                1 => { // OUTER
                    if self.emitter_property.emitter_shape == 0 {
                        vel.x = rng.gen_range(-100.0..=100.0);
                        vel.y = rng.gen_range(-100.0..=100.0);
                        vel.z = rng.gen_range(-100.0..=100.0);
                    } else {
                        if pos.length_squared() > 0.0 {
                            vel = pos.normalize() * 100.0;
                        }
                    }
                }
                2 => { // INNER
                    if pos.length_squared() > 0.0 {
                        vel = pos.normalize() * -100.0;
                    }
                }
                _ => {} // FREE
            }

            let dir_x = get_time_event_blend_value_f32(self.local_time, &self.emitter_property.time_event_emitting_direction_x);
            let dir_y = get_time_event_blend_value_f32(self.local_time, &self.emitter_property.time_event_emitting_direction_y);
            let dir_z = get_time_event_blend_value_f32(self.local_time, &self.emitter_property.time_event_emitting_direction_z);
            let mut emit_dir = Vec3::new(dir_x, dir_y, dir_z);

            if self.emitter_property.emitting_direction.x > 0.0 {
                let d = self.emitter_property.emitting_direction.x / 2.0;
                emit_dir.x += rng.gen_range(-d..=d) * 1000.0;
            }
            if self.emitter_property.emitting_direction.y > 0.0 {
                let d = self.emitter_property.emitting_direction.y / 2.0;
                emit_dir.y += rng.gen_range(-d..=d) * 1000.0;
            }
            if self.emitter_property.emitting_direction.z > 0.0 {
                let d = self.emitter_property.emitting_direction.z / 2.0;
                emit_dir.z += rng.gen_range(-d..=d) * 1000.0;
            }

            vel += emit_dir;
            let velocity_factor = get_time_event_blend_value_f32(self.local_time, &self.emitter_property.time_event_emitting_velocity);
            vel *= velocity_factor;
            p.velocity = vel;

            p.half_size.x = get_time_event_blend_value_f32(self.local_time, &self.emitter_property.time_event_size_x);
            p.half_size.y = get_time_event_blend_value_f32(self.local_time, &self.emitter_property.time_event_size_y);

            let rot_begin = self.particle_property.rotation_random_starting_begin as f32;
            let rot_end = self.particle_property.rotation_random_starting_end as f32;
            if rot_begin < rot_end {
                p.rotation = rng.gen_range(rot_begin..=rot_end);
            } else if rot_begin > rot_end {
                p.rotation = rng.gen_range(rot_end..=rot_begin);
            } else {
                p.rotation = rot_begin;
            }

            p.rotation_type = self.particle_property.rotation_type;
            p.rotation_speed = self.particle_property.rotation_speed;
            p.texture_animation_type = self.particle_property.texture_animation_type;

            if self.particle_property.texture_animation_random_start_frame_flag && self.particle_property.texture_animation_frame_count > 0 {
                p.frame_index = rng.gen_range(0..self.particle_property.texture_animation_frame_count) as u8;
            }

            self.particles.push(p);
            self.current_emission_count += 1;
        }
    }

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

            let mut v3_up = Vec3::ZERO;
            let mut v3_cross = Vec3::ZERO;

            if !self.particle_property.stretch_flag {
                match self.particle_property.billboard_type {
                    BillboardType::Lie => {
                        let rot_rad = p.rotation.to_radians();
                        let f_cos = rot_rad.cos();
                        let f_sin = rot_rad.sin();
                        v3_up = Vec3::new(f_cos, -f_sin, 0.0);
                        v3_cross = Vec3::new(f_sin, f_cos, 0.0);
                    }
                    BillboardType::TwoFace | BillboardType::ThreeFace | BillboardType::Y => {
                        v3_up = Vec3::new(0.0, 0.0, 1.0);
                        if v3_up.x * camera_view.y - v3_up.y * camera_view.x < 0.0 {
                            v3_up = -v3_up;
                        }
                        let view_xy = Vec3::new(camera_view.x, camera_view.y, 0.0);
                        v3_cross = v3_up.cross(view_xy).normalize();

                        if p.rotation != 0.0 {
                            let rot_rad = p.rotation.to_radians();
                            let f_cos = -rot_rad.sin();
                            let f_sin = rot_rad.cos();
                            let temp_up = v3_up * f_cos - v3_cross * f_sin;
                            v3_cross = v3_cross * f_cos + v3_up * f_sin;
                            v3_up = temp_up;
                        }
                    }
                    _ => { // ALL or NONE
                        if p.rotation == 0.0 {
                            v3_up = -camera_cross;
                            v3_cross = camera_up;
                        } else {
                            let rot_rad = p.rotation.to_radians();
                            let q = Quat::from_axis_angle(camera_view, rot_rad);
                            v3_up = q * (-camera_cross);
                            v3_cross = q * camera_up;
                        }
                    }
                }
            } else {
                v3_up = p.position - p.last_position;
                if let Some(mat) = self.local_matrix {
                    // D3DXVec3TransformNormal in C++
                    v3_up = Mat4::from_cols(mat.col(0), mat.col(1), mat.col(2), Vec4::new(0.0, 0.0, 0.0, 1.0)).transform_vector3(v3_up);
                }
                
                let length = v3_up.length();
                if length == 0.0 {
                    v3_up = Vec3::new(0.0, 0.0, 1.0);
                } else {
                    v3_up *= (1.0 + (1.0 + length).ln()) / length;
                }
                
                v3_cross = v3_up.cross(camera_view).normalize();
            }

            v3_cross = -(p.half_size.x * p.scale.x) * v3_cross;
            v3_up = (p.half_size.y * p.scale.y) * v3_up;

            let mut final_pos = p.position;
            if let (Some(mat), true) = (self.local_matrix, self.particle_property.attach_flag) {
                final_pos = mat.transform_point3(final_pos);
            }

            let diffuse = pack_color(p.color);

            vertices.push(ParticleVertex {
                position: (final_pos - v3_up + v3_cross).into(),
                diffuse,
                tex_coord: [0.0, 1.0],
            });
            vertices.push(ParticleVertex {
                position: (final_pos - v3_up - v3_cross).into(),
                diffuse,
                tex_coord: [0.0, 0.0],
            });
            vertices.push(ParticleVertex {
                position: (final_pos + v3_up + v3_cross).into(),
                diffuse,
                tex_coord: [1.0, 1.0],
            });
            vertices.push(ParticleVertex {
                position: (final_pos + v3_up - v3_cross).into(),
                diffuse,
                tex_coord: [1.0, 0.0],
            });
            
            if self.particle_property.billboard_type == BillboardType::TwoFace {
                let rot_q = Quat::from_axis_angle(v3_up, PI / 2.0); // 90 degrees
                let v3_cross_rot = rot_q * v3_cross;
                
                vertices.push(ParticleVertex {
                    position: (final_pos - v3_up + v3_cross_rot).into(),
                    diffuse,
                    tex_coord: [0.0, 1.0],
                });
                vertices.push(ParticleVertex {
                    position: (final_pos - v3_up - v3_cross_rot).into(),
                    diffuse,
                    tex_coord: [0.0, 0.0],
                });
                vertices.push(ParticleVertex {
                    position: (final_pos + v3_up + v3_cross_rot).into(),
                    diffuse,
                    tex_coord: [1.0, 1.0],
                });
                vertices.push(ParticleVertex {
                    position: (final_pos + v3_up - v3_cross_rot).into(),
                    diffuse,
                    tex_coord: [1.0, 0.0],
                });
            } else if self.particle_property.billboard_type == BillboardType::ThreeFace {
                let rot_q1 = Quat::from_axis_angle(v3_up, PI / 3.0); // 60 degrees
                let rot_q2 = Quat::from_axis_angle(v3_up, -PI / 3.0); // -60 degrees
                
                let v3_cross_rot1 = rot_q1 * v3_cross;
                let v3_cross_rot2 = rot_q2 * v3_cross;
                
                vertices.push(ParticleVertex {
                    position: (final_pos - v3_up + v3_cross_rot1).into(),
                    diffuse,
                    tex_coord: [0.0, 1.0],
                });
                vertices.push(ParticleVertex {
                    position: (final_pos - v3_up - v3_cross_rot1).into(),
                    diffuse,
                    tex_coord: [0.0, 0.0],
                });
                vertices.push(ParticleVertex {
                    position: (final_pos + v3_up + v3_cross_rot1).into(),
                    diffuse,
                    tex_coord: [1.0, 1.0],
                });
                vertices.push(ParticleVertex {
                    position: (final_pos + v3_up - v3_cross_rot1).into(),
                    diffuse,
                    tex_coord: [1.0, 0.0],
                });

                vertices.push(ParticleVertex {
                    position: (final_pos - v3_up + v3_cross_rot2).into(),
                    diffuse,
                    tex_coord: [0.0, 1.0],
                });
                vertices.push(ParticleVertex {
                    position: (final_pos - v3_up - v3_cross_rot2).into(),
                    diffuse,
                    tex_coord: [0.0, 0.0],
                });
                vertices.push(ParticleVertex {
                    position: (final_pos + v3_up + v3_cross_rot2).into(),
                    diffuse,
                    tex_coord: [1.0, 1.0],
                });
                vertices.push(ParticleVertex {
                    position: (final_pos + v3_up - v3_cross_rot2).into(),
                    diffuse,
                    tex_coord: [1.0, 0.0],
                });
            }
        }

        vertices
    }
}
