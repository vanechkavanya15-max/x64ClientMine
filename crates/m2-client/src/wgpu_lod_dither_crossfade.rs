//! WGPU LOD Dither Crossfade Module
//!
//! Implementacja plynnego przejscia poziomow szczegolowosci (LOD Crossfade)
//! z wykorzystaniem ditheringu macierzy Bayera 4x4 (Screen-Door transparency).
//! Eliminuje zjawisko wyskakiwania modeli (popping artifacts) w scenie 3D.

/// Macierz Bayera 4x4 znormalizowana do zakresu [0.0, 1.0] (16 wartosci progowych).
pub const BAYER_MATRIX_4X4: [[f32; 4]; 4] = [
    [0.0 / 16.0, 8.0 / 16.0, 2.0 / 16.0, 10.0 / 16.0],
    [12.0 / 16.0, 4.0 / 16.0, 14.0 / 16.0, 6.0 / 16.0],
    [3.0 / 16.0, 11.0 / 16.0, 1.0 / 16.0, 9.0 / 16.0],
    [15.0 / 16.0, 7.0 / 16.0, 13.0 / 16.0, 5.0 / 16.0],
];

/// Progi odleglosci dla poszczegolnych poziomow LOD.
#[derive(Debug, Clone, Copy)]
pub struct LodDistanceThresholds {
    pub lod0_end: f32,
    pub lod1_start: f32,
    pub lod1_end: f32,
    pub lod2_start: f32,
    pub transition_band: f32,
}

impl Default for LodDistanceThresholds {
    fn default() -> Self {
        Self {
            lod0_end: 1500.0,
            lod1_start: 1200.0,
            lod1_end: 3500.0,
            lod2_start: 3000.0,
            transition_band: 300.0,
        }
    }
}

/// Stan przejscia LOD dla pojedynczego obiektu/aktora w kadrze.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LodCrossfadeState {
    pub active_lod: usize,
    pub target_lod: usize,
    pub blend_factor: f32, // 0.0 = czyste active_lod, 1.0 = pelne target_lod
    pub in_transition: bool,
}

impl LodCrossfadeState {
    pub fn new(active_lod: usize) -> Self {
        Self {
            active_lod,
            target_lod: active_lod,
            blend_factor: 0.0,
            in_transition: false,
        }
    }

    /// Oblicza stan przejscia na podstawie odleglosci kamery od obiektu.
    pub fn evaluate(distance: f32, thresholds: &LodDistanceThresholds) -> Self {
        let d = distance.max(0.0);

        if d <= thresholds.lod1_start {
            // Czysty LOD 0
            Self {
                active_lod: 0,
                target_lod: 0,
                blend_factor: 0.0,
                in_transition: false,
            }
        } else if d < thresholds.lod0_end {
            // Przejscie LOD 0 -> LOD 1
            let factor = ((d - thresholds.lod1_start) / (thresholds.lod0_end - thresholds.lod1_start)).clamp(0.0, 1.0);
            Self {
                active_lod: 0,
                target_lod: 1,
                blend_factor: factor,
                in_transition: true,
            }
        } else if d <= thresholds.lod2_start {
            // Czysty LOD 1
            Self {
                active_lod: 1,
                target_lod: 1,
                blend_factor: 0.0,
                in_transition: false,
            }
        } else if d < thresholds.lod1_end {
            // Przejscie LOD 1 -> LOD 2
            let factor = ((d - thresholds.lod2_start) / (thresholds.lod1_end - thresholds.lod2_start)).clamp(0.0, 1.0);
            Self {
                active_lod: 1,
                target_lod: 2,
                blend_factor: factor,
                in_transition: true,
            }
        } else {
            // Czysty LOD 2
            Self {
                active_lod: 2,
                target_lod: 2,
                blend_factor: 0.0,
                in_transition: false,
            }
        }
    }

    /// Sprawdza czy dany piksel ekranowy (pixel_x, pixel_y) powinien byc odrzucony
    /// (discard) w biezacym poziomie LOD przy uzyciu ditheringu Bayera.
    pub fn should_discard_pixel(&self, lod_index: usize, pixel_x: u32, pixel_y: u32) -> bool {
        if !self.in_transition {
            return lod_index != self.active_lod;
        }

        let bx = (pixel_x % 4) as usize;
        let by = (pixel_y % 4) as usize;
        let threshold = BAYER_MATRIX_4X4[by][bx];

        if lod_index == self.active_lod {
            // Schodzacy LOD: odrzucamy piksele gdy prog jest mniejszy niz blend_factor
            threshold < self.blend_factor
        } else if lod_index == self.target_lod {
            // Wchodzacy LOD: odrzucamy piksele gdy prog jest wiekszy lub rowny blend_factor
            threshold >= self.blend_factor
        } else {
            true
        }
    }
}

/// Generuje fragment kodu shaderowego WGSL do ditheringu w passie pikseli.
pub fn generate_wgsl_dither_function() -> &'static str {
    r#"
// Dithering Bayera 4x4 dla plynnego crossfade LOD
fn get_bayer_threshold(coord: vec2<u32>) -> f32 {
    let bayer = array<f32, 16>(
        0.0 / 16.0,  8.0 / 16.0,  2.0 / 16.0, 10.0 / 16.0,
        12.0 / 16.0, 4.0 / 16.0, 14.0 / 16.0,  6.0 / 16.0,
        3.0 / 16.0, 11.0 / 16.0,  1.0 / 16.0,  9.0 / 16.0,
        15.0 / 16.0, 7.0 / 16.0, 13.0 / 16.0,  5.0 / 16.0
    );
    let x = coord.x % 4u;
    let y = coord.y % 4u;
    return bayer[y * 4u + x];
}

fn apply_lod_dither_crossfade(frag_coord: vec4<f32>, blend_factor: f32, is_fade_in: bool) {
    if (blend_factor <= 0.0) { return; }
    let coord = vec2<u32>(u32(frag_coord.x), u32(frag_coord.y));
    let threshold = get_bayer_threshold(coord);
    if (is_fade_in) {
        if (threshold >= blend_factor) { discard; }
    } else {
        if (threshold < blend_factor) { discard; }
    }
}
"#
}
