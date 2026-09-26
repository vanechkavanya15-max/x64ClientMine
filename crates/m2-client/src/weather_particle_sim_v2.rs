use std::sync::Mutex;
use std::sync::OnceLock;
use glam::Vec3;

const MAX_PARTICLES: usize = 2000;
const BOUNDING_BOX_X: f32 = 4000.0;
const BOUNDING_BOX_Y: f32 = 4000.0;
const BOUNDING_BOX_Z: f32 = 2000.0;

#[derive(Clone, Copy, Debug)]
pub struct WeatherParticle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub active: bool,
}

pub struct WeatherSimulation {
    pub particles: Vec<WeatherParticle>,
    pub wind: Vec3,
    pub camera_pos: Vec3,
}

impl WeatherSimulation {
    pub fn new() -> Self {
        Self {
            particles: Vec::with_capacity(MAX_PARTICLES),
            wind: Vec3::new(0.0, 0.0, 0.0),
            camera_pos: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn set_camera(&mut self, pos: Vec3) {
        self.camera_pos = pos;
    }

    pub fn set_wind(&mut self, wind: Vec3) {
        self.wind = wind;
    }

    pub fn init_particles(&mut self) {
        while self.particles.len() < MAX_PARTICLES {
            self.particles.push(WeatherParticle {
                position: Vec3::new(0.0, 0.0, 0.0), // Need properly distributed initialization
                velocity: Vec3::new(0.0, 0.0, 0.0),
                active: false,
            });
        }
    }

    pub fn spawn_particle(&mut self, i: usize) {
        let half_x = BOUNDING_BOX_X / 2.0;
        let half_y = BOUNDING_BOX_Y / 2.0;
        let half_z = BOUNDING_BOX_Z / 2.0;
        
        // Use a simple deterministic psuedo-random to avoid relying on external crates not guaranteed to exist
        let r1 = ((i * 12345) % 1000) as f32 / 1000.0;
        let r2 = ((i * 54321) % 1000) as f32 / 1000.0;
        let r3 = ((i * 98765) % 1000) as f32 / 1000.0;

        self.particles[i].position = Vec3::new(
            self.camera_pos.x + (r1 * BOUNDING_BOX_X - half_x),
            self.camera_pos.y + (r2 * BOUNDING_BOX_Y - half_y),
            self.camera_pos.z + (r3 * BOUNDING_BOX_Z - half_z),
        );
        self.particles[i].velocity = Vec3::new(0.0, 0.0, -150.0); // Default gravity fall speed
        self.particles[i].active = true;
    }

    pub fn update(&mut self, dt: f32) -> usize {
        let half_x = BOUNDING_BOX_X / 2.0;
        let half_y = BOUNDING_BOX_Y / 2.0;
        let half_z = BOUNDING_BOX_Z / 2.0;
        
        let mut active_count = 0;

        for i in 0..self.particles.len() {
            if !self.particles[i].active {
                self.spawn_particle(i);
            }

            if self.particles[i].active {
                // Apply fall gravity and wind drift
                self.particles[i].position.z += self.particles[i].velocity.z * dt;
                self.particles[i].position.x += self.wind.x * dt;
                self.particles[i].position.y += self.wind.y * dt;

                // Wrap-around bounds checks relative to camera
                let dx = self.particles[i].position.x - self.camera_pos.x;
                let dy = self.particles[i].position.y - self.camera_pos.y;
                let dz = self.particles[i].position.z - self.camera_pos.z;

                if dx > half_x {
                    self.particles[i].position.x -= BOUNDING_BOX_X;
                } else if dx < -half_x {
                    self.particles[i].position.x += BOUNDING_BOX_X;
                }

                if dy > half_y {
                    self.particles[i].position.y -= BOUNDING_BOX_Y;
                } else if dy < -half_y {
                    self.particles[i].position.y += BOUNDING_BOX_Y;
                }

                if dz < -half_z { // Reached bottom
                    self.particles[i].position.z += BOUNDING_BOX_Z;
                } else if dz > half_z {
                    self.particles[i].position.z -= BOUNDING_BOX_Z;
                }

                active_count += 1;
            }
        }
        
        active_count
    }
}

static WEATHER_SIMULATION: OnceLock<Mutex<WeatherSimulation>> = OnceLock::new();

pub fn get_weather_sim() -> &'static Mutex<WeatherSimulation> {
    WEATHER_SIMULATION.get_or_init(|| Mutex::new(WeatherSimulation::new()))
}

#[no_mangle]
pub extern "C" fn m2_weather_update(dt: f32) -> usize {
    let mut sim = get_weather_sim().lock().unwrap();
    if sim.particles.is_empty() {
        sim.init_particles();
    }
    sim.update(dt)
}

#[no_mangle]
pub extern "C" fn m2_weather_set_camera(x: f32, y: f32, z: f32) {
    let mut sim = get_weather_sim().lock().unwrap();
    sim.set_camera(Vec3::new(x, y, z));
}

#[no_mangle]
pub extern "C" fn m2_weather_set_wind(x: f32, y: f32, z: f32) {
    let mut sim = get_weather_sim().lock().unwrap();
    sim.set_wind(Vec3::new(x, y, z));
}

#[no_mangle]
pub extern "C" fn m2_weather_reset() {
    let mut sim = get_weather_sim().lock().unwrap();
    sim.particles.clear();
}
