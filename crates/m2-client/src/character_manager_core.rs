use m2_client_bridge::ffi_guard::safe_ffi_call;
use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

// Reprezentacja pojedynczej instancji postaci (InstanceBase)
#[derive(Debug, Clone)]
pub struct InstanceBase {
    pub vid: u32,
    pub race: u16,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub name: String,
    pub active_loop_motion: Option<u32>,
    pub is_dead: bool,
}

impl InstanceBase {
    pub fn new(vid: u32, race: u16, x: f32, y: f32, z: f32) -> Self {
        Self {
            vid,
            race,
            x,
            y,
            z,
            name: String::new(),
            active_loop_motion: None,
            is_dead: false,
        }
    }
}

// Centralny menedzer wszystkich postaci w pamieci
#[derive(Debug, Default)]
pub struct CharacterManagerCore {
    // Mapa aktywnych postaci
    pub alive_instances: HashMap<u32, InstanceBase>,
    // Wskaznik glownej postaci (VID)
    pub main_actor_vid: Option<u32>,
}

impl CharacterManagerCore {
    // Tworzy nowa instancje menedzera
    pub fn new() -> Self {
        Self {
            alive_instances: HashMap::new(),
            main_actor_vid: None,
        }
    }

    // Tworzy nowa postac, zwraca true jesli pomyslnie dodano
    pub fn create_instance(&mut self, vid: u32, race: u16, x: f32, y: f32, z: f32) -> bool {
        if self.alive_instances.contains_key(&vid) {
            // Instancja o tym VID juz istnieje
            return false;
        }

        let instance = InstanceBase::new(vid, race, x, y, z);
        self.alive_instances.insert(vid, instance);
        true
    }

    pub fn create_instance_full(
        &mut self,
        vid: u32,
        race: u16,
        x: f32,
        y: f32,
        z: f32,
        name: String,
        is_dead: bool,
    ) -> bool {
        if self.alive_instances.contains_key(&vid) {
            return false;
        }

        let mut instance = InstanceBase::new(vid, race, x, y, z);
        instance.name = name;
        instance.is_dead = is_dead;
        self.alive_instances.insert(vid, instance);
        true
    }

    // Usuwa postac o podanym VID, zwraca true jesli usunieto
    pub fn delete_instance(&mut self, vid: u32) -> bool {
        if let Some(main_vid) = self.main_actor_vid {
            if main_vid == vid {
                self.main_actor_vid = None;
            }
        }

        self.alive_instances.remove(&vid).is_some()
    }

    // Pobiera niemutowalna referencje do instancji o podanym VID
    pub fn get_instance(&self, vid: u32) -> Option<&InstanceBase> {
        self.alive_instances.get(&vid)
    }

    // Pobiera mutowalna referencje do instancji o podanym VID
    pub fn get_instance_mut(&mut self, vid: u32) -> Option<&mut InstanceBase> {
        self.alive_instances.get_mut(&vid)
    }

    // Pobiera niemutowalna referencje do glownej instancji
    pub fn get_main_instance(&self) -> Option<&InstanceBase> {
        if let Some(main_vid) = self.main_actor_vid {
            self.get_instance(main_vid)
        } else {
            None
        }
    }

    // Zwraca ilosc aktywnych instancji
    pub fn count(&self) -> usize {
        self.alive_instances.len()
    }

    // Czysci menedzer usuwajac wszystkie instancje
    pub fn clear(&mut self) {
        self.alive_instances.clear();
        self.main_actor_vid = None;
    }
}

/// Globalny stan przechowujacy CharacterManagerCore, chroniony przez RwLock.
static CHARACTER_MANAGER: OnceLock<RwLock<CharacterManagerCore>> = OnceLock::new();

pub fn get_character_manager() -> &'static RwLock<CharacterManagerCore> {
    CHARACTER_MANAGER.get_or_init(|| RwLock::new(CharacterManagerCore::new()))
}

#[no_mangle]
pub extern "C" fn m2_chr_mgr_create_instance(
    vid: u32,
    race: u16,
    x: f32,
    y: f32,
    z: f32,
) -> bool {
    safe_ffi_call(false, || {
        let mut mgr = match get_character_manager().write() {
            Ok(lock) => lock,
            Err(_) => return false,
        };
        mgr.create_instance(vid, race, x, y, z)
    })
}

#[no_mangle]
pub extern "C" fn m2_chr_mgr_delete_instance(vid: u32) -> bool {
    safe_ffi_call(false, || {
        let mut mgr = match get_character_manager().write() {
            Ok(lock) => lock,
            Err(_) => return false,
        };
        mgr.delete_instance(vid)
    })
}

#[no_mangle]
pub extern "C" fn m2_chr_mgr_get_count() -> u32 {
    safe_ffi_call(0, || {
        let mgr = match get_character_manager().read() {
            Ok(lock) => lock,
            Err(_) => return 0,
        };
        mgr.count() as u32
    })
}
