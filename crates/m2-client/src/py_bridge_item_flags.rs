#![allow(dead_code)]

// Item flags
pub const ITEM_FLAG_REFINEABLE: u32         = 1 << 0;
pub const ITEM_FLAG_SAVE: u32               = 1 << 1;
pub const ITEM_FLAG_STACKABLE: u32          = 1 << 2;
pub const ITEM_FLAG_COUNT_PER_1GOLD: u32    = 1 << 3;
pub const ITEM_FLAG_SLOW_QUERY: u32         = 1 << 4;
pub const ITEM_FLAG_RARE: u32               = 1 << 5;
pub const ITEM_FLAG_UNIQUE: u32             = 1 << 6;
pub const ITEM_FLAG_MAKECOUNT: u32          = 1 << 7;
pub const ITEM_FLAG_IRREMOVABLE: u32        = 1 << 8;
pub const ITEM_FLAG_CONFIRM_WHEN_USE: u32   = 1 << 9;
pub const ITEM_FLAG_QUEST_USE: u32          = 1 << 10;
pub const ITEM_FLAG_QUEST_USE_MULTIPLE: u32 = 1 << 11;
pub const ITEM_FLAG_UNUSED03: u32           = 1 << 12;
pub const ITEM_FLAG_LOG: u32                = 1 << 13;
pub const ITEM_FLAG_APPLICABLE: u32         = 1 << 14;

// Item anti flags
pub const ITEM_ANTIFLAG_FEMALE: u32         = 1 << 0;
pub const ITEM_ANTIFLAG_MALE: u32           = 1 << 1;
pub const ITEM_ANTIFLAG_WARRIOR: u32        = 1 << 2;
pub const ITEM_ANTIFLAG_ASSASSIN: u32       = 1 << 3;
pub const ITEM_ANTIFLAG_SURA: u32           = 1 << 4;
pub const ITEM_ANTIFLAG_SHAMAN: u32         = 1 << 5;
pub const ITEM_ANTIFLAG_GET: u32            = 1 << 6;
pub const ITEM_ANTIFLAG_DROP: u32           = 1 << 7;
pub const ITEM_ANTIFLAG_SELL: u32           = 1 << 8;
pub const ITEM_ANTIFLAG_EMPIRE_A: u32       = 1 << 9;
pub const ITEM_ANTIFLAG_EMPIRE_B: u32       = 1 << 10;
pub const ITEM_ANTIFLAG_EMPIRE_R: u32       = 1 << 11;
pub const ITEM_ANTIFLAG_SAVE: u32           = 1 << 12;
pub const ITEM_ANTIFLAG_GIVE: u32           = 1 << 13;
pub const ITEM_ANTIFLAG_PKDROP: u32         = 1 << 14;
pub const ITEM_ANTIFLAG_STACK: u32          = 1 << 15;
pub const ITEM_ANTIFLAG_MYSHOP: u32         = 1 << 16;

// Types
pub const ITEM_TYPE_NONE: u8           = 0;
pub const ITEM_TYPE_WEAPON: u8         = 1;
pub const ITEM_TYPE_ARMOR: u8          = 2;
pub const ITEM_TYPE_USE: u8            = 3;
pub const ITEM_TYPE_AUTOUSE: u8        = 4;
pub const ITEM_TYPE_MATERIAL: u8       = 5;
pub const ITEM_TYPE_SPECIAL: u8        = 6;
pub const ITEM_TYPE_TOOL: u8           = 7;
pub const ITEM_TYPE_LOTTERY: u8        = 8;
pub const ITEM_TYPE_ELK: u8            = 9;
pub const ITEM_TYPE_METIN: u8          = 10;
pub const ITEM_TYPE_CONTAINER: u8      = 11;
pub const ITEM_TYPE_FISH: u8           = 12;
pub const ITEM_TYPE_ROD: u8            = 13;
pub const ITEM_TYPE_RESOURCE: u8       = 14;
pub const ITEM_TYPE_CAMPFIRE: u8       = 15;
pub const ITEM_TYPE_UNIQUE: u8         = 16;
pub const ITEM_TYPE_SKILLBOOK: u8      = 17;
pub const ITEM_TYPE_QUEST: u8          = 18;
pub const ITEM_TYPE_POLYMORPH: u8      = 19;
pub const ITEM_TYPE_TREASURE_BOX: u8   = 20;
pub const ITEM_TYPE_TREASURE_KEY: u8   = 21;
pub const ITEM_TYPE_SKILLFORGET: u8    = 22;
pub const ITEM_TYPE_GIFTBOX: u8        = 23;
pub const ITEM_TYPE_PICK: u8           = 24;
pub const ITEM_TYPE_HAIR: u8           = 25;
pub const ITEM_TYPE_TOTEM: u8          = 26;
pub const ITEM_TYPE_BLEND: u8          = 27;
pub const ITEM_TYPE_COSTUME: u8        = 28;

// SubTypes for USE
pub const USE_POTION: u8                    = 0;
pub const USE_TALISMAN: u8                  = 1;
pub const USE_TUNING: u8                    = 2;
pub const USE_MOVE: u8                      = 3;
pub const USE_TREASURE_BOX: u8              = 4;
pub const USE_MONEYBAG: u8                  = 5;
pub const USE_BAIT: u8                      = 6;
pub const USE_ABILITY_UP: u8                = 7;
pub const USE_AFFECT: u8                    = 8;
pub const USE_CREATE_STONE: u8              = 9;
pub const USE_SPECIAL: u8                   = 10;
pub const USE_POTION_NODELAY: u8            = 11;
pub const USE_CLEAR: u8                     = 12;
pub const USE_INVISIBILITY: u8              = 13;
pub const USE_DETACHMENT: u8                = 14;
pub const USE_BUCKET: u8                    = 15;
pub const USE_POTION_CONTINUE: u8           = 16;
pub const USE_CLEAN_SOCKET: u8              = 17;
pub const USE_CHANGE_ATTRIBUTE: u8          = 18;
pub const USE_ADD_ATTRIBUTE: u8             = 19;
pub const USE_ADD_ACCESSORY_SOCKET: u8      = 20;
pub const USE_PUT_INTO_ACCESSORY_SOCKET: u8 = 21;
pub const USE_ADD_ATTRIBUTE2: u8            = 22;
pub const USE_RECIPE: u8                    = 23;
pub const USE_CHANGE_ATTRIBUTE2: u8         = 24;
pub const USE_BIND: u8                      = 25;
pub const USE_UNBIND: u8                    = 26;
pub const USE_TIME_CHARGE_PER: u8           = 27;
pub const USE_TIME_CHARGE_FIX: u8           = 28;

#[no_mangle]
pub extern "C" fn m2_py_item_is_metin(item_type: u8) -> i32 {
    if item_type == ITEM_TYPE_METIN {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn m2_py_item_is_refine_scroll(item_type: u8, sub_type: u8) -> i32 {
    if item_type == ITEM_TYPE_USE && sub_type == USE_TUNING {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn m2_py_item_is_detach_scroll(item_type: u8, sub_type: u8) -> i32 {
    if item_type == ITEM_TYPE_USE && sub_type == USE_DETACHMENT {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn m2_py_item_is_key(item_type: u8) -> i32 {
    if item_type == ITEM_TYPE_TREASURE_KEY {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn m2_py_item_is_1gold_item(flags: u32) -> i32 {
    if (flags & ITEM_FLAG_COUNT_PER_1GOLD) != 0 {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn m2_py_item_is_anti_flag(anti_flags: u32, flag: u32) -> i32 {
    if (anti_flags & flag) != 0 {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn m2_py_item_is_flag(flags: u32, flag: u32) -> i32 {
    if (flags & flag) != 0 {
        1
    } else {
        0
    }
}
