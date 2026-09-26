pub const MOTION_WAIT: u32 = 1;
pub const MOTION_WALK: u32 = 2;
pub const MOTION_RUN: u32 = 3;
pub const MOTION_DAMAGE: u32 = 5;
pub const MOTION_DEAD: u32 = 11;
pub const MOTION_COMBO: u32 = 14;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActorMotionQueue {
    pub cur_motion_id: u32,
    pub motion_duration_ms: u32,
    pub motion_elapsed_ms: u32,
    pub is_looping: bool,
}

impl ActorMotionQueue {
    pub fn new() -> Self {
        Self {
            cur_motion_id: MOTION_WAIT,
            motion_duration_ms: 0,
            motion_elapsed_ms: 0,
            is_looping: true,
        }
    }

    pub fn set_motion(&mut self, motion_id: u32, duration_ms: u32, is_looping: bool) {
        if self.cur_motion_id == MOTION_DEAD && motion_id != MOTION_DEAD {
            return;
        }

        if self.cur_motion_id == MOTION_DAMAGE && self.motion_elapsed_ms < self.motion_duration_ms {
            if motion_id == MOTION_WALK || motion_id == MOTION_RUN || motion_id == MOTION_WAIT {
                return;
            }
        }
        
        if self.cur_motion_id == MOTION_COMBO && self.motion_elapsed_ms < self.motion_duration_ms {
             if motion_id == MOTION_WALK || motion_id == MOTION_RUN || motion_id == MOTION_WAIT {
                 return;
             }
        }


        self.cur_motion_id = motion_id;
        self.motion_duration_ms = duration_ms;
        self.motion_elapsed_ms = 0;
        self.is_looping = is_looping;
    }

    pub fn update_motion(&mut self, delta_time_ms: u32) -> bool {
        if self.is_looping {
            self.motion_elapsed_ms = self.motion_elapsed_ms.saturating_add(delta_time_ms);
            if self.motion_duration_ms > 0 {
                self.motion_elapsed_ms %= self.motion_duration_ms;
            }
            return false;
        }

        let old_elapsed = self.motion_elapsed_ms;
        self.motion_elapsed_ms = self.motion_elapsed_ms.saturating_add(delta_time_ms);

        if self.motion_duration_ms == 0 && old_elapsed == 0 && self.motion_elapsed_ms > 0 {
            return true;
        }
        
        if old_elapsed < self.motion_duration_ms && self.motion_elapsed_ms >= self.motion_duration_ms {
            return true;
        }

        false
    }
}
