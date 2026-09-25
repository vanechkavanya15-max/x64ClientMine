//! Modul `actor_sync` — Synchronizacja odrzutu (knockback) i kolizji miedzy aktorami
//! Wzorzec referencyjny C++: `source/GameLib/ActorInstanceSync.cpp oraz source/GameLib/ActorInstancePosition.cpp`
//! Opis: Odwzorowanie fizyki odrzutu po ostatnim ciosie combo lub skillu, kolizja sferyczna zapobiegajaca przenikaniu przez potwory i budynki.

use std::ops::{Add, AddAssign, Div, Mul, Sub};

pub const NAME_DAMAGE_FLYING: u32 = 1;
pub const NAME_STAND_UP: u32 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl Add for Vector3 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Vector3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f32> for Vector3 {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self::Output {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl Div<f32> for Vector3 {
    type Output = Self;
    fn div(self, scalar: f32) -> Self::Output {
        Self::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

pub type TPixelPosition = Vector3;

pub trait IPhysicsWorld {
    fn is_physical_collision(&self, pos: &Vector3) -> bool;
}

#[derive(Debug, Clone, Default)]
pub struct ActorSync {
    pub initialized: bool,
    
    // Pola z ActorInstancePosition.cpp
    pub shake_time: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub need_update_collision: bool,
    pub p_pos_atk: TPixelPosition,
    pub p_pos_last: TPixelPosition,
    pub p_pos_dst: TPixelPosition,
    pub p_pos_cur: TPixelPosition,
    pub p_pos_src: TPixelPosition,
    pub v3_pos: Vector3,
    pub v3_movement: Vector3,

    // Pola stanu
    pub resist_fallen: bool,
    pub dead: bool,
    pub stun: bool,
    pub using_skill: bool,
    pub blending: bool,
    pub blending_position: Vector3,
    pub motion_queue: Vec<u32>,
}

impl ActorSync {
    pub fn new() -> Self {
        Self {
            initialized: true,
            ..Default::default()
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    // --- Metody z ActorInstanceSync.cpp ---

    pub fn push(&mut self, world: Option<&dyn IPhysicsWorld>, x: i32, y: i32) {
        if self.is_resist_fallen() {
            return;
        }

        let src = self.get_position();
        let dst = Vector3::new(x as f32, -(y as f32), src.z);
        let delta = dst - src;

        let loop_value = 100;
        let inc = delta / (loop_value as f32);

        let mut movement = Vector3::new(0.0, 0.0, 0.0);

        if let Some(p_world) = world {
            for _ in 0..loop_value {
                if p_world.is_physical_collision(&(src + movement)) {
                    self.reset_blending_position();
                    return;
                }
                movement += inc;
            }
        } else {
            return;
        }

        self.set_blending_position(dst);

        if !self.is_using_skill() {
            let len = (delta.x * delta.x + delta.y * delta.y).sqrt();
            if len > 150.0 {
                self.intercept_once_motion(NAME_DAMAGE_FLYING);
                self.push_once_motion(NAME_STAND_UP);
            }
        }
    }

    pub fn temp_push(&mut self, world: Option<&dyn IPhysicsWorld>, x: i32, y: i32) {
        self.push(world, x, y);
    }

    pub fn is_syncing(&self) -> bool {
        if self.is_dead() {
            return true;
        }
        if self.is_stun() {
            return true;
        }
        if self.is_pushing() {
            return true;
        }
        false
    }

    pub fn is_pushing(&self) -> bool {
        self.blending
    }

    // --- Metody pomocnicze ---
    pub fn is_resist_fallen(&self) -> bool {
        self.resist_fallen
    }

    pub fn get_position(&self) -> Vector3 {
        Vector3::new(self.x, self.y, self.z)
    }

    pub fn reset_blending_position(&mut self) {
        self.blending = false;
        self.blending_position = self.get_position();
    }

    pub fn set_blending_position(&mut self, pos: Vector3) {
        self.blending = true;
        self.blending_position = pos;
    }

    pub fn get_blending_position(&self, pos: &mut Vector3) {
        *pos = self.blending_position;
    }

    pub fn is_using_skill(&self) -> bool {
        self.using_skill
    }

    pub fn intercept_once_motion(&mut self, motion: u32) {
        self.motion_queue.insert(0, motion);
    }

    pub fn push_once_motion(&mut self, motion: u32) {
        self.motion_queue.push(motion);
    }

    pub fn is_dead(&self) -> bool {
        self.dead
    }

    pub fn is_stun(&self) -> bool {
        self.stun
    }

    // --- Metody z ActorInstancePosition.cpp ---

    pub fn new_get_last_pixel_position_ref(&mut self) -> &TPixelPosition {
        self.p_pos_last = self.blending_position;
        self.p_pos_last.y = -self.p_pos_last.y;
        &self.p_pos_last
    }

    pub fn get_position_vector_ref(&mut self) -> &Vector3 {
        self.v3_pos.x = self.x;
        self.v3_pos.y = self.y;
        self.v3_pos.z = self.z;
        &self.v3_pos
    }

    pub fn get_movement_vector_ref(&self) -> &Vector3 {
        &self.v3_movement
    }

    pub fn new_set_atk_pixel_position(&mut self, pos: &TPixelPosition) {
        self.p_pos_atk = *pos;
    }

    pub fn set_cur_pixel_position(&mut self, pos: &TPixelPosition) {
        let v3_pos_cur = Vector3::new(pos.x, -pos.y, pos.z);
        self.set_pixel_position(&v3_pos_cur);
    }

    pub fn new_set_src_pixel_position(&mut self, pos: &TPixelPosition) {
        self.p_pos_src = *pos;
    }

    pub fn new_set_dst_pixel_position_z(&mut self, z: f32) {
        self.p_pos_dst.z = z;
    }

    pub fn new_set_dst_pixel_position(&mut self, pos: &TPixelPosition) {
        self.p_pos_dst = *pos;
    }

    pub fn new_get_atk_pixel_position_ref(&self) -> &TPixelPosition {
        &self.p_pos_atk
    }

    pub fn new_get_src_pixel_position_ref(&self) -> &TPixelPosition {
        &self.p_pos_src
    }

    pub fn new_get_dst_pixel_position_ref(&self) -> &TPixelPosition {
        &self.p_pos_dst
    }

    pub fn new_get_cur_pixel_position_ref(&mut self) -> &TPixelPosition {
        self.p_pos_cur.x = self.x;
        self.p_pos_cur.y = -self.y;
        self.p_pos_cur.z = self.z;
        &self.p_pos_cur
    }

    pub fn get_pixel_position(&self, pos: &mut TPixelPosition) {
        pos.x = self.x;
        pos.y = self.y;
        pos.z = self.z;
    }

    pub fn set_pixel_position(&mut self, pos: &TPixelPosition) {
        self.x = pos.x;
        self.y = pos.y;
        self.z = pos.z;
        self.need_update_collision = true;
    }

    pub fn initialize_position_data(&mut self) {
        self.shake_time = 0;
        self.x = 0.0;
        self.y = 0.0;
        self.z = 0.0;
        self.need_update_collision = false;

        let zero_pos = TPixelPosition::new(0.0, 0.0, 0.0);
        self.p_pos_atk = zero_pos;
        self.p_pos_last = zero_pos;
        self.p_pos_dst = zero_pos;
        self.p_pos_cur = zero_pos;
        self.p_pos_src = zero_pos;

        self.initialize_movement();
    }

    pub fn initialize_movement(&mut self) {
        self.v3_movement = Vector3::new(0.0, 0.0, 0.0);
    }
}
