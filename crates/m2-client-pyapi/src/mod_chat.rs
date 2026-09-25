//! Modul mod_chat dla skrzynki m2-client-pyapi
//! Modul Pythona chat: bufor wiadomosci, filtry czatu
//! Metody chat.AppendChat, chat.AppendChatWithDelay, chat.GetLineCount

use std::collections::HashSet;

/// Struktura reprezentujaca pojedyncza wiadomosc na czacie
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatMessage {
    pub chat_type: u8,
    pub message: String,
    pub delay: u32,
}

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct ModChat {
    pub messages: Vec<ChatMessage>,
    pub filters: HashSet<u8>,
    pub initialized: bool,
}

impl ModChat {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            filters: HashSet::new(),
            initialized: true,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Dodaje wiadomosc do czatu
    pub fn append_chat(&mut self, chat_type: u8, message: String) {
        if !self.filters.contains(&chat_type) {
            self.messages.push(ChatMessage {
                chat_type,
                message,
                delay: 0,
            });
        }
    }

    /// Dodaje wiadomosc z opoznieniem do czatu
    pub fn append_chat_with_delay(&mut self, chat_type: u8, message: String, delay: u32) {
        if !self.filters.contains(&chat_type) {
            self.messages.push(ChatMessage {
                chat_type,
                message,
                delay,
            });
        }
    }

    /// Zwraca ilosc zbuforowanych linii czatu
    pub fn get_line_count(&self) -> usize {
        self.messages.len()
    }

    /// Dodaje filtr dla okreslonego typu wiadomosci
    pub fn set_filter(&mut self, chat_type: u8) {
        self.filters.insert(chat_type);
    }

    /// Usuwa filtr dla okreslonego typu wiadomosci
    pub fn remove_filter(&mut self, chat_type: u8) {
        self.filters.remove(&chat_type);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_mod_chat() {
        let obj = ModChat::new();
        assert!(obj.is_ready());
    }

    #[test]
    fn test_append_chat() {
        let mut chat = ModChat::new();
        chat.append_chat(1, "Witaj".to_string());
        assert_eq!(chat.get_line_count(), 1);
        assert_eq!(chat.messages[0].message, "Witaj");
    }

    #[test]
    fn test_filters() {
        let mut chat = ModChat::new();
        chat.set_filter(2);
        chat.append_chat(2, "Ukryte".to_string());
        assert_eq!(chat.get_line_count(), 0);

        chat.remove_filter(2);
        chat.append_chat(2, "Widoczne".to_string());
        assert_eq!(chat.get_line_count(), 1);
        assert_eq!(chat.messages[0].message, "Widoczne");
    }
}
