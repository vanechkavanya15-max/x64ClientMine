use glam::{Vec2, Vec3, Vec4};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimeEvent<T> {
    pub time: f32,
    pub value: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmitterShape {
    Point = 0,
    Ellipse = 1,
    Square = 2,
    Sphere = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmitterAdvancedType {
    Free = 0,
    Outer = 1,
    Inner = 2,
}

#[derive(Debug, Clone)]
pub struct EmitterProperty {
    pub max_emission_count: u32,
    pub cycle_length: f32,
    pub cycle_loop_flag: bool,
    pub loop_count: i32,
    pub emitter_shape: EmitterShape,
    pub emitter_advanced_type: EmitterAdvancedType,
    
    pub emitting_size: Vec3,
    pub emitting_radius: f32,
    pub emit_from_edge_flag: bool,
    pub emitting_direction: Vec3,

    pub time_event_emitting_size: Vec<TimeEvent<f32>>,
    pub time_event_emitting_angular_velocity: Vec<TimeEvent<f32>>,
    pub time_event_emitting_direction_x: Vec<TimeEvent<f32>>,
    pub time_event_emitting_direction_y: Vec<TimeEvent<f32>>,
    pub time_event_emitting_direction_z: Vec<TimeEvent<f32>>,
    pub time_event_emitting_velocity: Vec<TimeEvent<f32>>,
    
    pub time_event_emission_count_per_second: Vec<TimeEvent<f32>>,
    pub time_event_life_time: Vec<TimeEvent<f32>>,
    pub time_event_size_x: Vec<TimeEvent<f32>>,
    pub time_event_size_y: Vec<TimeEvent<f32>>,
}

impl Default for EmitterProperty {
    fn default() -> Self {
        Self {
            max_emission_count: 0,
            cycle_length: 0.05,
            cycle_loop_flag: false,
            loop_count: 0,
            emitter_shape: EmitterShape::Point,
            emitter_advanced_type: EmitterAdvancedType::Free,
            emitting_size: Vec3::ZERO,
            emitting_radius: 0.0,
            emit_from_edge_flag: false,
            emitting_direction: Vec3::ZERO,
            time_event_emitting_size: vec![TimeEvent { time: 0.0, value: 0.0 }],
            time_event_emitting_angular_velocity: vec![TimeEvent { time: 0.0, value: 0.0 }],
            time_event_emitting_direction_x: vec![TimeEvent { time: 0.0, value: 0.0 }],
            time_event_emitting_direction_y: vec![TimeEvent { time: 0.0, value: 0.0 }],
            time_event_emitting_direction_z: vec![TimeEvent { time: 0.0, value: 0.0 }],
            time_event_emitting_velocity: vec![TimeEvent { time: 0.0, value: 0.0 }],
            time_event_emission_count_per_second: Vec::new(),
            time_event_life_time: Vec::new(),
            time_event_size_x: Vec::new(),
            time_event_size_y: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotationType {
    None = 0,
    TimeEvent = 1,
    Cw = 2,
    Ccw = 3,
    RandomDirection = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureAnimationType {
    None = 0,
    Cw = 1,
    Ccw = 2,
    RandomFrame = 3,
    RandomDirection = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BillboardType {
    None = 0,
    All = 1,
    Y = 2,
    Lie = 3,
    TwoFace = 4,
    ThreeFace = 5,
}

#[derive(Debug, Clone)]
pub struct ParticleProperty {
    pub src_blend_type: u8,
    pub dest_blend_type: u8,
    pub color_operation_type: u8,

    pub billboard_type: BillboardType,
    pub rotation_type: RotationType,
    
    pub rotation_speed: f32,
    pub rotation_random_starting_begin: u16,
    pub rotation_random_starting_end: u16,

    pub attach_flag: bool,
    pub stretch_flag: bool,
    
    pub tex_ani_type: TextureAnimationType,
    pub tex_ani_delay: f32,
    pub tex_ani_random_start_frame_flag: bool,

    pub time_event_gravity: Vec<TimeEvent<f32>>,
    pub time_event_air_resistance: Vec<TimeEvent<f32>>,
    pub time_event_scale_x: Vec<TimeEvent<f32>>,
    pub time_event_scale_y: Vec<TimeEvent<f32>>,
    
    pub time_event_color: Vec<TimeEvent<Vec4>>,
    pub time_event_rotation: Vec<TimeEvent<f32>>,
    
    pub textures: Vec<String>,
}

impl Default for ParticleProperty {
    fn default() -> Self {
        Self {
            src_blend_type: 5, // D3DBLEND_SRCALPHA
            dest_blend_type: 2, // D3DBLEND_ONE
            color_operation_type: 4, // D3DTOP_MODULATE
            billboard_type: BillboardType::None,
            rotation_type: RotationType::None,
            rotation_speed: 0.0,
            rotation_random_starting_begin: 0,
            rotation_random_starting_end: 0,
            attach_flag: false,
            stretch_flag: false,
            tex_ani_type: TextureAnimationType::None,
            tex_ani_delay: 0.0,
            tex_ani_random_start_frame_flag: false,
            time_event_gravity: Vec::new(),
            time_event_air_resistance: Vec::new(),
            time_event_scale_x: Vec::new(),
            time_event_scale_y: Vec::new(),
            time_event_color: Vec::new(),
            time_event_rotation: Vec::new(),
            textures: Vec::new(),
        }
    }
}

pub trait Blendable {
    fn blend(self, other: Self, percentage: f32) -> Self;
}

impl Blendable for f32 {
    fn blend(self, other: Self, percentage: f32) -> Self {
        self + (other - self) * percentage
    }
}

impl Blendable for Vec4 {
    fn blend(self, other: Self, percentage: f32) -> Self {
        self.lerp(other, percentage)
    }
}

impl Blendable for Vec3 {
    fn blend(self, other: Self, percentage: f32) -> Self {
        self.lerp(other, percentage)
    }
}

pub fn get_time_event_blend_value<T: Blendable + Copy>(time: f32, vec: &[TimeEvent<T>]) -> T {
    if vec.is_empty() {
        // Technically shouldn't be reached if parsing validates, but we need a default.
        // Returning the first element or a default value would require Default bound.
        // Let's assume callers provide at least one element or we return a placeholder 
        // if we can't. To avoid T: Default, we assume vec is not empty.
        panic!("TimeEvent vector is empty");
    }

    if vec.len() == 1 {
        return vec[0].value;
    }

    if time < vec[0].time {
        return vec[0].value;
    }

    let last = vec.last().unwrap();
    if time > last.time {
        return last.value;
    }

    match vec.binary_search_by(|event| event.time.partial_cmp(&time).unwrap()) {
        Ok(index) => vec[index].value,
        Err(index) => {
            if index == 0 {
                vec[0].value
            } else if index == vec.len() {
                vec[vec.len() - 1].value
            } else {
                let low = &vec[index - 1];
                let high = &vec[index];
                
                let time_diff = high.time - low.time;
                let perc = (time - low.time) / time_diff;
                
                low.value.blend(high.value, perc)
            }
        }
    }
}

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ParticleSystemData {
    pub emitter_property: EmitterProperty,
    pub particle_property: ParticleProperty,
}

impl ParticleSystemData {
    pub fn new() -> Self {
        Self {
            emitter_property: EmitterProperty::default(),
            particle_property: ParticleProperty::default(),
        }
    }
}

impl ParticleSystemData {
    pub fn parse(script: &str) -> Option<Self> {
        let mut data = Self::new();
        
        // This is a minimal tokenizer just to extract key-value pairs for properties.
        // In real use cases, Metin2 text file loader reads nested nodes.
        // For our tests, we will parse flat key-values to populate graphs.
        
        let lines: Vec<&str> = script.lines().collect();
        for mut line in lines {
            line = line.trim();
            if line.is_empty() || line.starts_with("//") {
                continue;
            }
            
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }
            
            let key = parts[0].to_lowercase();
            
            // Helper closure to parse a graph (time, value) sequence
            let parse_time_event_f32 = |parts: &[&str]| -> Vec<TimeEvent<f32>> {
                let mut vec = Vec::new();
                for i in (1..parts.len()).step_by(2) {
                    if i + 1 < parts.len() {
                        if let (Ok(time), Ok(val)) = (parts[i].parse::<f32>(), parts[i+1].parse::<f32>()) {
                            vec.push(TimeEvent { time, value: val });
                        }
                    }
                }
                vec
            };

            // Emitter properties
            if key == "maxemissioncount" && parts.len() > 1 {
                if let Ok(val) = parts[1].parse() { data.emitter_property.max_emission_count = val; }
            } else if key == "cyclelength" && parts.len() > 1 {
                if let Ok(val) = parts[1].parse() { data.emitter_property.cycle_length = val; }
            } else if key == "timeeventemittingvelocity" {
                data.emitter_property.time_event_emitting_velocity = parse_time_event_f32(&parts);
            } else if key == "timeeventemissioncountpersecond" {
                data.emitter_property.time_event_emission_count_per_second = parse_time_event_f32(&parts);
            } else if key == "timeeventlifetime" {
                data.emitter_property.time_event_life_time = parse_time_event_f32(&parts);
            } else if key == "timeeventsizex" {
                data.emitter_property.time_event_size_x = parse_time_event_f32(&parts);
            } else if key == "timeeventsizey" {
                data.emitter_property.time_event_size_y = parse_time_event_f32(&parts);
            }
            // Particle properties
            else if key == "billboardtype" && parts.len() > 1 {
                if let Ok(val) = parts[1].parse::<u8>() {
                    data.particle_property.billboard_type = match val {
                        1 => BillboardType::All,
                        2 => BillboardType::Y,
                        3 => BillboardType::Lie,
                        4 => BillboardType::TwoFace,
                        5 => BillboardType::ThreeFace,
                        _ => BillboardType::None,
                    };
                }
            } else if key == "gravity" && parts.len() > 1 {
                if let Ok(val) = parts[1].parse::<f32>() {
                    data.particle_property.time_event_gravity.push(TimeEvent { time: 0.0, value: val });
                }
            } else if key == "timeeventgravity" {
                data.particle_property.time_event_gravity = parse_time_event_f32(&parts);
            } else if key == "airresistance" && parts.len() > 1 {
                if let Ok(val) = parts[1].parse::<f32>() {
                    data.particle_property.time_event_air_resistance.push(TimeEvent { time: 0.0, value: val });
                }
            } else if key == "timeeventairresistance" {
                data.particle_property.time_event_air_resistance = parse_time_event_f32(&parts);
            } else if key == "timeeventscalex" {
                data.particle_property.time_event_scale_x = parse_time_event_f32(&parts);
            } else if key == "timeeventscaley" {
                data.particle_property.time_event_scale_y = parse_time_event_f32(&parts);
            }
            
                    else if key == "timeeventcolor" { data.parse_color_and_update(&parts); }
        }
        
        Some(data)
    }
}

impl ParticleSystemData {
    // Note: In C++ TimeEventColor is stored as 4 separate sequences (R, G, B, A)
    // and then combined into a single sequence of Vec4.
    // For simplicity in testing, we can parse `timeeventcolor` directly as:
    // time r g b a time r g b a...
    // Or support `timeeventcolorred` etc. if needed by tests.
}

impl ParticleSystemData {
    fn parse_time_event_color(parts: &[&str]) -> Vec<TimeEvent<Vec4>> {
        let mut vec = Vec::new();
        // format: time r g b a
        let mut i = 1;
        while i + 4 < parts.len() {
            if let (Ok(time), Ok(r), Ok(g), Ok(b), Ok(a)) = (
                parts[i].parse::<f32>(),
                parts[i+1].parse::<f32>(),
                parts[i+2].parse::<f32>(),
                parts[i+3].parse::<f32>(),
                parts[i+4].parse::<f32>(),
            ) {
                vec.push(TimeEvent { time, value: Vec4::new(r, g, b, a) });
            }
            i += 5;
        }
        vec
    }
}

impl ParticleSystemData {
    pub fn parse_color_and_update(&mut self, parts: &[&str]) {
        self.particle_property.time_event_color = Self::parse_time_event_color(parts);
    }
}


pub fn m2_particle_physics_step(
    position: &mut Vec3,
    velocity: &mut Vec3,
    elapsed_time: f32,
    gravity: f32,
    air_resistance: f32,
) {
    // Air resistance is applied first or simultaneously.
    // In C++ they update:
    // UpdateGravity: m_v3Velocity.z -= fGravity * elapsedTime;
    // UpdateAirResistance: m_v3Velocity *= 1.0f - airResistance;
    // position += velocity * fElapsedTime; // Actually in m2_particle_physics_step

    velocity.z -= gravity * elapsed_time;
    *velocity *= 1.0 - air_resistance;
    
    *position += *velocity * elapsed_time;
}

pub struct ParticleMesh {
    pub position: Vec3,
    pub tex_coord: Vec2,
}

pub fn calculate_billboard_transform(
    position: Vec3,
    last_position: Vec3,
    rotation: f32, // degrees
    half_size: Vec2,
    scale: Vec2,
    billboard_type: BillboardType,
    camera_view: glam::Mat4,
    camera_up: Vec3,
    camera_cross: Vec3, // In C++ c_rv3Cross is Right vector
    stretch_flag: bool,
) -> [ParticleMesh; 4] {
    let mut up = Vec3::ZERO;
    let mut cross = Vec3::ZERO;

    if !stretch_flag {
        match billboard_type {
            BillboardType::Lie => {
                let f_cos = f32::cos(rotation.to_radians());
                let f_sin = f32::sin(rotation.to_radians());
                up = Vec3::new(f_cos, -f_sin, 0.0);
                cross = Vec3::new(f_sin, f_cos, 0.0);
            }
            BillboardType::TwoFace | BillboardType::ThreeFace | BillboardType::Y => {
                up = Vec3::new(0.0, 0.0, 1.0);
                let cam_view_dir = Vec3::new(camera_view.z_axis.x, camera_view.z_axis.y, camera_view.z_axis.z).normalize_or_zero();
                
                // In C++ it cross checks against view vector
                let mut d3dd = Vec3::new(cam_view_dir.x, cam_view_dir.y, 0.0);
                if up.x * cam_view_dir.y - up.y * cam_view_dir.x < 0.0 {
                    up *= -1.0;
                }
                
                cross = up.cross(d3dd).normalize_or_zero();

                if rotation != 0.0 {
                    let f_cos = f32::cos((rotation + 90.0).to_radians()); // Note: C++ had -sin for cos, but this reflects logic. Actually we just use Quat.
                    // For now, simpler rotation logic reflecting Y axis
                    let quat = glam::Quat::from_axis_angle(cam_view_dir, rotation.to_radians());
                    up = quat.mul_vec3(up);
                    cross = quat.mul_vec3(cross);
                }
            }
            _ => { // ALL
                if rotation == 0.0 {
                    up = -camera_cross;
                    cross = camera_up;
                } else {
                    let cam_view_dir = Vec3::new(camera_view.z_axis.x, camera_view.z_axis.y, camera_view.z_axis.z).normalize_or_zero();
                    let quat = glam::Quat::from_axis_angle(cam_view_dir, rotation.to_radians());
                    
                    up = quat.mul_vec3(-camera_cross);
                    cross = quat.mul_vec3(camera_up);
                }
            }
        }
    } else {
        up = position - last_position;
        let length = up.length();
        if length == 0.0 {
            up = Vec3::new(0.0, 0.0, 1.0);
        } else {
            up *= (1.0 + f32::ln(1.0 + length)) / length;
        }

        let cam_view_dir = Vec3::new(camera_view.z_axis.x, camera_view.z_axis.y, camera_view.z_axis.z).normalize_or_zero();
        cross = up.cross(cam_view_dir).normalize_or_zero();
    }

    cross = -(half_size.x * scale.x) * cross;
    up = (half_size.y * scale.y) * up;

    // Output vertex quad order: top-left, bottom-left, top-right, bottom-right
    [
        ParticleMesh { position: position - up + cross, tex_coord: Vec2::new(0.0, 1.0) },
        ParticleMesh { position: position - up - cross, tex_coord: Vec2::new(0.0, 0.0) },
        ParticleMesh { position: position + up + cross, tex_coord: Vec2::new(1.0, 1.0) },
        ParticleMesh { position: position + up - cross, tex_coord: Vec2::new(1.0, 0.0) },
    ]
}
