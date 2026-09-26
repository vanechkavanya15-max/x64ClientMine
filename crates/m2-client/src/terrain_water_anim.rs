#[derive(Debug, Clone, PartialEq)]
pub struct TerrainWaterAnim {
    pub wave_frequency: f32,
    pub wave_amplitude: f32,
    pub uv_flow_speed: (f32, f32),
}

impl Default for TerrainWaterAnim {
    fn default() -> Self {
        Self {
            wave_frequency: 2.0,
            wave_amplitude: 10.0,
            uv_flow_speed: (0.05, 0.02),
        }
    }
}

impl TerrainWaterAnim {
    pub fn new(wave_frequency: f32, wave_amplitude: f32, uv_flow_speed: (f32, f32)) -> Self {
        Self {
            wave_frequency,
            wave_amplitude,
            uv_flow_speed,
        }
    }

    pub fn get_wave_height_offset(&self, local_x: f32, local_y: f32, cur_time_sec: f32) -> f32 {
        let phase = cur_time_sec * self.wave_frequency + local_x * 0.05 + local_y * 0.05;
        phase.sin() * self.wave_amplitude
    }

    pub fn get_water_uv(&self, base_u: f32, base_v: f32, cur_time_sec: f32) -> (f32, f32) {
        let u = (base_u + cur_time_sec * self.uv_flow_speed.0).fract();
        let v = (base_v + cur_time_sec * self.uv_flow_speed.1).fract();

        // Fix negative fract results if they happen
        let u = if u < 0.0 { u + 1.0 } else { u };
        let v = if v < 0.0 { v + 1.0 } else { v };

        (u, v)
    }
}
