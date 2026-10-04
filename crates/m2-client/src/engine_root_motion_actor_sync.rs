use glam::{Mat4, Vec3};

/// Synchronizuje przesuniecie kosci korzenia (Bip01) z pozycja aktora na mapie,
/// umozliwiajac plynne natarcia i skoki dyktowane przez animacje.
#[derive(Debug, Clone)]
pub struct RootMotionActorSync {
    pub accumulated_movement: Vec3,
    pub is_enabled: bool,
}

impl Default for RootMotionActorSync {
    fn default() -> Self {
        Self::new()
    }
}

impl RootMotionActorSync {
    pub fn new() -> Self {
        Self {
            accumulated_movement: Vec3::ZERO,
            is_enabled: true,
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.is_enabled = enabled;
    }

    pub fn clear(&mut self) {
        self.accumulated_movement = Vec3::ZERO;
    }

    /// Oblicza roznice w pozycji z dwoch macierzy (np. biezacej klatki vs poprzedniej) kosci Root.
    pub fn extract_root_bone_delta(prev_matrix: &Mat4, curr_matrix: &Mat4) -> Vec3 {
        let prev_translation = prev_matrix.w_axis.truncate();
        let curr_translation = curr_matrix.w_axis.truncate();
        curr_translation - prev_translation
    }

    /// Akumuluje obliczona delte przesuniecia po obroceniu jej wzgledem kierunku, w ktorym
    /// obrocony jest aktor na mapie. W engine Metin2 (i naszym engine) uzywana jest os Z jako "up".
    pub fn accumulate_movement(&mut self, local_movement: Vec3, actor_rotation_deg: f32) {
        if !self.is_enabled {
            return;
        }

        let rotation_rad = actor_rotation_deg.to_radians();
        let rotation_matrix = Mat4::from_rotation_z(rotation_rad);
        
        let world_movement = rotation_matrix.transform_vector3(local_movement);
        self.accumulated_movement += world_movement;
    }

    /// Aplikuje zgromadzone przesuniecie do docelowej pozycji aktora i czysci akumulator.
    pub fn apply_root_motion(&mut self, actor_position: &mut Vec3) {
        if self.is_enabled {
            *actor_position += self.accumulated_movement;
        }
        self.accumulated_movement = Vec3::ZERO;
    }

    /// Aplikuje przesuniecie bez bezposredniego mutowania, zwracajac nowa pozycje.
    pub fn apply_root_motion_and_clear(&mut self, actor_position: Vec3) -> Vec3 {
        let new_position = if self.is_enabled {
            actor_position + self.accumulated_movement
        } else {
            actor_position
        };
        self.accumulated_movement = Vec3::ZERO;
        new_position
    }
}
