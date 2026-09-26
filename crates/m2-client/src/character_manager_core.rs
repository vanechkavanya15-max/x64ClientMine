use std::collections::HashMap;

// Reprezentacja pojedynczej instancji postaci (InstanceBase)
#[derive(Debug, Clone)]
pub struct InstanceBase {
    pub vid: u32,
    pub race: u32,
    pub name: String,
    pub pos: (i32, i32),
}

impl InstanceBase {
    pub fn new(vid: u32, race: u32, name: String, pos: (i32, i32)) -> Self {
        Self { vid, race, name, pos }
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
    pub fn create_instance(&mut self, vid: u32, race: u32, name: String, pos: (i32, i32)) -> bool {
        if self.alive_instances.contains_key(&vid) {
            // Instancja o tym VID juz istnieje
            return false;
        }

        let instance = InstanceBase::new(vid, race, name, pos);
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
}
