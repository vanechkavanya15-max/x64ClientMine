use glam::{Mat4, Quat, Vec3};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MeshType {
    Ring = 0,
    Cylinder = 1,
    Plane = 2,
}

#[derive(Debug, Clone)]
pub struct EffectMesh {
    // Configuration
    pub mesh_type: MeshType,
    pub rotation_axis: Vec3,
    pub rotation_speed: f32, // Radians per second
    pub scale_start: f32,
    pub scale_end: f32,
    pub alpha_start: f32,
    pub alpha_end: f32,
    pub duration: f32,

    // State
    pub cur_time: f32,
    pub current_scale: f32,
    pub current_alpha: f32,
    pub current_rotation: f32,
}

impl EffectMesh {
    pub fn new(
        mesh_type: MeshType,
        rotation_axis: (f32, f32, f32),
        rotation_speed: f32,
        scale_start: f32,
        scale_end: f32,
        alpha_start: f32,
        alpha_end: f32,
        duration: f32,
    ) -> Self {
        let axis = Vec3::new(rotation_axis.0, rotation_axis.1, rotation_axis.2);
        let normalized_axis = if axis.length_squared() > 1e-6 {
            axis.normalize()
        } else {
            Vec3::Z // Default to Z-axis if zero vector provided
        };

        // Handle case where duration is <= 0 by clamping it to at least a tiny positive value
        let safe_duration = if duration <= 0.0 { 1e-6 } else { duration };

        Self {
            mesh_type,
            rotation_axis: normalized_axis,
            rotation_speed,
            scale_start,
            scale_end,
            alpha_start,
            alpha_end,
            duration: safe_duration,

            cur_time: 0.0,
            current_scale: scale_start,
            current_alpha: alpha_start,
            current_rotation: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if dt < 0.0 {
            return; // Ignore negative delta time
        }

        self.cur_time += dt;
        
        let progress = (self.cur_time / self.duration).clamp(0.0, 1.0);
        
        self.current_scale = self.lerp(self.scale_start, self.scale_end, progress);
        self.current_alpha = self.lerp(self.alpha_start, self.alpha_end, progress);
        self.current_rotation += self.rotation_speed * dt;
    }

    pub fn get_transform_matrix(&self, translation: (f32, f32, f32)) -> Mat4 {
        let translation_mat = Mat4::from_translation(Vec3::new(translation.0, translation.1, translation.2));
        let rotation_mat = Mat4::from_quat(Quat::from_axis_angle(self.rotation_axis, self.current_rotation));
        let scale_mat = Mat4::from_scale(Vec3::splat(self.current_scale));

        // Transform order: Translation * Rotation * Scale
        // In column-major right-to-left multiplication for vectors: v' = T * R * S * v
        translation_mat * rotation_mat * scale_mat
    }

    pub fn is_finished(&self) -> bool {
        self.cur_time >= self.duration
    }

    fn lerp(&self, start: f32, end: f32, t: f32) -> f32 {
        start + (end - start) * t
    }
}
