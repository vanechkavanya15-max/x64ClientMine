use glam::Vec3;
use rand::Rng;

pub const WEATHER_CUBE_RADIUS_XY: f32 = 3500.0;
pub const WEATHER_CUBE_RADIUS_Z: f32 = 2500.0;

fn wrap_around(val: &mut f32, center: f32, radius: f32) {
    if *val < center - radius {
        *val += 2.0 * radius;
    } else if *val > center + radius {
        *val -= 2.0 * radius;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SnowParticle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub half_width: f32,
    pub half_height: f32,
    pub period: f32,
    pub cur_radian: f32,
    pub amplitude: f32,
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
        self.position.x = pos.x + rng.gen_range(-WEATHER_CUBE_RADIUS_XY..=WEATHER_CUBE_RADIUS_XY);
        self.position.y = pos.y + rng.gen_range(-WEATHER_CUBE_RADIUS_XY..=WEATHER_CUBE_RADIUS_XY);
        self.position.z = pos.z + rng.gen_range(-WEATHER_CUBE_RADIUS_Z..=WEATHER_CUBE_RADIUS_Z);
        
        self.velocity.x = 0.0;
        self.velocity.y = 0.0;
        self.velocity.z = rng.gen_range(-200.0..=-50.0);
        
        self.half_width = rng.gen_range(2.0..=7.0);
        self.half_height = self.half_width;
        
        self.period = rng.gen_range(1.5..=5.0);
        self.cur_radian = rng.gen_range(-1.6..=1.6);
        self.amplitude = rng.gen_range(1.0..=3.0);
    }

    pub fn set_camera_vertex(&mut self, up: Vec3, cross: Vec3) {
        self.up = up * self.half_width;
        self.cross = cross * self.half_height;
    }

    pub fn update(&mut self, elapsed_time: f32, center_pos: Vec3, wind: Vec3) {
        let mut total_velocity = self.velocity + wind;
        
        total_velocity.x += self.cross.x * self.cur_radian.sin() / 10.0;
        total_velocity.y += self.cross.y * self.cur_radian.sin() / 10.0;
        self.cur_radian += self.period * elapsed_time;

        self.position += total_velocity * elapsed_time;

        wrap_around(&mut self.position.x, center_pos.x, WEATHER_CUBE_RADIUS_XY);
        wrap_around(&mut self.position.y, center_pos.y, WEATHER_CUBE_RADIUS_XY);
        wrap_around(&mut self.position.z, center_pos.z, WEATHER_CUBE_RADIUS_Z);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RainParticle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub half_width: f32,
    pub half_height: f32,
    pub up: Vec3,
    pub cross: Vec3,
}

impl Default for RainParticle {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            velocity: Vec3::ZERO,
            half_width: 0.0,
            half_height: 0.0,
            up: Vec3::ZERO,
            cross: Vec3::ZERO,
        }
    }
}

impl RainParticle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn init(&mut self, pos: Vec3, rng: &mut impl Rng) {
        self.position.x = pos.x + rng.gen_range(-WEATHER_CUBE_RADIUS_XY..=WEATHER_CUBE_RADIUS_XY);
        self.position.y = pos.y + rng.gen_range(-WEATHER_CUBE_RADIUS_XY..=WEATHER_CUBE_RADIUS_XY);
        self.position.z = pos.z + rng.gen_range(-WEATHER_CUBE_RADIUS_Z..=WEATHER_CUBE_RADIUS_Z);
        
        self.velocity.x = 0.0;
        self.velocity.y = 0.0;
        self.velocity.z = rng.gen_range(-1500.0..=-800.0);
        
        self.half_width = rng.gen_range(0.5..=1.5);
        self.half_height = rng.gen_range(10.0..=25.0);
    }

    pub fn set_camera_vertex(&mut self, _up: Vec3, cross: Vec3, wind: Vec3) {
        let dir = (self.velocity + wind).normalize_or_zero();
        self.up = -dir * self.half_height; 
        self.cross = cross * self.half_width;
    }

    pub fn update(&mut self, elapsed_time: f32, center_pos: Vec3, wind: Vec3) {
        let total_velocity = self.velocity + wind;
        self.position += total_velocity * elapsed_time;

        wrap_around(&mut self.position.x, center_pos.x, WEATHER_CUBE_RADIUS_XY);
        wrap_around(&mut self.position.y, center_pos.y, WEATHER_CUBE_RADIUS_XY);
        wrap_around(&mut self.position.z, center_pos.z, WEATHER_CUBE_RADIUS_Z);
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
    pub snow_particles: Vec<SnowParticle>,
    pub rain_particles: Vec<RainParticle>,
    pub max_snow: usize,
    pub max_rain: usize,
    pub snow_enable: bool,
    pub rain_enable: bool,
    pub center_pos: Vec3,
    pub wind: Vec3,
}

impl Default for WeatherEnvironment {
    fn default() -> Self {
        Self {
            snow_particles: Vec::with_capacity(3000),
            rain_particles: Vec::with_capacity(3000),
            max_snow: 3000,
            max_rain: 3000,
            snow_enable: false,
            rain_enable: false,
            center_pos: Vec3::ZERO,
            wind: Vec3::ZERO,
        }
    }
}

impl WeatherEnvironment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enable_snow(&mut self) {
        self.snow_enable = true;
    }

    pub fn disable_snow(&mut self) {
        self.snow_enable = false;
        self.snow_particles.clear();
    }

    pub fn enable_rain(&mut self) {
        self.rain_enable = true;
    }

    pub fn disable_rain(&mut self) {
        self.rain_enable = false;
        self.rain_particles.clear();
    }

    pub fn set_wind(&mut self, wind: Vec3) {
        self.wind = wind;
    }

    pub fn update_position(&mut self, pos: Vec3) {
        if !self.snow_enable && !self.rain_enable {
            return;
        }
        self.center_pos = pos;
    }

    pub fn deform(&mut self, elapsed_time: f32, view_dir: Vec3, rng: &mut impl Rng) {
        if !self.snow_enable && !self.rain_enable {
            return;
        }

        let mut changed_pos = self.center_pos + view_dir * (WEATHER_CUBE_RADIUS_XY / 2.0);
        changed_pos.z = self.center_pos.z;

        if self.snow_enable {
            let to_spawn = (self.max_snow.saturating_sub(self.snow_particles.len())).min(50);
            for _ in 0..to_spawn {
                let mut p = SnowParticle::new();
                p.init(changed_pos, rng);
                self.snow_particles.push(p);
            }
            for p in &mut self.snow_particles {
                p.update(elapsed_time, changed_pos, self.wind);
            }
        }

        if self.rain_enable {
            let to_spawn = (self.max_rain.saturating_sub(self.rain_particles.len())).min(50);
            for _ in 0..to_spawn {
                let mut p = RainParticle::new();
                p.init(changed_pos, rng);
                self.rain_particles.push(p);
            }
            for p in &mut self.rain_particles {
                p.update(elapsed_time, changed_pos, self.wind);
            }
        }
    }

    pub fn generate_snow_instance_buffer(&mut self, up: Vec3, cross: Vec3) -> Vec<InstancedWeatherParticle> {
        let mut instances = Vec::with_capacity(self.snow_particles.len());
        for p in &mut self.snow_particles {
            p.set_camera_vertex(up, cross);
            instances.push(InstancedWeatherParticle {
                position: p.position.into(),
                up: p.up.into(),
                cross: p.cross.into(),
            });
        }
        instances
    }

    pub fn generate_rain_instance_buffer(&mut self, up: Vec3, cross: Vec3) -> Vec<InstancedWeatherParticle> {
        let mut instances = Vec::with_capacity(self.rain_particles.len());
        for p in &mut self.rain_particles {
            p.set_camera_vertex(up, cross, self.wind);
            instances.push(InstancedWeatherParticle {
                position: p.position.into(),
                up: p.up.into(),
                cross: p.cross.into(),
            });
        }
        instances
    }
}
