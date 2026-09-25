//! Modul `ui_hud_overlay` — Glowny interfejs HUD w trakcie gry (Kule HP/SP, EXP, Quicksloty, Pasek Celu)
//! Wzorzec referencyjny C++: `source/UserInterface/PythonPlayer.cpp`
//! Opis: Zbudowanie pelnego HUD gry: dolny panel z czerwonym paskiem HP, niebieskim SP, zoltym Stamina, 4 kulami EXP (0..100%), 8 polami Quickslot (1-4, F1-F4) oraz gornym paskiem HP wybranego celu/potwora.

pub const POINT_MAX_NUM: usize = 255;
pub const POINT_MAGIC_NUMBER: i64 = 0xe73ac1da;

pub const QUICKSLOT_MAX_LINE: usize = 4;
pub const QUICKSLOT_MAX_COUNT_PER_LINE: usize = 8;
pub const QUICKSLOT_MAX_COUNT: usize = QUICKSLOT_MAX_LINE * QUICKSLOT_MAX_COUNT_PER_LINE;
pub const QUICKSLOT_MAX_NUM: usize = 36;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointType {
    None = 0,
    Level = 1,
    Exp = 3,
    NextExp = 4,
    Hp = 5,
    MaxHp = 6,
    Sp = 7,
    MaxSp = 8,
    Stamina = 9,
    MaxStamina = 10,
}

impl PointType {
    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(PointType::None),
            1 => Some(PointType::Level),
            3 => Some(PointType::Exp),
            4 => Some(PointType::NextExp),
            5 => Some(PointType::Hp),
            6 => Some(PointType::MaxHp),
            7 => Some(PointType::Sp),
            8 => Some(PointType::MaxSp),
            9 => Some(PointType::Stamina),
            10 => Some(PointType::MaxStamina),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct QuickSlot {
    pub slot_type: u8,
    pub position: u8,
}

#[derive(Debug, Clone)]
pub struct PlayerStatus {
    pub points: [i64; POINT_MAX_NUM],
    pub quick_slots: [QuickSlot; QUICKSLOT_MAX_NUM],
    pub quick_page_index: usize,
}

impl Default for PlayerStatus {
    fn default() -> Self {
        Self {
            points: [POINT_MAGIC_NUMBER; POINT_MAX_NUM],
            quick_slots: [QuickSlot::default(); QUICKSLOT_MAX_NUM],
            quick_page_index: 0,
        }
    }
}

impl PlayerStatus {
    pub fn set_point(&mut self, e_point: usize, val: i64) {
        if e_point < POINT_MAX_NUM {
            self.points[e_point] = val ^ POINT_MAGIC_NUMBER;
        }
    }

    pub fn get_point(&self, e_point: usize) -> i64 {
        if e_point < POINT_MAX_NUM {
            self.points[e_point] ^ POINT_MAGIC_NUMBER
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct UiHudOverlay {
    pub initialized: bool,
    pub player_status: PlayerStatus,
    pub target_vid: u32,
    pub target_end_time: u32,
    pub is_consuming_stamina: bool,
    pub consume_stamina_per_sec: f32,
    pub current_stamina: f32,
}

impl UiHudOverlay {
    pub fn new() -> Self {
        Self {
            initialized: true,
            player_status: PlayerStatus::default(),
            target_vid: 0,
            target_end_time: 0,
            is_consuming_stamina: false,
            consume_stamina_per_sec: 0.0,
            current_stamina: 0.0,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn set_status(&mut self, status_type: u32, value: i64) {
        if status_type as usize >= POINT_MAX_NUM {
            return;
        }
        self.player_status.set_point(status_type as usize, value);
    }

    pub fn get_status(&self, status_type: u32) -> i64 {
        if status_type as usize >= POINT_MAX_NUM {
            return 0;
        }
        self.player_status.get_point(status_type as usize)
    }

    pub fn get_quick_page(&self) -> usize {
        self.player_status.quick_page_index
    }

    pub fn set_quick_page(&mut self, page_index: i32) {
        if page_index < 0 {
            let max_line = QUICKSLOT_MAX_LINE as i32;
            let target_page = (max_line + (page_index % max_line)) % max_line;
            self.player_status.quick_page_index = target_page as usize;
        } else if page_index as usize >= QUICKSLOT_MAX_LINE {
            self.player_status.quick_page_index = (page_index as usize) % QUICKSLOT_MAX_LINE;
        } else {
            self.player_status.quick_page_index = page_index as usize;
        }
    }

    pub fn local_to_global_quick_slot_index(&self, local_index: usize) -> usize {
        self.player_status.quick_page_index * QUICKSLOT_MAX_COUNT_PER_LINE + local_index
    }

    pub fn add_quick_slot(&mut self, quick_slot_index: usize, icon_type: u8, icon_position: u8) {
        if quick_slot_index >= QUICKSLOT_MAX_NUM {
            return;
        }
        self.player_status.quick_slots[quick_slot_index].slot_type = icon_type;
        self.player_status.quick_slots[quick_slot_index].position = icon_position;
    }

    pub fn delete_quick_slot(&mut self, quick_slot_index: usize) {
        if quick_slot_index >= QUICKSLOT_MAX_NUM {
            return;
        }
        self.player_status.quick_slots[quick_slot_index].slot_type = 0;
        self.player_status.quick_slots[quick_slot_index].position = 0;
    }

    pub fn move_quick_slot(&mut self, source: usize, target: usize) {
        if source >= QUICKSLOT_MAX_NUM || target >= QUICKSLOT_MAX_NUM {
            return;
        }
        self.player_status.quick_slots.swap(source, target);
    }

    pub fn start_stamina_consume(&mut self, consume_per_sec: u32, current_stamina: u32) {
        self.is_consuming_stamina = true;
        self.consume_stamina_per_sec = consume_per_sec as f32;
        self.current_stamina = current_stamina as f32;
        self.set_status(PointType::Stamina as u32, current_stamina as i64);
    }

    pub fn stop_stamina_consume(&mut self, current_stamina: u32) {
        self.is_consuming_stamina = false;
        self.consume_stamina_per_sec = 0.0;
        self.current_stamina = current_stamina as f32;
        self.set_status(PointType::Stamina as u32, current_stamina as i64);
    }

    pub fn update_stamina(&mut self, elapsed_time: f32) {
        if self.is_consuming_stamina {
            self.current_stamina -= elapsed_time * self.consume_stamina_per_sec;
            if self.current_stamina < 0.0 {
                self.current_stamina = 0.0;
            }
            self.set_status(PointType::Stamina as u32, self.current_stamina as i64);
        }
    }

    pub fn set_target(&mut self, vid: u32) {
        self.target_vid = vid;
    }

    pub fn get_target_vid(&self) -> u32 {
        self.target_vid
    }

    pub fn get_exp_percentage(&self) -> f32 {
        let exp = self.get_status(PointType::Exp as u32);
        let next_exp = self.get_status(PointType::NextExp as u32);
        if next_exp <= 0 {
            return 0.0;
        }
        (exp as f32 / next_exp as f32) * 100.0
    }

    pub fn get_hp_percentage(&self) -> f32 {
        let hp = self.get_status(PointType::Hp as u32);
        let max_hp = self.get_status(PointType::MaxHp as u32);
        if max_hp <= 0 {
            return 0.0;
        }
        (hp as f32 / max_hp as f32) * 100.0
    }

    pub fn get_sp_percentage(&self) -> f32 {
        let sp = self.get_status(PointType::Sp as u32);
        let max_sp = self.get_status(PointType::MaxSp as u32);
        if max_sp <= 0 {
            return 0.0;
        }
        (sp as f32 / max_sp as f32) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_ui_hud_overlay() {
        let instance = UiHudOverlay::new();
        assert!(instance.is_ready());
    }
}
