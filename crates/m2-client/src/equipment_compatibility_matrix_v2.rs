#![allow(dead_code)]

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EItemType {
    None = 0,
    Weapon = 1,
    Armor = 2,
    Use = 3,
    Autouse = 4,
    Material = 5,
    Special = 6,
    Tool = 7,
    Lottery = 8,
    Elk = 9,
    Metin = 10,
    Container = 11,
    Fish = 12,
    Rod = 13,
    Resource = 14,
    Campfire = 15,
    Unique = 16,
    Skillbook = 17,
    Quest = 18,
    Polymorph = 19,
    TreasureBox = 20,
    TreasureKey = 21,
    Skillforget = 22,
    Giftbox = 23,
    Pick = 24,
    Hair = 25,
    Totem = 26,
    Blend = 27,
    Costume = 28,
    Ds = 29,
    SpecialDs = 30,
    Extract = 31,
    SecondaryCoin = 32,
    Ring = 33,
    Belt = 34,
    MaxNum = 35,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EWindows {
    ReservedWindow = 0,
    Inventory = 1,
    Equipment = 2,
    Safebox = 3,
    Mall = 4,
    DragonSoulInventory = 5,
    Ground = 6,
    BeltInventory = 7,
    WindowTypeMax = 8,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EWearPositions {
    WearBody = 0,
    WearHead = 1,
    WearFoots = 2,
    WearWrist = 3,
    WearWeapon = 4,
    WearNeck = 5,
    WearEar = 6,
    WearUnique1 = 7,
    WearUnique2 = 8,
    WearArrow = 9,
    WearShield = 10,
    WearAbility1 = 11,
    WearAbility2 = 12,
    WearAbility3 = 13,
    WearAbility4 = 14,
    WearAbility5 = 15,
    WearAbility6 = 16,
    WearAbility7 = 17,
    WearAbility8 = 18,
    WearCostumeBody = 19,
    WearCostumeHair = 20,
    WearRing1 = 21,
    WearRing2 = 22,
    WearBelt = 23,
    WearMaxNum = 24,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ELimitTypes {
    LimitNone = 0,
    LimitLevel = 1,
    LimitStr = 2,
    LimitDex = 3,
    LimitInt = 4,
    LimitCon = 5,
    LimitPcbang = 6,
    LimitRealTime = 7,
    LimitRealTimeStartFirstUse = 8,
    LimitTimerBasedOnWear = 9,
    LimitMaxNum = 10,
}

pub const ITEM_ANTIFLAG_FEMALE: u32        = 1 << 0;
pub const ITEM_ANTIFLAG_MALE: u32          = 1 << 1;
pub const ITEM_ANTIFLAG_WARRIOR: u32       = 1 << 2;
pub const ITEM_ANTIFLAG_ASSASSIN: u32      = 1 << 3;
pub const ITEM_ANTIFLAG_SURA: u32          = 1 << 4;
pub const ITEM_ANTIFLAG_SHAMAN: u32        = 1 << 5;
pub const ITEM_ANTIFLAG_GET: u32           = 1 << 6;
pub const ITEM_ANTIFLAG_DROP: u32          = 1 << 7;
pub const ITEM_ANTIFLAG_SELL: u32          = 1 << 8;
pub const ITEM_ANTIFLAG_EMPIRE_A: u32      = 1 << 9;
pub const ITEM_ANTIFLAG_EMPIRE_B: u32      = 1 << 10;
pub const ITEM_ANTIFLAG_EMPIRE_R: u32      = 1 << 11;
pub const ITEM_ANTIFLAG_SAVE: u32          = 1 << 12;
pub const ITEM_ANTIFLAG_GIVE: u32          = 1 << 13;
pub const ITEM_ANTIFLAG_PKDROP: u32        = 1 << 14;
pub const ITEM_ANTIFLAG_STACK: u32         = 1 << 15;
pub const ITEM_ANTIFLAG_MYSHOP: u32        = 1 << 16;

pub const C_INVENTORY_PAGE_SIZE: u16 = 5 * 9;
pub const C_INVENTORY_PAGE_COUNT: u16 = 2;
pub const C_ITEM_SLOT_COUNT: u16 = C_INVENTORY_PAGE_SIZE * C_INVENTORY_PAGE_COUNT;
pub const C_EQUIPMENT_START: u16 = C_ITEM_SLOT_COUNT;
pub const C_WEAR_MAX: u16 = 32;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TItemPos {
    pub window_type: u8,
    pub cell: u16,
}

impl Default for TItemPos {
    fn default() -> Self {
        Self {
            window_type: EWindows::Inventory as u8,
            cell: u16::MAX,
        }
    }
}

impl TItemPos {
    pub fn new(window_type: u8, cell: u16) -> Self {
        Self { window_type, cell }
    }

    pub fn is_equip_cell(&self) -> bool {
        if self.window_type == EWindows::Inventory as u8 || self.window_type == EWindows::Equipment as u8 {
            return (C_EQUIPMENT_START + C_WEAR_MAX > self.cell) && (C_EQUIPMENT_START <= self.cell);
        }
        false
    }
}

pub fn is_equipment(item_type: u8) -> bool {
    item_type == EItemType::Weapon as u8 || item_type == EItemType::Armor as u8
}

pub fn is_anti_flag(item_anti_flags: u32, flag: u32) -> bool {
    (item_anti_flags & flag) != 0
}

#[repr(C, packed(1))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TItemLimit {
    pub b_type: u8,
    pub l_value: i32,
}

pub fn check_item_level_limit(limit_array: &[TItemLimit], player_level: i32) -> bool {
    for limit in limit_array {
        if limit.b_type == ELimitTypes::LimitLevel as u8 {
            if player_level < limit.l_value {
                return false;
            }
        }
    }
    true
}

pub fn is_wear_position_valid(item_wear_flag: u32, target_wear_pos: u8) -> bool {
    if target_wear_pos >= EWearPositions::WearMaxNum as u8 {
        return false;
    }
    (item_wear_flag & (1 << target_wear_pos)) != 0
}
