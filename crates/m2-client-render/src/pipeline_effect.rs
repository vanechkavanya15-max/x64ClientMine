//! Modul pipeline_effect dla skrzynki m2-client-render
//! Potok renderowania czasteczek (Particles / Billboards)
//! Czworokaty zwrocone w strone kamery (billboards), tryb mieszania Additive i AlphaBlend

/// Tryb mieszania dla efektow czasteczkowych i billboardow.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    Additive,
    AlphaBlend,
}

impl Default for BlendMode {
    fn default() -> Self {
        Self::AlphaBlend
    }
}

/// Reprezentacja pojedynczego wierzcholka dla czworokata (billboard).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub uv: [f32; 2],
}

/// Definicja pojedynczego elementu billboard (czasteczka).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Billboard {
    pub position: [f32; 3],
    pub size: [f32; 2],
    pub color: [f32; 4],
    pub blend_mode: BlendMode,
}

impl Default for Billboard {
    fn default() -> Self {
        Self {
            position: [0.0; 3],
            size: [1.0, 1.0],
            color: [1.0; 4],
            blend_mode: BlendMode::default(),
        }
    }
}

impl Billboard {
    /// Oblicza 4 wierzcholki billboardu zwrocone w strone kamery.
    /// Zwraca tablice wierzcholkow w kolejnosci: lewy-dolny, prawy-dolny, prawy-gorny, lewy-gorny.
    pub fn compute_vertices(&self, camera_right: &[f32; 3], camera_up: &[f32; 3]) -> [Vertex; 4] {
        let half_w = self.size[0] * 0.5;
        let half_h = self.size[1] * 0.5;

        let px = self.position[0];
        let py = self.position[1];
        let pz = self.position[2];

        let rx = camera_right[0];
        let ry = camera_right[1];
        let rz = camera_right[2];

        let ux = camera_up[0];
        let uy = camera_up[1];
        let uz = camera_up[2];

        let mut v = [Vertex::default(); 4];

        // lewy-dolny (-right, -up)
        v[0].position = [
            px - rx * half_w - ux * half_h,
            py - ry * half_w - uy * half_h,
            pz - rz * half_w - uz * half_h,
        ];
        v[0].uv = [0.0, 1.0];
        v[0].color = self.color;

        // prawy-dolny (+right, -up)
        v[1].position = [
            px + rx * half_w - ux * half_h,
            py + ry * half_w - uy * half_h,
            pz + rz * half_w - uz * half_h,
        ];
        v[1].uv = [1.0, 1.0];
        v[1].color = self.color;

        // prawy-gorny (+right, +up)
        v[2].position = [
            px + rx * half_w + ux * half_h,
            py + ry * half_w + uy * half_h,
            pz + rz * half_w + uz * half_h,
        ];
        v[2].uv = [1.0, 0.0];
        v[2].color = self.color;

        // lewy-gorny (-right, +up)
        v[3].position = [
            px - rx * half_w + ux * half_h,
            py - ry * half_w + uy * half_h,
            pz - rz * half_w + uz * half_h,
        ];
        v[3].uv = [0.0, 0.0];
        v[3].color = self.color;

        v
    }
}

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct PipelineEffect {
    pub initialized: bool,
    pub billboards: Vec<Billboard>,
}

impl PipelineEffect {
    pub fn new() -> Self {
        Self {
            initialized: true,
            billboards: Vec::new(),
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Dodaje nowy billboard do potoku renderowania.
    pub fn add_billboard(&mut self, billboard: Billboard) {
        self.billboards.push(billboard);
    }

    /// Czysci liste billboardow w potoku.
    pub fn clear(&mut self) {
        self.billboards.clear();
    }

    /// Sortuje billboardy z uwzglednieniem trybu mieszania AlphaBlend.
    /// Czasteczki z AlphaBlend musza byc renderowane od najdalszych do najblizszych wzgledem kamery.
    pub fn sort_billboards(&mut self, camera_pos: &[f32; 3]) {
        self.billboards.sort_by(|a, b| {
            // Sortowanie tyczy sie glownie czasteczek AlphaBlend.
            // Additive mozemy zignorowac dla scislej kolejnosci, ale dla bezpieczenstwa posortujemy wszystkie.
            let dist_a = Self::dist_sq(&a.position, camera_pos);
            let dist_b = Self::dist_sq(&b.position, camera_pos);
            // Sortowanie od najwiekszego do najmniejszego dystansu (najdalsze najpierw)
            dist_b.partial_cmp(&dist_a).unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    /// Pomocnicza metoda do obliczania kwadratu dystansu miedzy punktami 3D.
    fn dist_sq(p1: &[f32; 3], p2: &[f32; 3]) -> f32 {
        let dx = p1[0] - p2[0];
        let dy = p1[1] - p2[1];
        let dz = p1[2] - p2[2];
        dx * dx + dy * dy + dz * dz
    }

    /// Przetwarza wszystkie billboardy w przygotowaniu do renderowania i zwraca wektor wierzcholkow.
    pub fn process_for_rendering(&self, camera_right: &[f32; 3], camera_up: &[f32; 3]) -> Vec<Vertex> {
        let mut vertices = Vec::with_capacity(self.billboards.len() * 4);
        for bb in &self.billboards {
            let bb_verts = bb.compute_vertices(camera_right, camera_up);
            vertices.extend_from_slice(&bb_verts);
        }
        vertices
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_pipeline_effect() {
        let obj = PipelineEffect::new();
        assert!(obj.is_ready());
    }
}
