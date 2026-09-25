use crate::ffi_guard::safe_ffi_call;
use std::sync::{OnceLock, RwLock};

pub const GUILD_INFO: u32 = 0;

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct GuildInfo {
    pub guild_id: u32,
    pub name: String,
    pub master_name: String,
    pub level: u8,
    pub exp: u32,
}

impl GuildInfo {
    pub fn new(id: u32, name: &str) -> Self {
        Self {
            guild_id: id,
            name: name.to_string(),
            master_name: String::new(),
            level: 0,
            exp: 0,
        }
    }

    pub fn update_exp(&mut self, exp: u32) {
        self.exp = exp;
    }

    pub fn set_level(&mut self, lvl: u8) {
        self.level = lvl;
    }
}

// Globalny stan gildii gracza
static CURRENT_GUILD: OnceLock<RwLock<Option<GuildInfo>>> = OnceLock::new();

pub fn get_current_guild() -> &'static RwLock<Option<GuildInfo>> {
    CURRENT_GUILD.get_or_init(|| RwLock::new(None))
}

pub fn set_current_guild(guild: GuildInfo) {
    if let Ok(mut lock) = get_current_guild().write() {
        *lock = Some(guild);
    }
}

pub fn clear_current_guild() {
    if let Ok(mut lock) = get_current_guild().write() {
        *lock = None;
    }
}

#[no_mangle]
pub unsafe extern "C" fn m2_guild_get_level() -> u8 {
    safe_ffi_call(0, || {
        if let Ok(lock) = get_current_guild().read() {
            if let Some(guild) = &*lock {
                return guild.level;
            }
        }
        0
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_guild_free(ptr: *mut GuildInfo) {
    safe_ffi_call((), || {
        if !ptr.is_null() {
            drop(Box::from_raw(ptr));
        }
    });
}
