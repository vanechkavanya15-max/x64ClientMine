pub const ITEM_ANTIFLAG_FEMALE: u32 = 1 << 0;
pub const ITEM_ANTIFLAG_MALE: u32 = 1 << 1;
pub const ITEM_ANTIFLAG_WARRIOR: u32 = 1 << 2;
pub const ITEM_ANTIFLAG_ASSASSIN: u32 = 1 << 3;
pub const ITEM_ANTIFLAG_SURA: u32 = 1 << 4;
pub const ITEM_ANTIFLAG_SHAMAN: u32 = 1 << 5;
pub const ITEM_ANTIFLAG_GET: u32 = 1 << 6;
pub const ITEM_ANTIFLAG_DROP: u32 = 1 << 7;
pub const ITEM_ANTIFLAG_SELL: u32 = 1 << 8;
pub const ITEM_ANTIFLAG_EMPIRE_A: u32 = 1 << 9;
pub const ITEM_ANTIFLAG_EMPIRE_B: u32 = 1 << 10;
pub const ITEM_ANTIFLAG_EMPIRE_R: u32 = 1 << 11;
pub const ITEM_ANTIFLAG_SAVE: u32 = 1 << 12;
pub const ITEM_ANTIFLAG_GIVE: u32 = 1 << 13;
pub const ITEM_ANTIFLAG_PKDROP: u32 = 1 << 14;
pub const ITEM_ANTIFLAG_STACK: u32 = 1 << 15;
pub const ITEM_ANTIFLAG_MYSHOP: u32 = 1 << 16;
pub const ITEM_ANTIFLAG_SAFEBOX: u32 = 1 << 17;

pub const LIMIT_NONE: u8 = 0;
pub const LIMIT_LEVEL: u8 = 1;
pub const LIMIT_STR: u8 = 2;
pub const LIMIT_DEX: u8 = 3;
pub const LIMIT_INT: u8 = 4;
pub const LIMIT_CON: u8 = 5;
pub const LIMIT_PCBANG: u8 = 6;
pub const LIMIT_REAL_TIME: u8 = 7;
pub const LIMIT_REAL_TIME_START_FIRST_USE: u8 = 8;
pub const LIMIT_TIMER_BASED_ON_WEAR: u8 = 9;

pub fn can_equip_full(
    anti_flags: u32,
    limit_type: u8,
    limit_value: i32,
    player_class: u8,
    player_sex: u8,
    player_level: u32,
    stats: (i32, i32, i32, i32),
    cur_time: u32,
) -> bool {
    if player_sex == 0 && (anti_flags & ITEM_ANTIFLAG_MALE) != 0 {
        return false;
    }
    if player_sex == 1 && (anti_flags & ITEM_ANTIFLAG_FEMALE) != 0 {
        return false;
    }

    match player_class {
        0 => if (anti_flags & ITEM_ANTIFLAG_WARRIOR) != 0 { return false; },
        1 => if (anti_flags & ITEM_ANTIFLAG_ASSASSIN) != 0 { return false; },
        2 => if (anti_flags & ITEM_ANTIFLAG_SURA) != 0 { return false; },
        3 => if (anti_flags & ITEM_ANTIFLAG_SHAMAN) != 0 { return false; },
        _ => {}
    }

    match limit_type {
        LIMIT_NONE => {}
        LIMIT_LEVEL => {
            if (player_level as i32) < limit_value {
                return false;
            }
        }
        LIMIT_STR => {
            if stats.0 < limit_value {
                return false;
            }
        }
        LIMIT_DEX => {
            if stats.1 < limit_value {
                return false;
            }
        }
        LIMIT_INT => {
            if stats.2 < limit_value {
                return false;
            }
        }
        LIMIT_CON => {
            if stats.3 < limit_value {
                return false;
            }
        }
        LIMIT_PCBANG => {}
        LIMIT_REAL_TIME | LIMIT_REAL_TIME_START_FIRST_USE => {
            if limit_value != 0 && (cur_time as i32) >= limit_value {
                return false;
            }
        }
        LIMIT_TIMER_BASED_ON_WEAR => {
            if limit_value == 0 {
                return false;
            }
        }
        _ => {}
    }

    true
}

#[no_mangle]
pub extern "C" fn m2_item_can_equip_full(
    anti_flags: u32,
    limit_type: u8,
    limit_value: i32,
    player_class: u8,
    player_sex: u8,
    player_level: u32,
    stat_str: i32,
    stat_dex: i32,
    stat_int: i32,
    stat_con: i32,
    cur_time: u32,
) -> bool {
    can_equip_full(
        anti_flags,
        limit_type,
        limit_value,
        player_class,
        player_sex,
        player_level,
        (stat_str, stat_dex, stat_int, stat_con),
        cur_time,
    )
}
