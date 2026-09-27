#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AudioListener3D {
    pub pos: [f32; 3],
    pub forward: [f32; 3],
    pub up: [f32; 3],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AudioSource3D {
    pub pos: [f32; 3],
    pub min_distance: f32,
    pub max_distance: f32,
    pub base_volume: f32,
}

pub fn calculate_attenuation(listener: &AudioListener3D, source: &AudioSource3D) -> f32 {
    let dx = source.pos[0] - listener.pos[0];
    let dy = source.pos[1] - listener.pos[1];
    let dz = source.pos[2] - listener.pos[2];

    let distance = (dx * dx + dy * dy + dz * dz).sqrt();

    if distance <= source.min_distance {
        source.base_volume
    } else if distance >= source.max_distance {
        0.0
    } else {
        let range = source.max_distance - source.min_distance;
        if range <= 0.0 {
            return 0.0;
        }
        let factor = 1.0 - ((distance - source.min_distance) / range);
        source.base_volume * factor
    }
}

pub fn calculate_stereo_panning(listener: &AudioListener3D, source: &AudioSource3D) -> (f32, f32) {
    // 1. Calculate the 'right' vector from the listener's forward and up vectors
    // right = forward x up
    let right_x = listener.forward[1] * listener.up[2] - listener.forward[2] * listener.up[1];
    let right_y = listener.forward[2] * listener.up[0] - listener.forward[0] * listener.up[2];
    let right_z = listener.forward[0] * listener.up[1] - listener.forward[1] * listener.up[0];

    // Normalize right vector
    let right_len = (right_x * right_x + right_y * right_y + right_z * right_z).sqrt();
    let (right_x, right_y, right_z) = if right_len > 1e-6 {
        (right_x / right_len, right_y / right_len, right_z / right_len)
    } else {
        (1.0, 0.0, 0.0) // Fallback if forward and up are collinear
    };

    // 2. Calculate the vector from listener to source
    let dx = source.pos[0] - listener.pos[0];
    let dy = source.pos[1] - listener.pos[1];
    let dz = source.pos[2] - listener.pos[2];

    // Normalize direction vector
    let dist = (dx * dx + dy * dy + dz * dz).sqrt();
    let (dir_x, dir_y, dir_z) = if dist > 1e-6 {
        (dx / dist, dy / dist, dz / dist)
    } else {
        (0.0, 0.0, 0.0) // Source is at exactly the same position as listener
    };

    // 3. Dot product of 'right' and 'direction' to find left/right positioning
    // Returns value between -1.0 (fully left) and 1.0 (fully right)
    let dot = dir_x * right_x + dir_y * right_y + dir_z * right_z;
    
    // Clamp dot product to [-1.0, 1.0] to prevent floating point inaccuracies from going out of bounds
    let pan = dot.clamp(-1.0, 1.0);

    // 4. Constant power panning
    // We map pan [-1, 1] to angle [0, PI/2]
    // left: cos(angle), right: sin(angle)
    // pan = -1 -> angle = 0 -> left = 1, right = 0
    // pan = 0  -> angle = PI/4 -> left = sqrt(2)/2, right = sqrt(2)/2
    // pan = 1  -> angle = PI/2 -> left = 0, right = 1
    
    let angle = (pan + 1.0) * std::f32::consts::FRAC_PI_4;
    let left = angle.cos();
    let right = angle.sin();

    (left, right)
}
