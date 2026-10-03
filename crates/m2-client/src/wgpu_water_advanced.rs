use glam::{Vec2, Vec3};

/// Advanced parameters for calculating water wave deformations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AdvancedWaveParams {
    pub amplitude: f32,
    pub frequency: f32,
    pub speed: f32,
    pub direction: Vec2,
}

impl Default for AdvancedWaveParams {
    fn default() -> Self {
        Self {
            amplitude: 1.0,
            frequency: 1.0,
            speed: 1.0,
            direction: Vec2::new(1.0, 0.0),
        }
    }
}

/// Calculates the wave height displacement at a given 2D position and time.
/// Uses a sine wave based on the dot product of position and direction.
pub fn calculate_advanced_wave_height(
    position: Vec2,
    time_sec: f32,
    params: &AdvancedWaveParams,
) -> f32 {
    let dir = params.direction.normalize_or_zero();
    let dot_prod = position.dot(dir);
    
    let phase = (dot_prod * params.frequency) + (time_sec * params.speed);
    
    phase.sin() * params.amplitude
}

/// Blends water color with skybox color using a Fresnel factor.
/// The fresnel_factor should be between 0.0 and 1.0.
pub fn blend_skybox_fresnel(
    water_color: Vec3,
    skybox_color: Vec3,
    fresnel_factor: f32,
) -> Vec3 {
    let f = fresnel_factor.clamp(0.0, 1.0);
    // Standard linear interpolation: water_color * (1 - f) + skybox_color * f
    water_color.lerp(skybox_color, f)
}

/// Calculates the alpha transparency for depth fade near shallows.
/// Fades out smoothly from max_depth (fully opaque, 1.0) to 0.0 depth (fully transparent, 0.0).
pub fn calculate_depth_fade(
    water_depth: f32,
    max_depth: f32,
) -> f32 {
    if max_depth <= 0.0 {
        return 1.0;
    }
    
    if water_depth <= 0.0 {
        return 0.0;
    }
    
    if water_depth >= max_depth {
        return 1.0;
    }
    
    // Smoothstep or linear fade: we'll use linear for simplicity, but clamp to ensure it's robust.
    (water_depth / max_depth).clamp(0.0, 1.0)
}
