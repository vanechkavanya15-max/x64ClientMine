#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FogMode {
    None,
    Linear,
    Exp2,
}

#[derive(Debug, Clone)]
pub struct WgpuFogManager {
    mode: FogMode,
    color: [f32; 4],
    min_fog: f32,
    sight_range: f32,
    density: f32,
}

impl Default for WgpuFogManager {
    fn default() -> Self {
        Self::new()
    }
}

impl WgpuFogManager {
    pub fn new() -> Self {
        Self {
            mode: FogMode::None,
            color: [1.0, 1.0, 1.0, 1.0],
            min_fog: 2400.0,
            sight_range: 25600.0,
            density: 0.001,
        }
    }

    pub fn set_mode(&mut self, mode: FogMode) {
        self.mode = mode;
    }

    pub fn get_mode(&self) -> FogMode {
        self.mode
    }

    pub fn set_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.color = [r, g, b, a];
    }

    pub fn get_color(&self) -> [f32; 4] {
        self.color
    }

    pub fn set_min_fog(&mut self, min_fog: f32) {
        self.min_fog = min_fog;
        if self.min_fog > self.sight_range {
            self.min_fog = self.sight_range;
        }
    }

    pub fn get_min_fog(&self) -> f32 {
        self.min_fog
    }

    pub fn set_sight_range(&mut self, sight_range: f32) {
        self.sight_range = sight_range;
        if self.min_fog > self.sight_range {
            self.min_fog = self.sight_range;
        }
    }

    pub fn get_sight_range(&self) -> f32 {
        self.sight_range
    }

    pub fn set_density(&mut self, density: f32) {
        self.density = density;
    }

    pub fn get_density(&self) -> f32 {
        self.density
    }

    pub fn get_shader_params(&self) -> [f32; 4] {
        let mode_val = match self.mode {
            FogMode::None => 0.0,
            FogMode::Linear => 1.0,
            FogMode::Exp2 => 2.0,
        };
        // Return [fog_start, fog_end, density, mode]
        [self.min_fog, self.sight_range, self.density, mode_val]
    }
}
