//! Pet system (Zwierzaki): summoning, feeding, exp progression, evolution stages, and passive stat bonuses.

use std::collections::HashMap;
use std::ffi::{CStr, c_char};
use std::sync::{Mutex, OnceLock};
use m2_client_bridge::ffi_guard::safe_ffi_call;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PetEvolutionStage {
    Young = 0,    // Lv 1 - 40
    Wild = 1,     // Lv 41 - 80
    Brave = 2,    // Lv 81 - 105
    Heroic = 3,   // Lv 106 - 120
}

impl From<u8> for PetEvolutionStage {
    fn from(val: u8) -> Self {
        match val {
            0 => PetEvolutionStage::Young,
            1 => PetEvolutionStage::Wild,
            2 => PetEvolutionStage::Brave,
            _ => PetEvolutionStage::Heroic,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PetInstance {
    pub pet_id: u32,
    pub vnum: u32,
    pub name: String,
    pub level: u32,
    pub exp: u64,
    pub stage: PetEvolutionStage,
    pub duration_seconds: u32,
    pub max_duration_seconds: u32,
    pub hp_bonus_pct: f32,
    pub def_bonus_pct: f32,
}

impl PetInstance {
    pub fn new(pet_id: u32, vnum: u32, name: String, level: u32) -> Self {
        let mut pet = Self {
            pet_id,
            vnum,
            name,
            level: level.clamp(1, 120),
            exp: 0,
            stage: PetEvolutionStage::Young,
            duration_seconds: 86400,     // 1 day default
            max_duration_seconds: 604800, // 7 days max
            hp_bonus_pct: 1.0,
            def_bonus_pct: 1.0,
        };
        pet.update_stage_from_level();
        pet.recalculate_bonuses();
        pet
    }

    pub fn update_stage_from_level(&mut self) {
        if self.level <= 40 {
            self.stage = PetEvolutionStage::Young;
        } else if self.level <= 80 {
            self.stage = PetEvolutionStage::Wild;
        } else if self.level <= 105 {
            self.stage = PetEvolutionStage::Brave;
        } else {
            self.stage = PetEvolutionStage::Heroic;
        }
    }

    pub fn recalculate_bonuses(&mut self) {
        let base_mult = match self.stage {
            PetEvolutionStage::Young => 1.0,
            PetEvolutionStage::Wild => 1.5,
            PetEvolutionStage::Brave => 2.0,
            PetEvolutionStage::Heroic => 3.0,
        };
        self.hp_bonus_pct = (self.level as f32 * 0.1) * base_mult;
        self.def_bonus_pct = (self.level as f32 * 0.08) * base_mult;
    }

    pub fn feed(&mut self, add_seconds: u32) {
        self.duration_seconds = (self.duration_seconds + add_seconds).min(self.max_duration_seconds);
    }

    pub fn evolve(&mut self) -> Result<PetEvolutionStage, &'static str> {
        match self.stage {
            PetEvolutionStage::Young if self.level >= 40 => {
                self.stage = PetEvolutionStage::Wild;
                self.recalculate_bonuses();
                Ok(self.stage)
            }
            PetEvolutionStage::Wild if self.level >= 80 => {
                self.stage = PetEvolutionStage::Brave;
                self.recalculate_bonuses();
                Ok(self.stage)
            }
            PetEvolutionStage::Brave if self.level >= 105 => {
                self.stage = PetEvolutionStage::Heroic;
                self.recalculate_bonuses();
                Ok(self.stage)
            }
            _ => Err("Level too low for next evolution stage"),
        }
    }
}

#[derive(Debug, Default)]
pub struct PetManager {
    pub active_pets: HashMap<u32, PetInstance>,
    pub summoned_pet_id: Option<u32>,
    pub next_pet_id: u32,
}

static PET_MANAGER: OnceLock<Mutex<PetManager>> = OnceLock::new();

pub fn get_pet_manager() -> &'static Mutex<PetManager> {
    PET_MANAGER.get_or_init(|| Mutex::new(PetManager::default()))
}

// C-FFI exports
#[no_mangle]
pub extern "C" fn m2_pet_summon(vnum: u32, name: *const c_char, level: u32) -> u32 {
    safe_ffi_call(0, || {
        let pet_name = if !name.is_null() {
            unsafe { CStr::from_ptr(name) }.to_string_lossy().into_owned()
        } else {
            "Pet".to_string()
        };

        let mut mgr = get_pet_manager().lock().unwrap();
        mgr.next_pet_id += 1;
        let pet_id = mgr.next_pet_id;
        let pet = PetInstance::new(pet_id, vnum, pet_name, level);
        mgr.active_pets.insert(pet_id, pet);
        mgr.summoned_pet_id = Some(pet_id);
        pet_id
    })
}

#[no_mangle]
pub extern "C" fn m2_pet_dismiss(pet_id: u32) -> bool {
    safe_ffi_call(false, || {
        let mut mgr = get_pet_manager().lock().unwrap();
        if mgr.summoned_pet_id == Some(pet_id) {
            mgr.summoned_pet_id = None;
        }
        mgr.active_pets.remove(&pet_id).is_some()
    })
}

#[no_mangle]
pub extern "C" fn m2_pet_feed(pet_id: u32, duration_add: u32) -> bool {
    safe_ffi_call(false, || {
        let mut mgr = get_pet_manager().lock().unwrap();
        if let Some(pet) = mgr.active_pets.get_mut(&pet_id) {
            pet.feed(duration_add);
            true
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_pet_evolve(pet_id: u32) -> u8 {
    safe_ffi_call(0, || {
        let mut mgr = get_pet_manager().lock().unwrap();
        if let Some(pet) = mgr.active_pets.get_mut(&pet_id) {
            match pet.evolve() {
                Ok(new_stage) => new_stage as u8,
                Err(_) => pet.stage as u8,
            }
        } else {
            0
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_pet_get_stage(pet_id: u32) -> u8 {
    safe_ffi_call(0, || {
        let mgr = get_pet_manager().lock().unwrap();
        mgr.active_pets.get(&pet_id).map(|p| p.stage as u8).unwrap_or(0)
    })
}

#[no_mangle]
pub extern "C" fn m2_pet_get_level(pet_id: u32) -> u32 {
    safe_ffi_call(0, || {
        let mgr = get_pet_manager().lock().unwrap();
        mgr.active_pets.get(&pet_id).map(|p| p.level).unwrap_or(0)
    })
}
