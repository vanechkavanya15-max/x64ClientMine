//! Modul `character_manager` — Centralny rejestr wszystkich postaci w swiecie CPythonCharacterManager
//! Wzorzec referencyjny C++: `source/UserInterface/PythonCharacterManager.cpp`
//! Opis: Odwzorowanie CPythonCharacterManager: CreateInstance, DeleteInstance, GetInstancePtr(vid), GetMainInstancePtr(), iteracja i aktualizacja wszystkich aktorow w klatce.

use std::collections::{HashMap, VecDeque};
use crate::instance_base::InstanceBase;

pub struct CreateData {
    pub vid: u32,
    pub is_main: bool,
    pub name: String,
    pub race: u32,
}

pub trait InstanceBaseExt {
    fn get_virtual_id(&self) -> u32;
    fn get_name_string(&self) -> &str;
    fn is_dead(&self) -> bool;
    fn can_pick_instance(&self) -> bool;
    fn intersect_bounding_box(&self) -> bool;
    fn intersect_defending_sphere(&self) -> bool;
    fn update_deleting(&mut self) -> bool;
    fn delete_blend_out(&mut self);
    fn is_pc(&self) -> bool;
    fn get_guild_id(&self) -> u32;
    fn change_guild(&mut self, guild_id: u32);
    fn refresh_text_tail(&mut self);
    fn on_selected(&mut self);
    fn on_unselected(&mut self);
    fn get_virtual_number(&self) -> u32;
    fn get_distance(&self, other: &Self) -> f32;
    fn is_movie_mode(&self) -> bool;
    fn update(&mut self);
}

impl InstanceBaseExt for InstanceBase {
    fn get_virtual_id(&self) -> u32 {
        0
    }
    fn get_name_string(&self) -> &str {
        ""
    }
    fn is_dead(&self) -> bool {
        false
    }
    fn can_pick_instance(&self) -> bool {
        true
    }
    fn intersect_bounding_box(&self) -> bool {
        false
    }
    fn intersect_defending_sphere(&self) -> bool {
        false
    }
    fn update_deleting(&mut self) -> bool {
        false
    }
    fn delete_blend_out(&mut self) {
    }
    fn is_pc(&self) -> bool {
        true
    }
    fn get_guild_id(&self) -> u32 {
        0
    }
    fn change_guild(&mut self, _guild_id: u32) {
    }
    fn refresh_text_tail(&mut self) {
    }
    fn on_selected(&mut self) {
    }
    fn on_unselected(&mut self) {
    }
    fn get_virtual_number(&self) -> u32 {
        0
    }
    fn get_distance(&self, _other: &Self) -> f32 {
        0.0
    }
    fn is_movie_mode(&self) -> bool {
        false
    }
    fn update(&mut self) {
    }
}

pub struct CharacterManager {
    pub initialized: bool,
    pub alive_inst_map: HashMap<u32, InstanceBase>,
    pub dead_inst_list: VecDeque<InstanceBase>,
    pub main_inst_vid: Option<u32>,
    pub bind_inst_vid: Option<u32>,
    pub pick_inst_vid: Option<u32>,
    pub picked_inst_vids: Vec<u32>,
}

impl Default for CharacterManager {
    fn default() -> Self {
        Self::new()
    }
}

impl CharacterManager {
    pub fn new() -> Self {
        Self {
            initialized: true,
            alive_inst_map: HashMap::new(),
            dead_inst_list: VecDeque::new(),
            main_inst_vid: None,
            bind_inst_vid: None,
            pick_inst_vid: None,
            picked_inst_vids: Vec::new(),
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn get_instance_ptr(&self, vid: u32) -> Option<&InstanceBase> {
        self.alive_inst_map.get(&vid)
    }
    
    pub fn get_instance_ptr_mut(&mut self, vid: u32) -> Option<&mut InstanceBase> {
        self.alive_inst_map.get_mut(&vid)
    }

    pub fn get_instance_ptr_by_name(&self, name: &str) -> Option<&InstanceBase> {
        self.alive_inst_map.values().find(|inst| inst.get_name_string() == name)
    }

    pub fn get_main_instance_ptr(&self) -> Option<&InstanceBase> {
        self.main_inst_vid.and_then(|vid| self.get_instance_ptr(vid))
    }

    pub fn set_main_instance(&mut self, vid: u32) -> bool {
        if self.alive_inst_map.contains_key(&vid) {
            self.main_inst_vid = Some(vid);
            true
        } else {
            false
        }
    }

    pub fn clear_main_instance(&mut self) {
        self.main_inst_vid = None;
    }

    pub fn select_instance(&mut self, vid: u32) {
        if self.alive_inst_map.contains_key(&vid) {
            self.bind_inst_vid = Some(vid);
        }
    }

    pub fn get_selected_instance_ptr(&self) -> Option<&InstanceBase> {
        self.bind_inst_vid.and_then(|vid| self.get_instance_ptr(vid))
    }

    pub fn create_instance(&mut self, create_data: CreateData) -> Option<u32> {
        if self.alive_inst_map.contains_key(&create_data.vid) {
            return None;
        }

        let instance = InstanceBase::new();
        let vid = create_data.vid;
        
        self.alive_inst_map.insert(vid, instance);

        if create_data.is_main {
            self.select_instance(vid);
            self.main_inst_vid = Some(vid);
        }

        Some(vid)
    }

    pub fn delete_instance(&mut self, vid: u32) {
        if self.bind_inst_vid == Some(vid) {
            self.bind_inst_vid = None;
        }
        if self.main_inst_vid == Some(vid) {
            self.main_inst_vid = None;
        }
        if self.pick_inst_vid == Some(vid) {
            self.pick_inst_vid = None;
        }

        self.alive_inst_map.remove(&vid);
    }
    
    pub fn update(&mut self) {
        for instance in self.alive_inst_map.values_mut() {
            instance.update();
        }
        self.update_deleting();
    }
    
    pub fn delete_instance_by_fade(&mut self, vid: u32) {
        if let Some(mut inst) = self.alive_inst_map.remove(&vid) {
            inst.delete_blend_out();
            self.dead_inst_list.push_back(inst);
        }
    }
    
    pub fn update_deleting(&mut self) {
        let mut i = 0;
        while i < self.dead_inst_list.len() {
            if self.dead_inst_list[i].update_deleting() {
                i += 1;
            } else {
                self.dead_inst_list.remove(i);
            }
        }
    }
    
    pub fn destroy_alive_instance_map(&mut self) {
        self.alive_inst_map.clear();
        self.main_inst_vid = None;
        self.bind_inst_vid = None;
        self.pick_inst_vid = None;
    }

    pub fn destroy_dead_instance_list(&mut self) {
        self.dead_inst_list.clear();
    }

    pub fn delete_all_instances(&mut self) {
        self.destroy_alive_instance_map();
        self.destroy_dead_instance_list();
    }
    
    pub fn get_close_instance(&self, vid: u32) -> Option<u32> {
        let p_instance = self.get_instance_ptr(vid)?;
        
        let mut min_distance = 10000.0;
        let mut close_instance_vid = None;

        for (target_vid, target_instance) in &self.alive_inst_map {
            if *target_vid == vid {
                continue;
            }

            let distance = p_instance.get_distance(target_instance);
            if distance < min_distance {
                min_distance = distance;
                close_instance_vid = Some(*target_vid);
            }
        }
        
        close_instance_vid
    }

    pub fn pick_all(&self) -> i32 {
        for (vid, instance) in &self.alive_inst_map {
            if instance.intersect_defending_sphere() {
                return *vid as i32;
            }
        }
        -1
    }

    pub fn refresh_all_pc_text_tail(&mut self) {
        for instance in self.alive_inst_map.values_mut() {
            if instance.is_pc() {
                instance.refresh_text_tail();
            }
        }
    }

    pub fn refresh_all_guild_mark(&mut self) {
        for instance in self.alive_inst_map.values_mut() {
            if instance.is_pc() {
                let guild_id = instance.get_guild_id();
                instance.change_guild(guild_id);
                instance.refresh_text_tail();
            }
        }
    }
    
    fn update_picked_actor_list(&mut self) {
        self.picked_inst_vids.clear();
        
        for (vid, instance) in &self.alive_inst_map {
            if instance.can_pick_instance() {
                if instance.is_dead() {
                    if instance.intersect_bounding_box() {
                        self.picked_inst_vids.push(*vid);
                    }
                } else if instance.intersect_defending_sphere() {
                    self.picked_inst_vids.push(*vid);
                }
            }
        }
    }
    
    fn sort_picked_actor_list(&mut self) {
        // C++ version sorts based on distance to camera. 
        // Here we just keep the list for simplicity in test unless we have a camera.
    }
    
    fn update_sort_picked_actor_list(&mut self) {
        self.update_picked_actor_list();
        self.sort_picked_actor_list();
    }
    
    pub fn new_pick(&mut self) {
        self.update_sort_picked_actor_list();
        
        let main_vid = self.main_inst_vid;
        
        // Exact picking
        for vid in &self.picked_inst_vids {
            let vid = *vid;
            if Some(vid) != main_vid {
                let mut can_pick = false;
                let mut intersect_bb = false;
                if let Some(inst) = self.alive_inst_map.get(&vid) {
                    can_pick = inst.can_pick_instance();
                    intersect_bb = inst.intersect_bounding_box();
                }
                
                if intersect_bb {
                    if let Some(pick_vid) = self.pick_inst_vid {
                        if pick_vid != vid {
                            if let Some(pick_inst) = self.alive_inst_map.get_mut(&pick_vid) {
                                pick_inst.on_unselected();
                            }
                        }
                    }
                    
                    if can_pick {
                        self.pick_inst_vid = Some(vid);
                        if let Some(pick_inst) = self.alive_inst_map.get_mut(&vid) {
                            pick_inst.on_selected();
                        }
                        return;
                    }
                }
            }
        }
        
        // If not picked exactly, pick any in order
        for vid in &self.picked_inst_vids {
            let vid = *vid;
            if Some(vid) != main_vid {
                let mut can_pick = false;
                if let Some(inst) = self.alive_inst_map.get(&vid) {
                    can_pick = inst.can_pick_instance();
                }
                
                if let Some(pick_vid) = self.pick_inst_vid {
                    if pick_vid != vid {
                        if let Some(pick_inst) = self.alive_inst_map.get_mut(&pick_vid) {
                            pick_inst.on_unselected();
                        }
                    }
                }
                
                if can_pick {
                    self.pick_inst_vid = Some(vid);
                    if let Some(pick_inst) = self.alive_inst_map.get_mut(&vid) {
                        pick_inst.on_selected();
                    }
                    return;
                }
            }
        }
        
        // Pick main instance
        if let Some(main_vid) = main_vid {
            let mut can_pick = false;
            if let Some(main_inst) = self.alive_inst_map.get(&main_vid) {
                can_pick = main_inst.can_pick_instance();
            }
            
            if can_pick && self.picked_inst_vids.contains(&main_vid) {
                if let Some(pick_vid) = self.pick_inst_vid {
                    if pick_vid != main_vid {
                        if let Some(pick_inst) = self.alive_inst_map.get_mut(&pick_vid) {
                            pick_inst.on_unselected();
                        }
                    }
                }
                self.pick_inst_vid = Some(main_vid);
                if let Some(pick_inst) = self.alive_inst_map.get_mut(&main_vid) {
                    pick_inst.on_selected();
                }
                return;
            }
        }
        
        if let Some(pick_vid) = self.pick_inst_vid {
            if let Some(pick_inst) = self.alive_inst_map.get_mut(&pick_vid) {
                pick_inst.on_unselected();
            }
            self.pick_inst_vid = None;
        }
    }
    
    pub fn old_pick(&mut self) {
        let mut picked_vid = None;
        
        for (vid, instance) in &self.alive_inst_map {
            if Some(*vid) == self.main_inst_vid {
                continue;
            }
            
            if instance.intersect_defending_sphere() {
                picked_vid = Some(*vid);
                break;
            }
        }
        
        if let Some(vid) = picked_vid {
            if let Some(pick_vid) = self.pick_inst_vid {
                if pick_vid != vid {
                    if let Some(pick_inst) = self.alive_inst_map.get_mut(&pick_vid) {
                        pick_inst.on_unselected();
                    }
                }
            }
            
            self.pick_inst_vid = Some(vid);
            if let Some(inst) = self.alive_inst_map.get_mut(&vid) {
                inst.on_selected();
            }
            return;
        }
        
        if let Some(pick_vid) = self.pick_inst_vid {
            if let Some(pick_inst) = self.alive_inst_map.get_mut(&pick_vid) {
                pick_inst.on_unselected();
            }
            self.pick_inst_vid = None;
        }
    }
}
