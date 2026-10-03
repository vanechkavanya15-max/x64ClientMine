use glam::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PanningResult {
    pub total_volume: f32, // Distance-attenuated total volume
    pub pan: f32,          // -1.0 (Full Left) to 1.0 (Full Right)
    pub left_volume: f32,  // Scaled left channel volume
    pub right_volume: f32, // Scaled right channel volume
}

pub struct SpatialAudioPanner {
    max_distance: f32,
    reference_distance: f32,
}

impl SpatialAudioPanner {
    /// Creates a new SpatialAudioPanner with a specified maximum audible distance.
    pub fn new(max_distance: f32) -> Self {
        Self {
            max_distance,
            reference_distance: 1.0,
        }
    }

    /// Sets the reference distance up to which the volume is 100%.
    pub fn with_reference_distance(mut self, dist: f32) -> Self {
        self.reference_distance = dist;
        self
    }

    /// Calculates the panning and volume attenuation for a sound source.
    ///
    /// # Arguments
    /// * `listener_pos` - Position of the camera/listener.
    /// * `listener_forward` - Forward vector of the camera.
    /// * `listener_up` - Up vector of the camera.
    /// * `source_pos` - Position of the sound source.
    /// * `source_base_volume` - Base volume of the sound (typically 0.0 to 1.0).
    pub fn calculate(
        &self,
        listener_pos: Vec3,
        listener_forward: Vec3,
        listener_up: Vec3,
        source_pos: Vec3,
        source_base_volume: f32,
    ) -> PanningResult {
        let to_source = source_pos - listener_pos;
        let distance = to_source.length();

        // Calculate distance attenuation (Linear falloff)
        let mut volume = 0.0;
        if distance < self.max_distance {
            if distance <= self.reference_distance {
                volume = source_base_volume;
            } else {
                let attenuation = 1.0 - ((distance - self.reference_distance) / (self.max_distance - self.reference_distance));
                volume = source_base_volume * attenuation.max(0.0).min(1.0);
            }
        }

        // If source is at exact same position or completely attenuated, return center pan
        if distance < f32::EPSILON || volume <= 0.0 {
            // For center pan, equal power panning yields ~0.707 per channel
            // but for a 0 distance we might just want to output full volume equally.
            // Let's stick to equal power for consistency so left/right are volume * 0.707.
            // Wait, for 2D panning, center pan typically gives volume * cos(PI/4) = volume * 0.707.
            let center_factor = std::f32::consts::FRAC_PI_4.cos();
            return PanningResult {
                total_volume: volume,
                pan: 0.0,
                left_volume: volume * center_factor,
                right_volume: volume * center_factor,
            };
        }

        let dir_to_source = to_source / distance;
        let forward = listener_forward.normalize_or_zero();
        let up = listener_up.normalize_or_zero();
        
        // Calculate the listener's right vector
        let right = forward.cross(up).normalize_or_zero();

        // The dot product with the right vector gives the lateral position
        // -1.0 means exactly to the left, 1.0 means exactly to the right
        let pan = dir_to_source.dot(right).clamp(-1.0, 1.0);

        // Equal power panning
        // Mapping pan (-1.0 to 1.0) to angle (0.0 to PI/2)
        let pan_normalized = (pan + 1.0) * 0.5;
        let angle = pan_normalized * std::f32::consts::FRAC_PI_2;
        
        let left_volume = volume * angle.cos();
        let right_volume = volume * angle.sin();

        PanningResult {
            total_volume: volume,
            pan,
            left_volume,
            right_volume,
        }
    }
}
