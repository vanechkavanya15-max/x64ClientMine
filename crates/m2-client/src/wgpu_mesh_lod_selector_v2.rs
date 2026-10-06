/// Selektor LOD modeli 3D i budynkow z uwzglednieniem odleglosci, histerezy i klatek animacji (Faza Pelnego Parytetu C++).
/// Wzorowany na `CGrannyLODController::UpdateLODLevel` z `ActorInstance.cpp` / `LODController.cpp`.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LodLevel {
    Lod0, // Wysoki poziom detali
    Lod1, // Sredni poziom detali
    Lod2, // Niski poziom detali / brak kosci
    Lod3, // Najnizszy poziom detali (jesli dostepny)
}

impl LodLevel {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => LodLevel::Lod0,
            1 => LodLevel::Lod1,
            2 => LodLevel::Lod2,
            _ => LodLevel::Lod3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LodSelectionResult {
    pub level: LodLevel,
    pub ani_fps: u32,
}

pub struct WgpuMeshLodSelectorV2 {
    pub lod_distance: f32,
    pub model_count: usize,
    pub min_lod_mode_enable: bool,
    pub hysteresis_threshold: f32,
    
    current_level: LodLevel,
}

pub const LOD_APPLY_MIN: f32 = 500.0;
pub const ANIFPS_MIN: u32 = 30;
pub const ANIFPS_MAX: u32 = 120;

impl WgpuMeshLodSelectorV2 {
    pub fn new(lod_distance: f32, model_count: usize) -> Self {
        Self {
            lod_distance,
            model_count: model_count.max(1),
            min_lod_mode_enable: false,
            hysteresis_threshold: 50.0,
            current_level: LodLevel::Lod0,
        }
    }

    pub fn set_current_level(&mut self, level: LodLevel) {
        self.current_level = level;
    }

    pub fn get_current_level(&self) -> LodLevel {
        self.current_level
    }

    pub fn update_lod_level(&mut self, distance_from_center: f32, distance_from_camera: f32) -> LodSelectionResult {
        if self.model_count <= 1 {
            return LodSelectionResult {
                level: LodLevel::Lod0,
                ani_fps: ANIFPS_MAX,
            };
        }

        let mut ani_fps;
        
        if distance_from_center > LOD_APPLY_MIN {
            let f_lod_factor = if self.lod_distance > 0.0 {
                (self.lod_distance - distance_from_camera).clamp(0.0, self.lod_distance)
            } else {
                0.0
            };
            
            if self.lod_distance > 0.0 {
                let range = (ANIFPS_MAX - ANIFPS_MIN) as f32;
                ani_fps = ((range * f_lod_factor / self.lod_distance) + ANIFPS_MIN as f32) as u32;
            } else {
                ani_fps = ANIFPS_MIN;
            }
            
            ani_fps = ani_fps.max(1);
            ani_fps = (ani_fps / 10) * 10;
            if ani_fps == 0 { 
                ani_fps = 10; 
            }
            
            let mut raw_lod: u8;
            
            if self.lod_distance <= 5000.0 {
                let h = self.hysteresis_threshold;
                raw_lod = match self.current_level {
                    LodLevel::Lod0 => {
                        if distance_from_camera > 500.0 + h {
                            if distance_from_camera > 1500.0 + h {
                                if distance_from_camera > 2500.0 + h { 3 } else { 2 }
                            } else { 1 }
                        } else { 0 }
                    }
                    LodLevel::Lod1 => {
                        if distance_from_camera < 500.0 - h { 0 }
                        else if distance_from_camera > 1500.0 + h {
                            if distance_from_camera > 2500.0 + h { 3 } else { 2 }
                        } else { 1 }
                    }
                    LodLevel::Lod2 => {
                        if distance_from_camera < 1500.0 - h {
                            if distance_from_camera < 500.0 - h { 0 } else { 1 }
                        } else if distance_from_camera > 2500.0 + h { 3 }
                        else { 2 }
                    }
                    LodLevel::Lod3 => {
                        if distance_from_camera < 2500.0 - h {
                            if distance_from_camera < 1500.0 - h {
                                if distance_from_camera < 500.0 - h { 0 } else { 1 }
                            } else { 2 }
                        } else { 3 }
                    }
                };
            } else {
                let lod_step = self.lod_distance / (self.model_count as f32);
                if lod_step > 0.0 {
                    raw_lod = (f_lod_factor / lod_step) as u8;
                } else {
                    raw_lod = 0;
                }
                
                let max_possible = (self.model_count.saturating_sub(1)) as u8;
                if raw_lod <= max_possible {
                    raw_lod = max_possible - raw_lod;
                } else {
                    raw_lod = 0;
                }
            }
            
            let max_lod = (self.model_count.saturating_sub(1)) as u8;
            if raw_lod > max_lod {
                raw_lod = max_lod;
            }
            
            if self.min_lod_mode_enable {
                raw_lod = 0;
            }
            
            self.current_level = LodLevel::from_u8(raw_lod);
        } else {
            ani_fps = ANIFPS_MAX;
            self.current_level = LodLevel::Lod0;
        }
        
        LodSelectionResult {
            level: self.current_level,
            ani_fps,
        }
    }
}
