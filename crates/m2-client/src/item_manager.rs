//! Modul `item_manager` — Menedzer przedmiotow lezacych na ziemi CPythonItem
//! Wzorzec referencyjny C++: `source/UserInterface/PythonItem.cpp`
//! Opis: Odwzorowanie CPythonItem: rejestr przedmiotow lezacych na mapie (GroundItemInstance), animacja upadku i obrotu na ziemi, przypisanie wlasciciela (ownership), podnoszenie.

use std::collections::HashMap;

pub const INVALID_ID: u32 = 0xffffffff;
pub const VNUM_MONEY: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UseSound {
    None,
    Default,
    Armor,
    Weapon,
    Bow,
    Accessory,
    Potion,
    Portal,
    Num,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DropSound {
    Default,
    Armor,
    Weapon,
    Bow,
    Accessory,
    Num,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PixelPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Default)]
pub struct GroundItemInstance {
    pub vnum: u32,
    pub end_position: Vector3,
    pub rotation_axis: Vector3,
    pub q_end: Quaternion,
    pub center: Vector3,
    pub start_time: u32,
    pub end_time: u32,
    pub drop_sound_type: u32,
    pub anim_ended: bool,
    pub effect_instance_index: u32,
    pub ownership: String,
}

impl GroundItemInstance {
    pub fn update(&mut self, current_time: u32) -> bool {
        if self.anim_ended {
            return false;
        }

        if self.end_time < current_time {
            // Animacja rotacyjna zakodowana w cpp kwaternionowo uproszczona, 
            // docelowo obiekt osiaga `end_position` + qAdjust jesli uzywany jest CGraphicThingInstance.
            // Tutaj po prostu zamykamy pozycje dropu do end_position w zaleznosci od interfejsu (mock)
            
            self.anim_ended = true;
            // Play drop sound mocked: __PlayDropSound(eDropSoundType, v3EndPosition)
        } else {
            let time = current_time.saturating_sub(self.start_time);
            let total_time = self.end_time.saturating_sub(self.start_time);
            let rate = if total_time > 0 {
                time as f32 / total_time as f32
            } else {
                1.0
            };

            let mut _v3_new_position = self.end_position;
            // Parabola upadku: v3NewPosition.z += 100 - 100 * rate * (3*rate - 2)
            _v3_new_position.z += 100.0 - 100.0 * rate * (3.0 * rate - 2.0);

            // Reszta z ThingInstance.SetPosition zostala by wywolana na module klienta
        }

        !self.anim_ended
    }
}

#[derive(Debug, Clone, Default)]
pub struct ItemManager {
    pub initialized: bool,
    pub ground_items: HashMap<u32, GroundItemInstance>,
    pub picked_item_id: u32,
}

impl ItemManager {
    pub fn new() -> Self {
        Self {
            initialized: true,
            ground_items: HashMap::new(),
            picked_item_id: INVALID_ID,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn update(&mut self, current_time: u32) {
        for item in self.ground_items.values_mut() {
            item.update(current_time);
        }

        // Mock pick item functionality, leaving picked_item_id logic
        // m_dwPickedItemID = __Pick(c_rkPtMouse);
    }

    pub fn create_item(
        &mut self,
        vid: u32,
        vnum: u32,
        x: f32,
        y: f32,
        z: f32,
        _b_drop: bool,
    ) {
        let instance = GroundItemInstance {
            vnum,
            end_position: Vector3 { x, y, z },
            rotation_axis: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            q_end: Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
            center: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            start_time: 0,
            end_time: 0,
            drop_sound_type: DropSound::Default as u32,
            anim_ended: false,
            effect_instance_index: 0,
            ownership: String::new(),
        };
        self.ground_items.insert(vid, instance);
    }

    pub fn delete_item(&mut self, vid: u32) {
        self.ground_items.remove(&vid);
    }

    pub fn delete_all_items(&mut self) {
        self.ground_items.clear();
    }

    pub fn set_ownership(&mut self, vid: u32, name: &str) {
        if let Some(item) = self.ground_items.get_mut(&vid) {
            item.ownership = name.to_string();
        }
    }

    pub fn get_ownership(&self, vid: u32) -> Option<String> {
        self.ground_items.get(&vid).map(|item| item.ownership.clone())
    }

    pub fn get_ground_item_position(&self, vid: u32) -> Option<PixelPosition> {
        self.ground_items.get(&vid).map(|item| PixelPosition {
            x: item.end_position.x,
            y: item.end_position.y,
            z: item.end_position.z,
        })
    }

    pub fn get_picked_item_id(&self) -> Option<u32> {
        if self.picked_item_id == INVALID_ID {
            None
        } else {
            Some(self.picked_item_id)
        }
    }

    pub fn get_virtual_number_of_ground_item(&self, vid: u32) -> u32 {
        self.ground_items.get(&vid).map(|item| item.vnum).unwrap_or(0)
    }

    fn distance_approx(dx: i32, dy: i32) -> i32 {
        let min;
        let max;
        let dx = dx.abs();
        let dy = dy.abs();

        if dx < dy {
            min = dx;
            max = dy;
        } else {
            min = dy;
            max = dx;
        }

        // coefficients equivalent to ( 123/128 * max ) and ( 51/128 * min )
        ((max << 8) + (max << 3) - (max << 4) - (max << 1) +
            (min << 7) - (min << 5) + (min << 3) - (min << 1)) >> 8
    }

    pub fn get_close_item(&self, pos: &PixelPosition, distance: u32) -> Option<u32> {
        let mut close_item_id = 0;
        let mut close_item_distance = 0;
        let mut apro_min = 0;

        for (&vid, item) in &self.ground_items {
            let dist = Self::distance_approx(
                pos.x as i32 - item.end_position.x as i32,
                pos.y as i32 - (-(item.end_position.y as i32)),
            );

            if apro_min == 0 || apro_min >= dist {
                apro_min = dist;
                close_item_id = vid;
                close_item_distance = dist;
            }
        }

        if close_item_distance > distance as i32 {
            return None;
        }

        if close_item_id == 0 && apro_min == 0 {
            return None;
        }

        Some(close_item_id)
    }

    pub fn get_close_money(&self, pos: &PixelPosition, distance: u32) -> Option<u32> {
        let mut close_item_id = 0;
        let mut close_item_distance = u32::MAX;

        for (&vid, item) in &self.ground_items {
            if item.vnum != VNUM_MONEY {
                continue;
            }

            let dx = pos.x as i64 - item.end_position.x as i64;
            let dy = pos.y as i64 - (-(item.end_position.y as f64)) as i64;
            let dist = (dx * dx + dy * dy) as u32;

            if dist < close_item_distance {
                close_item_id = vid;
                close_item_distance = dist;
            }
        }

        if close_item_distance > distance.wrapping_mul(distance) {
            return None;
        }
        
        if close_item_id == 0 {
            return None;
        }

        Some(close_item_id)
    }

    pub fn render(&self) {
        // CPythonGraphic::Instance().SetDiffuseOperation();
        // rInstance.Render(); rInstance.BlendRender();
    }

    pub fn play_use_sound(&self, _dw_item_id: u32) {}
    pub fn play_drop_sound(&self, _dw_item_id: u32) {}
    pub fn play_use_potion_sound(&self) {}
    
    pub fn set_use_sound_file_name(&mut self, _e_item_type: u32, _c_rst_file_name: &str) {}
    pub fn set_drop_sound_file_name(&mut self, _e_item_type: u32, _c_rst_file_name: &str) {}
    
    pub fn get_info(&self) -> String {
        format!("Item: Inst {}, Pool ?", self.ground_items.len())
    }

    pub fn get_no_grade_name_data_count(&self) -> u32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_item_manager() {
        let instance = ItemManager::new();
        assert!(instance.is_ready());
    }
}
