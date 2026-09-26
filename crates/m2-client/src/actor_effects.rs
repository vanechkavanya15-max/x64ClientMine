use std::collections::HashMap;

pub const EFFECT_POISON: u32 = 1;
pub const EFFECT_STUN: u32 = 2;
pub const EFFECT_FIRE: u32 = 3;
pub const EFFECT_LEVEL_UP: u32 = 4;
pub const EFFECT_REFINED_PLUS7: u32 = 5;
pub const EFFECT_REFINED_PLUS8: u32 = 6;
pub const EFFECT_REFINED_PLUS9: u32 = 7;

#[derive(Debug, Default, Clone)]
pub struct ActorEffects {
    active_effects: HashMap<u32, f32>,
}

impl ActorEffects {
    pub fn new() -> Self {
        Self {
            active_effects: HashMap::new(),
        }
    }

    pub fn add_effect(&mut self, effect_flag: u32, duration: f32) {
        if duration > 0.0 {
            // Jesli efekt juz istnieje i nowy czas trwania jest dluzszy, nadpisz
            if let Some(current_duration) = self.active_effects.get(&effect_flag) {
                if duration > *current_duration {
                    self.active_effects.insert(effect_flag, duration);
                }
            } else {
                self.active_effects.insert(effect_flag, duration);
            }
        }
    }

    pub fn remove_effect(&mut self, effect_flag: u32) {
        self.active_effects.remove(&effect_flag);
    }

    pub fn has_effect(&self, effect_flag: u32) -> bool {
        self.active_effects.contains_key(&effect_flag)
    }

    pub fn get_remaining_time(&self, effect_flag: u32) -> Option<f32> {
        self.active_effects.get(&effect_flag).copied()
    }

    pub fn update(&mut self, delta_time: f32) {
        if delta_time < 0.0 {
            return;
        }

        self.active_effects.retain(|_, duration| {
            *duration -= delta_time;
            *duration > 0.0
        });
    }

    pub fn clear(&mut self) {
        self.active_effects.clear();
    }
}
