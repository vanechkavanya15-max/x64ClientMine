use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub const ATTRIBUTE_WATER: u8 = 1 << 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FootstepSurfaceType {
    Grass,
    Stone,
    Water,
    Dirt,
}

pub struct AudioFootstepMaterialSystem;

impl AudioFootstepMaterialSystem {
    /// Probkowanie atrybutu kafelka mapy pod stopami postaci (ATTRIBUTE_WATER, trawa, bruk, ziemia).
    pub fn get_surface_type(attr_val: u8, tile_val: u8) -> FootstepSurfaceType {
        if (attr_val & ATTRIBUTE_WATER) != 0 {
            return FootstepSurfaceType::Water;
        }

        // Proste deterministyczne mapowanie indeksu kafelka (tile) na material.
        match tile_val {
            0..=5 => FootstepSurfaceType::Grass,
            6..=15 => FootstepSurfaceType::Stone,
            16..=25 => FootstepSurfaceType::Dirt,
            _ => FootstepSurfaceType::Grass, // default fallback
        }
    }

    /// Wyzwalanie zdarzenia dzwiekowego w klatce postawienia stopy z animacji (.msa)
    /// z doborem losowego wariantu dzwieku kroku (np. footstep_grass_01..04.wav).
    pub fn trigger_footstep_event(
        x: f32,
        y: f32,
        attr_val: u8,
        tile_val: u8,
        time_ms: u32,
    ) -> String {
        let surface = Self::get_surface_type(attr_val, tile_val);
        
        // Deterministyczny dobor wariantu na podstawie pozycji i czasu (zeby uniknac mocks)
        let mut hasher = DefaultHasher::new();
        x.to_bits().hash(&mut hasher);
        y.to_bits().hash(&mut hasher);
        time_ms.hash(&mut hasher);
        
        let hash_val = hasher.finish();
        // 4 warianty: 01, 02, 03, 04
        let variant = (hash_val % 4) + 1; 

        let prefix = match surface {
            FootstepSurfaceType::Grass => "sound/common/walk/walk_grass",
            FootstepSurfaceType::Stone => "sound/common/walk/walk_stone",
            FootstepSurfaceType::Water => "sound/common/walk/walk_water",
            FootstepSurfaceType::Dirt  => "sound/common/walk/walk_dirt",
        };

        format!("{}_{:02}.wav", prefix, variant)
    }
}
