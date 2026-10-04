use glam::Vec3;
use rand::Rng;

pub const SNOW_DISTANCE: f32 = 70000.0;

#[derive(Debug, Clone, Copy)]
pub struct SnowParticle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub half_width: f32,
    pub half_height: f32,
    pub period: f32,
    pub cur_radian: f32,
    pub amplitude: f32,
    pub activate: bool,
    pub changed_size: bool,
    pub up: Vec3,
    pub cross: Vec3,
}

impl Default for SnowParticle {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            velocity: Vec3::ZERO,
            half_width: 0.0,
            half_height: 0.0,
            period: 0.0,
            cur_radian: 0.0,
            amplitude: 0.0,
            activate: false,
            changed_size: false,
            up: Vec3::ZERO,
            cross: Vec3::ZERO,
        }
    }
}

impl SnowParticle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn init(&mut self, pos: Vec3, rng: &mut impl Rng) {
        let rot: f32 = rng.gen_range(0.0..36000.0) / 100.0;
        let distance: f32 = rng.gen_range(0.0..SNOW_DISTANCE) / 10.0;

        let rot_rad = rot.to_radians();
        self.position.x = pos.x + distance * rot_rad.sin();
        self.position.y = pos.y + distance * rot_rad.cos();
        self.position.z = pos.z + rng.gen_range(1500.0..2000.0);
        
        self.velocity.x = 0.0;
        self.velocity.y = 0.0;
        self.velocity.z = rng.gen_range(-200.0..-50.0); // Note: gen_range requires low < high
        
        self.half_width = rng.gen_range(2.0..7.0);
        self.half_height = self.half_width;
        
        self.activate = true;
        self.changed_size = false;

        self.period = rng.gen_range(1.5..5.0);
        self.cur_radian = rng.gen_range(-1.6..1.6);
        self.amplitude = rng.gen_range(1.0..3.0);
    }

    pub fn set_camera_vertex(&mut self, up: Vec3, cross: Vec3) {
        self.up = up * self.half_width;
        self.cross = cross * self.half_height;
    }

    pub fn update(&mut self, elapsed_time: f32, center_pos: Vec3) {
        self.position += self.velocity * elapsed_time;

        self.position.x += self.cross.x * self.cur_radian.sin() / 10.0;
        self.position.y += self.cross.y * self.cur_radian.sin() / 10.0;
        self.cur_radian += self.period * elapsed_time;

        if self.position.z < center_pos.z - 500.0 {
            self.activate = false;
        } else if (self.position.x - center_pos.x).abs() > SNOW_DISTANCE {
            self.activate = false;
        } else if (self.position.y - center_pos.y).abs() > SNOW_DISTANCE {
            self.activate = false;
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstancedWeatherParticle {
    pub position: [f32; 3],
    pub up: [f32; 3],
    pub cross: [f32; 3],
}

pub struct WeatherEnvironment {
    pub particles: Vec<SnowParticle>,
    pub max_particles: usize,
    pub snow_enable: bool,
    pub center_pos: Vec3,
}

impl Default for WeatherEnvironment {
    fn default() -> Self {
        Self {
            particles: Vec::with_capacity(3000),
            max_particles: 3000,
            snow_enable: false,
            center_pos: Vec3::ZERO,
        }
    }
}

impl WeatherEnvironment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enable(&mut self) {
        self.snow_enable = true;
    }

    pub fn disable(&mut self) {
        self.snow_enable = false;
    }

    pub fn update_position(&mut self, pos: Vec3) {
        if !self.snow_enable && self.particles.is_empty() {
            return;
        }
        self.center_pos = pos;
    }

    pub fn deform(&mut self, elapsed_time: f32, view_dir: Vec3, rng: &mut impl Rng) {
        if !self.snow_enable && self.particles.is_empty() {
            return;
        }

        let mut changed_pos = self.center_pos + view_dir * 3500.0;
        changed_pos.z = self.center_pos.z;

        // Update existing
        for particle in &mut self.particles {
            particle.update(elapsed_time, changed_pos);
        }

        // Remove inactive
        self.particles.retain(|p| p.activate);

        // Spawn new if enabled
        if self.snow_enable {
            let to_spawn = (self.max_particles.saturating_sub(self.particles.len())).min(10);
            for _ in 0..to_spawn {
                let mut new_particle = SnowParticle::new();
                new_particle.init(changed_pos, rng);
                self.particles.push(new_particle);
            }
        }
    }

    pub fn generate_instance_buffer(&mut self, up: Vec3, cross: Vec3) -> Vec<InstancedWeatherParticle> {
        let mut instances = Vec::with_capacity(self.particles.len());

        for particle in &mut self.particles {
            particle.set_camera_vertex(up, cross);
            instances.push(InstancedWeatherParticle {
                position: particle.position.into(),
                up: particle.up.into(),
                cross: particle.cross.into(),
            });
        }

        instances
    }
}
