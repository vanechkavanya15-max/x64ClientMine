use glam::Vec3;

/// Calculates the water Fresnel term.
/// Schlick's approximation: R(theta) = R0 + (1 - R0) * (1 - cos(theta))^5
/// But here we also have bias and power parameters as specified in prompt, wait!
/// Prompt specified:
/// 1. calculate_water_fresnel(view_dir, normal, bias, power).
/// 2. calculate_water_foam(depth, shore_threshold, time_sec).
pub fn calculate_water_fresnel(view_dir: Vec3, normal: Vec3, bias: f32, power: f32) -> f32 {
    let cos_theta = view_dir.dot(normal).clamp(0.0, 1.0);
    let fresnel = bias + (1.0 - bias) * (1.0 - cos_theta).powf(power);
    fresnel.clamp(0.0, 1.0)
}

/// Calculates water foam based on depth, shore threshold, and time.
pub fn calculate_water_foam(depth: f32, shore_threshold: f32, time_sec: f32) -> f32 {
    if depth >= shore_threshold {
        return 0.0;
    }
    
    let depth_factor = if shore_threshold > 0.0 {
        1.0 - (depth / shore_threshold)
    } else {
        1.0
    };
    
    let time_fluctuation = (time_sec * 2.0).sin() * 0.1 + 0.9;
    
    (depth_factor * time_fluctuation).clamp(0.0, 1.0)
}
