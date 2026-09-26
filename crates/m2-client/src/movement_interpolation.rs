#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MovementInterpolator {
    pub last_network_pos: (f32, f32),
    pub last_network_time_ms: u32,
    pub target_network_pos: (f32, f32),
    pub target_network_time_ms: u32,
    pub last_z: f32,
    pub target_z: f32,
    pub average_network_gap_ms: u32,
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
            last_z: 0.0,
            target_z: 0.0,
            average_network_gap_ms: 100,
        }
    }

    pub fn receive_network_packet(&mut self, pos: (f32, f32), time_ms: u32, current_time_ms: u32) {
        self.receive_network_packet_3d((pos.0, pos.1, 0.0), time_ms, current_time_ms);
    }

    pub fn receive_network_packet_3d(&mut self, pos: (f32, f32, f32), time_ms: u32, current_time_ms: u32) {
        // Prevent teleportation/jumping by using the currently interpolated position
        // as the new starting point if we're currently moving.
        let current_pos_3d = self.get_interpolated_position_3d(current_time_ms);
        self.last_network_pos = (current_pos_3d.0, current_pos_3d.1);
        self.last_z = current_pos_3d.2;
        self.last_network_time_ms = current_time_ms;

        self.target_network_pos = (pos.0, pos.1);
        self.target_z = pos.2;
        self.target_network_time_ms = time_ms;
    }

    pub fn get_interpolated_position(&self, current_time_ms: u32) -> (f32, f32) {
        let (x, y, _) = self.get_interpolated_position_3d(current_time_ms);
        (x, y)
    }

    pub fn get_interpolated_position_3d(&self, current_time_ms: u32) -> (f32, f32, f32) {
        if self.target_network_time_ms <= self.last_network_time_ms {
            return (self.target_network_pos.0, self.target_network_pos.1, self.target_z);
        }

        let elapsed = current_time_ms.saturating_sub(self.last_network_time_ms);
        let interval = self.target_network_time_ms.saturating_sub(self.last_network_time_ms);

        if interval == 0 {
            return (self.target_network_pos.0, self.target_network_pos.1, self.target_z);
        }

        let mut t = elapsed as f32 / interval as f32;
        if t < 0.0 {
            t = 0.0;
        } else if t > 1.0 {
            t = 1.0;
        }

        let x = Self::lerp(self.last_network_pos.0, self.target_network_pos.0, t);
        let y = Self::lerp(self.last_network_pos.1, self.target_network_pos.1, t);
        let z = Self::lerp(self.last_z, self.target_z, t);

        (x, y, z)
    }

    /// Aktualizuje sredni odstep czasowy sieci (network gap) jak w CInstanceBase::PushTCPState:
    /// m_nAverageNetworkGap = (m_nAverageNetworkGap * 70 + nNetworkGap * 30) / 100;
    pub fn update_network_gap(&mut self, gap_ms: u32) {
        self.average_network_gap_ms = (self.average_network_gap_ms * 70 + gap_ms * 30) / 100;
    }

    fn lerp(start: f32, end: f32, t: f32) -> f32 {
        start + (end - start) * t
    }
}
