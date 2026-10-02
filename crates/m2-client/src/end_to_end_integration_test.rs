use std::ffi::c_char;

// Definitions from src/UserInterface/Packet.h
pub const CHARACTER_NAME_MAX_LEN: usize = 64;
pub const ID_MAX_NUM: usize = 30;
pub const PASS_MAX_NUM: usize = 16;
pub const PLAYER_PER_ACCOUNT4: usize = 4;
pub const GUILD_NAME_MAX_LEN: usize = 12;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TSimplePlayerInformation {
    pub dwID: u32,
    pub szName: [u8; CHARACTER_NAME_MAX_LEN + 1],
    pub byJob: u8,
    pub byLevel: u8,
    pub dwPlayMinutes: u32,
    pub byST: u8,
    pub byHT: u8,
    pub byDX: u8,
    pub byIQ: u8,
    pub wMainPart: u16,
    pub byHairPart: u8,
    pub x: i32,
    pub y: i32,
    pub lAddr: i32,
    pub wPort: u16,
    pub bySkillGroup: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCLoginSuccess4 {
    pub header: u16,
    pub length: u16,
    pub akSimplePlayerInformation: [TSimplePlayerInformation; PLAYER_PER_ACCOUNT4],
    pub guild_id: [u32; PLAYER_PER_ACCOUNT4],
    pub guild_name: [[u8; GUILD_NAME_MAX_LEN + 1]; PLAYER_PER_ACCOUNT4],
    pub handle: u32,
    pub random_key: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketCGEnterFrontGame {
    pub header: u16,
    pub length: u16,
}

pub const MUSIC_NAME_MAX_LEN: usize = 24;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCMainCharacter {
    pub header: u16,
    pub length: u16,
    pub dwVID: u32,
    pub wRaceNum: u16,
    pub szName: [u8; CHARACTER_NAME_MAX_LEN + 1],
    pub szBGMName: [u8; MUSIC_NAME_MAX_LEN + 1],
    pub fBGMVol: f32,
    pub lX: i32,
    pub lY: i32,
    pub lZ: i32,
    pub byEmpire: u8,
    pub bySkillGroup: u8,
}

pub const CHR_EQUIPPART_NUM: usize = 4;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCCharacterAdd {
    pub header: u16,
    pub length: u16,
    pub dwVID: u32,
    pub angle: f32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub bType: u8,
    pub wRaceNum: u16,
    pub bMovingSpeed: u8,
    pub bAttackSpeed: u8,
    pub bStateFlag: u8,
    pub dwAffectFlag: [u32; 2],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCMove {
    pub header: u16,
    pub length: u16,
    pub bFunc: u8,
    pub bArg: u8,
    pub bRot: u8,
    pub dwVID: u32,
    pub lX: i32,
    pub lY: i32,
    pub dwTime: u32,
    pub dwDuration: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCDamageInfo {
    pub header: u16,
    pub length: u16,
    pub dwVID: u32,
    pub flag: u8,
    pub damage: i32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TItemPos {
    pub window_type: u8,
    pub cell: u16,
}

pub const ITEM_SOCKET_SLOT_MAX_NUM: usize = 3; // Wlasciwy rozmiar z GameType.h (3 wg zrodel)
pub const ITEM_ATTRIBUTE_SLOT_MAX_NUM: usize = 7;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPlayerItemAttribute {
    pub bType: u8,
    pub sValue: i16,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCItemSet {
    pub header: u16,
    pub length: u16,
    pub pos: TItemPos,
    pub vnum: u32,
    pub count: u8,
    pub flags: u32,
    pub anti_flags: u32,
    pub highlight: u8,
    pub alSockets: [i32; ITEM_SOCKET_SLOT_MAX_NUM],
    pub aAttr: [TPlayerItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM],
}

// Ensure default implementations for creating blank test packets
impl Default for TPacketGCLoginSuccess4 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl Default for TPacketGCMainCharacter {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl Default for TPacketGCCharacterAdd {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl Default for TPacketGCMove {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl Default for TPacketGCDamageInfo {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl Default for TPacketGCItemSet {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

pub fn dummy_module_function() -> bool {
    true
}
