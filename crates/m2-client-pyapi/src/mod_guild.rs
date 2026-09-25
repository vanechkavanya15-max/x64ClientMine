//! Modul mod_guild dla skrzynki m2-client-pyapi
//! Modul Pythona guild: dane gildii, czlonkowie, wojny
//! Metody guild.GetGuildID, guild.GetGuildName, guild.GetMemberCount

/// Struktura przechowujaca podstawowe dane gildii
#[derive(Debug, Clone, Default)]
pub struct GuildData {
    pub id: u32,
    pub name: String,
    pub member_count: u32,
}

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct ModGuild {
    pub initialized: bool,
    pub guild_data: Option<GuildData>,
}

impl ModGuild {
    pub fn new() -> Self {
        Self {
            initialized: true,
            guild_data: None,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Pobiera ID gildii (zwraca 0 jezeli brak danych)
    pub fn get_guild_id(&self) -> u32 {
        self.guild_data.as_ref().map(|d| d.id).unwrap_or(0)
    }

    /// Pobiera nazwe gildii (zwraca pusty string jezeli brak danych)
    pub fn get_guild_name(&self) -> String {
        self.guild_data.as_ref().map(|d| d.name.clone()).unwrap_or_default()
    }

    /// Pobiera liczbe czlonkow gildii (zwraca 0 jezeli brak danych)
    pub fn get_member_count(&self) -> u32 {
        self.guild_data.as_ref().map(|d| d.member_count).unwrap_or(0)
    }

    /// Ustawia dane gildii
    pub fn set_guild_data(&mut self, id: u32, name: String, member_count: u32) {
        self.guild_data = Some(GuildData {
            id,
            name,
            member_count,
        });
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

    #[test]
    fn test_guild_data_defaults() {
        let obj = ModGuild::new();
        assert_eq!(obj.get_guild_id(), 0);
        assert_eq!(obj.get_guild_name(), "");
        assert_eq!(obj.get_member_count(), 0);
    }

    #[test]
    fn test_set_guild_data() {
        let mut obj = ModGuild::new();
        obj.set_guild_data(123, "Wojownicy".to_string(), 42);
        assert_eq!(obj.get_guild_id(), 123);
        assert_eq!(obj.get_guild_name(), "Wojownicy");
        assert_eq!(obj.get_member_count(), 42);
    }
}
