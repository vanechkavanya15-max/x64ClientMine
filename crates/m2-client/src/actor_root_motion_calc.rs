use glam::{Vec3, Mat4};

/// Oblicza skumulowane przesuniecie root motion na podstawie wektora bazowego i rotacji postaci (Z-up).
pub fn calculate_accumulated_movement(base_movement: Vec3, rotation_deg: f32) -> Vec3 {
    let rotation_rad = rotation_deg.to_radians();
    // Metin2 uzywa osi Z jako "up"
    let rotation_matrix = Mat4::from_rotation_z(rotation_rad);
    rotation_matrix.transform_vector3(base_movement)
}

/// Struktura reprezentujaca pozycje i ruch aktora dla root motion
#[derive(Debug, Clone, Default)]
pub struct ActorRootMotionState {
    pub position: Vec3,
}

impl ActorRootMotionState {
    pub fn new(position: Vec3) -> Self {
        Self { position }
    }

    /// Sumuje wektor do pozycji, umozliwiajac plynne doskoki i szarze
    pub fn accumulate_movement(&mut self, movement: Vec3) {
        self.position += movement;
    }
}
