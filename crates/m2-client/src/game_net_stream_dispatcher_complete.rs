/// Dispatcher of Game Phase packets
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
    pub const CHANGE_SPEED: u16 = 0x0308;
    pub const SHOP: u16 = 0x0513;
    pub const SHOP_SIGN: u16 = 0x051F;
    pub const EXCHANGE: u16 = 0x0602;
    pub const QUEST_INFO: u16 = 0x0605;
    pub const REQUEST_MAKE_GUILD: u16 = 0x0606;
    pub const SCRIPT: u16 = 0x0607;
    pub const QUEST_CONFIRM: u16 = 0x0608;
    pub const TARGET: u16 = 0x0411;
    pub const MOUNT: u16 = 0x0412;
    pub const PLAYER_POINTS: u16 = 0x0214;
    pub const CREATE_FLY: u16 = 0x0416;
    pub const FLY_TARGETING: u16 = 0x0417;
    pub const ADD_FLY_TARGETING: u16 = 0x0418;
    pub const SKILL_LEVEL: u16 = 0x041A;
    pub const SKILL_LEVEL_NEW: u16 = 0x041B;
    pub const SKILL_COOLTIME_END: u16 = 0x0419;
    pub const MESSENGER: u16 = 0x0B01;
    pub const GUILD: u16 = 0x0B02;
    pub const MARK_UPDATE: u16 = 0x0B03;
    pub const PARTY_INVITE: u16 = 0x0C01;
    pub const PARTY_ADD: u16 = 0x0C02;
    pub const PARTY_UPDATE: u16 = 0x0C03;
    pub const PARTY_REMOVE: u16 = 0x0C04;
    pub const PARTY_LINK: u16 = 0x0C05;
    pub const PARTY_UNLINK: u16 = 0x0C06;
    pub const PARTY_PARAMETER: u16 = 0x0C07;
    pub const SAFEBOX_SET: u16 = 0x0801;
    pub const SAFEBOX_DEL: u16 = 0x0802;
    pub const SAFEBOX_WRONG_PASSWORD: u16 = 0x0803;
    pub const SAFEBOX_SIZE: u16 = 0x0804;
    pub const SAFEBOX_MONEY_CHANGE: u16 = 0x0805;
    pub const FISHING: u16 = 0x0901;
    pub const DUNGEON: u16 = 0x0902;
    pub const TIME: u16 = 0x0903;
    pub const WALK_MODE: u16 = 0x0A01;
    pub const CHANGE_SKILL_GROUP: u16 = 0x0A02;
    pub const REFINE_INFORMATION: u16 = 0x0A03;
    pub const REFINE_INFORMATION_NEW: u16 = 0x0A04;
    pub const SEPCIAL_EFFECT: u16 = 0x0A11;
    pub const SPECIFIC_EFFECT: u16 = 0x0A12;
    pub const NPC_POSITION: u16 = 0x0A21;
    pub const CHANNEL: u16 = 0x0A22;
    pub const VIEW_EQUIP: u16 = 0x0A23;
    pub const LAND_LIST: u16 = 0x0A31;
    pub const TARGET_CREATE_NEW: u16 = 0x0A42;
    pub const TARGET_UPDATE: u16 = 0x0A43;
    pub const TARGET_DELETE: u16 = 0x0A44;
    pub const AFFECT_ADD: u16 = 0x0A51;
    pub const AFFECT_REMOVE: u16 = 0x0A52;
    pub const MALL_OPEN: u16 = 0x0A61;
    pub const MALL_SET: u16 = 0x0A62;
    pub const MALL_DEL: u16 = 0x0A63;
    pub const LOVER_INFO: u16 = 0x0A71;
    pub const LOVE_POINT_UPDATE: u16 = 0x0A72;
    pub const DIG_MOTION: u16 = 0x0A81;
    pub const DRAGON_SOUL_REFINE: u16 = 0x0A91;
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
            return gc::PHASE;
        },
        gc::KEY_CHALLENGE => {
            return gc::KEY_CHALLENGE;
        },
        gc::KEY_COMPLETE => {
            return gc::KEY_COMPLETE;
        },
        gc::PING => {
            return gc::PING;
        },
        gc::OBSERVER_ADD => {
            return gc::OBSERVER_ADD;
        },
        gc::OBSERVER_REMOVE => {
            return gc::OBSERVER_REMOVE;
        },
        gc::OBSERVER_MOVE => {
            return gc::OBSERVER_MOVE;
        },
        gc::WARP => {
            let buf_slice = unsafe { std::slice::from_raw_parts(buf, packet_len) };
            crate::recv_warp_phase_v2::m2_net_recv_warp(buf_slice.as_ptr(), buf_slice.len());
            return gc::WARP;
        },
        gc::PVP => {
            let buf_slice = unsafe { std::slice::from_raw_parts(buf, packet_len) };
            crate::recv_combat_skills_complete::m2_recv_pvp(buf_slice.as_ptr(), buf_slice.len());
            return gc::PVP;
        },
        gc::DUEL_START => {
            let buf_slice = unsafe { std::slice::from_raw_parts(buf, packet_len) };
            crate::recv_combat_skills_complete::m2_recv_duel_start(buf_slice.as_ptr(), buf_slice.len());
            return gc::DUEL_START;
        },
        gc::CHARACTER_ADD => {
            let buf_slice = unsafe { std::slice::from_raw_parts(buf, packet_len) };
            crate::recv_actors::m2_client_parse_char_add(buf_slice.as_ptr(), buf_slice.len());
            return gc::CHARACTER_ADD;
        },
        gc::CHAR_ADDITIONAL_INFO => {
            return gc::CHAR_ADDITIONAL_INFO;
        },
        gc::CHARACTER_ADD2 => {
            return gc::CHARACTER_ADD2;
        },
        gc::CHARACTER_UPDATE => {
            return gc::CHARACTER_UPDATE;
        },
        gc::CHARACTER_UPDATE2 => {
            return gc::CHARACTER_UPDATE2;
        },
        gc::CHARACTER_DEL => {
            return gc::CHARACTER_DEL;
        },
        gc::CHAT => {
            return gc::CHAT;
        },
        gc::SYNC_POSITION => {
            return gc::SYNC_POSITION;
        },
        gc::OWNERSHIP => {
            return gc::OWNERSHIP;
        },
        gc::WHISPER => {
            return gc::WHISPER;
        },
        gc::MOVE => {
            return gc::MOVE;
        },
        gc::CHARACTER_POSITION => {
            return gc::CHARACTER_POSITION;
        },
        gc::STUN => {
            return gc::STUN;
        },
        gc::DEAD => {
            return gc::DEAD;
        },
        gc::PLAYER_POINT_CHANGE => {
            let buf_slice = unsafe { std::slice::from_raw_parts(buf, packet_len) };
            crate::recv_actors::m2_client_parse_point_change(buf_slice.as_ptr(), buf_slice.len());
            return gc::PLAYER_POINT_CHANGE;
        },
        gc::DAMAGE_INFO => {
            let buf_slice = unsafe { std::slice::from_raw_parts(buf, packet_len) };
            crate::recv_combat_skills_complete::m2_recv_damage_info(buf_slice.as_ptr(), buf_slice.len());
            return gc::DAMAGE_INFO;
        },
        gc::ITEM_DEL => {
            let buf_slice = unsafe { std::slice::from_raw_parts(buf, packet_len) };
            crate::recv_item_trade_complete::m2_net_recv_item_del(buf_slice.as_ptr(), buf_slice.len());
            return gc::ITEM_DEL;
        },
        gc::ITEM_SET => {
            let buf_slice = unsafe { std::slice::from_raw_parts(buf, packet_len) };
            crate::recv_item_trade_complete::m2_net_recv_item_set(buf_slice.as_ptr(), buf_slice.len());
            return gc::ITEM_SET;
        },
        gc::ITEM_GET => {
            return gc::ITEM_GET;
        },
        gc::ITEM_USE => {
            return gc::ITEM_USE;
        },
        gc::ITEM_UPDATE => {
            let buf_slice = unsafe { std::slice::from_raw_parts(buf, packet_len) };
            crate::recv_item_trade_complete::m2_net_recv_item_update(buf_slice.as_ptr(), buf_slice.len());
            return gc::ITEM_UPDATE;
        },
        gc::ITEM_GROUND_ADD => {
            let buf_slice = unsafe { std::slice::from_raw_parts(buf, packet_len) };
            crate::recv_item_trade_complete::m2_net_recv_item_ground_add(buf_slice.as_ptr(), buf_slice.len());
            return gc::ITEM_GROUND_ADD;
        },
        gc::ITEM_GROUND_DEL => {
            let buf_slice = unsafe { std::slice::from_raw_parts(buf, packet_len) };
            crate::recv_item_trade_complete::m2_net_recv_item_ground_del(buf_slice.as_ptr(), buf_slice.len());
            return gc::ITEM_GROUND_DEL;
        },
        gc::ITEM_OWNERSHIP => {
            return gc::ITEM_OWNERSHIP;
        },
        gc::QUICKSLOT_ADD => {
            return gc::QUICKSLOT_ADD;
        },
        gc::QUICKSLOT_DEL => {
            return gc::QUICKSLOT_DEL;
        },
        gc::QUICKSLOT_SWAP => {
            return gc::QUICKSLOT_SWAP;
        },
        gc::MOTION => {
            return gc::MOTION;
        },
        gc::CHANGE_SPEED => {
            return gc::CHANGE_SPEED;
        },
        gc::SHOP => {
            let buf_slice = unsafe { std::slice::from_raw_parts(buf, packet_len) };
            crate::recv_item_trade_complete::m2_net_recv_shop(buf_slice.as_ptr(), buf_slice.len());
            return gc::SHOP;
        },
        gc::SHOP_SIGN => {
            return gc::SHOP_SIGN;
        },
        gc::EXCHANGE => {
            let buf_slice = unsafe { std::slice::from_raw_parts(buf, packet_len) };
            crate::recv_exchange_logic_v2::m2_net_recv_exchange(buf_slice.as_ptr(), buf_slice.len());
            return gc::EXCHANGE;
        },
        gc::QUEST_INFO => {
            return gc::QUEST_INFO;
        },
        gc::REQUEST_MAKE_GUILD => {
            return gc::REQUEST_MAKE_GUILD;
        },
        gc::SCRIPT => {
            let buf_slice = unsafe { std::slice::from_raw_parts(buf, packet_len) };
            crate::recv_quest_dialog_v2::m2_quest_recv_script_packet(buf_slice.as_ptr(), buf_slice.len());
            return gc::SCRIPT;
        },
        gc::QUEST_CONFIRM => {
            return gc::QUEST_CONFIRM;
        },
        gc::TARGET => {
            return gc::TARGET;
        },
        gc::MOUNT => {
            return gc::MOUNT;
        },
        gc::PLAYER_POINTS => {
            return gc::PLAYER_POINTS;
        },
        gc::CREATE_FLY => {
            let buf_slice = unsafe { std::slice::from_raw_parts(buf, packet_len) };
            crate::recv_combat_skills_complete::m2_recv_create_fly(buf_slice.as_ptr(), buf_slice.len());
            return gc::CREATE_FLY;
        },
        gc::FLY_TARGETING => {
            let buf_slice = unsafe { std::slice::from_raw_parts(buf, packet_len) };
            crate::recv_combat_skills_complete::m2_recv_fly_targeting(buf_slice.as_ptr(), buf_slice.len());
            return gc::FLY_TARGETING;
        },
        gc::ADD_FLY_TARGETING => {
            return gc::ADD_FLY_TARGETING;
        },
        gc::SKILL_LEVEL => {
            let buf_slice = unsafe { std::slice::from_raw_parts(buf, packet_len) };
            crate::recv_combat_skills_complete::m2_recv_skill_level(buf_slice.as_ptr(), buf_slice.len());
            return gc::SKILL_LEVEL;
        },
        gc::SKILL_LEVEL_NEW => {
            let buf_slice = unsafe { std::slice::from_raw_parts(buf, packet_len) };
            crate::recv_combat_skills_complete::m2_recv_skill_level_new(buf_slice.as_ptr(), buf_slice.len());
            return gc::SKILL_LEVEL_NEW;
        },
        gc::SKILL_COOLTIME_END => {
            let buf_slice = unsafe { std::slice::from_raw_parts(buf, packet_len) };
            crate::recv_combat_skills_complete::m2_recv_skill_cooltime_end(buf_slice.as_ptr(), buf_slice.len());
            return gc::SKILL_COOLTIME_END;
        },
        gc::MESSENGER => {
            let buf_slice = unsafe { std::slice::from_raw_parts(buf, packet_len) };
            crate::recv_messenger_friends_v2::m2_client_parse_messenger(buf_slice.as_ptr(), buf_slice.len());
            return gc::MESSENGER;
        },
        gc::GUILD => {
            return gc::GUILD;
        },
        gc::MARK_UPDATE => {
            return gc::MARK_UPDATE;
        },
        gc::PARTY_INVITE => {
            return gc::PARTY_INVITE;
        },
        gc::PARTY_ADD => {
            return gc::PARTY_ADD;
        },
        gc::PARTY_UPDATE => {
            return gc::PARTY_UPDATE;
        },
        gc::PARTY_REMOVE => {
            return gc::PARTY_REMOVE;
        },
        gc::PARTY_LINK => {
            return gc::PARTY_LINK;
        },
        gc::PARTY_UNLINK => {
            return gc::PARTY_UNLINK;
        },
        gc::PARTY_PARAMETER => {
            return gc::PARTY_PARAMETER;
        },
        gc::SAFEBOX_SET => {
            return gc::SAFEBOX_SET;
        },
        gc::SAFEBOX_DEL => {
            return gc::SAFEBOX_DEL;
        },
        gc::SAFEBOX_WRONG_PASSWORD => {
            return gc::SAFEBOX_WRONG_PASSWORD;
        },
        gc::SAFEBOX_SIZE => {
            return gc::SAFEBOX_SIZE;
        },
        gc::SAFEBOX_MONEY_CHANGE => {
            return gc::SAFEBOX_MONEY_CHANGE;
        },
        gc::FISHING => {
            return gc::FISHING;
        },
        gc::DUNGEON => {
            return gc::DUNGEON;
        },
        gc::TIME => {
            return gc::TIME;
        },
        gc::WALK_MODE => {
            return gc::WALK_MODE;
        },
        gc::CHANGE_SKILL_GROUP => {
            return gc::CHANGE_SKILL_GROUP;
        },
        gc::REFINE_INFORMATION => {
            let buf_slice = unsafe { std::slice::from_raw_parts(buf, packet_len) };
            crate::recv_item_trade_complete::m2_net_recv_refine_info(buf_slice.as_ptr(), buf_slice.len());
            return gc::REFINE_INFORMATION;
        },
        gc::REFINE_INFORMATION_NEW => {
            let buf_slice = unsafe { std::slice::from_raw_parts(buf, packet_len) };
            crate::recv_item_trade_complete::m2_net_recv_refine_info_new(buf_slice.as_ptr(), buf_slice.len());
            return gc::REFINE_INFORMATION_NEW;
        },
        gc::SEPCIAL_EFFECT => {
            return gc::SEPCIAL_EFFECT;
        },
        gc::SPECIFIC_EFFECT => {
            return gc::SPECIFIC_EFFECT;
        },
        gc::NPC_POSITION => {
            return gc::NPC_POSITION;
        },
        gc::CHANNEL => {
            return gc::CHANNEL;
        },
        gc::VIEW_EQUIP => {
            return gc::VIEW_EQUIP;
        },
        gc::LAND_LIST => {
            return gc::LAND_LIST;
        },
        gc::TARGET_CREATE_NEW => {
            return gc::TARGET_CREATE_NEW;
        },
        gc::TARGET_UPDATE => {
            return gc::TARGET_UPDATE;
        },
        gc::TARGET_DELETE => {
            return gc::TARGET_DELETE;
        },
        gc::AFFECT_ADD => {
            return gc::AFFECT_ADD;
        },
        gc::AFFECT_REMOVE => {
            return gc::AFFECT_REMOVE;
        },
        gc::MALL_OPEN => {
            return gc::MALL_OPEN;
        },
        gc::MALL_SET => {
            return gc::MALL_SET;
        },
        gc::MALL_DEL => {
            return gc::MALL_DEL;
        },
        gc::LOVER_INFO => {
            return gc::LOVER_INFO;
        },
        gc::LOVE_POINT_UPDATE => {
            return gc::LOVE_POINT_UPDATE;
        },
        gc::DIG_MOTION => {
            return gc::DIG_MOTION;
        },
        gc::DRAGON_SOUL_REFINE => {
            return gc::DRAGON_SOUL_REFINE;
        },
        _ => {}
    }
    
    header_val
}
