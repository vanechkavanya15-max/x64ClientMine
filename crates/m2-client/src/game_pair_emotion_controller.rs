pub const NAME_WAIT: u32 = 0;
pub const FUNC_EMOTION: u32 = 5;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct PixelPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl PixelPosition {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn distance(&self, other: &PixelPosition) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

pub fn get_degree_from_position(x: f32, y: f32) -> f32 {
    let len = (x * x + y * y).sqrt();
    if len < 1e-5 {
        return 0.0;
    }
    let dx = x / len;
    let dy = y / len;

    // Dot product with (0, -1)
    let dot = -dy;
    // clamp to avoid acos domain error
    let dot = dot.clamp(-1.0, 1.0);
    
    let mut angle_rad = dot.acos();
    let mut angle_deg = angle_rad.to_degrees();

    if dx < 0.0 {
        angle_deg = 360.0 - angle_deg;
    }

    angle_deg
}

pub fn get_degree_from_position2(sx: f32, sy: f32, ex: f32, ey: f32) -> f32 {
    get_degree_from_position(ex - sx, ey - sy)
}

pub fn race_to_job(race: u32) -> u32 {
    let job_num = 4;
    race % job_num
}

#[derive(Debug, Clone, Default)]
pub struct MoveAfterFunc {
    pub e_func: u32,
    pub u_arg: u32,
    pub u_arg_expanded: u32,
    pub pos_dst: PixelPosition,
}

#[derive(Debug, Clone, Default)]
pub struct ActorInstance {
    pub vid: u32,
    pub race: u32,
    pub is_main_instance: bool,
    pub is_waiting: bool,
    pub is_walking: bool,
    pub cur_pixel_position: PixelPosition,
    pub tcp_state_enabled: bool,
    
    pub current_motion: u32,
    pub motion_loop: bool,
    
    pub rotation: f32,
    pub advancing_rotation: f32,

    pub tcp_move_after_func: MoveAfterFunc,
}

impl ActorInstance {
    pub fn new(vid: u32, race: u32, is_main: bool) -> Self {
        Self {
            vid,
            race,
            is_main_instance: is_main,
            ..Default::default()
        }
    }
}

impl ActorInstance {
    pub fn look_at_dest_instance(&mut self, target: &ActorInstance) {
        let dx = target.cur_pixel_position.x - self.cur_pixel_position.x;
        let dy = target.cur_pixel_position.y - self.cur_pixel_position.y;
        let rot = get_degree_from_position(dx, dy);
        self.rotation = rot;
        self.advancing_rotation = rot;
    }

    pub fn process_function_emotion(&mut self, motion_number: u32, target_vid: u32, pos_dst: PixelPosition, target_instance: &mut ActorInstance) {
        if self.is_walking {
            self.is_walking = false;
        }

        self.tcp_state_enabled = true;
        self.cur_pixel_position = pos_dst;

        if target_instance.vid == target_vid {
            target_instance.tcp_state_enabled = true;
            if target_instance.is_walking {
                target_instance.is_walking = false;
            }

            let motion_number_1 = (motion_number >> 16) & 0xFFFF;
            let motion_number_2 = motion_number & 0xFFFF;

            let src_job = race_to_job(self.race);
            let dst_job = race_to_job(target_instance.race);

            self.look_at_dest_instance(target_instance);
            self.current_motion = motion_number_1 + dst_job;
            self.motion_loop = false;

            target_instance.look_at_dest_instance(self);
            target_instance.current_motion = motion_number_2 + src_job;
            target_instance.motion_loop = false;
        }
    }

    pub fn act_dual_emotion(&mut self, target_instance: &mut ActorInstance, motion_number_1: u16, motion_number_2: u16) {
        if !self.is_waiting {
            self.current_motion = NAME_WAIT;
            self.motion_loop = true;
        }
        if !target_instance.is_waiting {
            target_instance.current_motion = NAME_WAIT;
            target_instance.motion_loop = true;
        }

        let emotion_distance = 100.0f32;
        let dx = self.cur_pixel_position.x - target_instance.cur_pixel_position.x;
        let dy = self.cur_pixel_position.y - target_instance.cur_pixel_position.y;
        
        let mut distance = (dx * dx + dy * dy).sqrt();
        if distance < 1e-5 {
            distance = 1.0;
        }

        let mut dst_position = PixelPosition::default();
        dst_position.x = target_instance.cur_pixel_position.x + (dx / distance) * emotion_distance;
        dst_position.y = target_instance.cur_pixel_position.y + (dy / distance) * emotion_distance;
        dst_position.z = self.cur_pixel_position.z;

        let motion_combined = ((motion_number_1 as u32) << 16) | (motion_number_2 as u32);
        
        self.tcp_move_after_func = MoveAfterFunc {
            e_func: FUNC_EMOTION,
            u_arg: motion_combined,
            u_arg_expanded: target_instance.vid,
            pos_dst: dst_position,
        };

        self.tcp_state_enabled = false;
        target_instance.tcp_state_enabled = false;
    }
}
