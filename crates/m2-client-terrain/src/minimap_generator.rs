//! Modul minimap_generator dla skrzynki m2-client-terrain
//! Generator kafelkow minimapy z wysokosci i tekstur terenu
//! Tworzenie kafelka minimapy 256x256 dla zadanego sektora mapy

pub const MINIMAP_WIDTH: usize = 256;
pub const MINIMAP_HEIGHT: usize = 256;
pub const PIXELS_COUNT: usize = MINIMAP_WIDTH * MINIMAP_HEIGHT;
pub const OUTPUT_RGB_SIZE: usize = PIXELS_COUNT * 3;

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct MinimapGenerator {
    pub initialized: bool,
}

impl MinimapGenerator {
    pub fn new() -> Self {
        Self { initialized: true }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Generuje kafelek minimapy w formacie RGB (3 bajty na piksel)
    /// na podstawie podanej mapy wysokosci i opcjonalnej mapy kolorow (base color/texture map).
    pub fn generate_tile(&self, height_data: &[u16], color_map: Option<&[u8]>) -> Result<Vec<u8>, &'static str> {
        if height_data.len() != PIXELS_COUNT {
            return Err("Nieprawidlowy rozmiar mapy wysokosci (wymagane 256x256)");
        }

        if let Some(cmap) = color_map {
            if cmap.len() != OUTPUT_RGB_SIZE {
                return Err("Nieprawidlowy rozmiar mapy kolorow (wymagane 256x256x3)");
            }
        }

        let mut output = vec![0u8; OUTPUT_RGB_SIZE];

        for y in 0..MINIMAP_HEIGHT {
            for x in 0..MINIMAP_WIDTH {
                let idx = y * MINIMAP_WIDTH + x;
                
                // Oblicz cieniowanie na podstawie wysokosci w stosunku do sasiada (np. w lewo/w gore)
                let h_current = height_data[idx] as f32;
                
                // Proste cieniowanie za pomoca pochodnej
                let mut shading: f32 = 1.0;
                if x > 0 && y > 0 {
                    let h_left = height_data[y * MINIMAP_WIDTH + (x - 1)] as f32;
                    let h_top = height_data[(y - 1) * MINIMAP_WIDTH + x] as f32;
                    
                    let dx = h_current - h_left;
                    let dy = h_current - h_top;
                    
                    // Skalar cieniowania bazujacy na symulowanym swietle padajacym z gory po przekatnej
                    let slope = (dx + dy) * 0.1;
                    
                    // Ograniczenie cieniowania do rozsadnych granic
                    shading = (1.0 + slope).clamp(0.5, 1.5);
                }

                let out_idx = idx * 3;
                
                if let Some(cmap) = color_map {
                    // Nakladanie cieniowania na mape kolorow
                    output[out_idx] = (cmap[out_idx] as f32 * shading).clamp(0.0, 255.0) as u8;
                    output[out_idx + 1] = (cmap[out_idx + 1] as f32 * shading).clamp(0.0, 255.0) as u8;
                    output[out_idx + 2] = (cmap[out_idx + 2] as f32 * shading).clamp(0.0, 255.0) as u8;
                } else {
                    // Brak mapy kolorow: generowanie obrazu szarego zaleznego od wysokosci i cieniowania
                    let base_gray = (h_current / 256.0).clamp(0.0, 255.0);
                    let final_color = (base_gray * shading).clamp(0.0, 255.0) as u8;
                    
                    output[out_idx] = final_color;
                    output[out_idx + 1] = final_color;
                    output[out_idx + 2] = final_color;
                }
            }
        }

        Ok(output)
    }
}
