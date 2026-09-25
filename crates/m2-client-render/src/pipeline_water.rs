//! Modul pipeline_water dla skrzynki m2-client-render
//! Potok renderowania tafli wody z animacja fal i efektem Fresnela
//! Przesuniecie normalnych fal wodnych, odbicie swiatla i znieksztalcenie zalamania

/// Parametry konfiguracyjne dla wody (fale, Fresnel)
#[derive(Debug, Clone)]
pub struct WaterParams {
    pub wave_speed: f32,
    pub wave_amplitude: f32,
    pub fresnel_bias: f32,
    pub fresnel_scale: f32,
    pub fresnel_power: f32,
    pub refraction_distortion_factor: f32,
}

impl Default for WaterParams {
    fn default() -> Self {
        Self {
            wave_speed: 1.0,
            wave_amplitude: 0.1,
            fresnel_bias: 0.1,
            fresnel_scale: 1.0,
            fresnel_power: 2.0,
            refraction_distortion_factor: 0.05,
        }
    }
}

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct PipelineWater {
    pub initialized: bool,
    pub time: f32,
    pub params: WaterParams,
}

impl PipelineWater {
    pub fn new() -> Self {
        Self {
            initialized: true,
            time: 0.0,
            params: WaterParams::default(),
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Aktualizuje czas dla animacji fal
    pub fn update_time(&mut self, delta_time: f32) {
        self.time += delta_time;
    }

    /// Ustawia parametry wody
    pub fn set_params(&mut self, params: WaterParams) {
        self.params = params;
    }

    /// Oblicza przesuniecie fali (wave offset) bazujac na czasie i pozycji
    pub fn calculate_wave_offset(&self, x: f32, z: f32) -> f32 {
        let phase = self.time * self.params.wave_speed;
        let wave1 = (x * 0.5 + phase).sin();
        let wave2 = (z * 0.3 + phase * 1.2).cos();
        (wave1 + wave2) * self.params.wave_amplitude
    }

    /// Oblicza wspolczynnik odbicia Fresnela
    pub fn calculate_fresnel(&self, cos_theta: f32) -> f32 {
        let clamped_cos = cos_theta.clamp(0.0, 1.0);
        let reflection = self.params.fresnel_bias + self.params.fresnel_scale * (1.0 - clamped_cos).powf(self.params.fresnel_power);
        reflection.clamp(0.0, 1.0)
    }

    /// Oblicza znieksztalcenie zalamania (refraction distortion)
    pub fn calculate_refraction_distortion(&self, normal_x: f32, normal_z: f32) -> (f32, f32) {
        let factor = self.params.refraction_distortion_factor;
        (normal_x * factor, normal_z * factor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_pipeline_water() {
        let obj = PipelineWater::new();
        assert!(obj.is_ready());
    }
}
