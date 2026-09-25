//! Modul `race_manager` — Zarzadca ras grywalnych i potworow CRaceManager
//! Wzorzec referencyjny C++: `source/GameLib/RaceManager.cpp`
//! Opis: Odwzorowanie CRaceManager: rejestr mapujacy ID rasy (0..7 wojownik/ninja/sura/szaman oraz ID potworow i NPC) na obiekty CRaceData.

use std::collections::HashMap;
use crate::race_data::RaceData;

pub fn is_guild_race(race: u32) -> bool {
    if race >= 14000 && race < 15000 {
        return true;
    }
    if race == 20043 {
        return true;
    }
    false
}

pub fn is_npc_race(race: u32) -> bool {
    race > 9000
}

pub fn get_race_resource_pathes(race: u32) -> Vec<String> {
    let mut pathes = Vec::new();

    if is_guild_race(race) {
        pathes.push("d:/ymir work/guild/".to_string());
        pathes.push("d:/ymir work/npc/".to_string());
        pathes.push("d:/ymir work/npc2/".to_string());
        pathes.push("d:/ymir work/monster/".to_string());
        pathes.push("d:/ymir work/monster2/".to_string());
    } else if is_npc_race(race) {
        if race >= 30000 {
            pathes.push("d:/ymir work/npc2/".to_string());
            pathes.push("d:/ymir work/npc/".to_string());
            pathes.push("d:/ymir work/monster/".to_string());
            pathes.push("d:/ymir work/monster2/".to_string());
            pathes.push("d:/ymir work/guild/".to_string());
        } else {
            pathes.push("d:/ymir work/npc/".to_string());
            pathes.push("d:/ymir work/npc2/".to_string());
            pathes.push("d:/ymir work/monster/".to_string());
            pathes.push("d:/ymir work/monster2/".to_string());
            pathes.push("d:/ymir work/guild/".to_string());
        }
    } else if race == 8507 || race == 8510 {
        pathes.push("d:/ymir work/monster2/".to_string());
        pathes.push("d:/ymir work/monster/".to_string());
        pathes.push("d:/ymir work/npc/".to_string());
        pathes.push("d:/ymir work/npc2/".to_string());
        pathes.push("d:/ymir work/guild/".to_string());
    } else if race > 8000 {
        pathes.push("d:/ymir work/monster/".to_string());
        pathes.push("d:/ymir work/monster2/".to_string());
        pathes.push("d:/ymir work/npc/".to_string());
        pathes.push("d:/ymir work/npc2/".to_string());
        pathes.push("d:/ymir work/guild/".to_string());
    } else if race > 2000 {
        pathes.push("d:/ymir work/monster2/".to_string());
        pathes.push("d:/ymir work/monster/".to_string());
        pathes.push("d:/ymir work/npc/".to_string());
        pathes.push("d:/ymir work/npc2/".to_string());
        pathes.push("d:/ymir work/guild/".to_string());
    } else if race >= 1400 && race <= 1700 {
        pathes.push("d:/ymir work/monster2/".to_string());
        pathes.push("d:/ymir work/monster/".to_string());
        pathes.push("d:/ymir work/npc/".to_string());
        pathes.push("d:/ymir work/npc2/".to_string());
        pathes.push("d:/ymir work/guild/".to_string());
    } else {
        pathes.push("d:/ymir work/monster/".to_string());
        pathes.push("d:/ymir work/monster2/".to_string());
        pathes.push("d:/ymir work/npc/".to_string());
        pathes.push("d:/ymir work/npc2/".to_string());
        pathes.push("d:/ymir work/guild/".to_string());
    }
    pathes
}

#[derive(Debug, Clone, Default)]
pub struct RaceManager {
    pub initialized: bool,
    pub race_data_map: HashMap<u32, RaceData>,
    pub race_name_src_name_map: HashMap<String, String>,
    pub race_key_race_name_map: HashMap<u32, String>,
    pub path_name: String,
    pub selected_race: Option<u32>,
}

impl RaceManager {
    pub fn new() -> Self {
        Self {
            initialized: true,
            race_data_map: HashMap::new(),
            race_name_src_name_map: HashMap::new(),
            race_key_race_name_map: HashMap::new(),
            path_name: String::new(),
            selected_race: None,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn register_race_name(&mut self, race_index: u32, name: &str) {
        self.race_key_race_name_map.insert(race_index, name.to_string());
    }

    pub fn register_race_src_name(&mut self, name: &str, src_name: &str) {
        self.race_name_src_name_map.insert(name.to_string(), src_name.to_string());
    }

    pub fn set_path_name(&mut self, path_name: &str) {
        self.path_name = path_name.to_string();
    }

    pub fn get_full_path_file_name(&self, file_name: &str) -> String {
        if !file_name.starts_with('.') {
            let mut full_path = self.path_name.clone();
            full_path.push_str(file_name);
            full_path
        } else {
            file_name.to_string()
        }
    }

    pub fn create_race(&mut self, race_index: u32) {
        if self.race_data_map.contains_key(&race_index) {
            println!("RaceManager::CreateRace : Race {} already created", race_index);
            return;
        }

        let race_data = RaceData::new();
        // race_data.set_race(race_index); // Wymagane dodanie w RaceData, mockujemy dla kompatybilnosci
        self.race_data_map.insert(race_index, race_data);
    }

    pub fn select_race(&mut self, race_index: u32) {
        if !self.race_data_map.contains_key(&race_index) {
            // W C++: assert(!"Failed to select race data!");
            return;
        }
        self.selected_race = Some(race_index);
    }

    pub fn get_selected_race_data_pointer(&self) -> Option<&RaceData> {
        if let Some(index) = self.selected_race {
            self.race_data_map.get(&index)
        } else {
            None
        }
    }

    fn load_race_data(&self, race_index: u32) -> Option<RaceData> {
        let race_name = self.race_key_race_name_map.get(&race_index)?;
        if race_name.is_empty() {
            return None;
        }

        if race_name.starts_with('#') {
            let path_name = &race_name[1..];
            let _shape_file_name = format!("{}shape.msm", path_name);
            let _motion_list_file_name = format!("{}motlist.txt", path_name);

            let race_data = RaceData::new();
            // pRaceData->LoadRaceData(&shape_file_name) ... 
            // __LoadRaceMotionList(...) ...
            return Some(race_data);
        }

        let vec_st_full_path_name = get_race_resource_pathes(race_index);
        let race_data = RaceData::new();

        for path in vec_st_full_path_name {
            let mut st_full_path_name = path.clone();
            if let Some(src_name) = self.race_name_src_name_map.get(race_name) {
                st_full_path_name.push_str(src_name);
            } else {
                st_full_path_name.push_str(race_name);
            }
            st_full_path_name.push('/');
            
            let mut st_msm_file_name = st_full_path_name.clone();
            st_msm_file_name.push_str(race_name);
            st_msm_file_name.push_str(".msm");

            // Mock load
            let _st_motion_list_file_name = st_full_path_name.clone();
            // st_motion_list_file_name.push_str(race_data.get_motion_list_file_name());

            return Some(race_data);
        }

        None
    }

    pub fn get_race_data_pointer(&mut self, race_index: u32) -> Option<&RaceData> {
        if !self.race_data_map.contains_key(&race_index) {
            if let Some(race_data) = self.load_race_data(race_index) {
                self.race_data_map.insert(race_index, race_data);
                return self.race_data_map.get(&race_index);
            }
            println!("CRaceManager::GetRaceDataPointer: cannot load data by dwRaceIndex {}", race_index);
            return None;
        }
        self.race_data_map.get(&race_index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_race_manager() {
        let instance = RaceManager::new();
        assert!(instance.is_ready());
    }
}
