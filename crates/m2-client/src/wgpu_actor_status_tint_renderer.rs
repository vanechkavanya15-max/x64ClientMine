#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActorNegativeStatus {
    None,
    Poison,
    Burn,
    Freeze,
    Slow,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StatusTint {
    pub add_color: [f32; 4],
    pub multiply_color: [f32; 4],
    pub animation_speed_multiplier: f32,
}

impl Default for StatusTint {
    fn default() -> Self {
        Self {
            add_color: [0.0, 0.0, 0.0, 0.0],
            multiply_color: [1.0, 1.0, 1.0, 1.0],
            animation_speed_multiplier: 1.0,
        }
    }
}

pub struct ActorStatusTintRenderer;

impl ActorStatusTintRenderer {
    /// Oblicza wartości tintu oraz modyfikatorów animacji w oparciu o aktywny status i czas.
    pub fn calculate_status_tint(status: ActorNegativeStatus, time_ms: u32) -> StatusTint {
        match status {
            ActorNegativeStatus::None => StatusTint::default(),
            ActorNegativeStatus::Poison => {
                // Trucizna: zielony tint, pulsujący w czasie
                // Pulsowanie za pomocą funkcji sinus. Przeliczamy czas na radiany.
                let pulse_frequency = 0.003; // regulacja prędkości pulsowania
                let time_sec = time_ms as f32 * pulse_frequency;
                let pulse = (time_sec.sin() * 0.5 + 0.5) * 0.5; // Zakres 0.0 - 0.5
                
                StatusTint {
                    add_color: [0.0, pulse, 0.0, 0.0],
                    multiply_color: [0.5, 1.0, 0.5, 1.0], // Zieleniste przyciemnienie
                    animation_speed_multiplier: 1.0,
                }
            }
            ActorNegativeStatus::Burn => {
                // Podpalenie: czerwono-pomarańczowy tint. Brak pulsowania lub szybkie migotanie.
                // Aby było bardziej statyczne, stałe wartości.
                StatusTint {
                    add_color: [0.3, 0.1, 0.0, 0.0],
                    multiply_color: [1.2, 0.8, 0.8, 1.0], // Podbicie czerwieni, zmniejszenie reszty
                    animation_speed_multiplier: 1.0,
                }
            }
            ActorNegativeStatus::Freeze | ActorNegativeStatus::Slow => {
                // Zamrożenie/Spowolnienie: Lodowato błękitny tint i spowolnienie animacji
                StatusTint {
                    add_color: [0.0, 0.2, 0.4, 0.0],
                    multiply_color: [0.6, 0.8, 1.0, 1.0],
                    animation_speed_multiplier: if status == ActorNegativeStatus::Freeze { 0.0 } else { 0.5 },
                }
            }
        }
    }
    
    /// Stosuje obliczone wartości tintu do bazowego koloru wierzchołka lub piksela.
    pub fn apply_to_vertex_color(base_color: [f32; 4], tint: &StatusTint) -> [f32; 4] {
        let r = (base_color[0] * tint.multiply_color[0] + tint.add_color[0]).clamp(0.0, 1.0);
        let g = (base_color[1] * tint.multiply_color[1] + tint.add_color[1]).clamp(0.0, 1.0);
        let b = (base_color[2] * tint.multiply_color[2] + tint.add_color[2]).clamp(0.0, 1.0);
        let a = (base_color[3] * tint.multiply_color[3] + tint.add_color[3]).clamp(0.0, 1.0);
        
        [r, g, b, a]
    }
}
