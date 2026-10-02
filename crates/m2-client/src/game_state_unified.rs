use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};

pub const ITEM_SOCKET_SLOT_MAX_NUM: usize = 3;
pub const ITEM_ATTRIBUTE_SLOT_MAX_NUM: usize = 7;
pub const POINT_MAX_NUM: usize = 255;
pub const QUICKSLOT_MAX_NUM: usize = 36;
pub const SKILL_MAX_NUM: usize = 255;
pub const PARTY_AFFECT_SLOT_MAX_NUM: usize = 7;
pub const GUILD_NAME_MAX_LEN: usize = 12;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TSkillInstance {
    pub dwIndex: u32,
    pub iType: i32,
    pub iGrade: i32,
    pub iLevel: i32,
    pub fcurEfficientPercentage: f32,
    pub fnextEfficientPercentage: f32,
    pub isCoolTime: bool, // BOOL is basically i32/bool
    pub fCoolTime: f32,
    pub fLastUsedTime: f32,
    pub bActive: bool,
}

impl Default for TSkillInstance {
    fn default() -> Self {
        Self {
            dwIndex: 0,
            iType: 0,
            iGrade: 0,
            iLevel: 0,
            fcurEfficientPercentage: 0.0,
            fnextEfficientPercentage: 0.0,
            isCoolTime: false,
            fCoolTime: 0.0,
            fLastUsedTime: 0.0,
            bActive: false,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TQuickSlot {
    pub Type: u8,
    pub Position: u8,
}

impl Default for TQuickSlot {
    fn default() -> Self {
        Self { Type: 0, Position: 0 }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TPlayerItemAttribute {
    pub bType: u8,
    pub sValue: i16,
}

impl Default for TPlayerItemAttribute {
    fn default() -> Self {
        Self { bType: 0, sValue: 0 }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TItemData {
    pub vnum: u32,
    pub count: u8,
    pub flags: u32,
    pub anti_flags: u32,
    pub alSockets: [i32; ITEM_SOCKET_SLOT_MAX_NUM],
    pub aAttr: [TPlayerItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM],
}

impl Default for TItemData {
    fn default() -> Self {
        Self {
            vnum: 0,
            count: 0,
            flags: 0,
            anti_flags: 0,
            alSockets: [0; ITEM_SOCKET_SLOT_MAX_NUM],
            aAttr: [TPlayerItemAttribute::default(); ITEM_ATTRIBUTE_SLOT_MAX_NUM],
        }
    }
}

#[derive(Debug, Clone)]
pub struct TPartyMemberInfo {
    pub dwVID: u32,
    pub dwPID: u32,
    pub strName: String,
    pub byState: u8,
    pub byHPPercentage: u8,
    pub sAffects: [i16; PARTY_AFFECT_SLOT_MAX_NUM],
}

#[derive(Debug, Clone)]
pub struct SGroundItemInstance {
    pub dwVirtualNumber: u32,
    pub v3EndPosition: (f32, f32, f32),
    pub v3RotationAxis: (f32, f32, f32),
    pub qEnd: (f32, f32, f32, f32),
    pub v3Center: (f32, f32, f32),
    // CGraphicThingInstance equivalent omitted / simplified
    pub dwStartTime: u32,
    pub dwEndTime: u32,
    pub eDropSoundType: u32,
    pub bAnimEnded: bool,
    pub dwEffectInstanceIndex: u32,
    pub stOwnership: String,
}

#[derive(Debug, Clone)]
pub struct TGuildInfo {
    pub dwGuildID: u32,
    pub szGuildName: String,
    pub dwMasterPID: u32,
    pub dwGuildLevel: u32,
    pub dwCurrentExperience: u32,
    pub dwCurrentMemberCount: u32,
    pub dwMaxMemberCount: u32,
    pub dwGuildMoney: u32,
    pub bHasLand: bool, // BOOL
}

#[derive(Debug, Clone)]
pub struct SPlayerStatus {
    pub aQuickSlot: [TQuickSlot; QUICKSLOT_MAX_NUM],
    pub aSkill: [TSkillInstance; SKILL_MAX_NUM],
    pub m_alPoint: [i32; POINT_MAX_NUM],
    pub lQuickPageIndex: i32,
}

impl Default for SPlayerStatus {
    fn default() -> Self {
        Self {
            aQuickSlot: [TQuickSlot::default(); QUICKSLOT_MAX_NUM],
            aSkill: [TSkillInstance::default(); SKILL_MAX_NUM],
            m_alPoint: [0; POINT_MAX_NUM],
            lQuickPageIndex: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerInfo {
    pub status: SPlayerStatus,
    pub vid: u32,
    pub hp: i32,
    pub sp: i32,
    pub st: i32,
    pub dx: i32,
    pub ht: i32,
    pub iq: i32,
    pub gold: i64,
    pub exp: u32,
    pub level: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rotation: f32,
    pub empire: u8,
    pub skill_group: u8,
}

impl Default for PlayerInfo {
    fn default() -> Self {
        Self {
            status: SPlayerStatus::default(),
            vid: 0,
            hp: 0,
            sp: 0,
            st: 0,
            dx: 0,
            ht: 0,
            iq: 0,
            gold: 0,
            exp: 0,
            level: 0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            rotation: 0.0,
            empire: 0,
            skill_group: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EquipmentSlot {
    Helm = 0,
    Armor,
    Shield,
    Weapon,
    Shoes,
    Bracelet,
    Necklace,
    Earrings,
    Belt,
    Sash, // custom extension maybe (szarfa)
    Talisman, // custom extension maybe (talizman)
}

#[derive(Debug, Clone)]
pub struct Inventory {
    // 180 slots (4 strony po 45 slotow)
    pub items: HashMap<usize, TItemData>,
    pub equipment: HashMap<EquipmentSlot, TItemData>,
    // 2 decks of 6 slots
    pub dragon_soul: HashMap<usize, TItemData>,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            items: HashMap::new(),
            equipment: HashMap::new(),
            dragon_soul: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ActorInstanceData {
    pub vid: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rotation: f32,
    pub name: String,
    // Add other relevant fields if needed
}

#[derive(Debug, Clone)]
pub struct TargetInfo {
    pub vid: u32,
    pub hp_percentage: u8,
}

#[derive(Debug, Clone, Default)]
pub struct ChatHistory {
    pub messages: VecDeque<String>,
}

impl ChatHistory {
    pub fn new() -> Self {
        Self {
            messages: VecDeque::with_capacity(500),
        }
    }

    pub fn push(&mut self, message: String) {
        if self.messages.len() >= 500 {
            self.messages.pop_front();
        }
        self.messages.push_back(message);
    }
}

pub struct UnifiedGameState {
    pub player: RwLock<PlayerInfo>,
    pub inventory: RwLock<Inventory>,
    pub actors: RwLock<HashMap<u32, ActorInstanceData>>,
    pub ground_items: RwLock<Vec<SGroundItemInstance>>,
    pub party: RwLock<Option<Vec<TPartyMemberInfo>>>,
    pub guild: RwLock<Option<TGuildInfo>>,
    pub target: RwLock<Option<TargetInfo>>,
    pub chat_history: RwLock<ChatHistory>,
}

impl UnifiedGameState {
    pub fn new() -> Self {
        Self {
            player: RwLock::new(PlayerInfo::default()),
            inventory: RwLock::new(Inventory::default()),
            actors: RwLock::new(HashMap::new()),
            ground_items: RwLock::new(Vec::new()),
            party: RwLock::new(None),
            guild: RwLock::new(None),
            target: RwLock::new(None),
            chat_history: RwLock::new(ChatHistory::new()),
        }
    }

    pub fn get_player_stat(&self, stat_index: usize) -> Option<i32> {
        let player = self.player.read().unwrap();
        if stat_index < POINT_MAX_NUM {
            Some(player.status.m_alPoint[stat_index])
        } else {
            None
        }
    }

    pub fn set_player_stat(&self, stat_index: usize, value: i32) {
        let mut player = self.player.write().unwrap();
        if stat_index < POINT_MAX_NUM {
            player.status.m_alPoint[stat_index] = value;
        }
    }

    pub fn set_item(&self, slot: usize, item: TItemData) {
        let mut inventory = self.inventory.write().unwrap();
        if slot < 180 {
            inventory.items.insert(slot, item);
        }
    }

    pub fn remove_item(&self, slot: usize) -> Option<TItemData> {
        let mut inventory = self.inventory.write().unwrap();
        inventory.items.remove(&slot)
    }

    pub fn add_actor(&self, vid: u32, actor_data: ActorInstanceData) {
        let mut actors = self.actors.write().unwrap();
        actors.insert(vid, actor_data);
    }

    pub fn remove_actor(&self, vid: u32) -> Option<ActorInstanceData> {
        let mut actors = self.actors.write().unwrap();
        actors.remove(&vid)
    }
}

impl Default for UnifiedGameState {
    fn default() -> Self {
        Self::new()
    }
}
