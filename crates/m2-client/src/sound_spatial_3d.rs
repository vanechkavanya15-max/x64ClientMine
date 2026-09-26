#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpatialSoundOutput {
    pub effective_volume: f32,
    pub pan: f32,
    pub is_audible: bool,
}

pub fn calculate_spatial_sound(
    sx: f32, sy: f32, sz: f32,
    cx: f32, cy: f32, cz: f32,
    yaw: f32,
) -> SpatialSoundOutput {
    let dx = sx - cx;
    let dy = sy - cy;
    let dz = sz - cz;
    
    let dist_sq = dx * dx + dy * dy + dz * dz;
    let dist = dist_sq.sqrt();
    
    if dist > 5000.0 {
        return SpatialSoundOutput {
            effective_volume: 0.0,
            pan: 0.0,
            is_audible: false,
        };
    }
    
    let attenuation = (1.0 - (dist / 5000.0)).clamp(0.0, 1.0);
    
    let rel_x = dx * (-yaw.sin()) + dy * yaw.cos();
    let pan = (rel_x / dist.max(1.0)).clamp(-1.0, 1.0);
    
    SpatialSoundOutput {
        effective_volume: attenuation,
        pan,
        is_audible: true,
    }
}
