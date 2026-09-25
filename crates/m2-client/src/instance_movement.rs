use std::ops::Sub;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct PixelPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Sub for PixelPosition {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

pub fn clamp_degree(mut rot: f32) -> f32 {
    while rot >= 360.0 {
        rot -= 360.0;
    }
    while rot < 0.0 {
        rot += 360.0;
    }
    rot
}

pub fn get_degree_difference(r1: f32, r2: f32) -> f32 {
    let mut diff = (r1 - r2).abs();
    if diff > 180.0 {
        diff = 360.0 - diff;
    }
    diff
}

pub fn get_rotating_direction(cur_rot: f32, adv_rot: f32) -> i32 {
    let diff = adv_rot - cur_rot;
    
    let mut norm_diff = diff;
    while norm_diff >= 360.0 { norm_diff -= 360.0; }
    while norm_diff < 0.0 { norm_diff += 360.0; }

    if norm_diff < 180.0 {
        1
    } else {
        -1
    }
}

pub fn get_degree_from_position(x: f32, y: f32) -> f32 {
    let len = (x * x + y * y).sqrt();
    if len < 0.0001 {
        return 0.0;
    }
    
    let dir_x = x.floor() / len;
    let dir_y = y.floor() / len;

    // D3DXVec3Dot(&vtDir, &vtStan) where vtStan = (0, -1, 0)
    let dot = dir_y * -1.0; 
    let mut ret = dot.acos().to_degrees();

    if dir_x < 0.0 {
        ret = 360.0 - ret;
    }

    ret
}
 
 #[derive(Debug, Clone, Default)]
 pub struct InstanceMovement {
     pub initialized: bool,
    
    pub attack_speed: f32,
    pub move_speed: f32,
    pub max_rot_spd: f32,
    pub rot_spd: f32,
    pub rotating_direction: i32,
    
    pub is_syncing: bool,
    pub is_lock: bool,
    pub is_using_skill: bool,
    pub is_using_moving_skill: bool,
    pub is_waiting: bool,
    pub is_walking: bool,
    pub is_pushing: bool,
    pub is_attacked: bool,
    pub is_knock_down: bool,
    pub is_attacking: bool,
    pub is_act_emotion: bool,
    pub is_going: bool,
    
    pub cur_pos: PixelPosition,
    pub dst_pos: PixelPosition,
    pub src_pos: PixelPosition,
    
    pub cur_rot: f32,
    pub adv_rot: f32,
    pub dst_rot: f32,
 }
 
 impl InstanceMovement {
     pub fn new() -> Self {
        let mut slf = Self::default();
        slf.initialized = true;
        slf
     }
 
     pub fn is_ready(&self) -> bool {
         self.initialized
     }
 
    pub fn set_attack_speed(&mut self, mut atk_spd: u32) {
        if atk_spd > 1100 {
            atk_spd = 0;
        }
        self.attack_speed = (atk_spd as f32) / 100.0;
    }

    pub fn set_move_speed(&mut self, mut mov_spd: u32) {
        if mov_spd > 1100 {
            mov_spd = 0;
        }
        self.move_speed = (mov_spd as f32) / 100.0;
    }

    pub fn set_rotation_speed(&mut self, rot_spd: f32) {
        self.max_rot_spd = rot_spd;
    }

    pub fn new_stop(&mut self) {
        if self.is_syncing {
            return;
        }
        if self.is_lock {
            return;
        }
        if self.is_using_skill {
            return;
        }

        if !self.is_waiting() {
            self.end_walking(0.0);
        }
        
        // mock for m_GraphicThingInstance.__OnStop()
    }

    pub fn new_sync_pixel_position(&mut self, x: i32, y: i32) {
        self.cur_pos.x = x as f32;
        self.cur_pos.y = y as f32;
    }

    pub fn start_walking(&mut self) {
        self.is_walking = true;
    }

    pub fn end_walking(&mut self, _blending_time: f32) {
        self.is_going = false;
        
        if self.is_walking() || !self.is_attacked() {
            self.is_walking = false;
        }
    }

    pub fn end_walking_without_blending(&mut self) {
        self.end_walking(0.0);
    }

    pub fn is_waiting(&self) -> bool {
        self.is_waiting
    }

    pub fn is_walking(&self) -> bool {
        self.is_walking
    }

    pub fn is_pushing(&self) -> bool {
        self.is_pushing
    }

    pub fn is_attacked(&self) -> bool {
        self.is_attacked
    }

    pub fn is_knock_down(&self) -> bool {
        self.is_knock_down
    }

    pub fn is_attacking(&self) -> bool {
        self.is_attacking
    }

    pub fn is_acting_emotion(&self) -> bool {
        self.is_act_emotion
    }

    pub fn is_going(&self) -> bool {
        self.is_going
    }

    pub fn new_can_move_to_dest_pixel_position(&self, dest: &PixelPosition) -> bool {
        if self.cur_pos.x == dest.x && self.cur_pos.y == dest.y {
            return false;
        }
        true
    }

    pub fn new_get_advancing_rotation_from_dir_pixel_position(&self, dir: &PixelPosition) -> f32 {
        let dir_rot = get_degree_from_position(dir.x, -dir.y);
        clamp_degree(dir_rot)
    }

    pub fn new_get_advancing_rotation_from_dest_pixel_position(&self, dest: &PixelPosition) -> f32 {
        self.new_get_advancing_rotation_from_pixel_position(&self.cur_pos, dest)
    }

    pub fn new_get_advancing_rotation_from_pixel_position(&self, src: &PixelPosition, dest: &PixelPosition) -> f32 {
        let delta = *dest - *src;
        self.new_get_advancing_rotation_from_dir_pixel_position(&delta)
    }

    pub fn new_set_advancing_rotation_from_dir_pixel_position(&mut self, dir: &PixelPosition) {
        let clamp_dir_rot = self.new_get_advancing_rotation_from_dir_pixel_position(dir);
        self.adv_rot = clamp_dir_rot;
        self.rotating_direction = get_rotating_direction(self.cur_rot, self.adv_rot);
    }

    pub fn new_set_advancing_rotation_from_pixel_position(&mut self, src: &PixelPosition, dest: &PixelPosition) {
        let delta = *dest - *src;
        self.new_set_advancing_rotation_from_dir_pixel_position(&delta);
    }

    pub fn new_set_advancing_rotation_from_dest_pixel_position(&mut self, dest: &PixelPosition) -> bool {
        if !self.new_can_move_to_dest_pixel_position(dest) {
            return false;
        }

        self.new_set_advancing_rotation_from_pixel_position(&self.cur_pos.clone(), dest);
        true
    }

    pub fn set_advancing_rotation(&mut self, rotation: f32) {
        let rot_diff = get_degree_difference(self.cur_rot, rotation);

        if rot_diff > 45.0 {
            self.rot_spd = self.max_rot_spd;
        } else {
            self.rot_spd = self.max_rot_spd * 5.0 / 12.0;
        }

        self.adv_rot = clamp_degree(rotation);
        self.rotating_direction = get_rotating_direction(self.cur_rot, self.adv_rot);
    }

    pub fn new_move_to_dest_pixel_position_direction(&mut self, dest: &PixelPosition) -> bool {
        let dst_rot = self.new_get_advancing_rotation_from_pixel_position(&self.cur_pos, dest);
        self.new_goto(dest, dst_rot)
    }

    pub fn new_goto(&mut self, dest: &PixelPosition, dst_rot: f32) -> bool {
        if self.is_syncing {
            return false;
        }

        if self.is_using_moving_skill {
            self.set_advancing_rotation(dst_rot);
            return true;
        }

        if self.is_lock {
            return false;
        }

        if !self.new_can_move_to_dest_pixel_position(dest) {
            if !self.is_waiting() {
                self.end_walking(0.0);
            }
            return true;
        }

        self.src_pos = self.cur_pos;
        self.dst_pos = *dest;
        self.dst_pos.z = self.src_pos.z;
        self.dst_rot = dst_rot;
        self.is_going = true;

        if !self.is_walking() {
            self.start_walking();
        }

        self.new_set_advancing_rotation_from_pixel_position(&self.src_pos.clone(), &self.dst_pos.clone());

        true
    }

    pub fn new_move_to_direction(&mut self, dir_rot: f32) {
        if self.is_syncing {
            return;
        }

        if self.is_using_moving_skill {
            self.set_advancing_rotation(dir_rot);
            return;
        }

        if self.is_lock {
            return;
        }

        self.is_going = false;

        self.set_advancing_rotation(dir_rot);

        if !self.is_walking() {
            self.start_walking();
        }

        let mut adv_dir_x = 0.0;
        let mut adv_dir_y = -1.0;
        
        let rad = dir_rot.to_radians();
        let s = rad.sin();
        let c = rad.cos();

        // D3DXMatrixRotationZ behavior on (0, -1, 0)
        let rot_x = adv_dir_x * c - adv_dir_y * s;
        let rot_y = adv_dir_x * s + adv_dir_y * c;
        
        adv_dir_x = rot_x * 300.0;
        adv_dir_y = rot_y * 300.0;

        let mut dest = PixelPosition::default();
        dest.x = self.cur_pos.x + adv_dir_x;
        dest.y = -( -self.cur_pos.y + adv_dir_y ); // match y inversion logic from C++
        dest.z = self.cur_pos.z;

        self.src_pos = self.cur_pos;
        self.dst_pos = dest;
    }

    pub fn end_going(&mut self) {
        if !self.is_waiting() {
            self.end_walking(0.0);
        }
        self.is_going = false;
    }

    pub fn set_run_mode(&mut self) {
        // mock
    }
 
    pub fn set_walk_mode(&mut self) {
        // mock
     }
 }
