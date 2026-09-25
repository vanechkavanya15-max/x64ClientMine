//! Modul `instance_effect` — Efekty wizualne przypiete do postaci (swiecenie +9, stany)
//! Wzorzec referencyjny C++: `source/UserInterface/InstanceBaseEffect.cpp`
//! Opis: Odwzorowanie efektow postaci: swiecenie zbroi i broni (+7, +8, +9), efekt awansu poziomu (Level Up), zatrucie (zielony), omdlenie (gwiazdki), kolo celu pod nogami.

use std::collections::HashMap;

// Typy efektow (EffectType) z C++ enum
pub const EFFECT_DUST: u32 = 0;
pub const EFFECT_STUN: u32 = 1;
pub const EFFECT_HIT: u32 = 2;
// ... (wybrane najwazniejsze)
pub const EFFECT_LEVELUP: u32 = 11;
pub const EFFECT_SKILLUP: u32 = 12;
pub const EFFECT_WEAPON: u32 = 23;
pub const EFFECT_AFFECT: u32 = 25;
pub const EFFECT_TARGET: u32 = 34;

// Typy stanow (AffectType) z C++ enum
pub const AFFECT_POISON: u32 = 3;
pub const AFFECT_SLOW: u32 = 4;
pub const AFFECT_STUN: u32 = 5;
pub const AFFECT_GEOMGYEONG: u32 = 15;
pub const AFFECT_CHEONGEUN: u32 = 16;
pub const AFFECT_REVIVE_INVISIBILITY: u32 = 27;

// Typy udoskonalen (RefinedEffectType)
pub const EFFECT_REFINED_NONE: u32 = 0;
pub const EFFECT_SWORD_REFINED7: u32 = 1;
pub const EFFECT_SWORD_REFINED8: u32 = 2;
pub const EFFECT_SWORD_REFINED9: u32 = 3;
pub const EFFECT_BODYARMOR_REFINED7: u32 = 16;
pub const EFFECT_BODYARMOR_REFINED8: u32 = 17;
pub const EFFECT_BODYARMOR_REFINED9: u32 = 18;

#[derive(Debug, Clone, Default)]
pub struct InstanceEffect {
    pub initialized: bool,
    
    // Slownik aktywnych efektow: Klucz to typ efektu (EffectKey), Wartosc to ID instancji (Handle)
    // Odpowiada SEffectContainer::Dict z C++
    pub active_effects: HashMap<u32, u32>,
    
    // Konfiguracja CRC efektow (symulacja rejestracji z C++)
    pub registered_effects_crc: HashMap<u32, u32>,
    pub registered_effects_bones: HashMap<u32, String>,
    
    // Licznik do generowania unikalnych uchwytow (Handle) efektow
    handle_counter: u32,
    
    // Specyficzne pola z C++
    geomgyeong_effect_handle: u32,
}

impl InstanceEffect {
    pub fn new() -> Self {
        Self {
            initialized: true,
            active_effects: HashMap::new(),
            registered_effects_crc: HashMap::new(),
            registered_effects_bones: HashMap::new(),
            handle_counter: 1,
            geomgyeong_effect_handle: 0,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }
    
    /// Rejestruje efekt (odpowiada RegisterEffect z C++)
    pub fn register_effect(&mut self, effect_type: u32, attach_bone: Option<&str>, crc: u32) {
        self.registered_effects_crc.insert(effect_type, crc);
        if let Some(bone) = attach_bone {
            self.registered_effects_bones.insert(effect_type, bone.to_string());
        }
    }

    /// Podpina efekt do postaci (odpowiada __AttachEffect z C++)
    pub fn attach_effect(&mut self, effect_type: u32) -> u32 {
        if !self.registered_effects_crc.contains_key(&effect_type) {
            // W C++ jesli efekt nie jest zarejestrowany, zwraca 0
            return 0; 
        }
        
        let handle = self.handle_counter;
        self.handle_counter += 1;
        
        // Zapisujemy instancje efektu
        self.active_effects.insert(effect_type, handle);
        handle
    }

    /// Odpina efekt po jego uchwycie/handle (odpowiada __DetachEffect)
    pub fn detach_effect(&mut self, effect_handle: u32) {
        // Znajdz klucz efektu po uchwycie
        let mut effect_key_to_remove = None;
        for (key, val) in self.active_effects.iter() {
            if *val == effect_handle {
                effect_key_to_remove = Some(*key);
                break;
            }
        }
        
        if let Some(key) = effect_key_to_remove {
            self.active_effects.remove(&key);
        }
    }

    /// Implementacja ukladania stanow (odpowiada __SetAffect z C++)
    pub fn set_affect(&mut self, affect: u32, is_visible: bool) {
        match affect {
            AFFECT_GEOMGYEONG => {
                if is_visible {
                    if self.geomgyeong_effect_handle != 0 {
                        self.detach_effect(self.geomgyeong_effect_handle);
                    }
                    self.geomgyeong_effect_handle = self.attach_effect(EFFECT_WEAPON + 1); // Symulacja WEAPON_ONEHAND
                } else {
                    self.detach_effect(self.geomgyeong_effect_handle);
                    self.geomgyeong_effect_handle = 0;
                }
                return;
            }
            AFFECT_STUN => {
                // SetSleep(isVisible) etc, symulacja dodania efektu STUN
            }
            _ => {}
        }
        
        let effect_key = EFFECT_AFFECT + affect;
        
        if is_visible {
            if !self.active_effects.contains_key(&effect_key) {
                self.attach_effect(effect_key);
            }
        } else {
            if let Some(handle) = self.active_effects.get(&effect_key).copied() {
                self.detach_effect(handle);
            }
        }
    }
    
    /// Symulacja wywolania LevelUp
    pub fn trigger_level_up(&mut self) {
        self.attach_effect(EFFECT_LEVELUP);
    }
    
    /// Ustawienie celu
    pub fn set_target(&mut self, is_visible: bool) {
        if is_visible {
            self.attach_effect(EFFECT_TARGET);
        } else {
            if let Some(handle) = self.active_effects.get(&EFFECT_TARGET).copied() {
                self.detach_effect(handle);
            }
        }
    }
    
    /// Obsluga efektow broni/zbroi
    pub fn set_refined_effect(&mut self, effect_type: u32) {
        // Usun ewentualne stare efekty udoskonalen
        let old_effects: Vec<u32> = self.active_effects.keys().copied()
            .filter(|&k| k >= EFFECT_SWORD_REFINED7 && k <= EFFECT_BODYARMOR_REFINED9)
            .collect();
            
        for key in old_effects {
            if let Some(handle) = self.active_effects.get(&key).copied() {
                self.detach_effect(handle);
            }
        }
        
        if effect_type != EFFECT_REFINED_NONE {
            self.attach_effect(effect_type);
        }
    }
}
