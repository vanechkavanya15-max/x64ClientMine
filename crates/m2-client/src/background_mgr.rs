//! Modul `background_mgr` — Zarzadca map swiata i atlasu CPythonBackground (AtlasInfo.txt)
//! Wzorzec referencyjny C++: `source/UserInterface/PythonBackground.cpp`
//! Opis: Odwzorowanie CPythonBackground: parsowanie AtlasInfo.txt, mapowanie globalnych wspolrzednych serwerowych (X, Y) na nazwe mapy (np. metin2_map_a1, c1) i wspolrzedne lokalne.

use std::collections::{HashMap, HashSet};

pub const TERRAIN_SIZE: u32 = 128;
pub const CELLSCALE: u32 = 200;
pub const TERRAIN_XSIZE: u32 = TERRAIN_SIZE * CELLSCALE;
pub const TERRAIN_YSIZE: u32 = TERRAIN_SIZE * CELLSCALE;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShadowLevel {
    None = 0,
    Ground = 1,
    GroundAndSolo = 2,
    All = 3,
    AllHigh = 4,
    AllMax = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistanceSetNum {
    Distance0 = 0,
    Distance1 = 1,
    Distance2 = 2,
    Distance3 = 3,
    Distance4 = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DayMode {
    Light = 0,
    Dark = 1,
}

#[derive(Debug, Clone, Default)]
pub struct ViewDistanceSet {
    pub fog_start: f32,
    pub fog_end: f32,
    pub far_clip: f32,
    pub sky_box_scale: [f32; 3],
}

#[derive(Debug, Clone)]
pub struct MapInfo {
    pub name: String,
    pub base_x: u32,
    pub base_y: u32,
    pub size_x: u32,
    pub size_y: u32,
    pub end_x: u32,
    pub end_y: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct ReserveTargetEffect {
    pub ilx: i32,
    pub ily: i32,
}

#[derive(Debug, Clone)]
pub struct BackgroundMgr {
    pub initialized: bool,

    pub map_name: String,
    pub day_mode: DayMode,
    pub x_mas_tree_grade: i32,
    pub shadow_level: ShadowLevel,
    pub view_distance_num: DistanceSetNum,
    pub visible_guild_area: bool,

    pub base_x: u32,
    pub base_y: u32,

    pub view_distance_set: [ViewDistanceSet; 5],

    pub map_info_vector: Vec<MapInfo>,

    pub showing_portal_id_set: HashSet<i32>,
    pub dungeon_map_name_set: HashSet<String>,
    pub target_id_chr_id_map: HashMap<u32, u32>,
    pub target_id_reserve_effect_map: HashMap<u32, ReserveTargetEffect>,
}

impl Default for BackgroundMgr {
    fn default() -> Self {
        Self::new()
    }
}

impl BackgroundMgr {
    pub fn new() -> Self {
        let mut mgr = Self {
            initialized: true,
            map_name: String::new(),
            day_mode: DayMode::Light,
            x_mas_tree_grade: 0,
            shadow_level: ShadowLevel::None, // Default?
            view_distance_num: DistanceSetNum::Distance4,
            visible_guild_area: false,
            base_x: 0,
            base_y: 0,
            view_distance_set: Default::default(),
            map_info_vector: Vec::new(),
            showing_portal_id_set: HashSet::new(),
            dungeon_map_name_set: HashSet::new(),
            target_id_chr_id_map: HashMap::new(),
            target_id_reserve_effect_map: HashMap::new(),
        };
        
        mgr.set_view_distance_set(DistanceSetNum::Distance4, 25600.0);
        mgr.set_view_distance_set(DistanceSetNum::Distance3, 25600.0);
        mgr.set_view_distance_set(DistanceSetNum::Distance2, 25600.0);
        mgr.set_view_distance_set(DistanceSetNum::Distance1, 25600.0);
        mgr.set_view_distance_set(DistanceSetNum::Distance0, 25600.0);
        mgr
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }
    
    pub fn parse_atlas_info(&mut self, content: &str) {
        self.map_info_vector.clear();
        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5 {
                if parts[0].is_empty() {
                    continue;
                }
                
                let name = parts[0].to_string();
                if let (Ok(base_x), Ok(base_y), Ok(size_x), Ok(size_y)) = (
                    parts[1].parse::<u32>(),
                    parts[2].parse::<u32>(),
                    parts[3].parse::<u32>(),
                    parts[4].parse::<u32>(),
                ) {
                    let end_x = base_x + size_x * TERRAIN_XSIZE;
                    let end_y = base_y + size_y * TERRAIN_YSIZE;
                    
                    self.map_info_vector.push(MapInfo {
                        name,
                        base_x,
                        base_y,
                        size_x,
                        size_y,
                        end_x,
                        end_y,
                    });
                }
            }
        }
    }

    pub fn global_position_to_map_info(&self, global_x: u32, global_y: u32) -> Option<&MapInfo> {
        self.map_info_vector.iter().find(|map_info| {
            global_x >= map_info.base_x
                && global_x < map_info.end_x
                && global_y >= map_info.base_y
                && global_y < map_info.end_y
        })
    }
    
    pub fn global_position_to_local_position(&self, global_x: &mut u32, global_y: &mut u32) {
        *global_x = global_x.saturating_sub(self.base_x);
        *global_y = global_y.saturating_sub(self.base_y);
    }

    pub fn local_position_to_global_position(&self, local_x: &mut u32, local_y: &mut u32) {
        *local_x = local_x.saturating_add(self.base_x);
        *local_y = local_y.saturating_add(self.base_y);
    }

    pub fn warp(&mut self, dw_x: u32, dw_y: u32) -> bool {
        let (map_name, base_x, base_y) = {
            let map_info = match self.global_position_to_map_info(dw_x, dw_y) {
                Some(info) => info,
                None => return false,
            };
            (map_info.name.clone(), map_info.base_x, map_info.base_y)
        };

        self.base_x = base_x;
        self.base_y = base_y;
        self.map_name = map_name;

        self.showing_portal_id_set.clear();
        self.target_id_chr_id_map.clear();
        self.target_id_reserve_effect_map.clear();
        
        true
    }

    pub fn set_view_distance_set(&mut self, e_num: DistanceSetNum, far_clip: f32) {
        let index = e_num as usize;
        self.view_distance_set[index].fog_start = far_clip * 0.5;
        self.view_distance_set[index].fog_end = far_clip * 0.7;
        
        let sky_box_scale = far_clip * 0.6;
        self.view_distance_set[index].sky_box_scale = [sky_box_scale, sky_box_scale, sky_box_scale];
        self.view_distance_set[index].far_clip = far_clip;
        
        if self.view_distance_num == e_num {
            self.select_view_distance_num(e_num);
        }
    }
    
    pub fn select_view_distance_num(&mut self, e_num: DistanceSetNum) {
        self.view_distance_num = e_num;
    }

    pub fn get_far_clip(&self) -> f32 {
        let index = self.view_distance_num as usize;
        let mut far_clip = self.view_distance_set[index].far_clip;
        if far_clip == 0.0 {
            far_clip = 25600.0;
        }
        far_clip
    }

    pub fn get_distance_set_info(&self) -> (DistanceSetNum, f32, f32, f32) {
        let index = self.view_distance_num as usize;
        let ds = &self.view_distance_set[index];
        (self.view_distance_num, ds.fog_start, ds.fog_end, ds.far_clip)
    }

    pub fn change_to_day(&mut self) {
        self.day_mode = DayMode::Light;
    }

    pub fn change_to_night(&mut self) {
        self.day_mode = DayMode::Dark;
    }
    
    pub fn visible_guild_area(&mut self) {
        self.visible_guild_area = true;
    }
    
    pub fn disable_guild_area(&mut self) {
        self.visible_guild_area = false;
    }

    pub fn register_dungeon_map_name(&mut self, map_name: &str) {
        self.dungeon_map_name_set.insert(map_name.to_string());
    }

    pub fn get_warp_map_name(&self) -> &str {
        &self.map_name
    }
    
    pub fn set_shadow_level(&mut self, level: ShadowLevel) -> bool {
        if self.shadow_level == level {
            return true;
        }
        self.shadow_level = level;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_background_mgr() {
        let instance = BackgroundMgr::new();
        assert!(instance.is_ready());
    }
}
