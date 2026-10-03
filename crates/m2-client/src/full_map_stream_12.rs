use std::collections::{HashMap, HashSet};

use crate::terrain_sector_grid::global_to_sector;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AmbienceSound {
    Wind,
    River,
    Birds,
}

#[derive(Debug, Clone, Default)]
pub struct SectorEnvironment {
    pub has_water: bool,
    pub has_trees: bool,
}

pub struct MapAmbienceSoundTrigger {
    sectors: HashMap<(u32, u32), SectorEnvironment>,
    active_sounds: HashSet<AmbienceSound>,
}

impl MapAmbienceSoundTrigger {
    pub fn new() -> Self {
        let mut trigger = Self {
            sectors: HashMap::new(),
            active_sounds: HashSet::new(),
        };
        // Wind is always active by default
        trigger.active_sounds.insert(AmbienceSound::Wind);
        trigger
    }

    pub fn add_sector_environment(&mut self, col: u32, row: u32, env: SectorEnvironment) {
        self.sectors.insert((col, row), env);
    }

    pub fn update_player_position(&mut self, global_x: i32, global_y: i32, base_x: i32, base_y: i32) {
        let sector_coords = global_to_sector(global_x, global_y, base_x, base_y);
        
        self.active_sounds.clear();
        self.active_sounds.insert(AmbienceSound::Wind);

        if let Some(env) = self.sectors.get(&(sector_coords.col, sector_coords.row)) {
            if env.has_water {
                self.active_sounds.insert(AmbienceSound::River);
            }
            if env.has_trees {
                self.active_sounds.insert(AmbienceSound::Birds);
            }
        }
    }

    pub fn get_active_sounds(&self) -> &HashSet<AmbienceSound> {
        &self.active_sounds
    }
}

impl Default for MapAmbienceSoundTrigger {
    fn default() -> Self {
        Self::new()
    }
}
