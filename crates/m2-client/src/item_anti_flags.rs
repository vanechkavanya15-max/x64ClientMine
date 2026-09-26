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
