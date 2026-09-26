use std::collections::HashMap;

pub const ITEM_ATTRIBUTE_SLOT_MAX_NUM: usize = 7;

pub const APPLY_NONE: u8 = 0;
pub const APPLY_MAX_HP: u8 = 1;
pub const APPLY_MAX_SP: u8 = 2;
pub const APPLY_CON: u8 = 3;
pub const APPLY_INT: u8 = 4;
pub const APPLY_STR: u8 = 5;
pub const APPLY_DEX: u8 = 6;
pub const APPLY_ATT_SPEED: u8 = 7;
pub const APPLY_MOV_SPEED: u8 = 8;
pub const APPLY_CAST_SPEED: u8 = 9;
pub const APPLY_HP_REGEN: u8 = 10;
pub const APPLY_SP_REGEN: u8 = 11;
pub const APPLY_POISON_PCT: u8 = 12;
pub const APPLY_STUN_PCT: u8 = 13;
pub const APPLY_SLOW_PCT: u8 = 14;
pub const APPLY_CRITICAL_PCT: u8 = 15;
pub const APPLY_PENETRATE_PCT: u8 = 16;
pub const APPLY_ATTBONUS_HUMAN: u8 = 17;
pub const APPLY_ATTBONUS_ANIMAL: u8 = 18;
pub const APPLY_ATTBONUS_ORC: u8 = 19;
pub const APPLY_ATTBONUS_MILGYO: u8 = 20;
pub const APPLY_ATTBONUS_UNDEAD: u8 = 21;
pub const APPLY_ATTBONUS_DEVIL: u8 = 22;
pub const APPLY_STEAL_HP: u8 = 23;
pub const APPLY_STEAL_SP: u8 = 24;
pub const APPLY_MANA_BURN_PCT: u8 = 25;
pub const APPLY_DAMAGE_SP_RECOVER: u8 = 26;
pub const APPLY_BLOCK: u8 = 27;
pub const APPLY_DODGE: u8 = 28;
pub const APPLY_RESIST_SWORD: u8 = 29;
pub const APPLY_RESIST_TWOHAND: u8 = 30;
pub const APPLY_RESIST_DAGGER: u8 = 31;
pub const APPLY_RESIST_BELL: u8 = 32;
pub const APPLY_RESIST_FAN: u8 = 33;
pub const APPLY_RESIST_BOW: u8 = 34;
pub const APPLY_RESIST_FIRE: u8 = 35;
pub const APPLY_RESIST_ELEC: u8 = 36;
pub const APPLY_RESIST_MAGIC: u8 = 37;
pub const APPLY_RESIST_WIND: u8 = 38;
pub const APPLY_REFLECT_MELEE: u8 = 39;
pub const APPLY_REFLECT_CURSE: u8 = 40;
pub const APPLY_POISON_REDUCE: u8 = 41;
pub const APPLY_KILL_SP_RECOVER: u8 = 42;
pub const APPLY_EXP_DOUBLE_BONUS: u8 = 43;
pub const APPLY_GOLD_DOUBLE_BONUS: u8 = 44;
pub const APPLY_ITEM_DROP_BONUS: u8 = 45;
pub const APPLY_POTION_BONUS: u8 = 46;
pub const APPLY_KILL_HP_RECOVER: u8 = 47;
pub const APPLY_IMMUNE_STUN: u8 = 48;
pub const APPLY_IMMUNE_SLOW: u8 = 49;
pub const APPLY_IMMUNE_FALL: u8 = 50;
pub const APPLY_SKILL: u8 = 51;
pub const APPLY_BOW_DISTANCE: u8 = 52;
pub const APPLY_ATT_GRADE_BONUS: u8 = 53;
pub const APPLY_DEF_GRADE_BONUS: u8 = 54;
pub const APPLY_MAGIC_ATT_GRADE: u8 = 55;
pub const APPLY_MAGIC_DEF_GRADE: u8 = 56;
pub const APPLY_CURSE_PCT: u8 = 57;
pub const APPLY_MAX_STAMINA: u8 = 58;
pub const APPLY_ATT_BONUS_TO_WARRIOR: u8 = 59;
pub const APPLY_ATT_BONUS_TO_ASSASSIN: u8 = 60;
pub const APPLY_ATT_BONUS_TO_SURA: u8 = 61;
pub const APPLY_ATT_BONUS_TO_SHAMAN: u8 = 62;
pub const APPLY_ATT_BONUS_TO_MONSTER: u8 = 63;
pub const APPLY_MALL_ATTBONUS: u8 = 64;
pub const APPLY_MALL_DEFBONUS: u8 = 65;
pub const APPLY_MALL_EXPBONUS: u8 = 66;
pub const APPLY_MALL_ITEMBONUS: u8 = 67;
pub const APPLY_MALL_GOLDBONUS: u8 = 68;
pub const APPLY_MAX_HP_PCT: u8 = 69;
pub const APPLY_MAX_SP_PCT: u8 = 70;
pub const APPLY_SKILL_DAMAGE_BONUS: u8 = 71;
pub const APPLY_NORMAL_HIT_DAMAGE_BONUS: u8 = 72;
pub const APPLY_SKILL_DEFEND_BONUS: u8 = 73;
pub const APPLY_NORMAL_HIT_DEFEND_BONUS: u8 = 74;
pub const APPLY_EXTRACT_HP_PCT: u8 = 75;
pub const APPLY_PC_BANG_EXP_BONUS: u8 = 76;
pub const APPLY_PC_BANG_DROP_BONUS: u8 = 77;
pub const APPLY_RESIST_WARRIOR: u8 = 78;
pub const APPLY_RESIST_ASSASSIN: u8 = 79;
pub const APPLY_RESIST_SURA: u8 = 80;
pub const APPLY_RESIST_SHAMAN: u8 = 81;
pub const APPLY_ENERGY: u8 = 82;
pub const APPLY_DEF_GRADE: u8 = 83;
pub const APPLY_COSTUME_ATTR_BONUS: u8 = 84;
pub const APPLY_MAGIC_ATTBONUS_PER: u8 = 85;
pub const APPLY_MELEE_MAGIC_ATTBONUS_PER: u8 = 86;
pub const APPLY_RESIST_ICE: u8 = 87;
pub const APPLY_RESIST_EARTH: u8 = 88;
pub const APPLY_RESIST_DARK: u8 = 89;
pub const APPLY_ANTI_CRITICAL_PCT: u8 = 90;
pub const APPLY_ANTI_PENETRATE_PCT: u8 = 91;


#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ItemAttribute {
    pub attr_type: u8,
    pub value: i16,
}

#[derive(Clone, Debug)]
pub struct ItemInstance {
    attributes: [ItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM],
}

impl ItemInstance {
    pub fn new() -> Self {
        Self {
            attributes: [ItemAttribute::default(); ITEM_ATTRIBUTE_SLOT_MAX_NUM],
        }
    }

    pub fn set_attribute(&mut self, index: usize, attr_type: u8, value: i16) -> bool {
        if index >= ITEM_ATTRIBUTE_SLOT_MAX_NUM {
            return false;
        }
        self.attributes[index] = ItemAttribute { attr_type, value };
        true
    }

    pub fn get_attribute(&self, index: usize) -> Option<ItemAttribute> {
        if index >= ITEM_ATTRIBUTE_SLOT_MAX_NUM {
            return None;
        }
        Some(self.attributes[index])
    }
}

pub fn calculate_total_attributes(items: &[&ItemInstance]) -> HashMap<u8, i32> {
    let mut totals = HashMap::new();

    for item in items {
        for i in 0..ITEM_ATTRIBUTE_SLOT_MAX_NUM {
            if let Some(attr) = item.get_attribute(i) {
                if attr.attr_type != APPLY_NONE {
                    *totals.entry(attr.attr_type).or_insert(0) += attr.value as i32;
                }
            }
        }
    }

    totals
}
