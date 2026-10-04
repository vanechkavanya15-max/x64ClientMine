pub struct WgpuWaterCausticsAnimator {
    pub num_frames: u32,
    pub frame_duration_ms: u32,
}

impl Default for WgpuWaterCausticsAnimator {
    fn default() -> Self {
        Self {
            num_frames: 30,
            frame_duration_ms: 70,
        }
    }
}

impl WgpuWaterCausticsAnimator {
    pub fn new(num_frames: u32, frame_duration_ms: u32) -> Self {
        Self {
            num_frames,
            frame_duration_ms,
        }
    }

    pub fn get_current_frame(&self, time_ms: u32) -> u32 {
        if self.num_frames == 0 || self.frame_duration_ms == 0 {
            return 0;
        }
        (time_ms / self.frame_duration_ms) % self.num_frames
    }

    pub fn calculate_intensity(
        &self,
        water_depth: f32,
        max_depth: f32,
        water_transparency: f32,
    ) -> f32 {
        if max_depth <= 0.0 {
            return 0.0;
        }

        if water_depth <= 0.0 {
            return water_transparency;
        }

        let depth_factor = (1.0 - (water_depth / max_depth)).clamp(0.0, 1.0);
        depth_factor * water_transparency
    }
}
