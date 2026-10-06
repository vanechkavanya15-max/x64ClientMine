use glam::{Vec2, Vec3};
use std::time::{SystemTime, UNIX_EPOCH};

// Dummy motion constants representing C++ CRaceMotionData::EName
pub const NAME_DAMAGE: u32 = 5;
pub const NAME_DAMAGE_FLYING: u32 = 6;
pub const NAME_STAND_UP: u32 = 7;
pub const NAME_DAMAGE_BACK: u32 = 8;
pub const NAME_DAMAGE_FLYING_BACK: u32 = 9;
pub const NAME_STAND_UP_BACK: u32 = 10;
pub const NAME_WAIT: u32 = 1;

pub trait HitReactionActor {
    fn is_stun(&self) -> bool;
    fn die(&mut self);
    fn is_knock_down(&self) -> bool;
    fn is_lock(&self) -> bool;
    fn get_rotation(&self) -> f32;
    fn intercept_once_motion(&mut self, motion_name: u32) -> bool;
    fn push_once_motion(&mut self, motion_name: u32);
    fn push_loop_motion(&mut self, motion_name: u32);
    fn is_using_skill(&self) -> bool;
    fn is_stand_up_motion(&self) -> bool;
    fn set_real_dead(&mut self, is_dead: bool);
    
    // Position/Physics related
    fn get_position(&self) -> Vec3;
    fn set_falling_direction(&mut self, dx: f32, dy: f32);

    // Context tracking for physics tests
    fn shake_duration(&self) -> u32;
    fn set_shake_duration(&mut self, ms: u32);
    
    fn shake_time(&self) -> u32;
    fn set_shake_time(&mut self, time: u32);
}

// Emulating ELTimer_GetMSec()
fn get_current_msec() -> u32 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    since_the_epoch.as_millis() as u32
}

pub fn shake(actor: &mut dyn HitReactionActor, duration: u32) {
    let current_time = get_current_msec();
    actor.set_shake_time(current_time + duration);
    actor.set_shake_duration(duration); // For mocking
}

pub fn shake_process(actor: &mut dyn HitReactionActor, current_time: u32, world_matrix_pos: &mut Vec3) {
    let shake_time = actor.shake_time();
    if shake_time > 0 {
        let mut pos = Vec3::ZERO;
        
        if shake_time < current_time {
            actor.set_shake_time(0);
        } else {
            let shake_size = 10;
            // Using modulo is sufficient for logic replication (though rand() should be used in C++)
            // Here we just emulate some shifts based on current time modulo
            // To ensure strict testing parity, if current_time % 2 == 0 it adds, else subtracts.
            let x_mod = (current_time % 2) as i32;
            let y_mod = ((current_time + 1) % 2) as i32;
            let z_mod = ((current_time + 2) % 2) as i32;

            if x_mod == 0 { pos.x += (current_time % shake_size as u32) as f32; }
            else { pos.x -= (current_time % shake_size as u32) as f32; }

            if y_mod == 0 { pos.y += (current_time % shake_size as u32) as f32; }
            else { pos.y -= (current_time % shake_size as u32) as f32; }

            if z_mod == 0 { pos.z += (current_time % shake_size as u32) as f32; }
            else { pos.z -= (current_time % shake_size as u32) as f32; }
        }

        world_matrix_pos.x += pos.x;
        world_matrix_pos.y += pos.y;
        world_matrix_pos.z += pos.z;
    }
}

pub fn hit_stone(victim: &mut dyn HitReactionActor) {
    if victim.is_stun() {
        victim.die();
    } else {
        shake(victim, 100);
    }
}

pub fn hit_good(attacker: &dyn HitReactionActor, victim: &mut dyn HitReactionActor) {
    if victim.is_knock_down() {
        return;
    }

    if victim.is_stun() {
        victim.die();
    } else {
        shake(victim, 100);

        if !victim.is_lock() {
            let rot_rad = attacker.get_rotation().to_radians();
            let victim_rot_rad = victim.get_rotation().to_radians();

            let mut normal = Vec2::new(rot_rad.sin(), rot_rad.cos());
            let mut victim_normal = Vec2::new(victim_rot_rad.sin(), victim_rot_rad.cos());

            normal = normal.normalize_or_zero();
            victim_normal = victim_normal.normalize_or_zero();

            let scalar = normal.dot(victim_normal);

            if scalar < 0.0 {
                if victim.intercept_once_motion(NAME_DAMAGE) {
                    victim.push_loop_motion(NAME_WAIT);
                }
            } else {
                if victim.intercept_once_motion(NAME_DAMAGE_BACK) {
                    victim.push_loop_motion(NAME_WAIT);
                } else if victim.intercept_once_motion(NAME_DAMAGE) {
                    victim.push_loop_motion(NAME_WAIT);
                }
            }
        }
    }
}

pub fn hit_greate(attacker: &dyn HitReactionActor, victim: &mut dyn HitReactionActor) {
    if victim.is_knock_down() {
        return;
    }
    if victim.is_stand_up_motion() {
        return;
    }

    let rot_rad = attacker.get_rotation().to_radians();
    let victim_rot_rad = victim.get_rotation().to_radians();

    let mut normal = Vec2::new(rot_rad.sin(), rot_rad.cos());
    let mut victim_normal = Vec2::new(victim_rot_rad.sin(), victim_rot_rad.cos());

    normal = normal.normalize_or_zero();
    victim_normal = victim_normal.normalize_or_zero();

    let scalar = normal.dot(victim_normal);

    shake(victim, 100);

    if victim.is_using_skill() {
        return;
    }

    if victim.is_stun() {
        if scalar < 0.0 {
            victim.intercept_once_motion(NAME_DAMAGE_FLYING);
        } else {
            if !victim.intercept_once_motion(NAME_DAMAGE_FLYING_BACK) {
                victim.intercept_once_motion(NAME_DAMAGE_FLYING);
            }
        }

        victim.set_real_dead(true);
    } else {
        if scalar < 0.0 {
            if victim.intercept_once_motion(NAME_DAMAGE_FLYING) {
                victim.push_once_motion(NAME_STAND_UP);
                victim.push_loop_motion(NAME_WAIT);
            }
        } else {
            if !victim.intercept_once_motion(NAME_DAMAGE_FLYING_BACK) {
                if victim.intercept_once_motion(NAME_DAMAGE_FLYING) {
                    victim.push_once_motion(NAME_STAND_UP);
                    victim.push_loop_motion(NAME_WAIT);
                }
            } else {
                victim.push_once_motion(NAME_STAND_UP_BACK);
                victim.push_loop_motion(NAME_WAIT);
            }
        }
    }
}

pub fn push_circle(attacker_pos: Vec3, victim: &mut dyn HitReactionActor) {
    let src_pos = Vec3::new(attacker_pos.x, -attacker_pos.y, attacker_pos.z);
    let dst_pos = victim.get_position();

    let mut direction = Vec3::new(
        dst_pos.x - src_pos.x,
        dst_pos.y - src_pos.y,
        0.0
    );
    direction = direction.normalize_or_zero();

    victim.set_falling_direction(direction.x, direction.y);
}

pub fn push_direct(attacker_rotation: f32, victim: &mut dyn HitReactionActor) {
    let direction_x = (attacker_rotation + 270.0).to_radians().cos();
    let direction_y = (attacker_rotation + 270.0).to_radians().sin();

    victim.set_falling_direction(direction_x, direction_y);
}

// MR-3: Shaman on-mount hitting fix
pub fn get_invisible_time_adjust(race: u32, ui_skill: u32, f_atk_spd: f32, f_invisible_time: f32) -> f32 {
    const SHAMAN_W: u32 = 3;
    const SHAMAN_M: u32 = 7;

    if (race != SHAMAN_W && race != SHAMAN_M) || ui_skill != 0 || f_atk_spd < 1.3 {
        return 0.0;
    }

    let scale = (f_atk_spd - 1.3) / 1.3;
    let inv = f_invisible_time * 0.5;
    inv * scale
}

pub fn is_invisible(is_dead: bool, invisible_time: u32, current_second: u32) -> bool {
    if is_dead {
        return true;
    }

    if current_second >= invisible_time {
        return false;
    }

    true
}
