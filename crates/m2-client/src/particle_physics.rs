#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Particle {
    pub position: Vector3,
    pub velocity: Vector3,
    pub rotation: f32,
    pub rotation_speed: f32,
    pub is_dead: bool,
}

impl Particle {
    pub fn new(position: Vector3, velocity: Vector3, rotation: f32, rotation_speed: f32) -> Self {
        Self {
            position,
            velocity,
            rotation,
            rotation_speed,
            is_dead: false,
        }
    }
}

pub struct ParticlePhysicsEngine;

impl ParticlePhysicsEngine {
    pub fn update(particles: &mut Vec<Particle>, dt: f32, gravity: f32, air_res: f32) {
        let mut i = 0;
        while i < particles.len() {
            if particles[i].is_dead {
                particles.swap_remove(i);
            } else {
                let p = &mut particles[i];
                
                // Air Resistance
                let air_res_factor = (1.0 - air_res * dt).clamp(0.0, 1.0);
                p.velocity.x *= air_res_factor;
                p.velocity.y *= air_res_factor;
                p.velocity.z *= air_res_factor;

                // Gravity
                p.velocity.z -= gravity * dt;

                // Position update
                p.position.x += p.velocity.x * dt;
                p.position.y += p.velocity.y * dt;
                p.position.z += p.velocity.z * dt;

                // Rotation update
                p.rotation += p.rotation_speed * dt;

                i += 1;
            }
        }
    }

    /// Aktualizacja fizyki czasteczek zgodnie z oryginalnym kodem C++ (ParticleInstance.cpp L177).
    /// W C++ tlumienie oporu powietrza to bezposredni mnoznik klatki (1.0 - air_res) bez skalowania przez dt.
    pub fn update_native_c(particles: &mut Vec<Particle>, dt: f32, gravity: f32, air_res: f32) {
        let mut i = 0;
        while i < particles.len() {
            if particles[i].is_dead {
                particles.swap_remove(i);
            } else {
                let p = &mut particles[i];
                
                let air_res_factor = (1.0 - air_res).clamp(0.0, 1.0);
                p.velocity.x *= air_res_factor;
                p.velocity.y *= air_res_factor;
                p.velocity.z *= air_res_factor;

                p.velocity.z -= gravity * dt;

                p.position.x += p.velocity.x * dt;
                p.position.y += p.velocity.y * dt;
                p.position.z += p.velocity.z * dt;

                p.rotation += p.rotation_speed * dt;

                i += 1;
            }
        }
    }
}
