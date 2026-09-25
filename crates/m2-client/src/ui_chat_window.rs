//! Modul `ui_chat_window` — Okno czatu z kolorowaniem kanalow i przewijaniem CPythonChat
//! Wzorzec referencyjny C++: `source/UserInterface/PythonChat.cpp`
//! Opis: Odwzorowanie CPythonChat: historia wiadomosci, kolorowanie wg typu (TALK bialy, INFO czerwony/zloty, NOTICE zolty, PARTY niebieski, GUILD zielony, SHOUT), pasek wpisywania tekstu.

#[derive(Debug, Clone, Default)]
pub struct UiChatWindow {
    pub initialized: bool,
}

impl UiChatWindow {
    pub fn new() -> Self {
        Self { initialized: true }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_ui_chat_window() {
        let instance = UiChatWindow::new();
        assert!(instance.is_ready());
    }
}
