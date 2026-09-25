//! Modul pipeline_shadow dla skrzynki m2-client-render
//! Potok rzucania cieni slonecznych (Shadow Mapping)
//! Generowanie mapy glebi z perspektywy slonca, rzutowanie cienia na teren i postacie

/// Typ reprezentujacy macierz 4x4
pub type Matrix4x4 = [[f32; 4]; 4];

/// Glowna struktura implementacyjna
#[derive(Debug, Clone)]
pub struct PipelineShadow {
    pub initialized: bool,
    pub resolution: u32,
    pub sun_position: [f32; 3],
    pub sun_target: [f32; 3],
    pub light_view_proj: Matrix4x4,
}

impl Default for PipelineShadow {
    fn default() -> Self {
        Self::new(1024)
    }
}

impl PipelineShadow {
    /// Tworzy nowy potok cieni
    pub fn new(resolution: u32) -> Self {
        let identity_matrix = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        Self {
            initialized: true,
            resolution,
            sun_position: [0.0, 0.0, 0.0],
            sun_target: [0.0, 0.0, 0.0],
            light_view_proj: identity_matrix,
        }
    }

    /// Zwraca true jesli potok jest gotowy
    pub fn is_ready(&self) -> bool {
        self.initialized && self.resolution > 0
    }

    /// Ustawia parametry slonca
    pub fn set_sun_parameters(&mut self, pos: [f32; 3], target: [f32; 3]) {
        self.sun_position = pos;
        self.sun_target = target;
    }

    /// Aktualizuje macierze potoku
    pub fn update_matrices(&mut self) {
        // Symulacja aktualizacji macierzy w trybie headless.
        // W prawdziwym srodowisku uzylibysmy wlasciwych obliczen na macierzach.
        self.light_view_proj = [
            [2.0, 0.0, 0.0, 0.0],
            [0.0, 2.0, 0.0, 0.0],
            [0.0, 0.0, 2.0, 0.0],
            [self.sun_position[0], self.sun_position[1], self.sun_position[2], 1.0],
        ];
    }

    /// Generuje mape glebi (symulacja)
    pub fn generate_depth_map(&self) -> bool {
        // Sprawdzenie czy potok jest gotowy
        if !self.is_ready() {
            return false;
        }
        // W trybie headless zwracamy sukces
        true
    }

    /// Zwraca macierz widoku rzutowania
    pub fn get_light_view_proj(&self) -> Matrix4x4 {
        self.light_view_proj
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_pipeline_shadow() {
        let mut obj = PipelineShadow::new(2048);
        assert!(obj.is_ready());
        assert_eq!(obj.resolution, 2048);

        obj.set_sun_parameters([10.0, 20.0, 30.0], [0.0, 0.0, 0.0]);
        assert_eq!(obj.sun_position, [10.0, 20.0, 30.0]);
        assert_eq!(obj.sun_target, [0.0, 0.0, 0.0]);

        obj.update_matrices();
        let mat = obj.get_light_view_proj();
        assert_eq!(mat[3][0], 10.0);
        assert_eq!(mat[3][1], 20.0);
        assert_eq!(mat[3][2], 30.0);

        assert!(obj.generate_depth_map());
    }
}
