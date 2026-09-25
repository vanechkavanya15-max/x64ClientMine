//! Modul `race_data` — Struktura definicji rasy i wygladu CRaceData (.msm)
//! Wzorzec referencyjny C++: `source/GameLib/RaceData.cpp oraz source/GameLib/RaceDataFile.cpp`
//! Opis: Odwzorowanie CRaceData: parsowanie skryptow modeli .msm (BaseModelFileName, ShapeData, HairData, AttachingBoneName dla broni i efektow).

use std::collections::HashMap;

/// Zbiezne z serwerem, wazne by indeksy pokrywaly sie miedzy serwerem a klientem
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EParts {
    Main = 0,
    Weapon = 1,
    Head = 2,
    WeaponLeft = 3,
    Hair = 4,
    MaxNum = 5,
}

pub const SMOKE_NUM: usize = 4;

pub type ComboKey = u32;

pub fn make_combo_key(motion_mode: u16, combo_type: u16) -> ComboKey {
    ((motion_mode as u32) << 16) | (combo_type as u32)
}

pub fn combo_key_get_motion_mode(key: ComboKey) -> u16 {
    ((key >> 16) & 0xFFFF) as u16
}

pub fn combo_key_get_combo_type(key: ComboKey) -> u16 {
    (key & 0xFFFF) as u16
}

#[derive(Debug, Clone, Default)]
pub struct Skin {
    pub part: u32,
    pub src_file_name: String,
    pub dst_file_name: String,
}

#[derive(Debug, Clone, Default)]
pub struct Hair {
    pub model_file_name: String,
    pub skins: Vec<Skin>,
}

#[derive(Debug, Clone, Default)]
pub struct Shape {
    pub model_file_name: String,
    pub skins: Vec<Skin>,
}

#[derive(Debug, Clone, Default)]
pub struct Motion {
    pub percentage: u8,
    pub motion_file_name: String, 
}

#[derive(Debug, Clone, Default)]
pub struct MotionModeData {
    pub motion_mode_index: u16,
    pub motion_vector_map: HashMap<u16, Vec<Motion>>,
}

#[derive(Debug, Clone, Default)]
pub struct ComboData {
    pub combo_index_vector: Vec<u32>,
}

#[derive(Debug, Clone, Default)]
pub struct RaceData {
    pub initialized: bool,

    pub race_index: u32,
    pub smoke_effect_ids: [u32; SMOKE_NUM],

    pub base_model_file_name: String,
    pub tree_file_name: String,
    pub attribute_file_name: String,
    pub motion_list_file_name: String,
    pub smoke_bone_name: String,

    pub motion_mode_data_map: HashMap<u16, MotionModeData>,
    pub attaching_bone_name_map: HashMap<u32, String>,
    pub combo_attack_data_map: HashMap<ComboKey, ComboData>,
    pub normal_attack_index_map: HashMap<u16, u16>,

    pub hair_map: HashMap<u32, Hair>,
    pub shape_map: HashMap<u32, Shape>,
}

impl RaceData {
    pub fn new() -> Self {
        Self {
            initialized: true,
            race_index: 0,
            smoke_effect_ids: [0; SMOKE_NUM],
            base_model_file_name: String::new(),
            tree_file_name: String::new(),
            attribute_file_name: String::new(),
            motion_list_file_name: String::from("motlist.txt"),
            smoke_bone_name: String::new(),
            motion_mode_data_map: HashMap::new(),
            attaching_bone_name_map: HashMap::new(),
            combo_attack_data_map: HashMap::new(),
            normal_attack_index_map: HashMap::new(),
            hair_map: HashMap::new(),
            shape_map: HashMap::new(),
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn destroy(&mut self) {
        self.hair_map.clear();
        self.shape_map.clear();
        
        self.base_model_file_name.clear();
        self.tree_file_name.clear();
        self.attribute_file_name.clear();
        self.motion_list_file_name = String::from("motlist.txt");
        self.smoke_bone_name.clear();

        self.attaching_bone_name_map.clear();
        self.normal_attack_index_map.clear();
        self.combo_attack_data_map.clear();
        
        self.motion_mode_data_map.clear();

        self.race_index = 0;
        self.smoke_effect_ids = [0; SMOKE_NUM];
    }

    pub fn set_race(&mut self, race_index: u32) {
        self.race_index = race_index;
    }

    pub fn register_attaching_bone_name(&mut self, part_index: u32, bone_name: &str) {
        self.attaching_bone_name_map.insert(part_index, bone_name.to_string());
    }
    
    pub fn get_attaching_bone_name(&self, part_index: u32) -> Option<&String> {
        self.attaching_bone_name_map.get(&part_index)
    }

    pub fn register_motion_mode(&mut self, motion_mode_index: u16) {
        let data = MotionModeData {
            motion_mode_index,
            motion_vector_map: HashMap::new(),
        };
        self.motion_mode_data_map.insert(motion_mode_index, data);
    }

    pub fn register_motion_data(&mut self, motion_mode: u16, motion_index: u16, file_name: &str, percentage: u8) -> bool {
        if let Some(mode_data) = self.motion_mode_data_map.get_mut(&motion_mode) {
            let motion = Motion {
                percentage,
                motion_file_name: file_name.to_string(),
            };
            let vector = mode_data.motion_vector_map.entry(motion_index).or_insert_with(Vec::new);
            vector.push(motion);
            true
        } else {
            false
        }
    }

    pub fn set_motion_random_weight(&mut self, motion_mode: u16, motion_index: u16, motion_sub_index: u16, percentage: u8) -> bool {
        if let Some(mode_data) = self.motion_mode_data_map.get_mut(&motion_mode) {
            if let Some(vector) = mode_data.motion_vector_map.get_mut(&motion_index) {
                if (motion_sub_index as usize) < vector.len() {
                    vector[motion_sub_index as usize].percentage = percentage;
                    return true;
                }
            }
        }
        false
    }
    
    pub fn get_motion_vector_pointer(&self, motion_mode: u16, motion_index: u16) -> Option<&Vec<Motion>> {
        self.motion_mode_data_map.get(&motion_mode)?.motion_vector_map.get(&motion_index)
    }

    pub fn register_normal_attack(&mut self, motion_mode: u16, motion_index: u16) {
        self.normal_attack_index_map.insert(motion_mode, motion_index);
    }
    
    pub fn get_normal_attack_index(&self, motion_mode: u16) -> Option<u16> {
        self.normal_attack_index_map.get(&motion_mode).copied()
    }

    pub fn reserve_combo_attack(&mut self, motion_mode: u16, combo_type: u16, combo_count: u32) {
        let key = make_combo_key(motion_mode, combo_type);
        let data = ComboData {
            combo_index_vector: vec![0; combo_count as usize],
        };
        self.combo_attack_data_map.insert(key, data);
    }

    pub fn register_combo_attack(&mut self, motion_mode: u16, combo_type: u16, combo_index: u32, motion_index: u16) {
        let key = make_combo_key(motion_mode, combo_type);
        if let Some(combo_data) = self.combo_attack_data_map.get_mut(&key) {
            if (combo_index as usize) < combo_data.combo_index_vector.len() {
                combo_data.combo_index_vector[combo_index as usize] = motion_index as u32;
            }
        }
    }
    
    pub fn get_combo_data_pointer(&self, motion_mode: u16, combo_type: u16) -> Option<&ComboData> {
        let key = make_combo_key(motion_mode, combo_type);
        self.combo_attack_data_map.get(&key)
    }

    pub fn set_shape_model(&mut self, shape_index: u32, model_file_name: &str) {
        let shape = self.shape_map.entry(shape_index).or_insert_with(Shape::default);
        shape.model_file_name = model_file_name.to_string();
    }

    pub fn append_shape_skin(&mut self, shape_index: u32, part: u32, src_file_name: &str, dst_file_name: &str) {
        let shape = self.shape_map.entry(shape_index).or_insert_with(Shape::default);
        shape.skins.push(Skin {
            part,
            src_file_name: src_file_name.to_string(),
            dst_file_name: dst_file_name.to_string(),
        });
    }

    pub fn set_hair_skin(&mut self, hair_index: u32, part: u32, model_file_name: &str, src_file_name: &str, dst_file_name: &str) {
        let hair = self.hair_map.entry(hair_index).or_insert_with(Hair::default);
        hair.model_file_name = model_file_name.to_string();
        hair.skins.push(Skin {
            part,
            src_file_name: src_file_name.to_string(),
            dst_file_name: dst_file_name.to_string(),
        });
    }

    pub fn find_hair(&self, hair_index: u32) -> Option<&Hair> {
        self.hair_map.get(&hair_index)
    }

    pub fn find_shape(&self, shape_index: u32) -> Option<&Shape> {
        self.shape_map.get(&shape_index)
    }

    pub fn get_smoke_effect_id(&self, smoke_index: usize) -> u32 {
        if smoke_index < SMOKE_NUM {
            self.smoke_effect_ids[smoke_index]
        } else {
            0
        }
    }

    pub fn get_smoke_bone(&self) -> &String {
        &self.smoke_bone_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_race_data() {
        let instance = RaceData::new();
        assert!(instance.is_ready());
    }
    
    #[test]
    fn test_combo_key() {
        let key = make_combo_key(10, 5);
        assert_eq!(combo_key_get_motion_mode(key), 10);
        assert_eq!(combo_key_get_combo_type(key), 5);
    }
}
