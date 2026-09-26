use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use m2_client_bridge::ffi_guard::safe_ffi_call;

#[derive(Debug, Clone)]
pub struct FlyProjectile {
    pub id: u32,
    pub shooter_vid: u32,
    pub target_vid: u32,
    pub cur_pos: (f32, f32, f32),
    pub target_pos: (f32, f32, f32),
    pub speed: f32,
    pub has_hit: bool,
}

#[derive(Debug, Clone)]
pub struct EffectInstance {
    pub effect_id: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub life_time: f32,
}

pub struct EffectSystemState {
    next_effect_id: u32,
    next_flying_id: u32,
    effects: HashMap<u32, EffectInstance>,
    projectiles: HashMap<u32, FlyProjectile>,
}

impl EffectSystemState {
    pub fn new() -> Self {
        Self {
            next_effect_id: 1,
            next_flying_id: 1,
            effects: HashMap::new(),
            projectiles: HashMap::new(),
        }
    }
}

static EFFECT_STATE: OnceLock<Mutex<EffectSystemState>> = OnceLock::new();

fn get_state() -> std::sync::MutexGuard<'static, EffectSystemState> {
    EFFECT_STATE
        .get_or_init(|| Mutex::new(EffectSystemState::new()))
        .lock()
        .unwrap()
}

#[no_mangle]
pub extern "C" fn m2_effect_create(effect_id: u32, x: f32, y: f32, z: f32) -> u32 {
    safe_ffi_call(0, || {
        let mut state = get_state();
        let instance_id = state.next_effect_id;
        state.next_effect_id = state.next_effect_id.wrapping_add(1);
        if state.next_effect_id == 0 {
            state.next_effect_id = 1; // Unikamy ID 0
        }

        let instance = EffectInstance {
            effect_id,
            x,
            y,
            z,
            life_time: 0.0,
        };

        state.effects.insert(instance_id, instance);
        instance_id
    })
}

#[no_mangle]
pub extern "C" fn m2_effect_update(instance_id: u32, dt: f32) -> bool {
    safe_ffi_call(false, || {
        if instance_id == 0 {
            return false;
        }

        let mut state = get_state();
        if let Some(effect) = state.effects.get_mut(&instance_id) {
            effect.life_time += dt;
            true
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_effect_destroy(instance_id: u32) -> bool {
    safe_ffi_call(false, || {
        if instance_id == 0 {
            return false;
        }

        let mut state = get_state();
        state.effects.remove(&instance_id).is_some()
    })
}

#[no_mangle]
pub extern "C" fn m2_flying_spawn(
    shooter_vid: u32,
    target_vid: u32,
    sx: f32,
    sy: f32,
    sz: f32,
    speed: f32,
) -> u32 {
    safe_ffi_call(0, || {
        let mut state = get_state();
        let flying_id = state.next_flying_id;
        state.next_flying_id = state.next_flying_id.wrapping_add(1);
        if state.next_flying_id == 0 {
            state.next_flying_id = 1;
        }

        let target_pos = if let Some(pos) = crate::actor_ffi::get_actor_pos(target_vid) {
            pos
        } else {
            (sx, sy, sz)
        };

        let projectile = FlyProjectile {
            id: flying_id,
            shooter_vid,
            target_vid,
            cur_pos: (sx, sy, sz),
            target_pos,
            speed,
            has_hit: false,
        };

        state.projectiles.insert(flying_id, projectile);
        flying_id
    })
}

#[no_mangle]
pub extern "C" fn m2_flying_update(
    flying_id: u32,
    dt: f32,
    out_x: *mut f32,
    out_y: *mut f32,
    out_z: *mut f32,
) -> bool {
    safe_ffi_call(false, || {
        if flying_id == 0 {
            return false;
        }

        let mut state = get_state();
        
        let mut remove = false;
        let mut result = false;

        if let Some(proj) = state.projectiles.get_mut(&flying_id) {
            if proj.has_hit {
                remove = true;
            } else {
                if let Some(pos) = crate::actor_ffi::get_actor_pos(proj.target_vid) {
                    proj.target_pos = pos;
                }

                let dx = proj.target_pos.0 - proj.cur_pos.0;
                let dy = proj.target_pos.1 - proj.cur_pos.1;
                let dz = proj.target_pos.2 - proj.cur_pos.2;

                let distance_sq = dx * dx + dy * dy + dz * dz;
                let distance = distance_sq.sqrt();

                if distance < 20.0 {
                    proj.has_hit = true;
                    remove = true;
                } else {
                    let move_dist = proj.speed * dt;

                    if move_dist >= distance || distance - move_dist < 20.0 {
                        proj.cur_pos = proj.target_pos;
                        proj.has_hit = true;
                        remove = true;
                    } else {
                        let dir_x = dx / distance;
                        let dir_y = dy / distance;
                        let dir_z = dz / distance;

                        proj.cur_pos.0 += dir_x * move_dist;
                        proj.cur_pos.1 += dir_y * move_dist;
                        proj.cur_pos.2 += dir_z * move_dist;
                    }
                }

                if !out_x.is_null() {
                    unsafe { *out_x = proj.cur_pos.0 };
                }
                if !out_y.is_null() {
                    unsafe { *out_y = proj.cur_pos.1 };
                }
                if !out_z.is_null() {
                    unsafe { *out_z = proj.cur_pos.2 };
                }

                result = !proj.has_hit;
            }
        }

        if remove {
            state.projectiles.remove(&flying_id);
        }

        result
    })
}

#[no_mangle]
pub extern "C" fn m2_particle_physics_step(
    pos: *mut f32,
    vel: *mut f32,
    dt: f32,
    gravity: f32,
    air_resistance: f32,
) -> bool {
    safe_ffi_call(false, || {
        if pos.is_null() || vel.is_null() || dt <= 0.0 {
            return false;
        }
        unsafe {
            let vel_slice = std::slice::from_raw_parts_mut(vel, 3);
            let pos_slice = std::slice::from_raw_parts_mut(pos, 3);

            // 1. Grawitacja (odejmowana od osi Z)
            vel_slice[2] -= gravity * dt;

            // 2. Opor powietrza (tlumienie calego wektora predkosci)
            let air_factor = (1.0 - air_resistance).clamp(0.0, 1.0);
            vel_slice[0] *= air_factor;
            vel_slice[1] *= air_factor;
            vel_slice[2] *= air_factor;

            // 3. Calkowanie pozycji
            pos_slice[0] += vel_slice[0] * dt;
            pos_slice[1] += vel_slice[1] * dt;
            pos_slice[2] += vel_slice[2] * dt;
        }
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_flying_calculate_step(
    pos: *mut f32,
    vel: *mut f32,
    acc: *const f32,
    gravity: f32,
    dt: f32,
) -> f32 {
    safe_ffi_call(0.0f32, || {
        if pos.is_null() || vel.is_null() || dt <= 0.0 {
            return 0.0f32;
        }
        unsafe {
            let vel_slice = std::slice::from_raw_parts_mut(vel, 3);
            let pos_slice = std::slice::from_raw_parts_mut(pos, 3);

            if !acc.is_null() {
                let acc_slice = std::slice::from_raw_parts(acc, 3);
                vel_slice[0] += acc_slice[0] * dt;
                vel_slice[1] += acc_slice[1] * dt;
                vel_slice[2] += acc_slice[2] * dt;
            }

            vel_slice[2] += gravity * dt;

            let mv_x = vel_slice[0] * dt;
            let mv_y = vel_slice[1] * dt;
            let mv_z = vel_slice[2] * dt;

            let dist = (mv_x * mv_x + mv_y * mv_y + mv_z * mv_z).sqrt();

            pos_slice[0] += mv_x;
            pos_slice[1] += mv_y;
            pos_slice[2] += mv_z;

            dist
        }
    })
}

