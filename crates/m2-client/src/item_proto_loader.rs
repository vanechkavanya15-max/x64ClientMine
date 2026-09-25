//! Modul `item_proto_loader` — Parser i baza danych prototypow przedmiotow CItemData (item_proto)
//! Wzorzec referencyjny C++: `source/GameLib/ItemData.cpp`
//! Opis: Odwzorowanie CItemData i TItemTable: vnum, nazwa, typ (WEAPON, ARMOR, USE, METIN, COSTUME), podtyp, rozmiar w kratkach (1x1, 1x2, 1x3), wymagany poziom, wbudowane bonusy i wartosci.

use std::collections::HashMap;

pub const ITEM_NAME_MAX_LEN: usize = 24;
pub const ITEM_LIMIT_MAX_NUM: usize = 2;
pub const ITEM_VALUES_MAX_NUM: usize = 6;
pub const ITEM_SMALL_DESCR_MAX_LEN: usize = 256;
pub const ITEM_APPLY_MAX_NUM: usize = 3;
pub const ITEM_SOCKET_MAX_NUM: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ItemType {
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
    MaxNum,
}

impl From<u8> for ItemType {
    fn from(val: u8) -> Self {
        if val < ItemType::MaxNum as u8 {
            unsafe { std::mem::transmute(val) }
        } else {
            ItemType::None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum WeaponSubTypes {
    Sword = 0,
    Dagger,
    Bow,
    TwoHanded,
    Bell,
    Fan,
    Arrow,
    NumTypes,
    None = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ArmorSubTypes {
    Body = 0,
    Head,
    Shield,
    Wrist,
    Foots,
    Neck,
    Ear,
    NumTypes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum UseSubTypes {
    Potion = 0,
    Talisman,
    Tuning,
    Move,
    TreasureBox,
    Moneybag,
    Bait,
    AbilityUp,
    Affect,
    CreateStone,
    Special,
    PotionNodelay,
    Clear,
    Invisibility,
    Detachment,
    Bucket,
    PotionContinue,
    CleanSocket,
    ChangeAttribute,
    AddAttribute,
    AddAccessorySocket,
    PutIntoAccessorySocket,
    AddAttribute2,
    Recipe,
    ChangeAttribute2,
    Bind,
    Unbind,
    TimeChargePer,
    TimeChargeFix,
    PutIntoBeltSocket,
    PutIntoRingSocket,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct ItemLimit {
    pub b_type: u8,
    pub l_value: i32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct ItemApply {
    pub b_type: u8,
    pub l_value: i32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct ItemTable {
    pub dw_vnum: u32,
    pub dw_vnum_range: u32,
    pub sz_name: [u8; ITEM_NAME_MAX_LEN + 1],
    pub sz_locale_name: [u8; ITEM_NAME_MAX_LEN + 1],
    pub b_type: u8,
    pub b_sub_type: u8,
    pub b_weight: u8,
    pub b_size: u8,
    pub dw_anti_flags: u32,
    pub dw_flags: u32,
    pub dw_wear_flags: u32,
    pub dw_immune_flag: u32,
    pub dw_ibuy_item_price: u32,
    pub dw_isell_item_price: u32,
    pub a_limits: [ItemLimit; ITEM_LIMIT_MAX_NUM],
    pub a_applies: [ItemApply; ITEM_APPLY_MAX_NUM],
    pub al_values: [i32; ITEM_VALUES_MAX_NUM],
    pub al_sockets: [i32; ITEM_SOCKET_MAX_NUM],
    pub dw_refined_vnum: u32,
    pub w_refine_set: u16,
    pub b_alter_to_magic_item_pct: u8,
    pub b_specular: u8,
    pub b_gain_socket_pct: u8,
}

impl std::fmt::Debug for ItemTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ItemTable")
            .field("dw_vnum", &{self.dw_vnum})
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct ItemData {
    pub table: ItemTable,
}

impl ItemData {
    pub fn new(table: ItemTable) -> Self {
        Self { table }
    }

    pub fn get_table(&self) -> &ItemTable {
        &self.table
    }

    pub fn get_index(&self) -> u32 {
        self.table.dw_vnum
    }

    pub fn get_name(&self) -> String {
        let name_bytes = &self.table.sz_locale_name;
        match std::ffi::CStr::from_bytes_until_nul(name_bytes) {
            Ok(s) => s.to_string_lossy().into_owned(),
            Err(_) => String::from_utf8_lossy(name_bytes).into_owned(),
        }
    }

    pub fn get_type(&self) -> u8 {
        self.table.b_type
    }

    pub fn get_sub_type(&self) -> u8 {
        self.table.b_sub_type
    }

    pub fn get_use_type_string(&self) -> &'static str {
        if self.get_type() != ItemType::Use as u8 {
            return "NOT_USE_TYPE";
        }
        
        let sub_type = self.get_sub_type();
        if sub_type == UseSubTypes::Tuning as u8 {
            return "USE_TUNING";
        } else if sub_type == UseSubTypes::Detachment as u8 {
            return "USE_DETACHMENT";
        } else if sub_type == UseSubTypes::CleanSocket as u8 {
            return "USE_CLEAN_SOCKET";
        } else if sub_type == UseSubTypes::ChangeAttribute as u8 {
            return "USE_CHANGE_ATTRIBUTE";
        } else if sub_type == UseSubTypes::AddAttribute as u8 {
            return "USE_ADD_ATTRIBUTE";
        } else if sub_type == UseSubTypes::AddAttribute2 as u8 {
            return "USE_ADD_ATTRIBUTE2";
        } else if sub_type == UseSubTypes::AddAccessorySocket as u8 {
            return "USE_ADD_ACCESSORY_SOCKET";
        } else if sub_type == UseSubTypes::PutIntoAccessorySocket as u8 {
            return "USE_PUT_INTO_ACCESSORY_SOCKET";
        } else if sub_type == UseSubTypes::PutIntoBeltSocket as u8 {
            return "USE_PUT_INTO_BELT_SOCKET";
        } else if sub_type == UseSubTypes::PutIntoRingSocket as u8 {
            return "USE_PUT_INTO_RING_SOCKET";
        }
        "USE_UNKNOWN_TYPE"
    }

    pub fn get_weapon_type(&self) -> u32 {
        self.table.b_sub_type as u32
    }

    pub fn get_size(&self) -> u8 {
        self.table.b_size
    }

    pub fn is_anti_flag(&self, flag: u32) -> bool {
        (self.table.dw_anti_flags & flag) != 0
    }

    pub fn is_flag(&self, flag: u32) -> bool {
        (self.table.dw_flags & flag) != 0
    }

    pub fn is_wearable_flag(&self, flag: u32) -> bool {
        (self.table.dw_wear_flags & flag) != 0
    }

    pub fn has_next_grade(&self) -> bool {
        self.table.dw_refined_vnum != 0
    }

    pub fn get_wear_flags(&self) -> u32 {
        self.table.dw_wear_flags
    }

    pub fn get_ibuy_item_price(&self) -> u32 {
        self.table.dw_ibuy_item_price
    }

    pub fn get_isell_item_price(&self) -> u32 {
        self.table.dw_isell_item_price
    }

    pub fn get_limit(&self, index: u8) -> Option<ItemLimit> {
        if (index as usize) < ITEM_LIMIT_MAX_NUM {
            Some(self.table.a_limits[index as usize])
        } else {
            None
        }
    }

    pub fn get_apply(&self, index: u8) -> Option<ItemApply> {
        if (index as usize) < ITEM_APPLY_MAX_NUM {
            Some(self.table.a_applies[index as usize])
        } else {
            None
        }
    }

    pub fn get_value(&self, index: u8) -> i32 {
        if (index as usize) < ITEM_VALUES_MAX_NUM {
            self.table.al_values[index as usize]
        } else {
            0
        }
    }

    pub fn get_socket(&self, index: u8) -> i32 {
        if (index as usize) < ITEM_SOCKET_MAX_NUM {
            self.table.al_sockets[index as usize]
        } else {
            -1
        }
    }

    pub fn set_socket(&mut self, index: u8, value: u32) -> i32 {
        if (index as usize) < ITEM_SOCKET_MAX_NUM {
            self.table.al_sockets[index as usize] = value as i32;
            value as i32
        } else {
            -1
        }
    }

    pub fn get_socket_count(&self) -> i32 {
        self.table.b_gain_socket_pct as i32
    }

    pub fn get_icon_number(&self) -> u32 {
        self.table.dw_vnum
    }

    pub fn get_specular_power_u(&self) -> u32 {
        self.table.b_specular as u32
    }

    pub fn get_specular_power_f(&self) -> f32 {
        (self.table.b_specular as f32) / 100.0
    }

    pub fn get_refine(&self) -> u32 {
        self.table.dw_vnum % 10
    }

    pub fn is_equipment(&self) -> bool {
        let b_type = self.get_type();
        b_type == ItemType::Weapon as u8 || b_type == ItemType::Armor as u8
    }
}

#[derive(Debug, Clone, Default)]
pub struct ItemProtoLoader {
    pub initialized: bool,
    pub items: HashMap<u32, ItemData>,
}

impl ItemProtoLoader {
    pub fn new() -> Self {
        Self {
            initialized: true,
            items: HashMap::new(),
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn load_item(&mut self, table: ItemTable) -> bool {
        let item = ItemData::new(table);
        self.items.insert(table.dw_vnum, item);
        true
    }

    pub fn get_item_data(&self, vnum: u32) -> Option<&ItemData> {
        self.items.get(&vnum)
    }

    pub fn get_item_data_mut(&mut self, vnum: u32) -> Option<&mut ItemData> {
        self.items.get_mut(&vnum)
    }
}
