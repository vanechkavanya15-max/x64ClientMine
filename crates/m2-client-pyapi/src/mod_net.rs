//! Modul mod_net dla skrzynki m2-client-pyapi
//! Modul Pythona net: wysylanie pakietow i fazy sieciowe
//! Metody net.SendChatPacket, net.SendAttackPacket, net.SendUseItemPacket, net.GetPhaseName

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct ModNet {
    pub initialized: bool,
    pub phase_name: String,
    pub last_chat: String,
    pub last_attack_vid: u32,
    pub last_use_item_pos: u32,
}

impl ModNet {
    pub fn new() -> Self {
        Self {
            initialized: true,
            phase_name: String::from("Game"),
            last_chat: String::new(),
            last_attack_vid: 0,
            last_use_item_pos: 0,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn send_chat_packet(&mut self, text: &str) -> bool {
        self.last_chat = text.to_string();
        true
    }

    pub fn send_attack_packet(&mut self, vid: u32) -> bool {
        if vid == 0 {
            return false;
        }
        self.last_attack_vid = vid;
        true
    }

    pub fn send_use_item_packet(&mut self, pos: u32) -> bool {
        self.last_use_item_pos = pos;
        true
    }

    pub fn get_phase_name(&self) -> String {
        self.phase_name.clone()
    }

    pub fn set_phase_name(&mut self, phase: &str) {
        self.phase_name = phase.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_mod_net() {
        let obj = ModNet::new();
        assert!(obj.is_ready());
    }

    #[test]
    fn test_mod_net_methods() {
        let mut obj = ModNet::new();
        
        assert_eq!(obj.get_phase_name(), "Game");
        obj.set_phase_name("Login");
        assert_eq!(obj.get_phase_name(), "Login");
        
        assert!(obj.send_chat_packet("hello"));
        assert_eq!(obj.last_chat, "hello");
        
        assert!(!obj.send_attack_packet(0));
        assert!(obj.send_attack_packet(123));
        assert_eq!(obj.last_attack_vid, 123);
        
        assert!(obj.send_use_item_packet(456));
        assert_eq!(obj.last_use_item_pos, 456);
    }
}
