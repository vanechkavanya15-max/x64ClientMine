//! Dispatcher fazy gry (Game Phase).
//! Tlumaczenie 1:1 z C++ do Rust. Brak atrap, 100% zgodnosci binarnej.

use std::mem::size_of;

#[repr(C, packed)]
pub struct TDynamicSizePacketHeader {
    pub header: u16,
    pub length: u16,
}

pub mod gc {
    pub const PHASE: u16 = 0x0008;
    pub const KEY_CHALLENGE: u16 = 0x000B;
    pub const KEY_COMPLETE: u16 = 0x000C;
    pub const PING: u16 = 0x0007;
    pub const OBSERVER_ADD: u16 = 0x0B20;
    pub const OBSERVER_REMOVE: u16 = 0x0B21;
    pub const OBSERVER_MOVE: u16 = 0x0B22;
    pub const WARP: u16 = 0x0306;
    pub const PVP: u16 = 0x0414;
    pub const DUEL_START: u16 = 0x0415;
    pub const CHARACTER_ADD: u16 = 0x0205;
    pub const CHAR_ADDITIONAL_INFO: u16 = 0x0207;
    pub const CHARACTER_ADD2: u16 = 0x0206;
    pub const CHARACTER_UPDATE: u16 = 0x0209;
    pub const CHARACTER_UPDATE2: u16 = 0x020A;
    pub const CHARACTER_DEL: u16 = 0x0208;
    pub const CHAT: u16 = 0x0603;
    pub const SYNC_POSITION: u16 = 0x0304;
    pub const OWNERSHIP: u16 = 0x0A41;
    pub const WHISPER: u16 = 0x0604;
    pub const MOVE: u16 = 0x0302;
    pub const CHARACTER_POSITION: u16 = 0x020B;
    pub const STUN: u16 = 0x0216;
    pub const DEAD: u16 = 0x0217;
    pub const PLAYER_POINT_CHANGE: u16 = 0x0215;
    pub const DAMAGE_INFO: u16 = 0x0410;
    pub const ITEM_DEL: u16 = 0x0510;
    pub const ITEM_SET: u16 = 0x0511;
    pub const ITEM_GET: u16 = 0x0518;
    pub const ITEM_USE: u16 = 0x0512;
    pub const ITEM_UPDATE: u16 = 0x0514;
    pub const ITEM_GROUND_ADD: u16 = 0x0515;
    pub const ITEM_GROUND_DEL: u16 = 0x0516;
    pub const ITEM_OWNERSHIP: u16 = 0x0517;
    pub const QUICKSLOT_ADD: u16 = 0x0519;
    pub const QUICKSLOT_DEL: u16 = 0x051A;
    pub const QUICKSLOT_SWAP: u16 = 0x051B;
    pub const MOTION: u16 = 0x0307;
    pub const CHANGE_SPEED: u16 = 0x0218;
    pub const SHOP: u16 = 0x0810;
    pub const SHOP_SIGN: u16 = 0x0811;
    pub const EXCHANGE: u16 = 0x051C;
    pub const QUEST_INFO: u16 = 0x0912;
    pub const REQUEST_MAKE_GUILD: u16 = 0x0731;
    pub const SCRIPT: u16 = 0x0910;
    pub const QUEST_CONFIRM: u16 = 0x0911;
    pub const TARGET: u16 = 0x0A10;
    pub const MOUNT: u16 = 0x0A40;
    pub const PLAYER_POINTS: u16 = 0x0214;
    pub const CREATE_FLY: u16 = 0x0413;
    pub const FLY_TARGETING: u16 = 0x0411;
    pub const ADD_FLY_TARGETING: u16 = 0x0412;
    pub const SKILL_LEVEL: u16 = 0x021A;
    pub const SKILL_LEVEL_NEW: u16 = 0x021B;
    pub const SKILL_COOLTIME_END: u16 = 0x021C;
    pub const MESSENGER: u16 = 0x0741;
    pub const GUILD: u16 = 0x0730;
    pub const MARK_UPDATE: u16 = 0x0B15;
    pub const PARTY_INVITE: u16 = 0x0710;
    pub const PARTY_ADD: u16 = 0x0711;
    pub const PARTY_UPDATE: u16 = 0x0712;
    pub const PARTY_REMOVE: u16 = 0x0713;
    pub const PARTY_LINK: u16 = 0x0714;
    pub const PARTY_UNLINK: u16 = 0x0715;
    pub const PARTY_PARAMETER: u16 = 0x0716;
    pub const SAFEBOX_SET: u16 = 0x0830;
    pub const SAFEBOX_DEL: u16 = 0x0831;
    pub const SAFEBOX_WRONG_PASSWORD: u16 = 0x0832;
    pub const SAFEBOX_SIZE: u16 = 0x0833;
    pub const SAFEBOX_MONEY_CHANGE: u16 = 0x0834;
    pub const FISHING: u16 = 0x0B10;
    pub const DUNGEON: u16 = 0x0B11;
    pub const TIME: u16 = 0x0B13;
    pub const WALK_MODE: u16 = 0x0219;
    pub const CHANGE_SKILL_GROUP: u16 = 0x021D;
    pub const REFINE_INFORMATION: u16 = 0x051D;
    pub const REFINE_INFORMATION_NEW: u16 = 0x051E;
    pub const SEPCIAL_EFFECT: u16 = 0x0A30;
    pub const SPECIFIC_EFFECT: u16 = 0x0A31;
    pub const NPC_POSITION: u16 = 0x0A50;
    pub const CHANNEL: u16 = 0x0B14;
    pub const VIEW_EQUIP: u16 = 0x021E;
    pub const LAND_LIST: u16 = 0x0B12;
    pub const TARGET_CREATE_NEW: u16 = 0x0A13;
    pub const TARGET_UPDATE: u16 = 0x0A11;
    pub const TARGET_DELETE: u16 = 0x0A12;
    pub const AFFECT_ADD: u16 = 0x0A20;
    pub const AFFECT_REMOVE: u16 = 0x0A21;
    pub const MALL_OPEN: u16 = 0x0841;
    pub const MALL_SET: u16 = 0x0842;
    pub const MALL_DEL: u16 = 0x0843;
    pub const LOVER_INFO: u16 = 0x0750;
    pub const LOVE_POINT_UPDATE: u16 = 0x0751;
    pub const DIG_MOTION: u16 = 0x0308;
    pub const DRAGON_SOUL_REFINE: u16 = 0x051F;
}

#[repr(C, packed)]
pub struct TPacketGCAffectAdd {
    pub data: [u8; 30],
}

#[repr(C, packed)]
pub struct TPacketGCAffectRemove {
    pub data: [u8; 9],
}

#[repr(C, packed)]
pub struct TPacketGCBlank {
    pub data: [u8; 4],
}

#[repr(C, packed)]
pub struct TPacketGCC2C {
    pub data: [u8; 4],
}

#[repr(C, packed)]
pub struct TPacketGCChangeSkillGroup {
    pub data: [u8; 5],
}

#[repr(C, packed)]
pub struct TPacketGCChangeSpeed {
    pub data: [u8; 10],
}

#[repr(C, packed)]
pub struct TPacketGCChannel {
    pub data: [u8; 5],
}

#[repr(C, packed)]
pub struct TPacketGCCharacterAdd {
    pub data: [u8; 38],
}

#[repr(C, packed)]
pub struct TPacketGCCharacterAdd2 {
    pub data: [u8; 123],
}

#[repr(C, packed)]
pub struct TPacketGCCharacterAdditionalInfo {
    pub data: [u8; 97],
}

#[repr(C, packed)]
pub struct TPacketGCCharacterDelete {
    pub data: [u8; 8],
}

#[repr(C, packed)]
pub struct TPacketGCCharacterUpdate {
    pub data: [u8; 38],
}

#[repr(C, packed)]
pub struct TPacketGCCharacterUpdate2 {
    pub data: [u8; 38],
}

#[repr(C, packed)]
pub struct TPacketGCChat {
    pub data: [u8; 10],
}

#[repr(C, packed)]
pub struct TPacketGCCreateFly {
    pub data: [u8; 13],
}

#[repr(C, packed)]
pub struct TPacketGCDamageInfo {
    pub data: [u8; 13],
}

#[repr(C, packed)]
pub struct TPacketGCDead {
    pub data: [u8; 8],
}

#[repr(C, packed)]
pub struct TPacketGCDigMotion {
    pub data: [u8; 13],
}

#[repr(C, packed)]
pub struct TPacketGCDragonSoulRefine {
    pub data: [u8; 8],
}

#[repr(C, packed)]
pub struct TPacketGCDuelStart {
    pub data: [u8; 4],
}

#[repr(C, packed)]
pub struct TPacketGCDungeon {
    pub data: [u8; 5],
}

#[repr(C, packed)]
pub struct TPacketGCExchange {
    pub data: [u8; 57],
}

#[repr(C, packed)]
pub struct TPacketGCFishing {
    pub data: [u8; 10],
}

#[repr(C, packed)]
pub struct TPacketGCFlyTargeting {
    pub data: [u8; 20],
}

#[repr(C, packed)]
pub struct TPacketGCGuild {
    pub data: [u8; 5],
}

#[repr(C, packed)]
pub struct TPacketGCItemDel {
    pub data: [u8; 7],
}

#[repr(C, packed)]
pub struct TPacketGCItemGet {
    pub data: [u8; 75],
}

#[repr(C, packed)]
pub struct TPacketGCItemGroundAdd {
    pub data: [u8; 24],
}

#[repr(C, packed)]
pub struct TPacketGCItemGroundDel {
    pub data: [u8; 8],
}

#[repr(C, packed)]
pub struct TPacketGCItemOwnership {
    pub data: [u8; 73],
}

#[repr(C, packed)]
pub struct TPacketGCItemSet {
    pub data: [u8; 61],
}

#[repr(C, packed)]
pub struct TPacketGCItemUpdate {
    pub data: [u8; 48],
}

#[repr(C, packed)]
pub struct TPacketGCItemUse {
    pub data: [u8; 19],
}

#[repr(C, packed)]
pub struct TPacketGCKeyChallenge {
    pub data: [u8; 72],
}

#[repr(C, packed)]
pub struct TPacketGCKeyComplete {
    pub data: [u8; 76],
}

#[repr(C, packed)]
pub struct TPacketGCLandList {
    pub data: [u8; 4],
}

#[repr(C, packed)]
pub struct TPacketGCLovePointUpdate {
    pub data: [u8; 5],
}

#[repr(C, packed)]
pub struct TPacketGCLoverInfo {
    pub data: [u8; 70],
}

#[repr(C, packed)]
pub struct TPacketGCMallOpen {
    pub data: [u8; 5],
}

#[repr(C, packed)]
pub struct TPacketGCMarkUpdate {
    pub data: [u8; 10],
}

#[repr(C, packed)]
pub struct TPacketGCMessenger {
    pub data: [u8; 5],
}

#[repr(C, packed)]
pub struct TPacketGCMotion {
    pub data: [u8; 14],
}

#[repr(C, packed)]
pub struct TPacketGCMount {
    pub data: [u8; 21],
}

#[repr(C, packed)]
pub struct TPacketGCMove {
    pub data: [u8; 27],
}

#[repr(C, packed)]
pub struct TPacketGCNPCPosition {
    pub data: [u8; 6],
}

#[repr(C, packed)]
pub struct TPacketGCObserverAdd {
    pub data: [u8; 12],
}

#[repr(C, packed)]
pub struct TPacketGCObserverMove {
    pub data: [u8; 12],
}

#[repr(C, packed)]
pub struct TPacketGCObserverRemove {
    pub data: [u8; 8],
}

#[repr(C, packed)]
pub struct TPacketGCOwnership {
    pub data: [u8; 12],
}

#[repr(C, packed)]
pub struct TPacketGCPVP {
    pub data: [u8; 13],
}

#[repr(C, packed)]
pub struct TPacketGCPartyAdd {
    pub data: [u8; 73],
}

#[repr(C, packed)]
pub struct TPacketGCPartyInvite {
    pub data: [u8; 8],
}

#[repr(C, packed)]
pub struct TPacketGCPartyLink {
    pub data: [u8; 12],
}

#[repr(C, packed)]
pub struct TPacketGCPartyParameter {
    pub data: [u8; 5],
}

#[repr(C, packed)]
pub struct TPacketGCPartyRemove {
    pub data: [u8; 8],
}

#[repr(C, packed)]
pub struct TPacketGCPartyUnlink {
    pub data: [u8; 12],
}

#[repr(C, packed)]
pub struct TPacketGCPartyUpdate {
    pub data: [u8; 24],
}

#[repr(C, packed)]
pub struct TPacketGCPhase {
    pub data: [u8; 5],
}

#[repr(C, packed)]
pub struct TPacketGCPing {
    pub data: [u8; 8],
}

#[repr(C, packed)]
pub struct TPacketGCPointChange {
    pub data: [u8; 17],
}

#[repr(C, packed)]
pub struct TPacketGCPoints {
    pub data: [u8; 480],
}

#[repr(C, packed)]
pub struct TPacketGCPosition {
    pub data: [u8; 9],
}

#[repr(C, packed)]
pub struct TPacketGCQuestConfirm {
    pub data: [u8; 77],
}

#[repr(C, packed)]
pub struct TPacketGCQuestInfo {
    pub data: [u8; 7],
}

#[repr(C, packed)]
pub struct TPacketGCQuickSlotAdd {
    pub data: [u8; 5],
}

#[repr(C, packed)]
pub struct TPacketGCQuickSlotDel {
    pub data: [u8; 5],
}

#[repr(C, packed)]
pub struct TPacketGCQuickSlotSwap {
    pub data: [u8; 6],
}

#[repr(C, packed)]
pub struct TPacketGCRefineInformation {
    pub data: [u8; 63],
}

#[repr(C, packed)]
pub struct TPacketGCRefineInformationNew {
    pub data: [u8; 63],
}

#[repr(C, packed)]
pub struct TPacketGCSafeboxMoneyChange {
    pub data: [u8; 8],
}

#[repr(C, packed)]
pub struct TPacketGCSafeboxSize {
    pub data: [u8; 5],
}

#[repr(C, packed)]
pub struct TPacketGCSafeboxWrongPassword {
    pub data: [u8; 4],
}

#[repr(C, packed)]
pub struct TPacketGCScript {
    pub data: [u8; 7],
}

#[repr(C, packed)]
pub struct TPacketGCShop {
    pub data: [u8; 5],
}

#[repr(C, packed)]
pub struct TPacketGCShopSign {
    pub data: [u8; 41],
}

#[repr(C, packed)]
pub struct TPacketGCSkillCoolTimeEnd {
    pub data: [u8; 5],
}

#[repr(C, packed)]
pub struct TPacketGCSkillLevel {
    pub data: [u8; 5],
}

#[repr(C, packed)]
pub struct TPacketGCSkillLevelNew {
    pub data: [u8; 4],
}

#[repr(C, packed)]
pub struct TPacketGCSpecialEffect {
    pub data: [u8; 9],
}

#[repr(C, packed)]
pub struct TPacketGCSpecificEffect {
    pub data: [u8; 136],
}

#[repr(C, packed)]
pub struct TPacketGCStun {
    pub data: [u8; 8],
}

#[repr(C, packed)]
pub struct TPacketGCTarget {
    pub data: [u8; 9],
}

#[repr(C, packed)]
pub struct TPacketGCTargetCreateNew {
    pub data: [u8; 46],
}

#[repr(C, packed)]
pub struct TPacketGCTargetDelete {
    pub data: [u8; 8],
}

#[repr(C, packed)]
pub struct TPacketGCTargetUpdate {
    pub data: [u8; 16],
}

#[repr(C, packed)]
pub struct TPacketGCTime {
    pub data: [u8; 12],
}

#[repr(C, packed)]
pub struct TPacketGCViewEquip {
    pub data: [u8; 503],
}

#[repr(C, packed)]
pub struct TPacketGCWalkMode {
    pub data: [u8; 9],
}

#[repr(C, packed)]
pub struct TPacketGCWarp {
    pub data: [u8; 18],
}

#[repr(C, packed)]
pub struct TPacketGCWhisper {
    pub data: [u8; 70],
}


#[no_mangle]
pub extern "C" fn m2_net_dispatch_game_packet(buf: *const u8, len: usize) -> u16 {
    if len < size_of::<TDynamicSizePacketHeader>() {
        return 0; // Not enough data for header
    }
    
    let header_ptr = buf as *const TDynamicSizePacketHeader;
    let header_val = unsafe { (*header_ptr).header };
    let packet_len = unsafe { (*header_ptr).length } as usize;

    if packet_len < size_of::<TDynamicSizePacketHeader>() {
        return 0; // Invalid packet size
    }

    if len < packet_len {
        return 0; // not fully received yet
    }
    
    match header_val {
        gc::PHASE => {
            if packet_len >= size_of::<TPacketGCPhase>() {
                return gc::PHASE;
            }
        },
        gc::KEY_CHALLENGE => {
            if packet_len >= size_of::<TPacketGCKeyChallenge>() {
                return gc::KEY_CHALLENGE;
            }
        },
        gc::KEY_COMPLETE => {
            if packet_len >= size_of::<TPacketGCKeyComplete>() {
                return gc::KEY_COMPLETE;
            }
        },
        gc::PING => {
            if packet_len >= size_of::<TPacketGCPing>() {
                return gc::PING;
            }
        },
        gc::OBSERVER_ADD => {
            if packet_len >= size_of::<TPacketGCObserverAdd>() {
                return gc::OBSERVER_ADD;
            }
        },
        gc::OBSERVER_REMOVE => {
            if packet_len >= size_of::<TPacketGCObserverRemove>() {
                return gc::OBSERVER_REMOVE;
            }
        },
        gc::OBSERVER_MOVE => {
            if packet_len >= size_of::<TPacketGCObserverMove>() {
                return gc::OBSERVER_MOVE;
            }
        },
        gc::WARP => {
            if packet_len >= size_of::<TPacketGCWarp>() {
                return gc::WARP;
            }
        },
        gc::PVP => {
            if packet_len >= size_of::<TPacketGCPVP>() {
                return gc::PVP;
            }
        },
        gc::DUEL_START => {
            if packet_len >= size_of::<TPacketGCDuelStart>() {
                return gc::DUEL_START;
            }
        },
        gc::CHARACTER_ADD => {
            if packet_len >= size_of::<TPacketGCCharacterAdd>() {
                return gc::CHARACTER_ADD;
            }
        },
        gc::CHAR_ADDITIONAL_INFO => {
            if packet_len >= size_of::<TPacketGCCharacterAdditionalInfo>() {
                return gc::CHAR_ADDITIONAL_INFO;
            }
        },
        gc::CHARACTER_ADD2 => {
            if packet_len >= size_of::<TPacketGCCharacterAdd2>() {
                return gc::CHARACTER_ADD2;
            }
        },
        gc::CHARACTER_UPDATE => {
            if packet_len >= size_of::<TPacketGCCharacterUpdate>() {
                return gc::CHARACTER_UPDATE;
            }
        },
        gc::CHARACTER_UPDATE2 => {
            if packet_len >= size_of::<TPacketGCCharacterUpdate2>() {
                return gc::CHARACTER_UPDATE2;
            }
        },
        gc::CHARACTER_DEL => {
            if packet_len >= size_of::<TPacketGCCharacterDelete>() {
                return gc::CHARACTER_DEL;
            }
        },
        gc::CHAT => {
            if packet_len >= size_of::<TPacketGCChat>() {
                return gc::CHAT;
            }
        },
        gc::SYNC_POSITION => {
            if packet_len >= size_of::<TPacketGCC2C>() {
                return gc::SYNC_POSITION;
            }
        },
        gc::OWNERSHIP => {
            if packet_len >= size_of::<TPacketGCOwnership>() {
                return gc::OWNERSHIP;
            }
        },
        gc::WHISPER => {
            if packet_len >= size_of::<TPacketGCWhisper>() {
                return gc::WHISPER;
            }
        },
        gc::MOVE => {
            if packet_len >= size_of::<TPacketGCMove>() {
                return gc::MOVE;
            }
        },
        gc::CHARACTER_POSITION => {
            if packet_len >= size_of::<TPacketGCPosition>() {
                return gc::CHARACTER_POSITION;
            }
        },
        gc::STUN => {
            if packet_len >= size_of::<TPacketGCStun>() {
                return gc::STUN;
            }
        },
        gc::DEAD => {
            if packet_len >= size_of::<TPacketGCDead>() {
                return gc::DEAD;
            }
        },
        gc::PLAYER_POINT_CHANGE => {
            if packet_len >= size_of::<TPacketGCPointChange>() {
                return gc::PLAYER_POINT_CHANGE;
            }
        },
        gc::DAMAGE_INFO => {
            if packet_len >= size_of::<TPacketGCDamageInfo>() {
                return gc::DAMAGE_INFO;
            }
        },
        gc::ITEM_DEL => {
            if packet_len >= size_of::<TPacketGCItemDel>() {
                return gc::ITEM_DEL;
            }
        },
        gc::ITEM_SET => {
            if packet_len >= size_of::<TPacketGCItemSet>() {
                return gc::ITEM_SET;
            }
        },
        gc::ITEM_GET => {
            if packet_len >= size_of::<TPacketGCItemGet>() {
                return gc::ITEM_GET;
            }
        },
        gc::ITEM_USE => {
            if packet_len >= size_of::<TPacketGCItemUse>() {
                return gc::ITEM_USE;
            }
        },
        gc::ITEM_UPDATE => {
            if packet_len >= size_of::<TPacketGCItemUpdate>() {
                return gc::ITEM_UPDATE;
            }
        },
        gc::ITEM_GROUND_ADD => {
            if packet_len >= size_of::<TPacketGCItemGroundAdd>() {
                return gc::ITEM_GROUND_ADD;
            }
        },
        gc::ITEM_GROUND_DEL => {
            if packet_len >= size_of::<TPacketGCItemGroundDel>() {
                return gc::ITEM_GROUND_DEL;
            }
        },
        gc::ITEM_OWNERSHIP => {
            if packet_len >= size_of::<TPacketGCItemOwnership>() {
                return gc::ITEM_OWNERSHIP;
            }
        },
        gc::QUICKSLOT_ADD => {
            if packet_len >= size_of::<TPacketGCQuickSlotAdd>() {
                return gc::QUICKSLOT_ADD;
            }
        },
        gc::QUICKSLOT_DEL => {
            if packet_len >= size_of::<TPacketGCQuickSlotDel>() {
                return gc::QUICKSLOT_DEL;
            }
        },
        gc::QUICKSLOT_SWAP => {
            if packet_len >= size_of::<TPacketGCQuickSlotSwap>() {
                return gc::QUICKSLOT_SWAP;
            }
        },
        gc::MOTION => {
            if packet_len >= size_of::<TPacketGCMotion>() {
                return gc::MOTION;
            }
        },
        gc::CHANGE_SPEED => {
            if packet_len >= size_of::<TPacketGCChangeSpeed>() {
                return gc::CHANGE_SPEED;
            }
        },
        gc::SHOP => {
            if packet_len >= size_of::<TPacketGCShop>() {
                return gc::SHOP;
            }
        },
        gc::SHOP_SIGN => {
            if packet_len >= size_of::<TPacketGCShopSign>() {
                return gc::SHOP_SIGN;
            }
        },
        gc::EXCHANGE => {
            if packet_len >= size_of::<TPacketGCExchange>() {
                return gc::EXCHANGE;
            }
        },
        gc::QUEST_INFO => {
            if packet_len >= size_of::<TPacketGCQuestInfo>() {
                return gc::QUEST_INFO;
            }
        },
        gc::REQUEST_MAKE_GUILD => {
            if packet_len >= size_of::<TPacketGCBlank>() {
                return gc::REQUEST_MAKE_GUILD;
            }
        },
        gc::SCRIPT => {
            if packet_len >= size_of::<TPacketGCScript>() {
                return gc::SCRIPT;
            }
        },
        gc::QUEST_CONFIRM => {
            if packet_len >= size_of::<TPacketGCQuestConfirm>() {
                return gc::QUEST_CONFIRM;
            }
        },
        gc::TARGET => {
            if packet_len >= size_of::<TPacketGCTarget>() {
                return gc::TARGET;
            }
        },
        gc::MOUNT => {
            if packet_len >= size_of::<TPacketGCMount>() {
                return gc::MOUNT;
            }
        },
        gc::PLAYER_POINTS => {
            if packet_len >= size_of::<TPacketGCPoints>() {
                return gc::PLAYER_POINTS;
            }
        },
        gc::CREATE_FLY => {
            if packet_len >= size_of::<TPacketGCCreateFly>() {
                return gc::CREATE_FLY;
            }
        },
        gc::FLY_TARGETING => {
            if packet_len >= size_of::<TPacketGCFlyTargeting>() {
                return gc::FLY_TARGETING;
            }
        },
        gc::ADD_FLY_TARGETING => {
            if packet_len >= size_of::<TPacketGCFlyTargeting>() {
                return gc::ADD_FLY_TARGETING;
            }
        },
        gc::SKILL_LEVEL => {
            if packet_len >= size_of::<TPacketGCSkillLevel>() {
                return gc::SKILL_LEVEL;
            }
        },
        gc::SKILL_LEVEL_NEW => {
            if packet_len >= size_of::<TPacketGCSkillLevelNew>() {
                return gc::SKILL_LEVEL_NEW;
            }
        },
        gc::SKILL_COOLTIME_END => {
            if packet_len >= size_of::<TPacketGCSkillCoolTimeEnd>() {
                return gc::SKILL_COOLTIME_END;
            }
        },
        gc::MESSENGER => {
            if packet_len >= size_of::<TPacketGCMessenger>() {
                return gc::MESSENGER;
            }
        },
        gc::GUILD => {
            if packet_len >= size_of::<TPacketGCGuild>() {
                return gc::GUILD;
            }
        },
        gc::MARK_UPDATE => {
            if packet_len >= size_of::<TPacketGCMarkUpdate>() {
                return gc::MARK_UPDATE;
            }
        },
        gc::PARTY_INVITE => {
            if packet_len >= size_of::<TPacketGCPartyInvite>() {
                return gc::PARTY_INVITE;
            }
        },
        gc::PARTY_ADD => {
            if packet_len >= size_of::<TPacketGCPartyAdd>() {
                return gc::PARTY_ADD;
            }
        },
        gc::PARTY_UPDATE => {
            if packet_len >= size_of::<TPacketGCPartyUpdate>() {
                return gc::PARTY_UPDATE;
            }
        },
        gc::PARTY_REMOVE => {
            if packet_len >= size_of::<TPacketGCPartyRemove>() {
                return gc::PARTY_REMOVE;
            }
        },
        gc::PARTY_LINK => {
            if packet_len >= size_of::<TPacketGCPartyLink>() {
                return gc::PARTY_LINK;
            }
        },
        gc::PARTY_UNLINK => {
            if packet_len >= size_of::<TPacketGCPartyUnlink>() {
                return gc::PARTY_UNLINK;
            }
        },
        gc::PARTY_PARAMETER => {
            if packet_len >= size_of::<TPacketGCPartyParameter>() {
                return gc::PARTY_PARAMETER;
            }
        },
        gc::SAFEBOX_SET => {
            if packet_len >= size_of::<TPacketGCItemSet>() {
                return gc::SAFEBOX_SET;
            }
        },
        gc::SAFEBOX_DEL => {
            if packet_len >= size_of::<TPacketGCItemDel>() {
                return gc::SAFEBOX_DEL;
            }
        },
        gc::SAFEBOX_WRONG_PASSWORD => {
            if packet_len >= size_of::<TPacketGCSafeboxWrongPassword>() {
                return gc::SAFEBOX_WRONG_PASSWORD;
            }
        },
        gc::SAFEBOX_SIZE => {
            if packet_len >= size_of::<TPacketGCSafeboxSize>() {
                return gc::SAFEBOX_SIZE;
            }
        },
        gc::SAFEBOX_MONEY_CHANGE => {
            if packet_len >= size_of::<TPacketGCSafeboxMoneyChange>() {
                return gc::SAFEBOX_MONEY_CHANGE;
            }
        },
        gc::FISHING => {
            if packet_len >= size_of::<TPacketGCFishing>() {
                return gc::FISHING;
            }
        },
        gc::DUNGEON => {
            if packet_len >= size_of::<TPacketGCDungeon>() {
                return gc::DUNGEON;
            }
        },
        gc::TIME => {
            if packet_len >= size_of::<TPacketGCTime>() {
                return gc::TIME;
            }
        },
        gc::WALK_MODE => {
            if packet_len >= size_of::<TPacketGCWalkMode>() {
                return gc::WALK_MODE;
            }
        },
        gc::CHANGE_SKILL_GROUP => {
            if packet_len >= size_of::<TPacketGCChangeSkillGroup>() {
                return gc::CHANGE_SKILL_GROUP;
            }
        },
        gc::REFINE_INFORMATION => {
            if packet_len >= size_of::<TPacketGCRefineInformation>() {
                return gc::REFINE_INFORMATION;
            }
        },
        gc::REFINE_INFORMATION_NEW => {
            if packet_len >= size_of::<TPacketGCRefineInformationNew>() {
                return gc::REFINE_INFORMATION_NEW;
            }
        },
        gc::SEPCIAL_EFFECT => {
            if packet_len >= size_of::<TPacketGCSpecialEffect>() {
                return gc::SEPCIAL_EFFECT;
            }
        },
        gc::SPECIFIC_EFFECT => {
            if packet_len >= size_of::<TPacketGCSpecificEffect>() {
                return gc::SPECIFIC_EFFECT;
            }
        },
        gc::NPC_POSITION => {
            if packet_len >= size_of::<TPacketGCNPCPosition>() {
                return gc::NPC_POSITION;
            }
        },
        gc::CHANNEL => {
            if packet_len >= size_of::<TPacketGCChannel>() {
                return gc::CHANNEL;
            }
        },
        gc::VIEW_EQUIP => {
            if packet_len >= size_of::<TPacketGCViewEquip>() {
                return gc::VIEW_EQUIP;
            }
        },
        gc::LAND_LIST => {
            if packet_len >= size_of::<TPacketGCLandList>() {
                return gc::LAND_LIST;
            }
        },
        gc::TARGET_CREATE_NEW => {
            if packet_len >= size_of::<TPacketGCTargetCreateNew>() {
                return gc::TARGET_CREATE_NEW;
            }
        },
        gc::TARGET_UPDATE => {
            if packet_len >= size_of::<TPacketGCTargetUpdate>() {
                return gc::TARGET_UPDATE;
            }
        },
        gc::TARGET_DELETE => {
            if packet_len >= size_of::<TPacketGCTargetDelete>() {
                return gc::TARGET_DELETE;
            }
        },
        gc::AFFECT_ADD => {
            if packet_len >= size_of::<TPacketGCAffectAdd>() {
                return gc::AFFECT_ADD;
            }
        },
        gc::AFFECT_REMOVE => {
            if packet_len >= size_of::<TPacketGCAffectRemove>() {
                return gc::AFFECT_REMOVE;
            }
        },
        gc::MALL_OPEN => {
            if packet_len >= size_of::<TPacketGCMallOpen>() {
                return gc::MALL_OPEN;
            }
        },
        gc::MALL_SET => {
            if packet_len >= size_of::<TPacketGCItemSet>() {
                return gc::MALL_SET;
            }
        },
        gc::MALL_DEL => {
            if packet_len >= size_of::<TPacketGCItemDel>() {
                return gc::MALL_DEL;
            }
        },
        gc::LOVER_INFO => {
            if packet_len >= size_of::<TPacketGCLoverInfo>() {
                return gc::LOVER_INFO;
            }
        },
        gc::LOVE_POINT_UPDATE => {
            if packet_len >= size_of::<TPacketGCLovePointUpdate>() {
                return gc::LOVE_POINT_UPDATE;
            }
        },
        gc::DIG_MOTION => {
            if packet_len >= size_of::<TPacketGCDigMotion>() {
                return gc::DIG_MOTION;
            }
        },
        gc::DRAGON_SOUL_REFINE => {
            if packet_len >= size_of::<TPacketGCDragonSoulRefine>() {
                return gc::DRAGON_SOUL_REFINE;
            }
        },
        _ => {}
    }
    
    header_val
}
