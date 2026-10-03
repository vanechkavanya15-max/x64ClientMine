#[path = "../src/input_game_hotkeys_handler.rs"]
mod input_game_hotkeys_handler;

use input_game_hotkeys_handler::GameHotkeysHandler;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle_inventory_key() {
        let mut handler = GameHotkeysHandler::new();
        assert!(!handler.inventory_open);
        
        let result = handler.handle_key_down(0x49); // 'I'
        assert!(result);
        assert!(handler.inventory_open);
        
        let result2 = handler.handle_key_down(0x49); // 'I'
        assert!(result2);
        assert!(!handler.inventory_open);
    }

    #[test]
    fn test_toggle_status_key() {
        let mut handler = GameHotkeysHandler::new();
        assert!(!handler.status_open);
        
        let result = handler.handle_key_down(0x43); // 'C'
        assert!(result);
        assert!(handler.status_open);
        
        let result2 = handler.handle_key_down(0x43); // 'C'
        assert!(result2);
        assert!(!handler.status_open);
    }

    #[test]
    fn test_toggle_skills_key() {
        let mut handler = GameHotkeysHandler::new();
        assert!(!handler.skills_open);
        
        let result = handler.handle_key_down(0x56); // 'V'
        assert!(result);
        assert!(handler.skills_open);
        
        let result2 = handler.handle_key_down(0x56); // 'V'
        assert!(result2);
        assert!(!handler.skills_open);
    }

    #[test]
    fn test_chat_open_blocks_other_keys() {
        let mut handler = GameHotkeysHandler::new();
        
        // Open chat with ENTER (0x0D)
        let result = handler.handle_key_down(0x0D);
        assert!(result);
        assert!(handler.chat_open);
        
        // Try pressing 'I', 'C', 'V' - they should be ignored
        assert!(!handler.handle_key_down(0x49));
        assert!(!handler.inventory_open);
        
        assert!(!handler.handle_key_down(0x43));
        assert!(!handler.status_open);
        
        assert!(!handler.handle_key_down(0x56));
        assert!(!handler.skills_open);
    }

    #[test]
    fn test_escape_closes_chat_or_toggles_menu() {
        let mut handler = GameHotkeysHandler::new();
        
        // ESC when chat is closed -> toggles menu
        assert!(!handler.menu_open);
        assert!(handler.handle_key_down(0x1B));
        assert!(handler.menu_open);
        
        // Open chat with ENTER
        assert!(handler.handle_key_down(0x0D));
        assert!(handler.chat_open);
        
        // ESC when chat is open -> closes chat, doesn't toggle menu
        assert!(handler.handle_key_down(0x1B));
        assert!(!handler.chat_open);
        assert!(handler.menu_open); // Menu should still be open
        
        // ESC again -> toggles menu off
        assert!(handler.handle_key_down(0x1B));
        assert!(!handler.menu_open);
    }

    #[test]
    fn test_unmapped_keys_ignored() {
        let mut handler = GameHotkeysHandler::new();
        
        // Pressing an unmapped key like Space (0x20) or arbitrary letter like 'K' (0x4B)
        let result = handler.handle_key_down(0x20);
        assert!(!result);
        
        let result = handler.handle_key_down(0x4B);
        assert!(!result);
        
        // Assert all states are still false
        assert!(!handler.inventory_open);
        assert!(!handler.status_open);
        assert!(!handler.skills_open);
        assert!(!handler.menu_open);
        assert!(!handler.chat_open);
    }
}
