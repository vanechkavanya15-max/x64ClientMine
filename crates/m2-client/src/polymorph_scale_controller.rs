use glam::{Mat4, Vec3};

/// Controls the scaling of the skeleton matrix and collision radius
/// when an actor transforms into a monster (polymorph).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PolymorphScaleController {
    current_scale: f32,
    target_scale: f32,
    scale_speed: f32,
}

impl Default for PolymorphScaleController {
    fn default() -> Self {
        Self {
            current_scale: 1.0,
            target_scale: 1.0,
            scale_speed: 1.0,
        }
    }
}

impl PolymorphScaleController {
    /// Creates a new PolymorphScaleController with default scale of 1.0.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the scale factor directly without interpolation (instant scale).
    /// The scale factor is clamped to a minimum of 0.01 to avoid zero scaling issues.
    pub fn set_scale_instant(&mut self, scale: f32) {
        let clamped = scale.max(0.01);
        self.current_scale = clamped;
        self.target_scale = clamped;
    }

    /// Sets the target scale factor and the speed of scaling for interpolation.
    /// The scale factor and speed are clamped to a minimum of 0.01.
    pub fn set_target_scale(&mut self, scale: f32, speed: f32) {
        self.target_scale = scale.max(0.01);
        self.scale_speed = speed.max(0.01);
    }

    /// Updates the current scale towards the target scale based on delta time.
    /// Returns true if scaling is still in progress, false if it has reached the target.
    pub fn update(&mut self, delta_time: f32) -> bool {
        let diff = self.target_scale - self.current_scale;
        
        if diff.abs() < f32::EPSILON {
            self.current_scale = self.target_scale;
            return false;
        }

        let step = self.scale_speed * delta_time;
        
        if diff > 0.0 {
            self.current_scale += step;
            if self.current_scale > self.target_scale {
                self.current_scale = self.target_scale;
            }
        } else {
            self.current_scale -= step;
            if self.current_scale < self.target_scale {
                self.current_scale = self.target_scale;
            }
        }

        self.current_scale != self.target_scale
    }

    /// Gets the current scale factor.
    pub fn get_current_scale(&self) -> f32 {
        self.current_scale
    }

    /// Applies the current scale to a skeleton's world transform matrix.
    /// This scales the local coordinate space of the matrix without affecting its world translation.
    pub fn apply_scale_to_matrix(&self, matrix: &mut Mat4) {
        let scale_matrix = Mat4::from_scale(Vec3::splat(self.current_scale));
        // Multiply scale matrix on the right (local scale) to scale basis vectors,
        // keeping the translation unaffected.
        *matrix = *matrix * scale_matrix;
    }

    /// Calculates the scaled collision radius based on the original base radius.
    pub fn get_scaled_collision_radius(&self, base_radius: f32) -> f32 {
        base_radius * self.current_scale
    }
}
