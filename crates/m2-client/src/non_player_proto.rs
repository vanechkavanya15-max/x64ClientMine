//! Modul `non_player_proto` — Parser i baza danych potworow oraz NPC CPythonNonPlayer (mob_proto)
//! Wzorzec referencyjny C++: `source/UserInterface/PythonNonPlayer.cpp`
//! Opis: Odwzorowanie CPythonNonPlayer i TMobTable: vnum potwora, nazwa, poziom, ranga (PAWN, KNIGHT, BOSS, KING), typ (MONSTER, NPC, STONE, WARP), maksymalne HP.

use std::collections::HashMap;

pub const CHARACTER_NAME_MAX_LEN: usize = 24;
pub const MOB_ENCHANTS_MAX_NUM: usize = 6;
pub const MOB_RESISTS_MAX_NUM: usize = 11;
pub const MOB_ATTRIBUTE_MAX_NUM: usize = 12;
pub const MOB_SKILL_MAX_NUM: usize = 5;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClickEvent {
    None = 0,
    Battle = 1,
    Shop = 2,
    Talk = 3,
    Vehicle = 4,
    MaxNum,
}

impl Default for ClickEvent {
    fn default() -> Self {
        Self::None
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MobEnchant {
    Curse = 0,
    Slow,
    Poison,
    Stun,
    Critical,
    Penetrate,
    MaxNum,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MobResist {
    Sword = 0,
    Twohand,
    Dagger,
    Bell,
    Fan,
    Bow,
    Fire,
    Elect,
    Magic,
    Wind,
    Poison,
    MaxNum,
}

#[repr(C, packed(1))]
#[derive(Debug, Clone, Copy)]
pub struct MobSkillLevel {
    pub vnum: u32,
    pub level: u8,
}

impl Default for MobSkillLevel {
    fn default() -> Self {
        Self { vnum: 0, level: 0 }
    }
}

#[repr(C, packed(1))]
#[derive(Debug, Clone, Copy)]
pub struct MobTable {
    pub vnum: u32,
    pub name: [u8; CHARACTER_NAME_MAX_LEN + 1],
    pub locale_name: [u8; CHARACTER_NAME_MAX_LEN + 1],

    pub mob_type: u8, // Monster, NPC
    pub rank: u8,     // PAWN, KNIGHT, KING
    pub battle_type: u8,
    pub level: u8,
    pub size: u8,

    pub gold_min: u32,
    pub gold_max: u32,
    pub exp: u32,
    pub max_hp: u32,
    pub regen_cycle: u8,
    pub regen_percent: u8,
    pub def: u16,

    pub ai_flag: u32,
    pub race_flag: u32,
    pub immune_flag: u32,

    pub str: u8,
    pub dex: u8,
    pub con: u8,
    pub int: u8,
    pub damage_range: [u32; 2],

    pub attack_speed: i16,
    pub moving_speed: i16,
    pub aggresive_hp_pct: u8,
    pub aggressive_sight: u16,
    pub attack_range: u16,

    pub enchants: [u8; MOB_ENCHANTS_MAX_NUM],
    pub resists: [u8; MOB_RESISTS_MAX_NUM],

    pub resurrection_vnum: u32,
    pub drop_item_vnum: u32,

    pub mount_capacity: u8,
    pub on_click_type: u8,

    pub empire: u8,
    pub folder: [u8; 64 + 1],
    pub dam_multiply: f32,
    pub summon_vnum: u32,
    pub drain_sp: u32,
    pub monster_color: u32,
    pub polymorph_item_vnum: u32,

    pub skills: [MobSkillLevel; MOB_SKILL_MAX_NUM],

    pub berserk_point: u8,
    pub stone_skin_point: u8,
    pub god_speed_point: u8,
    pub death_blow_point: u8,
    pub revive_point: u8,
}

impl Default for MobTable {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[derive(Debug, Clone, Default)]
pub struct NonPlayerProto {
    pub initialized: bool,
    pub mob_tables: HashMap<u32, MobTable>,
}

impl NonPlayerProto {
    pub fn new() -> Self {
        Self {
            initialized: true,
            mob_tables: HashMap::new(),
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn clear(&mut self) {
        self.mob_tables.clear();
    }

    pub fn load_from_buffer(&mut self, data: &[u8]) -> bool {
        let table_size = std::mem::size_of::<MobTable>();
        if data.len() % table_size != 0 {
            return false;
        }

        let num_elements = data.len() / table_size;
        for i in 0..num_elements {
            let start = i * table_size;
            let end = start + table_size;
            let chunk = &data[start..end];
            let mut table = MobTable::default();
            unsafe {
                std::ptr::copy_nonoverlapping(
                    chunk.as_ptr(),
                    &mut table as *mut MobTable as *mut u8,
                    table_size,
                );
            }
            self.mob_tables.insert(table.vnum, table);
        }

        true
    }

    pub fn get_table(&self, vnum: u32) -> Option<&MobTable> {
        self.mob_tables.get(&vnum)
    }

    pub fn get_name(&self, vnum: u32) -> Option<String> {
        let table = self.get_table(vnum)?;
        let name_bytes = table.locale_name.split(|&b| b == 0).next().unwrap_or(&table.locale_name);
        Some(String::from_utf8_lossy(name_bytes).into_owned())
    }

    pub fn get_instance_type(&self, vnum: u32) -> Option<u8> {
        let table = self.get_table(vnum)?;
        Some(table.mob_type)
    }

    pub fn get_event_type(&self, vnum: u32) -> u8 {
        if let Some(table) = self.get_table(vnum) {
            table.on_click_type
        } else {
            ClickEvent::None as u8
        }
    }

    pub fn get_monster_name(&self, vnum: u32) -> String {
        self.get_name(vnum).unwrap_or_default()
    }

    pub fn get_monster_color(&self, vnum: u32) -> u32 {
        if let Some(table) = self.get_table(vnum) {
            table.monster_color
        } else {
            0
        }
    }
}
