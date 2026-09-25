//! Modul mod_net dla skrzynki m2-client-pyapi
//! Modul Pythona net: wysylanie pakietow i fazy sieciowe
//! Metody net.SendChatPacket, net.SendAttackPacket, net.SendUseItemPacket, net.GetPhaseName

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct ModNet {
    pub initialized: bool,
}

impl ModNet {
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
    fn test_skeleton_mod_net() {
        let obj = ModNet::new();
        assert!(obj.is_ready());
    }
}
