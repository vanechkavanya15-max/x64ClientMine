// ui_window_target.rs - Metin2 Target Board UI Window (Pasek wybranego celu)
use crate::ui_window_core::{WindowType, HORIZONTAL_ALIGN_CENTER};
use crate::ui_window_mgr::get_window_manager;

/// Informacje o aktualnie wybranym celu
#[derive(Debug, Clone, PartialEq)]
pub struct TargetInfo {
    pub vid: u32,
    pub name: String,
    pub level: u32,
    pub hp: i32,
    pub max_hp: i32,
    pub hp_percent: u8,
    pub distance: f32,
    pub is_enemy: bool,
}

impl Default for TargetInfo {
    fn default() -> Self {
        Self {
            vid: 0,
            name: String::new(),
            level: 1,
            hp: 0,
            max_hp: 1,
            hp_percent: 0,
            distance: 0.0,
            is_enemy: true,
        }
    }
}

/// Okno paska celu w interfejsie Metin2 (pasek HP wroga, imie, poziom, odleglosc)
#[derive(Debug)]
pub struct UiWindowTarget {
    pub root_handle: u32,
    pub name_text: u32,
    pub level_text: u32,
    pub distance_text: u32,
    pub hp_bar_bg: u32,
    pub hp_bar_fg: u32,
    pub hp_text: u32,
    pub close_btn: u32,
    pub current_target: Option<TargetInfo>,
    pub is_visible: bool,
    pub bar_width: i32,
    pub bar_height: i32,
    pub hp_max_width: i32,
}

impl Default for UiWindowTarget {
    fn default() -> Self {
        Self::new()
    }
}

impl UiWindowTarget {
    pub const DEFAULT_WIDTH: i32 = 250;
    pub const DEFAULT_HEIGHT: i32 = 68;
    pub const HP_BAR_MAX_WIDTH: i32 = 224;

    /// Tworzy i rejestruje komponenty okna paska celu w WindowManagerze
    pub fn new() -> Self {
        let mut wm = get_window_manager();

        // 1. Glowne okno paska celu (ThinBoard)
        let root_handle = wm.register(WindowType::ThinBoard);
        wm.set_size(root_handle, Self::DEFAULT_WIDTH, Self::DEFAULT_HEIGHT);
        wm.set_position(root_handle, 0, 15);
        wm.set_align(root_handle, HORIZONTAL_ALIGN_CENTER, 0);
        wm.set_name(root_handle, "TargetBoard");
        wm.hide(root_handle);

        // 2. Poziom celu ("Lv. XX")
        let level_text = wm.register(WindowType::TextLine);
        wm.set_parent(level_text, root_handle);
        wm.set_position(level_text, 12, 10);
        wm.set_size(level_text, 45, 16);
        wm.set_font_color(level_text, [1.0, 0.85, 0.2, 1.0]); // Zlocisty
        wm.set_text(level_text, "Lv. 1");
        wm.set_name(level_text, "TargetLevelText");

        // 3. Imie celu (Target Name)
        let name_text = wm.register(WindowType::TextLine);
        wm.set_parent(name_text, root_handle);
        wm.set_position(name_text, 60, 10);
        wm.set_size(name_text, 120, 16);
        wm.set_font_color(name_text, [1.0, 1.0, 1.0, 1.0]); // Bialy
        wm.set_text(name_text, "");
        wm.set_name(name_text, "TargetNameText");

        // 4. Odleglosc do celu ("XX.Xm")
        let distance_text = wm.register(WindowType::TextLine);
        wm.set_parent(distance_text, root_handle);
        wm.set_position(distance_text, 185, 10);
        wm.set_size(distance_text, 50, 16);
        wm.set_font_color(distance_text, [0.75, 0.85, 1.0, 1.0]); // Jasnoniebieski
        wm.set_text(distance_text, "0.0m");
        wm.set_name(distance_text, "TargetDistanceText");

        // 5. Tlo paska HP (Bar3D)
        let hp_bar_bg = wm.register(WindowType::Bar3D);
        wm.set_parent(hp_bar_bg, root_handle);
        wm.set_position(hp_bar_bg, 12, 30);
        wm.set_size(hp_bar_bg, Self::HP_BAR_MAX_WIDTH + 2, 16);
        wm.set_color(hp_bar_bg, [0.15, 0.05, 0.05, 0.95]); // Ciemny bordowy podklad
        wm.set_name(hp_bar_bg, "TargetHpBarBg");

        // 6. Wypelnienie paska HP (Bar)
        let hp_bar_fg = wm.register(WindowType::Bar);
        wm.set_parent(hp_bar_fg, hp_bar_bg);
        wm.set_position(hp_bar_fg, 1, 1);
        wm.set_size(hp_bar_fg, Self::HP_BAR_MAX_WIDTH, 14);
        wm.set_color(hp_bar_fg, [0.85, 0.15, 0.15, 1.0]); // Klasyczna czerwien Metin2
        wm.set_name(hp_bar_fg, "TargetHpBarFg");

        // 7. Tekst wartosci procentowej HP
        let hp_text = wm.register(WindowType::TextLine);
        wm.set_parent(hp_text, root_handle);
        wm.set_position(hp_text, 12, 48);
        wm.set_size(hp_text, Self::HP_BAR_MAX_WIDTH, 14);
        wm.set_align(hp_text, HORIZONTAL_ALIGN_CENTER, 0);
        wm.set_font_color(hp_text, [0.95, 0.95, 0.95, 1.0]);
        wm.set_text(hp_text, "100%");
        wm.set_name(hp_text, "TargetHpText");

        // 8. Przycisk zamkniecia paska celu
        let close_btn = wm.register(WindowType::Button);
        wm.set_parent(close_btn, root_handle);
        wm.set_position(close_btn, Self::DEFAULT_WIDTH - 20, 6);
        wm.set_size(close_btn, 14, 14);
        wm.set_text(close_btn, "x");
        wm.set_name(close_btn, "TargetCloseBtn");

        Self {
            root_handle,
            name_text,
            level_text,
            distance_text,
            hp_bar_bg,
            hp_bar_fg,
            hp_text,
            close_btn,
            current_target: None,
            is_visible: false,
            bar_width: Self::DEFAULT_WIDTH,
            bar_height: Self::DEFAULT_HEIGHT,
            hp_max_width: Self::HP_BAR_MAX_WIDTH,
        }
    }

    /// Ustawia nowy cel gracza i aktualizuje elementy wizualne
    pub fn set_target(
        &mut self,
        vid: u32,
        name: &str,
        level: u32,
        hp: i32,
        max_hp: i32,
        distance: f32,
    ) {
        let max_hp_clamped = max_hp.max(1);
        let hp_clamped = hp.clamp(0, max_hp_clamped);
        let hp_pct = ((hp_clamped as i64 * 100) / max_hp_clamped as i64) as u8;

        self.current_target = Some(TargetInfo {
            vid,
            name: name.to_string(),
            level,
            hp: hp_clamped,
            max_hp: max_hp_clamped,
            hp_percent: hp_pct,
            distance,
            is_enemy: true,
        });

        self.update_widgets();
        self.show();
    }

    /// Aktualizuje punkty zycia biezacego celu
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

    /// Aktualizuje wartosc procentowa HP bezposrednio z pakietu sieciowego
    pub fn update_hp_percent(&mut self, percent: u8) {
        if let Some(ref mut target) = self.current_target {
            target.hp_percent = percent.min(100);
            if percent == 0 {
                self.clear_target();
                return;
            }
            self.update_hp_bar_widget();
        }
    }

    /// Aktualizuje odleglosc do celu
    pub fn update_distance(&mut self, distance: f32) {
        if let Some(ref mut target) = self.current_target {
            target.distance = distance.max(0.0);
            let mut wm = get_window_manager();
            wm.set_text(self.distance_text, &format!("{:.1}m", target.distance));
        }
    }

    /// Odznacza cel i ukrywa okno
    pub fn clear_target(&mut self) {
        self.current_target = None;
        self.hide();
    }

    /// Pokazuje okno celu
    pub fn show(&mut self) {
        self.is_visible = true;
        let mut wm = get_window_manager();
        wm.show(self.root_handle);
    }

    /// Ukrywa okno celu
    pub fn hide(&mut self) {
        self.is_visible = false;
        let mut wm = get_window_manager();
        wm.hide(self.root_handle);
    }

    /// Zwraca czy okno jest widoczne
    pub fn is_visible(&self) -> bool {
        self.is_visible
    }

    /// Zwraca VID aktualnego celu
    pub fn target_vid(&self) -> Option<u32> {
        self.current_target.as_ref().map(|t| t.vid)
    }

    /// Zwraca imie aktualnego celu
    pub fn target_name(&self) -> Option<&str> {
        self.current_target.as_ref().map(|t| t.name.as_str())
    }

    /// Zwraca poziom aktualnego celu
    pub fn target_level(&self) -> Option<u32> {
        self.current_target.as_ref().map(|t| t.level)
    }

    /// Zwraca procent HP aktualnego celu
    pub fn target_hp_percent(&self) -> u8 {
        self.current_target.as_ref().map(|t| t.hp_percent).unwrap_or(0)
    }

    /// Zwraca dystans do aktualnego celu
    pub fn target_distance(&self) -> f32 {
        self.current_target.as_ref().map(|t| t.distance).unwrap_or(0.0)
    }

    /// Prywatna metoda synchronizacji wszystkich widgetow z aktualnymi danymi celu
    fn update_widgets(&mut self) {
        let Some(ref target) = self.current_target else { return; };
        let mut wm = get_window_manager();

        wm.set_text(self.name_text, &target.name);
        wm.set_text(self.level_text, &format!("Lv. {}", target.level));
        wm.set_text(self.distance_text, &format!("{:.1}m", target.distance));

        let fg_w = ((self.hp_max_width as f32) * (target.hp_percent as f32 / 100.0)).round() as i32;
        wm.set_size(self.hp_bar_fg, fg_w.max(0), 14);
        wm.set_text(self.hp_text, &format!("{}%", target.hp_percent));
    }

    /// Prywatna metoda aktualizacji samego paska HP
    fn update_hp_bar_widget(&mut self) {
        let Some(ref target) = self.current_target else { return; };
        let mut wm = get_window_manager();

        let fg_w = ((self.hp_max_width as f32) * (target.hp_percent as f32 / 100.0)).round() as i32;
        wm.set_size(self.hp_bar_fg, fg_w.max(0), 14);
        wm.set_text(self.hp_text, &format!("{}%", target.hp_percent));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_window_init_and_select() {
        let mut target_win = UiWindowTarget::new();
        assert!(!target_win.is_visible());
        assert_eq!(target_win.target_vid(), None);

        target_win.set_target(1001, "Dziki Pies", 5, 250, 250, 12.5);
        assert!(target_win.is_visible());
        assert_eq!(target_win.target_vid(), Some(1001));
        assert_eq!(target_win.target_name(), Some("Dziki Pies"));
        assert_eq!(target_win.target_level(), Some(5));
        assert_eq!(target_win.target_hp_percent(), 100);
        assert_eq!(target_win.target_distance(), 12.5);
    }

    #[test]
    fn test_target_window_hp_update_and_death() {
        let mut target_win = UiWindowTarget::new();
        target_win.set_target(1002, "Dzika Malpa", 25, 1000, 1000, 8.0);

        target_win.update_hp(500, 1000);
        assert_eq!(target_win.target_hp_percent(), 50);
        assert!(target_win.is_visible());

        // Smierc celu powinna zamknac okno
        target_win.update_hp(0, 1000);
        assert!(!target_win.is_visible());
        assert_eq!(target_win.target_vid(), None);
    }

    #[test]
    fn test_target_window_distance_update() {
        let mut target_win = UiWindowTarget::new();
        target_win.set_target(1003, "Metin Cienia", 45, 50000, 50000, 40.0);

        target_win.update_distance(15.2);
        assert_eq!(target_win.target_distance(), 15.2);

        target_win.clear_target();
        assert!(!target_win.is_visible());
    }
}
