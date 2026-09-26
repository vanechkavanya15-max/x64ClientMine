#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MovementInterpolator {
    pub last_network_pos: (f32, f32),
    pub last_network_time_ms: u32,
    pub target_network_pos: (f32, f32),
    pub target_network_time_ms: u32,
}

impl Default for MovementInterpolator {
    fn default() -> Self {
        Self::new()
    }
}

impl MovementInterpolator {
    pub fn new() -> Self {
        Self {
            last_network_pos: (0.0, 0.0),
            last_network_time_ms: 0,
            target_network_pos: (0.0, 0.0),
            target_network_time_ms: 0,
        }
    }

    pub fn receive_network_packet(&mut self, pos: (f32, f32), time_ms: u32, current_time_ms: u32) {
        // Prevent teleportation/jumping by using the currently interpolated position
        // as the new starting point if we're currently moving.
        let current_pos = self.get_interpolated_position(current_time_ms);
        self.last_network_pos = current_pos;
        
        // The new start time is the time we processed the packet (or current time)
        // rather than blindly trusting last_network_time_ms to avoid jumps.
        // Actually, if we use the time the packet was generated vs received, it could be tricky.
        // We will set the start time to current_time_ms.
        self.last_network_time_ms = current_time_ms;
        
        self.target_network_pos = pos;
        self.target_network_time_ms = time_ms;
        
        // If the server sends a target time in the past relative to current_time_ms,
        // or the interval is 0, we immediately snap to it (handled in get_interpolated_position).
        // A better approach for target time might be to estimate the interval based on
        // expected ping or server tick rate. For Metin2, target_network_time_ms is usually
        // the client's local timestamp of when the packet was received + some delay,
        // or server timestamp + offset.
        // Assuming time_ms is a future timestamp we want to reach.
    }

    pub fn get_interpolated_position(&self, current_time_ms: u32) -> (f32, f32) {
        if self.target_network_time_ms <= self.last_network_time_ms {
            return self.target_network_pos;
        }

        let elapsed = current_time_ms.saturating_sub(self.last_network_time_ms);
        let interval = self.target_network_time_ms.saturating_sub(self.last_network_time_ms);

        if interval == 0 {
            return self.target_network_pos;
        }

        let mut t = elapsed as f32 / interval as f32;
        if t < 0.0 {
            t = 0.0;
        } else if t > 1.0 {
            t = 1.0;
        }

        let x = Self::lerp(self.last_network_pos.0, self.target_network_pos.0, t);
        let y = Self::lerp(self.last_network_pos.1, self.target_network_pos.1, t);

        (x, y)
    }

    fn lerp(start: f32, end: f32, t: f32) -> f32 {
        start + (end - start) * t
    }
}
