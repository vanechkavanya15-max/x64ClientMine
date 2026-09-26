pub struct ActorRotation {
    cur_rot_deg: f32,
    target_rot_deg: f32,
    rot_speed_deg_per_sec: f32,
}

impl ActorRotation {
    pub fn new(initial_rot_deg: f32) -> Self {
        Self {
            cur_rot_deg: initial_rot_deg,
            target_rot_deg: initial_rot_deg,
            rot_speed_deg_per_sec: 720.0,
        }
    }

    pub fn set_target(&mut self, target_rot_deg: f32) {
        self.target_rot_deg = target_rot_deg;
    }

    pub fn update(&mut self, delta_time_seconds: f32) {
        let diff = (self.target_rot_deg - self.cur_rot_deg + 540.0).rem_euclid(360.0) - 180.0;
        let max_step = self.rot_speed_deg_per_sec * delta_time_seconds;
        let step = diff.clamp(-max_step, max_step);
        self.cur_rot_deg = (self.cur_rot_deg + step + 360.0).rem_euclid(360.0);
    }

    pub fn get_current_rotation(&self) -> f32 {
        self.cur_rot_deg
    }

    pub fn is_at_target(&self) -> bool {
        (self.target_rot_deg - self.cur_rot_deg).abs() < f32::EPSILON
    }
}
