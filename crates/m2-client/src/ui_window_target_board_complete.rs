// ui_window_target_board_complete.rs - Target Board implementation for Metin2

use crate::ui_window_core::{WindowType, HORIZONTAL_ALIGN_CENTER, HORIZONTAL_ALIGN_LEFT};
use crate::ui_window_mgr::get_window_manager;

/// Informacje o aktualnie wybranym celu dla Target Board
#[derive(Debug, Clone, PartialEq)]
pub struct TargetBoardInfoComplete {
    pub vid: u32,
    pub name: String,
    pub level: u32,
    pub hp: i32,
    pub max_hp: i32,
    pub hp_percent: u8,
    pub race_type: String,
    pub is_enemy: bool,
    pub is_poisoned: bool,
    pub is_bleeding: bool,
    pub is_slowed: bool,
}

impl Default for TargetBoardInfoComplete {
    fn default() -> Self {
        Self {
            vid: 0,
            name: String::new(),
            level: 1,
            hp: 0,
            max_hp: 1,
            hp_percent: 0,
            race_type: String::new(),
            is_enemy: true,
            is_poisoned: false,
            is_bleeding: false,
            is_slowed: false,
        }
    }
}

pub struct UiWindowTargetBoardComplete {
    pub root_handle: u32,
    pub name_text: u32,
    pub level_text: u32,
    pub race_text: u32,
    pub hp_bar_bg: u32,
    pub hp_bar_fg: u32,
    pub hp_text: u32,
    pub hp_numeric_text: u32,
    pub poison_icon: u32,
    pub bleeding_icon: u32,
    pub slow_icon: u32,
    
    // Interaction buttons
    pub btn_whisper: u32,
    pub btn_trade: u32,
    pub btn_party: u32,
    pub btn_guild: u32,
    pub btn_duel: u32,

    pub close_btn: u32,

    pub current_target: Option<TargetBoardInfoComplete>,
    pub is_visible: bool,
    
    pub bar_width: i32,
    pub bar_height: i32,
    pub hp_max_width: i32,
}

impl Default for UiWindowTargetBoardComplete {
    fn default() -> Self {
        Self::new()
    }
}

impl UiWindowTargetBoardComplete {
    pub const DEFAULT_WIDTH: i32 = 280;
    pub const DEFAULT_HEIGHT: i32 = 120;
    pub const HP_BAR_MAX_WIDTH: i32 = 250;

    pub fn new() -> Self {
        let mut wm = get_window_manager();

        // Root Window
        let root_handle = wm.register(WindowType::ThinBoard);
        wm.set_size(root_handle, Self::DEFAULT_WIDTH, Self::DEFAULT_HEIGHT);
        wm.set_position(root_handle, 0, 15);
        wm.set_align(root_handle, HORIZONTAL_ALIGN_CENTER, 0);
        wm.set_name(root_handle, "TargetBoardComplete");
        wm.hide(root_handle);

        // Level text
        let level_text = wm.register(WindowType::TextLine);
        wm.set_parent(level_text, root_handle);
        wm.set_position(level_text, 12, 10);
        wm.set_size(level_text, 45, 16);
        wm.set_font_color(level_text, [1.0, 0.85, 0.2, 1.0]);
        wm.set_text(level_text, "Lv. 1");
        wm.set_name(level_text, "TargetBoardLevelText");

        // Name text
        let name_text = wm.register(WindowType::TextLine);
        wm.set_parent(name_text, root_handle);
        wm.set_position(name_text, 60, 10);
        wm.set_size(name_text, 120, 16);
        wm.set_font_color(name_text, [1.0, 1.0, 1.0, 1.0]);
        wm.set_text(name_text, "");
        wm.set_name(name_text, "TargetBoardNameText");

        // Race / Mob type text
        let race_text = wm.register(WindowType::TextLine);
        wm.set_parent(race_text, root_handle);
        wm.set_position(race_text, 185, 10);
        wm.set_size(race_text, 60, 16);
        wm.set_font_color(race_text, [0.75, 0.85, 1.0, 1.0]);
        wm.set_text(race_text, "");
        wm.set_name(race_text, "TargetBoardRaceText");

        // HP Background
        let hp_bar_bg = wm.register(WindowType::Bar3D);
        wm.set_parent(hp_bar_bg, root_handle);
        wm.set_position(hp_bar_bg, 12, 30);
        wm.set_size(hp_bar_bg, Self::HP_BAR_MAX_WIDTH + 2, 16);
        wm.set_color(hp_bar_bg, [0.15, 0.05, 0.05, 0.95]);
        wm.set_name(hp_bar_bg, "TargetBoardHpBg");

        // HP Foreground
        let hp_bar_fg = wm.register(WindowType::Bar);
        wm.set_parent(hp_bar_fg, hp_bar_bg);
        wm.set_position(hp_bar_fg, 1, 1);
        wm.set_size(hp_bar_fg, Self::HP_BAR_MAX_WIDTH, 14);
        wm.set_color(hp_bar_fg, [0.85, 0.15, 0.15, 1.0]);
        wm.set_name(hp_bar_fg, "TargetBoardHpFg");

        // HP Percentage Text
        let hp_text = wm.register(WindowType::TextLine);
        wm.set_parent(hp_text, root_handle);
        wm.set_position(hp_text, 12, 48);
        wm.set_size(hp_text, Self::HP_BAR_MAX_WIDTH, 14);
        wm.set_align(hp_text, HORIZONTAL_ALIGN_CENTER, 0);
        wm.set_font_color(hp_text, [0.95, 0.95, 0.95, 1.0]);
        wm.set_text(hp_text, "100%");
        wm.set_name(hp_text, "TargetBoardHpPercentText");

        // HP Numeric Text
        let hp_numeric_text = wm.register(WindowType::TextLine);
        wm.set_parent(hp_numeric_text, root_handle);
        wm.set_position(hp_numeric_text, 12, 65);
        wm.set_size(hp_numeric_text, Self::HP_BAR_MAX_WIDTH, 14);
        wm.set_align(hp_numeric_text, HORIZONTAL_ALIGN_CENTER, 0);
        wm.set_font_color(hp_numeric_text, [0.8, 1.0, 0.8, 1.0]);
        wm.set_text(hp_numeric_text, "0 / 0");
        wm.set_name(hp_numeric_text, "TargetBoardHpNumericText");

        // Status Icons
        let poison_icon = wm.register(WindowType::ImageBox);
        wm.set_parent(poison_icon, root_handle);
        wm.set_position(poison_icon, 12, 85);
        wm.set_size(poison_icon, 16, 16);
        wm.load_image(poison_icon, "d:/ymir work/ui/skill/poison.sub");
        wm.hide(poison_icon);
        wm.set_name(poison_icon, "TargetBoardPoisonIcon");

        let bleeding_icon = wm.register(WindowType::ImageBox);
        wm.set_parent(bleeding_icon, root_handle);
        wm.set_position(bleeding_icon, 32, 85);
        wm.set_size(bleeding_icon, 16, 16);
        wm.load_image(bleeding_icon, "d:/ymir work/ui/skill/bleeding.sub");
        wm.hide(bleeding_icon);
        wm.set_name(bleeding_icon, "TargetBoardBleedingIcon");

        let slow_icon = wm.register(WindowType::ImageBox);
        wm.set_parent(slow_icon, root_handle);
        wm.set_position(slow_icon, 52, 85);
        wm.set_size(slow_icon, 16, 16);
        wm.load_image(slow_icon, "d:/ymir work/ui/skill/slow.sub");
        wm.hide(slow_icon);
        wm.set_name(slow_icon, "TargetBoardSlowIcon");

        // Buttons
        let btn_whisper = wm.register(WindowType::Button);
        wm.set_parent(btn_whisper, root_handle);
        wm.set_position(btn_whisper, 80, 85);
        wm.set_size(btn_whisper, 35, 20);
        wm.set_text(btn_whisper, "Szept");
        wm.set_name(btn_whisper, "TargetBoardBtnWhisper");

        let btn_trade = wm.register(WindowType::Button);
        wm.set_parent(btn_trade, root_handle);
        wm.set_position(btn_trade, 120, 85);
        wm.set_size(btn_trade, 35, 20);
        wm.set_text(btn_trade, "Handel");
        wm.set_name(btn_trade, "TargetBoardBtnTrade");

        let btn_party = wm.register(WindowType::Button);
        wm.set_parent(btn_party, root_handle);
        wm.set_position(btn_party, 160, 85);
        wm.set_size(btn_party, 35, 20);
        wm.set_text(btn_party, "Grupa");
        wm.set_name(btn_party, "TargetBoardBtnParty");

        let btn_guild = wm.register(WindowType::Button);
        wm.set_parent(btn_guild, root_handle);
        wm.set_position(btn_guild, 200, 85);
        wm.set_size(btn_guild, 35, 20);
        wm.set_text(btn_guild, "Gildia");
        wm.set_name(btn_guild, "TargetBoardBtnGuild");

        let btn_duel = wm.register(WindowType::Button);
        wm.set_parent(btn_duel, root_handle);
        wm.set_position(btn_duel, 240, 85);
        wm.set_size(btn_duel, 35, 20);
        wm.set_text(btn_duel, "Pojedynek");
        wm.set_name(btn_duel, "TargetBoardBtnDuel");

        // Close Button
        let close_btn = wm.register(WindowType::Button);
        wm.set_parent(close_btn, root_handle);
        wm.set_position(close_btn, Self::DEFAULT_WIDTH - 20, 6);
        wm.set_size(close_btn, 14, 14);
        wm.set_text(close_btn, "x");
        wm.set_name(close_btn, "TargetBoardCloseBtn");

        Self {
            root_handle,
            name_text,
            level_text,
            race_text,
            hp_bar_bg,
            hp_bar_fg,
            hp_text,
            hp_numeric_text,
            poison_icon,
            bleeding_icon,
            slow_icon,
            btn_whisper,
            btn_trade,
            btn_party,
            btn_guild,
            btn_duel,
            close_btn,
            current_target: None,
            is_visible: false,
            bar_width: Self::DEFAULT_WIDTH,
            bar_height: Self::DEFAULT_HEIGHT,
            hp_max_width: Self::HP_BAR_MAX_WIDTH,
        }
    }

    pub fn set_target(
        &mut self,
        vid: u32,
        name: &str,
        level: u32,
        race_type: &str,
        hp: i32,
        max_hp: i32,
    ) {
        let max_hp_clamped = max_hp.max(1);
        let hp_clamped = hp.clamp(0, max_hp_clamped);
        let hp_pct = ((hp_clamped as i64 * 100) / max_hp_clamped as i64) as u8;

        self.current_target = Some(TargetBoardInfoComplete {
            vid,
            name: name.to_string(),
            level,
            hp: hp_clamped,
            max_hp: max_hp_clamped,
            hp_percent: hp_pct,
            race_type: race_type.to_string(),
            is_enemy: true,
            is_poisoned: false,
            is_bleeding: false,
            is_slowed: false,
        });

        self.update_widgets();
        self.show();
    }

    pub fn update_hp(&mut self, hp: i32, max_hp: i32) {
        if let Some(ref mut target) = self.current_target {
            let max_hp_clamped = max_hp.max(1);
            let hp_clamped = hp.clamp(0, max_hp_clamped);
            let hp_pct = ((hp_clamped as i64 * 100) / max_hp_clamped as i64) as u8;

            target.hp = hp_clamped;
            target.max_hp = max_hp_clamped;
            target.hp_percent = hp_pct;

            if hp <= 0 {
                self.clear_target();
                return;
            }

            self.update_hp_bar_widget();
        }
    }

    pub fn set_status_effects(&mut self, poison: bool, bleeding: bool, slow: bool) {
        if let Some(ref mut target) = self.current_target {
            target.is_poisoned = poison;
            target.is_bleeding = bleeding;
            target.is_slowed = slow;

            let mut wm = get_window_manager();
            if poison {
                wm.show(self.poison_icon);
            } else {
                wm.hide(self.poison_icon);
            }

            if bleeding {
                wm.show(self.bleeding_icon);
            } else {
                wm.hide(self.bleeding_icon);
            }

            if slow {
                wm.show(self.slow_icon);
            } else {
                wm.hide(self.slow_icon);
            }
        }
    }

    pub fn clear_target(&mut self) {
        self.current_target = None;
        self.hide();
    }

    pub fn show(&mut self) {
        self.is_visible = true;
        let mut wm = get_window_manager();
        wm.show(self.root_handle);
    }

    pub fn hide(&mut self) {
        self.is_visible = false;
        let mut wm = get_window_manager();
        wm.hide(self.root_handle);
    }

    pub fn is_visible(&self) -> bool {
        self.is_visible
    }

    pub fn target_vid(&self) -> Option<u32> {
        self.current_target.as_ref().map(|t| t.vid)
    }

    fn update_widgets(&mut self) {
        let Some(ref target) = self.current_target else { return; };
        let mut wm = get_window_manager();

        wm.set_text(self.name_text, &target.name);
        wm.set_text(self.level_text, &format!("Lv. {}", target.level));
        wm.set_text(self.race_text, &target.race_type);

        let fg_w = ((self.hp_max_width as f32) * (target.hp_percent as f32 / 100.0)).round() as i32;
        wm.set_size(self.hp_bar_fg, fg_w.max(0), 14);
        wm.set_text(self.hp_text, &format!("{}%", target.hp_percent));
        wm.set_text(self.hp_numeric_text, &format!("{} / {}", target.hp, target.max_hp));

        if target.is_poisoned { wm.show(self.poison_icon); } else { wm.hide(self.poison_icon); }
        if target.is_bleeding { wm.show(self.bleeding_icon); } else { wm.hide(self.bleeding_icon); }
        if target.is_slowed { wm.show(self.slow_icon); } else { wm.hide(self.slow_icon); }
    }

    fn update_hp_bar_widget(&mut self) {
        let Some(ref target) = self.current_target else { return; };
        let mut wm = get_window_manager();

        let fg_w = ((self.hp_max_width as f32) * (target.hp_percent as f32 / 100.0)).round() as i32;
        wm.set_size(self.hp_bar_fg, fg_w.max(0), 14);
        wm.set_text(self.hp_text, &format!("{}%", target.hp_percent));
        wm.set_text(self.hp_numeric_text, &format!("{} / {}", target.hp, target.max_hp));
    }
}
