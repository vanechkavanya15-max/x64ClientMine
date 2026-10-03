// ui_window_exchange.rs - Metin2 Player Exchange UI Window (Okno handlu miedzy graczami)
use crate::ui_window_core::{WindowType, HORIZONTAL_ALIGN_CENTER};
use crate::ui_window_mgr::get_window_manager;

pub const EXCHANGE_COLS: usize = 3;
pub const EXCHANGE_ROWS: usize = 4;
pub const EXCHANGE_SLOTS_COUNT: usize = EXCHANGE_COLS * EXCHANGE_ROWS; // 12 slotow per gracz

/// Przedmiot umieszczony w oknie handlu
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExchangeItem {
    pub slot_idx: u8,
    pub vnum: u32,
    pub count: u8,
}

/// Stan procesu handlu
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExchangeState {
    Closed,
    Open,
    Locked,
    Completed,
    Canceled,
}

/// Okno handlu miedzy dwoma graczami (dwie siatki 3x4, Yang, przycisk akceptacji)
pub struct UiWindowExchange {
    pub root_handle: u32,
    pub title_text: u32,
    pub my_label: u32,
    pub target_label: u32,
    pub my_slot_handles: [u32; EXCHANGE_SLOTS_COUNT],
    pub target_slot_handles: [u32; EXCHANGE_SLOTS_COUNT],
    pub my_yang_text: u32,
    pub target_yang_text: u32,
    pub accept_button: u32,
    pub target_accept_indicator: u32,
    pub cancel_button: u32,

    // Dane stanu biznesowego
    pub state: ExchangeState,
    pub target_vid: u32,
    pub target_name: String,
    pub my_items: [Option<ExchangeItem>; EXCHANGE_SLOTS_COUNT],
    pub target_items: [Option<ExchangeItem>; EXCHANGE_SLOTS_COUNT],
    pub my_yang: u32,
    pub target_yang: u32,
    pub my_accepted: bool,
    pub target_accepted: bool,
    pub is_visible: bool,
}

impl Default for UiWindowExchange {
    fn default() -> Self {
        Self::new()
    }
}

impl UiWindowExchange {
    pub const WINDOW_WIDTH: i32 = 280;
    pub const WINDOW_HEIGHT: i32 = 260;
    pub const SLOT_SIZE: i32 = 32;
    pub const SLOT_SPACING: i32 = 34;
    pub const MY_GRID_X: i32 = 18;
    pub const TARGET_GRID_X: i32 = 158;
    pub const GRID_START_Y: i32 = 45;

    /// Tworzy okno handlu i rejestruje wszystkie widgety UI
    pub fn new() -> Self {
        let mut wm = get_window_manager();

        // 1. Glowne okno (Board)
        let root_handle = wm.register(WindowType::Board);
        wm.set_size(root_handle, Self::WINDOW_WIDTH, Self::WINDOW_HEIGHT);
        wm.set_position(root_handle, 120, 120);
        wm.set_name(root_handle, "ExchangeWindow");
        wm.hide(root_handle);

        // 2. Naglowek okna handlu
        let title_text = wm.register(WindowType::TextLine);
        wm.set_parent(title_text, root_handle);
        wm.set_position(title_text, 0, 8);
        wm.set_size(title_text, Self::WINDOW_WIDTH, 16);
        wm.set_align(title_text, HORIZONTAL_ALIGN_CENTER, 0);
        wm.set_font_color(title_text, [1.0, 0.85, 0.25, 1.0]);
        wm.set_text(title_text, "Handel z graczem");
        wm.set_name(title_text, "ExchangeTitle");

        // 3. Etykieta lewej siatki: "Moje"
        let my_label = wm.register(WindowType::TextLine);
        wm.set_parent(my_label, root_handle);
        wm.set_position(my_label, Self::MY_GRID_X, 26);
        wm.set_size(my_label, 100, 14);
        wm.set_font_color(my_label, [0.8, 0.8, 0.8, 1.0]);
        wm.set_text(my_label, "Moje");
        wm.set_name(my_label, "ExchangeMyLabel");

        // 4. Etykieta prawej siatki: "Partner"
        let target_label = wm.register(WindowType::TextLine);
        wm.set_parent(target_label, root_handle);
        wm.set_position(target_label, Self::TARGET_GRID_X, 26);
        wm.set_size(target_label, 100, 14);
        wm.set_font_color(target_label, [0.8, 0.8, 0.8, 1.0]);
        wm.set_text(target_label, "Partner");
        wm.set_name(target_label, "ExchangeTargetLabel");

        // 5. Lewa siatka 3x4 (Moje przedmioty)
        let mut my_slot_handles = [0u32; EXCHANGE_SLOTS_COUNT];
        for row in 0..EXCHANGE_ROWS {
            for col in 0..EXCHANGE_COLS {
                let idx = row * EXCHANGE_COLS + col;
                let sx = Self::MY_GRID_X + (col as i32) * Self::SLOT_SPACING;
                let sy = Self::GRID_START_Y + (row as i32) * Self::SLOT_SPACING;

                let s_handle = wm.register(WindowType::Bar3D);
                wm.set_parent(s_handle, root_handle);
                wm.set_position(s_handle, sx, sy);
                wm.set_size(s_handle, Self::SLOT_SIZE, Self::SLOT_SIZE);
                wm.set_color(s_handle, [0.08, 0.08, 0.1, 0.9]);
                wm.set_name(s_handle, &format!("MyExchangeSlot_{}", idx));
                my_slot_handles[idx] = s_handle;
            }
        }

        // 6. Prawa siatka 3x4 (Przedmioty gracza)
        let mut target_slot_handles = [0u32; EXCHANGE_SLOTS_COUNT];
        for row in 0..EXCHANGE_ROWS {
            for col in 0..EXCHANGE_COLS {
                let idx = row * EXCHANGE_COLS + col;
                let sx = Self::TARGET_GRID_X + (col as i32) * Self::SLOT_SPACING;
                let sy = Self::GRID_START_Y + (row as i32) * Self::SLOT_SPACING;

                let s_handle = wm.register(WindowType::Bar3D);
                wm.set_parent(s_handle, root_handle);
                wm.set_position(s_handle, sx, sy);
                wm.set_size(s_handle, Self::SLOT_SIZE, Self::SLOT_SIZE);
                wm.set_color(s_handle, [0.08, 0.08, 0.1, 0.9]);
                wm.set_name(s_handle, &format!("TargetExchangeSlot_{}", idx));
                target_slot_handles[idx] = s_handle;
            }
        }

        // 7. Yang moje i partnera
        let yang_y = Self::GRID_START_Y + (EXCHANGE_ROWS as i32) * Self::SLOT_SPACING + 4;

        let my_yang_text = wm.register(WindowType::TextLine);
        wm.set_parent(my_yang_text, root_handle);
        wm.set_position(my_yang_text, Self::MY_GRID_X, yang_y);
        wm.set_size(my_yang_text, 105, 14);
        wm.set_font_color(my_yang_text, [1.0, 0.85, 0.2, 1.0]);
        wm.set_text(my_yang_text, "0 Yang");
        wm.set_name(my_yang_text, "ExchangeMyYang");

        let target_yang_text = wm.register(WindowType::TextLine);
        wm.set_parent(target_yang_text, root_handle);
        wm.set_position(target_yang_text, Self::TARGET_GRID_X, yang_y);
        wm.set_size(target_yang_text, 105, 14);
        wm.set_font_color(target_yang_text, [1.0, 0.85, 0.2, 1.0]);
        wm.set_text(target_yang_text, "0 Yang");
        wm.set_name(target_yang_text, "ExchangeTargetYang");

        // 8. Wskaznik akceptacji partnera
        let target_accept_indicator = wm.register(WindowType::Bar);
        wm.set_parent(target_accept_indicator, root_handle);
        wm.set_position(target_accept_indicator, Self::TARGET_GRID_X, yang_y + 18);
        wm.set_size(target_accept_indicator, 102, 4);
        wm.set_color(target_accept_indicator, [0.25, 0.25, 0.25, 1.0]); // Szary niezaakceptowany
        wm.set_name(target_accept_indicator, "ExchangeTargetAcceptIndicator");

        // 9. Przyciski: Akceptuj i Anuluj
        let btn_y = yang_y + 26;

        let accept_button = wm.register(WindowType::Button);
        wm.set_parent(accept_button, root_handle);
        wm.set_position(accept_button, Self::MY_GRID_X, btn_y);
        wm.set_size(accept_button, 102, 22);
        wm.set_text(accept_button, "Akceptuj");
        wm.set_name(accept_button, "ExchangeAcceptBtn");

        let cancel_button = wm.register(WindowType::Button);
        wm.set_parent(cancel_button, root_handle);
        wm.set_position(cancel_button, Self::TARGET_GRID_X, btn_y);
        wm.set_size(cancel_button, 102, 22);
        wm.set_text(cancel_button, "Anuluj");
        wm.set_name(cancel_button, "ExchangeCancelBtn");

        Self {
            root_handle,
            title_text,
            my_label,
            target_label,
            my_slot_handles,
            target_slot_handles,
            my_yang_text,
            target_yang_text,
            accept_button,
            target_accept_indicator,
            cancel_button,
            state: ExchangeState::Closed,
            target_vid: 0,
            target_name: String::new(),
            my_items: [None; EXCHANGE_SLOTS_COUNT],
            target_items: [Option::None; EXCHANGE_SLOTS_COUNT],
            my_yang: 0,
            target_yang: 0,
            my_accepted: false,
            target_accepted: false,
            is_visible: false,
        }
    }

    /// Rozpoczyna handel z innym graczem
    pub fn open(&mut self, target_vid: u32, target_name: &str) {
        self.state = ExchangeState::Open;
        self.target_vid = target_vid;
        self.target_name = target_name.to_string();
        self.my_items = [None; EXCHANGE_SLOTS_COUNT];
        self.target_items = [None; EXCHANGE_SLOTS_COUNT];
        self.my_yang = 0;
        self.target_yang = 0;
        self.my_accepted = false;
        self.target_accepted = false;
        self.is_visible = true;

        let mut wm = get_window_manager();
        wm.set_text(self.title_text, &format!("Handel: {}", target_name));
        wm.set_text(self.target_label, target_name);
        wm.set_text(self.my_yang_text, "0 Yang");
        wm.set_text(self.target_yang_text, "0 Yang");
        wm.set_text(self.accept_button, "Akceptuj");
        wm.set_color(self.target_accept_indicator, [0.25, 0.25, 0.25, 1.0]);

        // Wyczysc kolory slotow
        for &s_h in &self.my_slot_handles {
            wm.set_color(s_h, [0.08, 0.08, 0.1, 0.9]);
        }
        for &s_h in &self.target_slot_handles {
            wm.set_color(s_h, [0.08, 0.08, 0.1, 0.9]);
        }

        wm.show(self.root_handle);
    }

    /// Zamyka okno handlu
    pub fn close(&mut self) {
        if self.state != ExchangeState::Completed && self.state != ExchangeState::Canceled {
            self.state = ExchangeState::Closed;
        }
        self.is_visible = false;
        let mut wm = get_window_manager();
        wm.hide(self.root_handle);
    }

    /// Dodaje moj przedmiot do handlu
    pub fn add_my_item(&mut self, slot_idx: u8, vnum: u32, count: u8) -> Result<(), &'static str> {
        if self.my_accepted {
            return Err("Nie mozna modyfikowac po zaakceptowaniu");
        }
        let idx = slot_idx as usize;
        if idx >= EXCHANGE_SLOTS_COUNT {
            return Err("Nieprawidlowy slot handlu");
        }

        self.my_items[idx] = Some(ExchangeItem {
            slot_idx,
            vnum,
            count: count.max(1),
        });

        let mut wm = get_window_manager();
        wm.set_color(self.my_slot_handles[idx], [0.15, 0.22, 0.35, 0.95]);
        Ok(())
    }

    /// Usuwa moj przedmiot z handlu
    pub fn remove_my_item(&mut self, slot_idx: u8) -> Result<(), &'static str> {
        if self.my_accepted {
            return Err("Nie mozna modyfikowac po zaakceptowaniu");
        }
        let idx = slot_idx as usize;
        if idx >= EXCHANGE_SLOTS_COUNT {
            return Err("Nieprawidlowy slot handlu");
        }

        self.my_items[idx] = None;
        let mut wm = get_window_manager();
        wm.set_color(self.my_slot_handles[idx], [0.08, 0.08, 0.1, 0.9]);
        Ok(())
    }

    /// Dodaje przedmiot zaoferowany przez partnera
    pub fn set_target_item(&mut self, slot_idx: u8, vnum: u32, count: u8) -> Result<(), &'static str> {
        let idx = slot_idx as usize;
        if idx >= EXCHANGE_SLOTS_COUNT {
            return Err("Nieprawidlowy slot handlu");
        }

        self.target_items[idx] = Some(ExchangeItem {
            slot_idx,
            vnum,
            count: count.max(1),
        });

        let mut wm = get_window_manager();
        wm.set_color(self.target_slot_handles[idx], [0.15, 0.22, 0.35, 0.95]);
        Ok(())
    }

    /// Usuwa przedmiot zaoferowany przez partnera
    pub fn remove_target_item(&mut self, slot_idx: u8) -> Result<(), &'static str> {
        let idx = slot_idx as usize;
        if idx >= EXCHANGE_SLOTS_COUNT {
            return Err("Nieprawidlowy slot handlu");
        }

        self.target_items[idx] = None;
        let mut wm = get_window_manager();
        wm.set_color(self.target_slot_handles[idx], [0.08, 0.08, 0.1, 0.9]);
        Ok(())
    }

    /// Ustawia oferowana kwote Yang przez gracza
    pub fn set_my_yang(&mut self, yang: u32) {
        if self.my_accepted {
            return;
        }
        self.my_yang = yang;
        let mut wm = get_window_manager();
        wm.set_text(self.my_yang_text, &Self::format_yang(yang));
    }

    /// Ustawia kwote Yang zaoferowana przez partnera
    pub fn set_target_yang(&mut self, yang: u32) {
        self.target_yang = yang;
        let mut wm = get_window_manager();
        wm.set_text(self.target_yang_text, &Self::format_yang(yang));
    }

    /// Przlacza moja akceptacje handlu
    pub fn toggle_my_accept(&mut self) {
        self.my_accepted = !self.my_accepted;
        let mut wm = get_window_manager();
        if self.my_accepted {
            wm.set_text(self.accept_button, "Czekaj...");
        } else {
            wm.set_text(self.accept_button, "Akceptuj");
        }

        self.update_state();
    }

    /// Ustawia stan akceptacji partnera (z pakietu sieciowego GC_EXCHANGE_ACCEPT)
    pub fn set_target_accept(&mut self, accept: bool) {
        self.target_accepted = accept;
        let mut wm = get_window_manager();
        if accept {
            wm.set_color(self.target_accept_indicator, [0.15, 0.85, 0.2, 1.0]); // Zielony
        } else {
            wm.set_color(self.target_accept_indicator, [0.25, 0.25, 0.25, 1.0]); // Szary
        }

        self.update_state();
    }

    /// Sprawdza czy obie strony zaakceptowaly handel
    pub fn is_ready_to_complete(&self) -> bool {
        self.my_accepted && self.target_accepted
    }

    /// Konczy wymiane pomyslnie
    pub fn complete_exchange(&mut self) -> Result<(), &'static str> {
        if !self.is_ready_to_complete() {
            return Err("Obie strony musza zaakceptowac wymiane");
        }
        self.state = ExchangeState::Completed;
        self.close();
        Ok(())
    }

    /// Anuluje wymiane
    pub fn cancel_exchange(&mut self) {
        self.state = ExchangeState::Canceled;
        self.close();
    }

    /// Pomocnicze formatowanie waluty Yang
    pub fn format_yang(yang: u32) -> String {
        if yang == 0 {
            return "0 Yang".to_string();
        }

        let mut num = yang;
        let mut parts = Vec::new();
        while num > 0 {
            parts.push(num % 1000);
            num /= 1000;
        }

        let mut result = String::new();
        let last = parts.pop().unwrap();
        result.push_str(&last.to_string());

        while let Some(part) = parts.pop() {
            result.push_str(&format!(",{part:03}"));
        }

        format!("{result} Yang")
    }

    fn update_state(&mut self) {
        if self.my_accepted && self.target_accepted {
            self.state = ExchangeState::Locked;
        } else {
            self.state = ExchangeState::Open;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exchange_window_flow() {
        let mut exchange = UiWindowExchange::new();
        assert!(!exchange.is_visible);
        assert_eq!(exchange.state, ExchangeState::Closed);

        // Otwarcie handlu
        exchange.open(2001, "JanKowalski");
        assert!(exchange.is_visible);
        assert_eq!(exchange.state, ExchangeState::Open);
        assert_eq!(exchange.target_name, "JanKowalski");

        // Dodawanie przedmiotow
        assert!(exchange.add_my_item(0, 10, 1).is_ok()); // Miecz
        assert!(exchange.add_my_item(11, 27001, 50).is_ok()); // Czerwone poty
        assert!(exchange.set_target_item(0, 11200, 1).is_ok()); // Mnisia

        exchange.set_my_yang(100000);
        exchange.set_target_yang(50000);

        assert_eq!(exchange.my_yang, 100000);
        assert_eq!(exchange.target_yang, 50000);

        // Akceptacja
        assert!(!exchange.is_ready_to_complete());
        exchange.toggle_my_accept();
        assert!(exchange.my_accepted);
        assert!(!exchange.is_ready_to_complete());

        // Proba edycji po mojej akceptacji powinna zwrocic blad
        assert!(exchange.add_my_item(1, 20, 1).is_err());

        // Partner akceptuje
        exchange.set_target_accept(true);
        assert!(exchange.is_ready_to_complete());

        // Finalizacja
        assert!(exchange.complete_exchange().is_ok());
        assert_eq!(exchange.state, ExchangeState::Completed);
        assert!(!exchange.is_visible);
    }

    #[test]
    fn test_exchange_cancel() {
        let mut exchange = UiWindowExchange::new();
        exchange.open(2002, "Anna");
        exchange.cancel_exchange();
        assert_eq!(exchange.state, ExchangeState::Canceled);
        assert!(!exchange.is_visible);
    }
}
