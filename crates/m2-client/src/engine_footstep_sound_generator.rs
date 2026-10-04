use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub const ATTRIBUTE_WATER: u8 = 1 << 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FootstepSurfaceType {
    Grass,
    Stone,
    Mud,
    Water,
    Wood,
}

pub struct FootstepSoundGenerator;

impl FootstepSoundGenerator {
    /// Detects the type of surface under the character's feet based on terrain attributes and tile map.
    pub fn get_surface_type(attr_val: u8, tile_val: u8) -> FootstepSurfaceType {
        if (attr_val & ATTRIBUTE_WATER) != 0 {
            return FootstepSurfaceType::Water;
        }

        // Deterministic mapping of tile index to material.
        match tile_val {
            0..=5 => FootstepSurfaceType::Grass,
            6..=10 => FootstepSurfaceType::Stone,
            11..=15 => FootstepSurfaceType::Mud,
            16..=20 => FootstepSurfaceType::Wood,
            21..=25 => FootstepSurfaceType::Grass,
            _ => FootstepSurfaceType::Stone, // Default fallback
        }
    }

    /// Generates the path to the footstep sound file deterministically.
    pub fn generate_footstep_sound_path(
        x: f32,
        y: f32,
        attr_val: u8,
        tile_val: u8,
        time_ms: u32,
        is_pc: bool,
    ) -> String {
        let surface = Self::get_surface_type(attr_val, tile_val);
        
        // Deterministic variant selection based on position and time
        let mut hasher = DefaultHasher::new();
        x.to_bits().hash(&mut hasher);
        y.to_bits().hash(&mut hasher);
        time_ms.hash(&mut hasher);
        
        let hash_val = hasher.finish();
        // 4 variants: 01, 02, 03, 04
        let variant = (hash_val % 4) + 1; 

        let entity_type = if is_pc { "pc" } else { "mob" };

        let prefix = match surface {
            FootstepSurfaceType::Grass => "grass",
            FootstepSurfaceType::Stone => "stone",
            FootstepSurfaceType::Mud   => "mud",
            FootstepSurfaceType::Water => "water",
            FootstepSurfaceType::Wood  => "wood",
        };

        format!("sound/{}/walk/walk_{}_{:02}.wav", entity_type, prefix, variant)
    }

    /// Syncs foot impact with animation run/walk, returning sound if frame matches
    pub fn process_animation_footstep(
        current_frame: u32,
        event_frame: u32,
        x: f32,
        y: f32,
        attr_val: u8,
        tile_val: u8,
        time_ms: u32,
        is_pc: bool,
    ) -> Option<String> {
        if current_frame == event_frame {
            Some(Self::generate_footstep_sound_path(x, y, attr_val, tile_val, time_ms, is_pc))
        } else {
            None
        }
    }
}
