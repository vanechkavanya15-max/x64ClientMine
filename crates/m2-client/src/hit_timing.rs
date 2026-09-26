#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HitTiming {
    pub motion_duration_ms: u32,
    pub hit_starting_time_ms: u32,
    pub attack_speed: u32,
}

impl HitTiming {
    pub fn new(motion_duration_ms: u32, hit_starting_time_ms: u32, attack_speed: u32) -> Self {
        Self {
            motion_duration_ms,
            hit_starting_time_ms,
            attack_speed,
        }
    }

    pub fn scaled_duration_ms(&self) -> u32 {
        let speed = if self.attack_speed == 0 { 100 } else { self.attack_speed };
        (self.motion_duration_ms * 100) / speed
    }

    pub fn scaled_hit_time_ms(&self) -> u32 {
        let speed = if self.attack_speed == 0 { 100 } else { self.attack_speed };
        (self.hit_starting_time_ms * 100) / speed
    }

    pub fn is_hit_moment(elapsed_in_motion_ms: u32, scaled_hit_time_ms: u32, tolerance_ms: u32) -> bool {
        let start = scaled_hit_time_ms.saturating_sub(tolerance_ms);
        let end = scaled_hit_time_ms.saturating_add(tolerance_ms);
        elapsed_in_motion_ms >= start && elapsed_in_motion_ms <= end
    }
}
