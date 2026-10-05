pub struct WgpuWaterCausticsAnimator {
    pub num_frames: u32,
    pub frame_duration_ms: u32,
    pub base_path: String,
}

impl Default for WgpuWaterCausticsAnimator {
    fn default() -> Self {
        Self {
            num_frames: 30,
            frame_duration_ms: 70,
            base_path: "d:/ymir Work/special/water/".to_string(),
        }
    }
}

impl WgpuWaterCausticsAnimator {
    pub fn new(num_frames: u32, frame_duration_ms: u32, base_path: Option<String>) -> Self {
        Self {
            num_frames,
            frame_duration_ms,
            base_path: base_path.unwrap_or_else(|| "d:/ymir Work/special/water/".to_string()),
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
        if max_depth <= f32::EPSILON {
            return 0.0;
        }

        if water_depth <= f32::EPSILON {
            return water_transparency;
        }

        let depth_factor = (1.0 - (water_depth / max_depth)).clamp(0.0, 1.0);
        depth_factor * water_transparency
    }

    pub fn get_texture_path(&self, time_ms: u32) -> String {
        let frame = self.get_current_frame(time_ms);
        format!("{}{:02}.dds", self.base_path, frame + 1)
    }
}

pub struct WaterHeightAnimator {
    pub current_height: f32,
    pub height_begin: f32,
    pub height_end: f32,
    pub last_change_time_ms: u32,
    pub blend_time_ms: u32,
}

impl Default for WaterHeightAnimator {
    fn default() -> Self {
        Self {
            current_height: 0.0,
            height_begin: 0.0,
            height_end: 0.0,
            last_change_time_ms: 0,
            blend_time_ms: 300,
        }
    }
}

impl WaterHeightAnimator {
    pub fn new(current_time_ms: u32) -> Self {
        Self {
            current_height: 0.0,
            height_begin: 0.0,
            height_end: 0.0,
            last_change_time_ms: current_time_ms,
            blend_time_ms: 300,
        }
    }

    pub fn update(&mut self, current_time_ms: u32) -> f32 {
        let elapsed = current_time_ms.saturating_sub(self.last_change_time_ms);

        if elapsed > self.blend_time_ms {
            let mut rng = rand::thread_rng();
            use rand::Rng;
            self.blend_time_ms = rng.gen_range(1000..=3000);

            if self.height_end == 0.0 {
                let val: i32 = rng.gen_range(0..=15);
                self.height_end = -(val as f32);
            } else {
                self.height_end = 0.0;
            }

            self.height_begin = self.current_height;
            self.last_change_time_ms = current_time_ms;
        }

        let elapsed = current_time_ms.saturating_sub(self.last_change_time_ms);
        let progress = if self.blend_time_ms > 0 {
            (elapsed as f32) / (self.blend_time_ms as f32)
        } else {
            1.0
        };

        self.current_height = self.height_begin + (self.height_end - self.height_begin) * progress;
        self.current_height
    }
}
