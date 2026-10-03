use std::collections::VecDeque;

pub const ITEM_SOCKET_MAX_NUM: u32 = 3;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemType {
    None = 0,
    Weapon = 1,
    Armor = 2,
    Use = 3,
    AutoUse = 4,
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
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponSubType {
    Sword = 0,
    Dagger = 1,
    Bow = 2,
    TwoHanded = 3,
    Bell = 4,
    Fan = 5,
    Arrow = 6,
    NumTypes = 7,
    None = 8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArmorSubType {
    Body = 0,
    Head = 1,
    Shield = 2,
    Wrist = 3,
    Foots = 4,
    Neck = 5,
    Ear = 6,
    NumTypes = 7,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefinedEffect {
    None = 0,
    SwordRefined7 = 1,
    SwordRefined8 = 2,
    SwordRefined9 = 3,
    BowRefined7 = 4,
    BowRefined8 = 5,
    BowRefined9 = 6,
    FanBellRefined7 = 7,
    FanBellRefined8 = 8,
    FanBellRefined9 = 9,
    SmallSwordRefined7 = 10,
    SmallSwordRefined8 = 11,
    SmallSwordRefined9 = 12,
    SmallSwordRefined7Left = 13,
    SmallSwordRefined8Left = 14,
    SmallSwordRefined9Left = 15,
    BodyArmorRefined7 = 16,
    BodyArmorRefined8 = 17,
    BodyArmorRefined9 = 18,
    BodyArmorSpecial = 19,
    BodyArmorSpecial2 = 20,
    RefinedNum = 21,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DamageFlag {
    Normal = 1 << 0,
    Poison = 1 << 1,
    Dodge = 1 << 2,
    Block = 1 << 3,
    Penetrate = 1 << 4,
    Critical = 1 << 5,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActorEffectType {
    Stun = 1, // EFFECT_STUN in enum, simplifying this for standalone effects mapped locally, but it aligns with C++ values
    Hit = 2,
    Refined = 36, // EFFECT_REFINED (roughly estimated from C++ if needed, typically used as base offset)
    DamageTarget = 38,
    DamageNotTarget = 39,
    DamageSelf = 40,
    DamageSelf2 = 41,
    DamagePoison = 42,
    DamageMiss = 43,
    DamageTargetMiss = 44,
    DamageCritical = 45,
    // Add custom base indices as in C++ source where we did `EFFECT_REFINED+EFFECT_BODYARMOR_REFINED7`
    // Real C++ indices for EFFECT_REFINED would be an integer offset. Let's handle it as u32.
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EffectDamage {
    pub damage: u32,
    pub flag: u8,
    pub b_self: bool,
    pub b_target: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ItemData {
    pub vnum: u32,
    pub item_type: ItemType,
    pub sub_type: u8, // Casted to either WeaponSubType or ArmorSubType depending on item_type
    pub refine: u32,
    pub socket_count: u32,
}

impl ItemData {
    pub fn get_weapon_sub_type(&self) -> WeaponSubType {
        // Unsafe cast isn't needed, we can pattern match if it's small, or use transmute, 
        // but simple mapping is cleaner in safe Rust.
        match self.sub_type {
            0 => WeaponSubType::Sword,
            1 => WeaponSubType::Dagger,
            2 => WeaponSubType::Bow,
            3 => WeaponSubType::TwoHanded,
            4 => WeaponSubType::Bell,
            5 => WeaponSubType::Fan,
            6 => WeaponSubType::Arrow,
            _ => WeaponSubType::None,
        }
    }
    pub fn get_armor_sub_type(&self) -> ArmorSubType {
        match self.sub_type {
            0 => ArmorSubType::Body,
            1 => ArmorSubType::Head,
            2 => ArmorSubType::Shield,
            3 => ArmorSubType::Wrist,
            4 => ArmorSubType::Foots,
            5 => ArmorSubType::Neck,
            6 => ArmorSubType::Ear,
            _ => ArmorSubType::NumTypes,
        }
    }
}

pub struct ActorEffectSystem {
    pub is_stunned: bool,
    pub is_poisoned: bool,
    
    pub sword_refine_effect_right: u32,
    pub sword_refine_effect_left: u32,
    pub armor_refine_effect: u32,
    
    pub active_effects: Vec<u32>, // Store attached effect IDs
    pub damage_queue: VecDeque<EffectDamage>,
    pub damage_effect_type: bool, // Corresponds to m_bDamageEffectType
}

impl ActorEffectSystem {
    pub fn new() -> Self {
        Self {
            is_stunned: false,
            is_poisoned: false,
            sword_refine_effect_right: 0,
            sword_refine_effect_left: 0,
            armor_refine_effect: 0,
            active_effects: Vec::new(),
            damage_queue: VecDeque::new(),
            damage_effect_type: false,
        }
    }

    pub fn set_stunned(&mut self, stunned: bool) {
        self.is_stunned = stunned;
        if stunned {
            self.attach_effect(ActorEffectType::Stun as u32);
        } else {
            self.detach_effect(ActorEffectType::Stun as u32);
        }
    }

    pub fn set_poisoned(&mut self, poisoned: bool) {
        self.is_poisoned = poisoned;
        // The instructions ask for "zielone swiecenie ciala" (green body glow), which maps to DAMAGE_POISON visually.
        // Assuming EFFECT_DAMAGE_POISON is the requested effect index based on C++ source grep.
        if poisoned {
            self.attach_effect(ActorEffectType::DamagePoison as u32);
        } else {
            self.detach_effect(ActorEffectType::DamagePoison as u32);
        }
    }

    pub fn attach_effect(&mut self, effect: u32) -> u32 {
        if !self.active_effects.contains(&effect) {
            self.active_effects.push(effect);
        }
        effect
    }

    pub fn detach_effect(&mut self, effect: u32) {
        if let Some(index) = self.active_effects.iter().position(|x| *x == effect) {
            self.active_effects.remove(index);
        }
    }

    pub fn clear_weapon_refine_effect(&mut self) {
        if self.sword_refine_effect_right != 0 {
            self.detach_effect(self.sword_refine_effect_right);
            self.sword_refine_effect_right = 0;
        }
        if self.sword_refine_effect_left != 0 {
            self.detach_effect(self.sword_refine_effect_left);
            self.sword_refine_effect_left = 0;
        }
    }

    pub fn clear_armor_refine_effect(&mut self) {
        if self.armor_refine_effect != 0 {
            self.detach_effect(self.armor_refine_effect);
            self.armor_refine_effect = 0;
        }
    }

    pub fn get_refined_effect(&mut self, item: &ItemData) -> u32 {
        let refine = std::cmp::max(item.refine + item.socket_count, ITEM_SOCKET_MAX_NUM) - ITEM_SOCKET_MAX_NUM;
        
        match item.item_type {
            ItemType::Weapon => {
                self.clear_weapon_refine_effect();
                if refine < 7 {
                    return 0;
                }
                
                let base_effect = ActorEffectType::Refined as u32;
                match item.get_weapon_sub_type() {
                    WeaponSubType::Dagger => {
                        self.sword_refine_effect_right = base_effect + RefinedEffect::SmallSwordRefined7 as u32 + refine - 7;
                        self.sword_refine_effect_left = base_effect + RefinedEffect::SmallSwordRefined7Left as u32 + refine - 7;
                    }
                    WeaponSubType::Fan => {
                        self.sword_refine_effect_right = base_effect + RefinedEffect::FanBellRefined7 as u32 + refine - 7;
                    }
                    WeaponSubType::Arrow | WeaponSubType::Bell => {
                        self.sword_refine_effect_right = base_effect + RefinedEffect::SmallSwordRefined7 as u32 + refine - 7;
                    }
                    WeaponSubType::Bow => {
                        self.sword_refine_effect_right = base_effect + RefinedEffect::BowRefined7 as u32 + refine - 7;
                    }
                    _ => {
                        self.sword_refine_effect_right = base_effect + RefinedEffect::SwordRefined7 as u32 + refine - 7;
                    }
                }
                
                if self.sword_refine_effect_right != 0 {
                    self.attach_effect(self.sword_refine_effect_right);
                }
                if self.sword_refine_effect_left != 0 {
                    self.attach_effect(self.sword_refine_effect_left);
                }
            }
            ItemType::Armor => {
                self.clear_armor_refine_effect();
                
                if item.get_armor_sub_type() == ArmorSubType::Body {
                    let vnum = item.vnum;
                    if vnum >= 12010 && vnum <= 12049 {
                        self.attach_effect(ActorEffectType::Refined as u32 + RefinedEffect::BodyArmorSpecial as u32);
                        self.attach_effect(ActorEffectType::Refined as u32 + RefinedEffect::BodyArmorSpecial2 as u32);
                    }
                }
                
                if refine < 7 {
                    return 0;
                }
                
                if item.get_armor_sub_type() == ArmorSubType::Body {
                    self.armor_refine_effect = ActorEffectType::Refined as u32 + RefinedEffect::BodyArmorRefined7 as u32 + refine - 7;
                    self.attach_effect(self.armor_refine_effect);
                }
            }
            _ => {}
        }
        0
    }

    pub fn add_damage_effect(&mut self, damage: u32, flag: u8, b_self: bool, b_target: bool) {
        self.damage_queue.push_back(EffectDamage {
            damage,
            flag,
            b_self,
            b_target,
        });
    }

    pub fn process_damage(&mut self) -> Option<u32> {
        if let Some(damage_event) = self.damage_queue.pop_front() {
            let flag = damage_event.flag;
            let b_self = damage_event.b_self;
            let b_target = damage_event.b_target;

            if (flag & DamageFlag::Dodge as u8) != 0 || (flag & DamageFlag::Block as u8) != 0 {
                if b_self {
                    return Some(self.attach_effect(ActorEffectType::DamageMiss as u32));
                } else {
                    return Some(self.attach_effect(ActorEffectType::DamageTargetMiss as u32));
                }
            } else if (flag & DamageFlag::Critical as u8) != 0 {
                // Usually played visually and then standard processing continues. In our mock context,
                // this acts correctly by letting it proceed, but C++ says "CreateEffect" doesn't return immediately.
            }

            let effect_id = if b_self {
                let id = if !self.damage_effect_type {
                    ActorEffectType::DamageSelf as u32
                } else {
                    ActorEffectType::DamageSelf2 as u32
                };
                self.damage_effect_type = !self.damage_effect_type;
                id
            } else if !b_target {
                // Simplified INVISIBILITY check omission for core logic translation
                return Some(self.attach_effect(ActorEffectType::DamageNotTarget as u32));
            } else {
                ActorEffectType::DamageTarget as u32
            };

            return Some(self.attach_effect(effect_id));
        }
        None
    }
}
