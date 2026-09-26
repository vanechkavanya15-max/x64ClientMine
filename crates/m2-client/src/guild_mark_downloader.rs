use std::collections::HashMap;

const GUILD_MARK_WIDTH: usize = 16;
const GUILD_MARK_HEIGHT: usize = 12;
const GUILD_MARK_BPP: usize = 2; // RGB565 is 2 bytes per pixel
const GUILD_MARK_SIZE: usize = GUILD_MARK_WIDTH * GUILD_MARK_HEIGHT * GUILD_MARK_BPP; // 384 bytes

pub struct GuildMarkDownloader {
    guild_symbols: HashMap<u32, Vec<u8>>,
}

impl GuildMarkDownloader {
    pub fn new() -> Self {
        Self {
            guild_symbols: HashMap::new(),
        }
    }

    /// Sets the guild mark if the buffer size is exactly 384 bytes.
    /// Returns true if successful, false otherwise.
    pub fn set_guild_mark(&mut self, guild_id: u32, data: &[u8]) -> bool {
        if data.len() != GUILD_MARK_SIZE {
            return false;
        }

        self.guild_symbols.insert(guild_id, data.to_vec());
        true
    }

    /// Gets the guild mark data if it exists in the cache.
    pub fn get_guild_mark(&self, guild_id: u32) -> Option<&[u8]> {
        self.guild_symbols.get(&guild_id).map(|v| v.as_slice())
    }
}

impl Default for GuildMarkDownloader {
    fn default() -> Self {
        Self::new()
    }
}
