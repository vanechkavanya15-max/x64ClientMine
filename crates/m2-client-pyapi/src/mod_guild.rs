//! Modul mod_guild dla skrzynki m2-client-pyapi
//! Modul Pythona guild: dane gildii, czlonkowie, wojny
//! Metody guild.GetGuildID, guild.GetGuildName, guild.GetMemberCount

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct ModGuild {
    pub initialized: bool,
}

impl ModGuild {
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
    fn test_skeleton_mod_guild() {
        let obj = ModGuild::new();
        assert!(obj.is_ready());
    }
}
