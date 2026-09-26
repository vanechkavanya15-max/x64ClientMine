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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerClass {
    Warrior,
    Assassin,
    Sura,
    Shaman,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerSex {
    Male,
    Female,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub anti_flags: u32,
    pub limit_level: u32,
}

/// Sprawdza czy gracz spelnia wymagania klasy, plci oraz poziomu do zalozenia przedmiotu.
pub fn can_equip_by_player(
    item: &Item,
    player_class: PlayerClass,
    player_sex: PlayerSex,
    player_level: u32,
) -> bool {
    // 3. Wymagany poziom minimalny gracza: item.limit_level <= player.level
    if player_level < item.limit_level {
        return false;
    }

    // 1. Flagi klasowe
    let class_blocked = match player_class {
        PlayerClass::Warrior => (item.anti_flags & ITEM_ANTIFLAG_WARRIOR) != 0,
        PlayerClass::Assassin => (item.anti_flags & ITEM_ANTIFLAG_ASSASSIN) != 0,
        PlayerClass::Sura => (item.anti_flags & ITEM_ANTIFLAG_SURA) != 0,
        PlayerClass::Shaman => (item.anti_flags & ITEM_ANTIFLAG_SHAMAN) != 0,
    };

    if class_blocked {
        return false;
    }

    // 2. Flagi plci
    let sex_blocked = match player_sex {
        PlayerSex::Male => (item.anti_flags & ITEM_ANTIFLAG_MALE) != 0,
        PlayerSex::Female => (item.anti_flags & ITEM_ANTIFLAG_FEMALE) != 0,
    };

    if sex_blocked {
        return false;
    }

    true
}

pub const ANTI_FEMALE: u32 = 1 << 0;
pub const ANTI_MALE: u32 = 1 << 1;
pub const ANTI_MUSA: u32 = 1 << 2;
pub const ANTI_ASSASSIN: u32 = 1 << 3;
pub const ANTI_SURA: u32 = 1 << 4;
pub const ANTI_MUDANG: u32 = 1 << 5;

pub fn can_equip(anti_flags: u32, job: u8, sex: u8, player_level: u32, req_level: u32) -> bool {
    if player_level < req_level {
        return false;
    }
    
    if sex == 0 && (anti_flags & ANTI_MALE) != 0 {
        return false;
    }
    
    if sex == 1 && (anti_flags & ANTI_FEMALE) != 0 {
        return false;
    }
    
    if job == 0 && (anti_flags & ANTI_MUSA) != 0 {
        return false;
    }
    
    if job == 1 && (anti_flags & ANTI_ASSASSIN) != 0 {
        return false;
    }
    
    if job == 2 && (anti_flags & ANTI_SURA) != 0 {
        return false;
    }
    
    if job == 3 && (anti_flags & ANTI_MUDANG) != 0 {
        return false;
    }
    
    true
}

#[no_mangle]
pub extern "C" fn m2_item_can_equip(
    anti_flags: u32,
    job: u8,
    sex: u8,
    player_level: u32,
    req_level: u32,
) -> bool {
    can_equip(anti_flags, job, sex, player_level, req_level)
}
