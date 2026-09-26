use glam::Vec3;
use m2_client_bridge::ffi_guard::safe_ffi_call;
use std::sync::{Mutex, OnceLock};
use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub struct HitSphere {
    pub start_time: f32,
    pub end_time: f32,
    pub radius: f32,
    pub cone_angle_cos: f32,
    pub position: Vec3,
    pub direction: Vec3,
}

impl HitSphere {
    pub fn parse_from_msa(content: &str) -> Vec<HitSphere> {
        let mut spheres = Vec::new();
        
        let mut current_sphere = HitSphere {
            start_time: 0.0,
            end_time: 0.0,
            radius: 0.0,
            cone_angle_cos: -1.0, // Domyślnie 180 stopni (pełny obrót)
            position: Vec3::ZERO,
            direction: Vec3::Y,
        };
        let mut in_sphere = false;

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if line == "HitSphere" || line == "Group HitSphere" || line.starts_with("HitData") {
                if in_sphere {
                    spheres.push(current_sphere.clone());
                }
                in_sphere = true;
                current_sphere = HitSphere {
                    start_time: 0.0,
                    end_time: 0.0,
                    radius: 0.0,
                    cone_angle_cos: -1.0,
                    position: Vec3::ZERO,
                    direction: Vec3::Y,
                };
            } else if in_sphere {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.is_empty() {
                    continue;
                }
                match parts[0] {
                    "StartTime" => {
                        if parts.len() > 1 {
                            current_sphere.start_time = parts[1].parse().unwrap_or(0.0);
                        }
                    }
                    "EndTime" => {
                        if parts.len() > 1 {
                            current_sphere.end_time = parts[1].parse().unwrap_or(0.0);
                        }
                    }
                    "Radius" => {
                        if parts.len() > 1 {
                            current_sphere.radius = parts[1].parse().unwrap_or(0.0);
                        }
                    }
                    "ConeAngle" => {
                        if parts.len() > 1 {
                            let angle_deg: f32 = parts[1].parse().unwrap_or(180.0);
                            current_sphere.cone_angle_cos = (angle_deg * PI / 180.0 / 2.0).cos();
                        }
                    }
                    "Position" => {
                        if parts.len() > 3 {
                            current_sphere.position = Vec3::new(
                                parts[1].parse().unwrap_or(0.0),
                                parts[2].parse().unwrap_or(0.0),
                                parts[3].parse().unwrap_or(0.0),
                            );
                        }
                    }
                    "Direction" => {
                        if parts.len() > 3 {
                            current_sphere.direction = Vec3::new(
                                parts[1].parse().unwrap_or(0.0),
                                parts[2].parse().unwrap_or(0.0),
                                parts[3].parse().unwrap_or(0.0),
                            ).normalize_or_zero();
                        }
                    }
                    _ => {}
                }
            }
        }
        if in_sphere {
            spheres.push(current_sphere);
        }
        spheres
    }

    pub fn is_hit_moment(&self, anim_time: f32) -> bool {
        // Zgodnie z logika z C++ sprawdzamy czy czas jest pomiedzy (lub na rowni)
        anim_time >= self.start_time && anim_time <= self.end_time
    }

    pub fn is_target_in_cone_and_range(&self, target_pos: Vec3, actor_pos: Vec3) -> bool {
        let absolute_sphere_pos = actor_pos + self.position;
        let diff = target_pos - absolute_sphere_pos;
        let dist_sq = diff.length_squared();
        
        // Zasięg (range)
        if dist_sq > self.radius * self.radius {
            return false; 
        }
        
        // Stożek (cone)
        if dist_sq > 0.0 {
            let dist = dist_sq.sqrt();
            let dir_to_target = diff / dist;
            let dot = self.direction.dot(dir_to_target);
            if dot < self.cone_angle_cos {
                return false; 
            }
        }
        
        true
    }
}

pub struct SplashAttackStateV2 {
    pub hit_spheres: Vec<HitSphere>,
}

static SPLASH_ATTACK_STATE_V2: OnceLock<Mutex<SplashAttackStateV2>> = OnceLock::new();

fn get_state() -> &'static Mutex<SplashAttackStateV2> {
    SPLASH_ATTACK_STATE_V2.get_or_init(|| {
        Mutex::new(SplashAttackStateV2 {
            hit_spheres: Vec::new(),
        })
    })
}

#[no_mangle]
pub extern "C" fn m2_motion_is_hit_moment(anim_time: f32) -> bool {
    safe_ffi_call(false, || {
        let state = get_state().lock().unwrap();
        for sphere in &state.hit_spheres {
            if sphere.is_hit_moment(anim_time) {
                return true;
            }
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_splash_v2_load_msa(msa_content: *const std::ffi::c_char) -> bool {
    safe_ffi_call(false, || {
        if msa_content.is_null() {
            return false;
        }
        let c_str = unsafe { std::ffi::CStr::from_ptr(msa_content) };
        if let Ok(content) = c_str.to_str() {
            let mut state = get_state().lock().unwrap();
            state.hit_spheres = HitSphere::parse_from_msa(content);
            return true;
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_splash_v2_clear_spheres() {
    safe_ffi_call((), || {
        let mut state = get_state().lock().unwrap();
        state.hit_spheres.clear();
    });
}

#[no_mangle]
pub extern "C" fn m2_splash_v2_check_target(
    anim_time: f32,
    target_x: f32,
    target_y: f32,
    target_z: f32,
    actor_x: f32,
    actor_y: f32,
    actor_z: f32,
) -> bool {
    safe_ffi_call(false, || {
        let state = get_state().lock().unwrap();
        let target_pos = Vec3::new(target_x, target_y, target_z);
        let actor_pos = Vec3::new(actor_x, actor_y, actor_z);

        for sphere in &state.hit_spheres {
            if sphere.is_hit_moment(anim_time) {
                if sphere.is_target_in_cone_and_range(target_pos, actor_pos) {
                    return true;
                }
            }
        }
        false
    })
}
