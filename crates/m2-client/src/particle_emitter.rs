#[derive(Clone, Debug, PartialEq)]
pub enum Shape {
    Point,
    Sphere { emitting_radius: f32 },
    Box { emitting_size: (f32, f32, f32) },
}

#[derive(Clone, Debug, PartialEq)]
pub struct EmitterConfig {
    pub shape: Shape,
    pub emission_rate: f32,
    pub max_emission_count: u32,
    pub cycle_length: f32,
    pub cycle_loop_flag: bool,
    pub emitting_direction: (f32, f32, f32),
    pub emitting_force: f32,
    pub emitting_angle_variance: f32,
}

pub struct ParticleEmitter {
    config: EmitterConfig,
    accumulated_time: f32,
    fractional_particles: f32,
    total_emitted: u32,
    cycle_emitted: u32,
    is_active: bool,
}

impl ParticleEmitter {
    pub fn new(config: EmitterConfig) -> Self {
        Self {
            config,
            accumulated_time: 0.0,
            fractional_particles: 0.0,
            total_emitted: 0,
            cycle_emitted: 0,
            is_active: true,
        }
    }

    pub fn update(&mut self, dt: f32) -> u32 {
        if !self.is_active || dt <= 0.0 {
            return 0;
        }

        let mut emitted_this_frame = 0;
        let mut dt_remaining = dt;

        while dt_remaining > 0.0 && self.is_active {
            let mut step_dt = dt_remaining;

            if self.config.cycle_length > 0.0 {
                let time_to_end = self.config.cycle_length - self.accumulated_time;
                if step_dt >= time_to_end {
                    step_dt = time_to_end;
                }
            }

            self.accumulated_time += step_dt;
            dt_remaining -= step_dt;

            if self.config.max_emission_count == 0 || self.cycle_emitted < self.config.max_emission_count {
                self.fractional_particles += self.config.emission_rate * step_dt;
                let mut to_emit = self.fractional_particles as u32;

                if self.config.max_emission_count > 0 {
                    let remaining = self.config.max_emission_count.saturating_sub(self.cycle_emitted);
                    if to_emit > remaining {
                        to_emit = remaining;
                        self.fractional_particles = to_emit as f32; // prevent buildup
                    }
                }

                self.fractional_particles -= to_emit as f32;
                self.cycle_emitted += to_emit;
                self.total_emitted += to_emit;
                emitted_this_frame += to_emit;
            }

            if self.config.cycle_length > 0.0 && self.accumulated_time >= self.config.cycle_length {
                if self.config.cycle_loop_flag {
                    self.accumulated_time = 0.0;
                    self.cycle_emitted = 0;
                    self.fractional_particles = 0.0;
                } else {
                    self.is_active = false;
                }
            } else if self.config.max_emission_count > 0 && self.cycle_emitted >= self.config.max_emission_count {
                if !self.config.cycle_loop_flag && self.config.cycle_length <= 0.0 {
                    self.is_active = false;
                }
            }

            if step_dt < 0.00001 {
                break;
            }
        }

        emitted_this_frame
    }

    pub fn spawn_particle_position(&self, seed: u64) -> (f32, f32, f32) {
        match &self.config.shape {
            Shape::Point => (0.0, 0.0, 0.0),
            Shape::Sphere { emitting_radius } => {
                let mut s = seed;
                let mut get_rand = || {
                    s = s.wrapping_add(0x9e3779b97f4a7c15);
                    let mut x = s;
                    x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
                    x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
                    x = x ^ (x >> 31);
                    let float_bits = 0x3F800000 | (x >> 41) as u32;
                    f32::from_bits(float_bits) - 1.0
                };

                loop {
                    let x = get_rand() * 2.0 - 1.0;
                    let y = get_rand() * 2.0 - 1.0;
                    let z = get_rand() * 2.0 - 1.0;
                    let mag2 = x * x + y * y + z * z;
                    if mag2 <= 1.0 {
                        return (
                            x * emitting_radius,
                            y * emitting_radius,
                            z * emitting_radius,
                        );
                    }
                }
            }
            Shape::Box { emitting_size } => {
                let mut s = seed;
                let mut get_rand = || {
                    s = s.wrapping_add(0x9e3779b97f4a7c15);
                    let mut x = s;
                    x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
                    x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
                    x = x ^ (x >> 31);
                    let float_bits = 0x3F800000 | (x >> 41) as u32;
                    f32::from_bits(float_bits) - 1.0
                };

                let x = (get_rand() - 0.5) * emitting_size.0;
                let y = (get_rand() - 0.5) * emitting_size.1;
                let z = (get_rand() - 0.5) * emitting_size.2;
                (x, y, z)
            }
        }
    }

    pub fn spawn_particle_velocity(&self, seed: u64) -> (f32, f32, f32) {
        let mut s = seed;
        let mut get_rand = || {
            s = s.wrapping_add(0x9e3779b97f4a7c15);
            let mut x = s;
            x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
            x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
            x = x ^ (x >> 31);
            let float_bits = 0x3F800000 | (x >> 41) as u32;
            f32::from_bits(float_bits) - 1.0
        };

        let variance = self.config.emitting_angle_variance;
        
        let dx = self.config.emitting_direction.0;
        let dy = self.config.emitting_direction.1;
        let dz = self.config.emitting_direction.2;
        
        let mut vx = dx;
        let mut vy = dy;
        let mut vz = dz;

        if variance > 0.0 {
            let offset_x = (get_rand() * 2.0 - 1.0) * variance;
            let offset_y = (get_rand() * 2.0 - 1.0) * variance;
            let offset_z = (get_rand() * 2.0 - 1.0) * variance;
            
            vx += offset_x;
            vy += offset_y;
            vz += offset_z;
            
            let mag = (vx * vx + vy * vy + vz * vz).sqrt();
            if mag > 0.0001 {
                vx /= mag;
                vy /= mag;
                vz /= mag;
            }
        }

        (
            vx * self.config.emitting_force,
            vy * self.config.emitting_force,
            vz * self.config.emitting_force,
        )
    }
}
