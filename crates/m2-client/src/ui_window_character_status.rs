use crate::ui_window_core::{WindowType, HORIZONTAL_ALIGN_CENTER, HORIZONTAL_ALIGN_RIGHT};
use crate::ui_window_mgr::{get_window_manager, WindowManager};
use crate::recv_points::{GlobalGameState, POINT_HP, POINT_MAX_HP, POINT_SP, POINT_MAX_SP, POINT_ST, POINT_HT, POINT_DX, POINT_IQ, POINT_LEVEL};

// Stat constants since they might not all be in recv_points
pub const POINT_STAT: u8 = 26;
pub const POINT_ATT_POWER: u8 = 16;
pub const POINT_ATT_SPEED: u8 = 17;
pub const POINT_MOV_SPEED: u8 = 19;
pub const POINT_DEF_GRADE: u8 = 20;
pub const POINT_CASTING_SPEED: u8 = 21;

pub struct CharacterStatusWindow {
    pub root_handle: u32,
    pub is_visible: bool,
    
    // Status text handles
    pub hp_text: u32,
    pub sp_text: u32,
    pub vit_text: u32,
    pub int_text: u32,
    pub str_text: u32,
    pub dex_text: u32,
    
    // Additional stat text handles
    pub name_text: u32,
    pub level_text: u32,
    pub align_text: u32,
    pub defense_text: u32,
    pub attack_power_text: u32,
    pub move_speed_text: u32,
    pub attack_speed_text: u32,
    pub cast_speed_text: u32,
    
    // Buttons
    pub vit_plus_btn: u32,
    pub int_plus_btn: u32,
    pub str_plus_btn: u32,
    pub dex_plus_btn: u32,
}

impl CharacterStatusWindow {
    pub fn new() -> Self {
        let mut wm = get_window_manager();
        
        let root_handle = wm.register(WindowType::Normal);
        wm.set_size(root_handle, 250, 400);
        wm.set_position(root_handle, 100, 100);
        wm.windows.get_mut(&root_handle).unwrap().is_show = false; // Initially hidden
        wm.windows.get_mut(&root_handle).unwrap().name = "CharacterStatusWindow".to_string();

        let create_text = |wm: &mut WindowManager, parent: u32, x: i32, y: i32, name: &str| -> u32 {
            let handle = wm.register(WindowType::TextLine);
            wm.set_parent(handle, parent);
            wm.set_position(handle, x, y);
            wm.windows.get_mut(&handle).unwrap().name = name.to_string();
            handle
        };

        let create_btn = |wm: &mut WindowManager, parent: u32, x: i32, y: i32, name: &str| -> u32 {
            let handle = wm.register(WindowType::Button);
            wm.set_parent(handle, parent);
            wm.set_position(handle, x, y);
            wm.set_size(handle, 15, 15);
            wm.windows.get_mut(&handle).unwrap().text = "+".to_string();
            wm.windows.get_mut(&handle).unwrap().name = name.to_string();
            wm.windows.get_mut(&handle).unwrap().is_show = false;
            handle
        };

        let name_text = create_text(&mut wm, root_handle, 20, 20, "StatusNameText");
        let level_text = create_text(&mut wm, root_handle, 20, 40, "StatusLevelText");
        let align_text = create_text(&mut wm, root_handle, 20, 60, "StatusAlignText");

        let hp_text = create_text(&mut wm, root_handle, 20, 90, "StatusHpText");
        let sp_text = create_text(&mut wm, root_handle, 20, 110, "StatusSpText");
        
        let vit_text = create_text(&mut wm, root_handle, 20, 140, "StatusVitText");
        let vit_plus_btn = create_btn(&mut wm, root_handle, 120, 140, "StatusVitPlusBtn");
        
        let int_text = create_text(&mut wm, root_handle, 20, 160, "StatusIntText");
        let int_plus_btn = create_btn(&mut wm, root_handle, 120, 160, "StatusIntPlusBtn");
        
        let str_text = create_text(&mut wm, root_handle, 20, 180, "StatusStrText");
        let str_plus_btn = create_btn(&mut wm, root_handle, 120, 180, "StatusStrPlusBtn");
        
        let dex_text = create_text(&mut wm, root_handle, 20, 200, "StatusDexText");
        let dex_plus_btn = create_btn(&mut wm, root_handle, 120, 200, "StatusDexPlusBtn");

        let defense_text = create_text(&mut wm, root_handle, 20, 230, "StatusDefenseText");
        let attack_power_text = create_text(&mut wm, root_handle, 20, 250, "StatusAttackPowerText");
        let move_speed_text = create_text(&mut wm, root_handle, 20, 270, "StatusMoveSpeedText");
        let attack_speed_text = create_text(&mut wm, root_handle, 20, 290, "StatusAttackSpeedText");
        let cast_speed_text = create_text(&mut wm, root_handle, 20, 310, "StatusCastSpeedText");

        Self {
            root_handle,
            is_visible: false,
            hp_text,
            sp_text,
            vit_text,
            int_text,
            str_text,
            dex_text,
            name_text,
            level_text,
            align_text,
            defense_text,
            attack_power_text,
            move_speed_text,
            attack_speed_text,
            cast_speed_text,
            vit_plus_btn,
            int_plus_btn,
            str_plus_btn,
            dex_plus_btn,
        }
    }

    pub fn get_alignment_grade(alignment: i16) -> u32 {
        if alignment >= 12000 {
            0
        } else if alignment >= 8000 {
            1
        } else if alignment >= 4000 {
            2
        } else if alignment >= 1000 {
            3
        } else if alignment >= 0 {
            4
        } else if alignment > -4000 {
            5
        } else if alignment > -8000 {
            6
        } else if alignment > -12000 {
            7
        } else {
            8
        }
    }

    pub fn update_status_data(&mut self, state: &GlobalGameState, name: &str, alignment: i16) {
        let mut wm = get_window_manager();

        if let Some(win) = wm.windows.get_mut(&self.name_text) {
            win.text = format!("Name: {}", name);
        }
        if let Some(win) = wm.windows.get_mut(&self.level_text) {
            win.text = format!("Level: {}", state.level);
        }
        if let Some(win) = wm.windows.get_mut(&self.align_text) {
            let grade = Self::get_alignment_grade(alignment);
            let grade_str = match grade {
                0 => "Chivalric",
                1 => "Noble",
                2 => "Good",
                3 => "Friendly",
                4 => "Neutral",
                5 => "Aggressive",
                6 => "Fraudulent",
                7 => "Malicious",
                _ => "Cruel",
            };
            win.text = format!("Alignment: {} ({})", grade_str, alignment);
        }

        if let Some(win) = wm.windows.get_mut(&self.hp_text) {
            win.text = format!("HP: {}/{}", state.hp, state.max_hp);
        }
        if let Some(win) = wm.windows.get_mut(&self.sp_text) {
            win.text = format!("SP: {}/{}", state.sp, state.max_sp);
        }

        if let Some(win) = wm.windows.get_mut(&self.vit_text) {
            win.text = format!("VIT (HT): {}", state.ht);
        }
        if let Some(win) = wm.windows.get_mut(&self.int_text) {
            win.text = format!("INT (IQ): {}", state.iq);
        }
        if let Some(win) = wm.windows.get_mut(&self.str_text) {
            win.text = format!("STR (ST): {}", state.st);
        }
        if let Some(win) = wm.windows.get_mut(&self.dex_text) {
            win.text = format!("DEX (DX): {}", state.dx);
        }

        // Additional stats
        let def = state.points[POINT_DEF_GRADE as usize];
        let atk = state.points[POINT_ATT_POWER as usize];
        let mov = state.points[POINT_MOV_SPEED as usize];
        let atk_spd = state.points[POINT_ATT_SPEED as usize];
        let cast_spd = state.points[POINT_CASTING_SPEED as usize]; 

        if let Some(win) = wm.windows.get_mut(&self.defense_text) {
            win.text = format!("Defense: {}", def);
        }
        if let Some(win) = wm.windows.get_mut(&self.attack_power_text) {
            win.text = format!("Attack: {}", atk);
        }
        if let Some(win) = wm.windows.get_mut(&self.move_speed_text) {
            win.text = format!("Move Speed: {}", mov);
        }
        if let Some(win) = wm.windows.get_mut(&self.attack_speed_text) {
            win.text = format!("Attack Speed: {}", atk_spd);
        }
        if let Some(win) = wm.windows.get_mut(&self.cast_speed_text) {
            win.text = format!("Cast Speed: {}", cast_spd);
        }

        let stat_points = state.points[POINT_STAT as usize];
        let show_btns = stat_points > 0;

        if let Some(win) = wm.windows.get_mut(&self.vit_plus_btn) {
            win.is_show = show_btns;
        }
        if let Some(win) = wm.windows.get_mut(&self.int_plus_btn) {
            win.is_show = show_btns;
        }
        if let Some(win) = wm.windows.get_mut(&self.str_plus_btn) {
            win.is_show = show_btns;
        }
        if let Some(win) = wm.windows.get_mut(&self.dex_plus_btn) {
            win.is_show = show_btns;
        }
    }

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
        let mut wm = get_window_manager();
        if let Some(win) = wm.windows.get_mut(&self.root_handle) {
            win.is_show = self.is_visible;
        }
    }
}

impl Default for CharacterStatusWindow {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::recv_points::GlobalGameState;
    use crate::ui_window_mgr::get_window_manager;

    #[test]
    fn test_alignment_grade_calculation() {
        assert_eq!(CharacterStatusWindow::get_alignment_grade(12000), 0);
        assert_eq!(CharacterStatusWindow::get_alignment_grade(15000), 0);

        assert_eq!(CharacterStatusWindow::get_alignment_grade(8000), 1);
        assert_eq!(CharacterStatusWindow::get_alignment_grade(11999), 1);

        assert_eq!(CharacterStatusWindow::get_alignment_grade(4000), 2);
        assert_eq!(CharacterStatusWindow::get_alignment_grade(1000), 3);
        assert_eq!(CharacterStatusWindow::get_alignment_grade(0), 4);
        assert_eq!(CharacterStatusWindow::get_alignment_grade(-1), 5);
        assert_eq!(CharacterStatusWindow::get_alignment_grade(-3999), 5);
        assert_eq!(CharacterStatusWindow::get_alignment_grade(-4000), 6);
        assert_eq!(CharacterStatusWindow::get_alignment_grade(-7999), 6);
        assert_eq!(CharacterStatusWindow::get_alignment_grade(-8000), 7);
        assert_eq!(CharacterStatusWindow::get_alignment_grade(-11999), 7);
        assert_eq!(CharacterStatusWindow::get_alignment_grade(-12000), 8);
        assert_eq!(CharacterStatusWindow::get_alignment_grade(-20000), 8);
    }

    #[test]
    fn test_status_window_visibility_toggle() {
        let mut win = CharacterStatusWindow::new();
        
        assert_eq!(win.is_visible, false);
        
        let is_show_in_wm = {
            let wm = get_window_manager();
            wm.windows.get(&win.root_handle).unwrap().is_show
        };
        assert_eq!(is_show_in_wm, false);
        
        win.toggle_visibility();
        assert_eq!(win.is_visible, true);
        let is_show_in_wm2 = {
            let wm = get_window_manager();
            wm.windows.get(&win.root_handle).unwrap().is_show
        };
        assert_eq!(is_show_in_wm2, true);
    }

    #[test]
    fn test_stat_buttons_visibility() {
        let mut win = CharacterStatusWindow::new();
        let mut state = GlobalGameState::new();
        
        state.points[POINT_STAT as usize] = 0;
        win.update_status_data(&state, "TestPlayer", 12000);
        
        let (v, i, s, d) = {
            let wm = get_window_manager();
            (
                wm.windows.get(&win.vit_plus_btn).unwrap().is_show,
                wm.windows.get(&win.int_plus_btn).unwrap().is_show,
                wm.windows.get(&win.str_plus_btn).unwrap().is_show,
                wm.windows.get(&win.dex_plus_btn).unwrap().is_show,
            )
        };
        assert_eq!(v, false);
        assert_eq!(i, false);
        assert_eq!(s, false);
        assert_eq!(d, false);
        
        state.points[POINT_STAT as usize] = 5;
        win.update_status_data(&state, "TestPlayer", 12000);
        
        let (v, i, s, d) = {
            let wm = get_window_manager();
            (
                wm.windows.get(&win.vit_plus_btn).unwrap().is_show,
                wm.windows.get(&win.int_plus_btn).unwrap().is_show,
                wm.windows.get(&win.str_plus_btn).unwrap().is_show,
                wm.windows.get(&win.dex_plus_btn).unwrap().is_show,
            )
        };
        assert_eq!(v, true);
        assert_eq!(i, true);
        assert_eq!(s, true);
        assert_eq!(d, true);
    }

    #[test]
    fn test_status_data_binding() {
        let mut win = CharacterStatusWindow::new();
        let mut state = GlobalGameState::new();
        
        state.hp = 1200;
        state.max_hp = 1500;
        state.sp = 400;
        state.max_sp = 500;
        
        state.ht = 90;
        state.iq = 10;
        state.st = 40;
        state.dx = 35;
        
        state.level = 45;
        
        win.update_status_data(&state, "MetinHero", 8000);
        
        let wm = get_window_manager();
        assert_eq!(wm.windows.get(&win.hp_text).unwrap().text, "HP: 1200/1500");
        assert_eq!(wm.windows.get(&win.sp_text).unwrap().text, "SP: 400/500");
        assert_eq!(wm.windows.get(&win.vit_text).unwrap().text, "VIT (HT): 90");
        assert_eq!(wm.windows.get(&win.int_text).unwrap().text, "INT (IQ): 10");
        assert_eq!(wm.windows.get(&win.str_text).unwrap().text, "STR (ST): 40");
        assert_eq!(wm.windows.get(&win.dex_text).unwrap().text, "DEX (DX): 35");
        assert_eq!(wm.windows.get(&win.name_text).unwrap().text, "Name: MetinHero");
        assert_eq!(wm.windows.get(&win.level_text).unwrap().text, "Level: 45");
        assert_eq!(wm.windows.get(&win.align_text).unwrap().text, "Alignment: Noble (8000)");
    }

    #[test]
    fn test_additional_status_data_binding() {
        let mut win = CharacterStatusWindow::new();
        let mut state = GlobalGameState::new();
        
        state.points[POINT_DEF_GRADE as usize] = 200;
        state.points[POINT_ATT_POWER as usize] = 300;
        state.points[POINT_MOV_SPEED as usize] = 135;
        state.points[POINT_ATT_SPEED as usize] = 110;
        state.points[POINT_CASTING_SPEED as usize] = 120;
        
        win.update_status_data(&state, "TestPlayer", 12000);
        
        let wm = get_window_manager();
        assert_eq!(wm.windows.get(&win.defense_text).unwrap().text, "Defense: 200");
        assert_eq!(wm.windows.get(&win.attack_power_text).unwrap().text, "Attack: 300");
        assert_eq!(wm.windows.get(&win.move_speed_text).unwrap().text, "Move Speed: 135");
        assert_eq!(wm.windows.get(&win.attack_speed_text).unwrap().text, "Attack Speed: 110");
        assert_eq!(wm.windows.get(&win.cast_speed_text).unwrap().text, "Cast Speed: 120");
    }
}
