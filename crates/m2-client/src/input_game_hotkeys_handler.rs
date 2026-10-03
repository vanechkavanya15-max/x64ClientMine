#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameHotkeysHandler {
    pub inventory_open: bool,
    pub status_open: bool,
    pub skills_open: bool,
    pub menu_open: bool,
    pub chat_open: bool,
}

impl Default for GameHotkeysHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl GameHotkeysHandler {
    pub fn new() -> Self {
        Self {
            inventory_open: false,
            status_open: false,
            skills_open: false,
            menu_open: false,
            chat_open: false,
        }
    }

    pub fn handle_key_down(&mut self, vkey: u32) -> bool {
        // ESC (0x1B) always takes precedence.
        if vkey == 0x1B {
            if self.chat_open {
                // If chat is open, ESC just closes it.
                self.chat_open = false;
                println!("[UI] HUD: Zamkniecie chatu (ESC)");
                return true;
            } else {
                // Otherwise toggle system menu
                self.menu_open = !self.menu_open;
                println!("[UI] HUD: Otwarto Opcje Systemowe (ESC)");
                return true;
            }
        }

        // Enter (0x0D) toggles chat, regardless of what's open (unless menu is blocking, but in Metin2 you can chat while menu is open)
        if vkey == 0x0D {
            self.chat_open = !self.chat_open;
            if self.chat_open {
                println!("[UI] HUD: Otwarcie chatu (ENTER)");
            } else {
                println!("[UI] HUD: Zamkniecie chatu (ENTER)");
            }
            return true;
        }

        // If chat is open, ignore other hotkeys so user can type.
        if self.chat_open {
            return false;
        }

        // Handle window toggles
        match vkey {
            0x49 => { // 'I' - Inventory
                self.inventory_open = !self.inventory_open;
                println!("[UI] HUD: Otwarto okno Ekwipunku (I)");
                true
            }
            0x43 => { // 'C' - Character Status
                self.status_open = !self.status_open;
                println!("[UI] HUD: Otwarto okno Statusu Postaci (C)");
                true
            }
            0x56 => { // 'V' - Skills
                self.skills_open = !self.skills_open;
                println!("[UI] HUD: Otwarto okno Umiejetnosci (V)");
                true
            }
            _ => false, // Unmapped key
        }
    }
}
