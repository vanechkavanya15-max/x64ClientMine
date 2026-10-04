pub struct AudioSpatial3DEngine {
    listener_pos: [f32; 3],
    listener_forward: [f32; 3],
    listener_up: [f32; 3],
}

impl AudioSpatial3DEngine {
    pub fn new() -> Self {
        Self {
            listener_pos: [0.0, 0.0, 0.0],
            listener_forward: [0.0, 0.0, -1.0],
            listener_up: [0.0, 1.0, 0.0],
        }
    }

    pub fn set_listener_position(&mut self, x: f32, y: f32, z: f32) {
        self.listener_pos = [x, y, z];
    }

    pub fn set_listener_orientation(&mut self, forward_x: f32, forward_y: f32, forward_z: f32, up_x: f32, up_y: f32, up_z: f32) {
        // Normalize forward
        let flen = (forward_x * forward_x + forward_y * forward_y + forward_z * forward_z).sqrt();
        if flen > 0.0 {
            self.listener_forward = [forward_x / flen, forward_y / flen, forward_z / flen];
        } else {
            self.listener_forward = [0.0, 0.0, -1.0];
        }

        // Normalize up
        let ulen = (up_x * up_x + up_y * up_y + up_z * up_z).sqrt();
        if ulen > 0.0 {
            self.listener_up = [up_x / ulen, up_y / ulen, up_z / ulen];
        } else {
            self.listener_up = [0.0, 1.0, 0.0];
        }
    }

    /// Calculates spatial parameters: (volume, pan)
    /// volume: 0.0 (silent) to 1.0 (full volume)
    /// pan: -1.0 (full left) to 1.0 (full right)
    pub fn calculate_spatial_params(&self, source_x: f32, source_y: f32, source_z: f32, min_dist: f32, max_dist: f32) -> (f32, f32) {
        let dx = source_x - self.listener_pos[0];
        let dy = source_y - self.listener_pos[1];
        let dz = source_z - self.listener_pos[2];

        let distance = (dx * dx + dy * dy + dz * dz).sqrt();

        // Distance attenuation: linear roll-off
        let volume = if distance <= min_dist {
            1.0
        } else if distance >= max_dist {
            0.0
        } else {
            1.0 - (distance - min_dist) / (max_dist - min_dist)
        };

        // Stereo Pan
        // 1. Calculate Right vector = CrossProduct(Forward, Up)
        let right_x = self.listener_forward[1] * self.listener_up[2] - self.listener_forward[2] * self.listener_up[1];
        let right_y = self.listener_forward[2] * self.listener_up[0] - self.listener_forward[0] * self.listener_up[2];
        let right_z = self.listener_forward[0] * self.listener_up[1] - self.listener_forward[1] * self.listener_up[0];
        
        let right_len = (right_x * right_x + right_y * right_y + right_z * right_z).sqrt();
        let (rx, ry, rz) = if right_len > 0.0 {
            (right_x / right_len, right_y / right_len, right_z / right_len)
        } else {
            (1.0, 0.0, 0.0) // fallback
        };

        let mut pan = 0.0;
        if distance > 0.0 {
            let dir_x = dx / distance;
            let dir_y = dy / distance;
            let dir_z = dz / distance;

            // Pan is the dot product of normalized source direction and right vector
            pan = dir_x * rx + dir_y * ry + dir_z * rz;
            
            // Clamp pan to [-1.0, 1.0] just in case
            pan = pan.clamp(-1.0, 1.0);
        }

        (volume, pan)
    }
}
// Implemented distance attenuation and stereo pan based on reference C++ logic.
