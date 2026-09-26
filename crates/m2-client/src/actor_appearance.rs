pub const PART_MAIN: u8 = 0;
pub const PART_WEAPON: u8 = 1;
pub const PART_HEAD: u8 = 2;
pub const PART_WEAPON_LEFT: u8 = 3;
pub const PART_HAIR: u8 = 4;
pub const PART_SASH: u8 = 5;
pub const PART_MAX_NUM: u8 = 6;

pub const ITEM_TYPE_WEAPON: u8 = 1;
pub const ITEM_TYPE_ARMOR: u8 = 2;
pub const ITEM_TYPE_ROD: u8 = 13;

pub const WEAPON_SWORD: u8 = 0;
pub const WEAPON_DAGGER: u8 = 1;
pub const WEAPON_BOW: u8 = 2;
pub const WEAPON_TWO_HANDED: u8 = 3;
pub const WEAPON_BELL: u8 = 4;
pub const WEAPON_FAN: u8 = 5;
pub const WEAPON_ARROW: u8 = 6;
pub const WEAPON_NUM_TYPES: u8 = 7;
pub const WEAPON_NONE: u8 = WEAPON_NUM_TYPES + 1;

pub const MODE_GENERAL: u8 = 1;
pub const MODE_ONEHAND_SWORD: u8 = 2;
pub const MODE_TWOHAND_SWORD: u8 = 3;
pub const MODE_DUALHAND_SWORD: u8 = 4;
pub const MODE_BOW: u8 = 5;
pub const MODE_FAN: u8 = 6;
pub const MODE_BELL: u8 = 7;
pub const MODE_FISHING: u8 = 8;
pub const MODE_HORSE: u8 = 9;
pub const MODE_HORSE_ONEHAND_SWORD: u8 = 10;
pub const MODE_HORSE_TWOHAND_SWORD: u8 = 11;
pub const MODE_HORSE_DUALHAND_SWORD: u8 = 12;
pub const MODE_HORSE_BOW: u8 = 13;
pub const MODE_HORSE_FAN: u8 = 14;
pub const MODE_HORSE_BELL: u8 = 15;
pub const MODE_WEDDING_DRESS: u8 = 16;

#[derive(Clone, Default)]
pub struct ActorAppearance {
    parts: [u32; PART_MAX_NUM as usize],
    pub weapon_vnum: u32,
    pub armor_vnum: u32,
    pub hair_vnum: u32,
    pub sash_vnum: u32,
    motion_mode: u8,
    is_poly: bool,
    is_wearing_dress: bool,
    is_mounting: bool,
}

pub trait ItemDataManager {
    fn get_item_type(&self, vnum: u32) -> Option<u8>;
    fn get_weapon_type(&self, vnum: u32) -> Option<u8>;
}

impl ActorAppearance {
    pub fn new() -> Self {
        Self {
            parts: [0; PART_MAX_NUM as usize],
            weapon_vnum: 0,
            armor_vnum: 0,
            hair_vnum: 0,
            sash_vnum: 0,
            motion_mode: MODE_GENERAL,
            is_poly: false,
            is_wearing_dress: false,
            is_mounting: false,
        }
    }

    pub fn attach_weapon(&mut self, vnum: u32) {
        self.weapon_vnum = vnum;
        self.set_part_vnum(PART_WEAPON, vnum);
    }

    pub fn change_armor(&mut self, vnum: u32) {
        self.armor_vnum = vnum;
        self.set_part_vnum(PART_MAIN, vnum);
    }

    pub fn set_hair(&mut self, vnum: u32) {
        self.hair_vnum = vnum;
        self.set_part_vnum(PART_HAIR, vnum);
    }

    pub fn set_part_vnum(&mut self, part_type: u8, vnum: u32) {
        if part_type < PART_MAX_NUM {
            self.parts[part_type as usize] = vnum;
            match part_type {
                PART_MAIN => self.armor_vnum = vnum,
                PART_WEAPON => self.weapon_vnum = vnum,
                PART_HAIR => self.hair_vnum = vnum,
                PART_SASH => self.sash_vnum = vnum,
                _ => {}
            }
        }
    }

    pub fn get_part_vnum(&self, part_type: u8) -> u32 {
        if part_type < PART_MAX_NUM {
            self.parts[part_type as usize]
        } else {
            0
        }
    }

    pub fn set_poly(&mut self, poly: bool) {
        self.is_poly = poly;
    }

    pub fn set_wearing_dress(&mut self, dress: bool) {
        self.is_wearing_dress = dress;
    }

    pub fn set_mounting(&mut self, mount: bool) {
        self.is_mounting = mount;
    }

    pub fn get_motion_mode(&self) -> u8 {
        self.motion_mode
    }

    pub fn refresh_state(&mut self, item_mgr: &impl ItemDataManager) {
        let weapon_vnum = self.get_part_vnum(PART_WEAPON);

        let mut by_item_type = 0xFF;
        let mut by_sub_type = 0xFF;

        if weapon_vnum != 0 {
            if let Some(t) = item_mgr.get_item_type(weapon_vnum) {
                by_item_type = t;
            }
            if let Some(t) = item_mgr.get_weapon_type(weapon_vnum) {
                by_sub_type = t;
            }
        }

        if self.is_poly {
            self.motion_mode = MODE_GENERAL;
        } else if self.is_wearing_dress {
            self.motion_mode = MODE_WEDDING_DRESS;
        } else if by_item_type == ITEM_TYPE_ROD {
            if self.is_mounting {
                self.motion_mode = MODE_HORSE;
            } else {
                self.motion_mode = MODE_FISHING;
            }
        } else if self.is_mounting {
            self.motion_mode = match by_sub_type {
                WEAPON_SWORD => MODE_HORSE_ONEHAND_SWORD,
                WEAPON_TWO_HANDED => MODE_HORSE_TWOHAND_SWORD,
                WEAPON_DAGGER => MODE_HORSE_DUALHAND_SWORD,
                WEAPON_FAN => MODE_HORSE_FAN,
                WEAPON_BELL => MODE_HORSE_BELL,
                WEAPON_BOW => MODE_HORSE_BOW,
                _ => MODE_HORSE,
            };
        } else {
            self.motion_mode = match by_sub_type {
                WEAPON_SWORD => MODE_ONEHAND_SWORD,
                WEAPON_TWO_HANDED => MODE_TWOHAND_SWORD,
                WEAPON_DAGGER => MODE_DUALHAND_SWORD,
                WEAPON_BOW => MODE_BOW,
                WEAPON_FAN => MODE_FAN,
                WEAPON_BELL => MODE_BELL,
                WEAPON_ARROW => MODE_GENERAL,
                _ => MODE_GENERAL,
            };
        }
    }

    pub fn reset_parts(&mut self) {
        for i in 0..PART_MAX_NUM as usize {
            self.parts[i] = 0;
        }
        self.weapon_vnum = 0;
        self.armor_vnum = 0;
        self.hair_vnum = 0;
        self.sash_vnum = 0;
    }
}

use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};
use std::panic::{catch_unwind, AssertUnwindSafe};

static ACTOR_APPEARANCES: OnceLock<RwLock<HashMap<u32, ActorAppearance>>> = OnceLock::new();

fn get_actor_appearances() -> &'static RwLock<HashMap<u32, ActorAppearance>> {
    ACTOR_APPEARANCES.get_or_init(|| RwLock::new(HashMap::new()))
}

#[no_mangle]
pub extern "C" fn m2_actor_attach_weapon(vid: u32, weapon_vnum: u32) -> bool {
    catch_unwind(AssertUnwindSafe(|| {
        let mut appearances = get_actor_appearances().write().unwrap();
        let app = appearances.entry(vid).or_insert_with(ActorAppearance::new);
        app.attach_weapon(weapon_vnum);
        true
    })).unwrap_or(false)
}

#[no_mangle]
pub extern "C" fn m2_actor_change_armor(vid: u32, armor_vnum: u32) -> bool {
    catch_unwind(AssertUnwindSafe(|| {
        let mut appearances = get_actor_appearances().write().unwrap();
        let app = appearances.entry(vid).or_insert_with(ActorAppearance::new);
        app.change_armor(armor_vnum);
        true
    })).unwrap_or(false)
}

#[no_mangle]
pub extern "C" fn m2_actor_get_weapon(vid: u32) -> u32 {
    catch_unwind(AssertUnwindSafe(|| {
        let appearances = get_actor_appearances().read().unwrap();
        if let Some(app) = appearances.get(&vid) {
            app.weapon_vnum
        } else {
            0
        }
    })).unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn m2_actor_get_armor(vid: u32) -> u32 {
    catch_unwind(AssertUnwindSafe(|| {
        let appearances = get_actor_appearances().read().unwrap();
        if let Some(app) = appearances.get(&vid) {
            app.armor_vnum
        } else {
            0
        }
    })).unwrap_or(0)
}
