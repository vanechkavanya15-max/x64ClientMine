use std::ffi::CStr;
use std::mem::size_of;
use std::sync::{OnceLock, RwLock};

use crate::network_actor_mgr::{NetworkActorData, NetworkActorMgr, AffectFlagContainer};

pub const CHR_EQUIPPART_NUM: usize = 4;
pub const CHARACTER_NAME_MAX_LEN: usize = 64;

pub const HEADER_GC_CHARACTER_ADD: u16 = 0x0205;
pub const HEADER_GC_CHARACTER_ADD2: u16 = 0x0206;
pub const HEADER_GC_CHAR_ADDITIONAL_INFO: u16 = 0x0207;
pub const HEADER_GC_CHARACTER_DEL: u16 = 0x0208;
pub const HEADER_GC_CHARACTER_UPDATE: u16 = 0x0209;
pub const HEADER_GC_CHARACTER_UPDATE2: u16 = 0x020A;
pub const HEADER_GC_STUN: u16 = 0x0216;
pub const HEADER_GC_DEAD: u16 = 0x0217;
pub const HEADER_GC_CHANGE_SPEED: u16 = 0x0218;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_snake_case)]
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
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_snake_case)]
pub struct TPacketGCCharacterAdd2 {
    pub header: u16,
    pub length: u16,
    pub dwVID: u32,
    pub name: [u8; CHARACTER_NAME_MAX_LEN + 1],
    pub angle: f32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub bType: u8,
    pub wRaceNum: u16,
    pub awPart: [u16; CHR_EQUIPPART_NUM],
    pub bMovingSpeed: u8,
    pub bAttackSpeed: u8,
    pub bStateFlag: u8,
    pub dwAffectFlag: [u32; 2],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_snake_case)]
pub struct TPacketGCCharacterAdditionalInfo {
    pub header: u16,
    pub length: u16,
    pub dwVID: u32,
    pub name: [u8; CHARACTER_NAME_MAX_LEN + 1],
    pub awPart: [u16; CHR_EQUIPPART_NUM],
    pub bEmpire: u8,
    pub dwGuildID: u32,
    pub dwLevel: u32,
    pub sAlignment: i16,
    pub bPKMode: u8,
    pub dwMountVnum: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_snake_case)]
pub struct TPacketGCCharacterUpdate {
    pub header: u16,
    pub length: u16,
    pub dwVID: u32,
    pub awPart: [u16; CHR_EQUIPPART_NUM],
    pub bMovingSpeed: u8,
    pub bAttackSpeed: u8,
    pub bStateFlag: u8,
    pub dwAffectFlag: [u32; 2],
    pub dwGuildID: u32,
    pub sAlignment: i16,
    pub bPKMode: u8,
    pub dwMountVnum: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_snake_case)]
pub struct TPacketGCCharacterUpdate2 {
    pub header: u16,
    pub length: u16,
    pub dwVID: u32,
    pub awPart: [u16; CHR_EQUIPPART_NUM],
    pub bMovingSpeed: u8,
    pub bAttackSpeed: u8,
    pub bStateFlag: u8,
    pub dwAffectFlag: [u32; 2],
    pub dwGuildID: u32,
    pub sAlignment: i16,
    pub bPKMode: u8,
    pub dwMountVnum: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_snake_case)]
pub struct TPacketGCCharacterDelete {
    pub header: u16,
    pub length: u16,
    pub dwVID: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_snake_case)]
pub struct TPacketGCStun {
    pub header: u16,
    pub length: u16,
    pub vid: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_snake_case)]
pub struct TPacketGCDead {
    pub header: u16,
    pub length: u16,
    pub vid: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_snake_case)]
pub struct TPacketGCChangeSpeed {
    pub header: u16,
    pub length: u16,
    pub vid: u32,
    pub moving_speed: u16,
}

pub fn get_character_name(p_name: &[u8]) -> String {
    if let Ok(c_str) = CStr::from_bytes_until_nul(p_name) {
        c_str.to_string_lossy().into_owned()
    } else {
        let mut len = 0;
        while len <= CHARACTER_NAME_MAX_LEN && p_name[len] != 0 {
            len += 1;
        }
        String::from_utf8_lossy(&p_name[..len]).into_owned()
    }
}

pub fn parse_character_add(data: &[u8], actor_mgr: &mut NetworkActorMgr) -> Result<(), &'static str> {
    if data.len() < size_of::<TPacketGCCharacterAdd>() {
        return Err("Buffer too short");
    }
    let packet = unsafe { &*(data.as_ptr() as *const TPacketGCCharacterAdd) };
    
    let mut actor = {
    let mut actor = NetworkActorData::new();
    actor.vid = packet.dwVID;
    actor.cur_x = packet.x;
    actor.cur_y = packet.y;
    actor.cur_z = packet.z;
    actor
};
    actor.rot = packet.angle;
    actor.b_type = packet.bType;
    actor.race = packet.wRaceNum as u32;
    actor.mov_spd = packet.bMovingSpeed as u32;
    actor.atk_spd = packet.bAttackSpeed as u32;
    actor.state_flags = packet.bStateFlag as u32;
    actor.affect_flags.flags[0] = packet.dwAffectFlag[0];
    actor.affect_flags.flags[1] = packet.dwAffectFlag[1];
    
    actor_mgr.append_actor(actor);
    Ok(())
}

pub fn parse_character_add2(data: &[u8], actor_mgr: &mut NetworkActorMgr) -> Result<(), &'static str> {
    if data.len() < size_of::<TPacketGCCharacterAdd2>() {
        return Err("Buffer too short");
    }
    let packet = unsafe { &*(data.as_ptr() as *const TPacketGCCharacterAdd2) };
    
    let mut actor = {
    let mut actor = NetworkActorData::new();
    actor.vid = packet.dwVID;
    actor.cur_x = packet.x;
    actor.cur_y = packet.y;
    actor.cur_z = packet.z;
    actor
};
    actor.name = get_character_name(&packet.name);
    actor.rot = packet.angle;
    actor.b_type = packet.bType;
    actor.race = packet.wRaceNum as u32;
    actor.mov_spd = packet.bMovingSpeed as u32;
    actor.atk_spd = packet.bAttackSpeed as u32;
    actor.state_flags = packet.bStateFlag as u32;
    actor.affect_flags.flags[0] = packet.dwAffectFlag[0];
    actor.affect_flags.flags[1] = packet.dwAffectFlag[1];
    
    actor_mgr.append_actor(actor);
    Ok(())
}

pub fn parse_character_additional_info(data: &[u8], actor_mgr: &mut NetworkActorMgr) -> Result<(), &'static str> {
    if data.len() < size_of::<TPacketGCCharacterAdditionalInfo>() {
        return Err("Buffer too short");
    }
    let packet = unsafe { &*(data.as_ptr() as *const TPacketGCCharacterAdditionalInfo) };
    
    if let Some(actor) = actor_mgr.get_actor_mut(packet.dwVID) {
        actor.name = get_character_name(&packet.name);
        actor.empire_id = packet.bEmpire as u32;
    }
    Ok(())
}

pub fn parse_character_update(data: &[u8], actor_mgr: &mut NetworkActorMgr) -> Result<(), &'static str> {
    if data.len() < size_of::<TPacketGCCharacterUpdate>() {
        return Err("Buffer too short");
    }
    let packet = unsafe { &*(data.as_ptr() as *const TPacketGCCharacterUpdate) };
    
    if let Some(actor) = actor_mgr.get_actor_mut(packet.dwVID) {
        actor.mov_spd = packet.bMovingSpeed as u32;
        actor.atk_spd = packet.bAttackSpeed as u32;
        actor.state_flags = packet.bStateFlag as u32;
        actor.affect_flags.flags[0] = packet.dwAffectFlag[0];
        actor.affect_flags.flags[1] = packet.dwAffectFlag[1];
    }
    Ok(())
}

pub fn parse_character_update2(data: &[u8], actor_mgr: &mut NetworkActorMgr) -> Result<(), &'static str> {
    if data.len() < size_of::<TPacketGCCharacterUpdate2>() {
        return Err("Buffer too short");
    }
    let packet = unsafe { &*(data.as_ptr() as *const TPacketGCCharacterUpdate2) };
    
    if let Some(actor) = actor_mgr.get_actor_mut(packet.dwVID) {
        actor.mov_spd = packet.bMovingSpeed as u32;
        actor.atk_spd = packet.bAttackSpeed as u32;
        actor.state_flags = packet.bStateFlag as u32;
        actor.affect_flags.flags[0] = packet.dwAffectFlag[0];
        actor.affect_flags.flags[1] = packet.dwAffectFlag[1];
    }
    Ok(())
}

pub fn parse_character_delete(data: &[u8], actor_mgr: &mut NetworkActorMgr) -> Result<(), &'static str> {
    if data.len() < size_of::<TPacketGCCharacterDelete>() {
        return Err("Buffer too short");
    }
    let packet = unsafe { &*(data.as_ptr() as *const TPacketGCCharacterDelete) };
    actor_mgr.remove_actor(packet.dwVID);
    Ok(())
}

pub fn parse_stun(data: &[u8], actor_mgr: &mut NetworkActorMgr) -> Result<(), &'static str> {
    if data.len() < size_of::<TPacketGCStun>() {
        return Err("Buffer too short");
    }
    let packet = unsafe { &*(data.as_ptr() as *const TPacketGCStun) };
    if let Some(actor) = actor_mgr.get_actor_mut(packet.vid) {
        actor.is_stunned = true;
    }
    Ok(())
}

pub fn parse_dead(data: &[u8], actor_mgr: &mut NetworkActorMgr) -> Result<(), &'static str> {
    if data.len() < size_of::<TPacketGCDead>() {
        return Err("Buffer too short");
    }
    let packet = unsafe { &*(data.as_ptr() as *const TPacketGCDead) };
    if let Some(actor) = actor_mgr.get_actor_mut(packet.vid) {
        actor.is_dead = true;
    }
    Ok(())
}

pub fn parse_change_speed(data: &[u8], actor_mgr: &mut NetworkActorMgr) -> Result<(), &'static str> {
    if data.len() < size_of::<TPacketGCChangeSpeed>() {
        return Err("Buffer too short");
    }
    let packet = unsafe { &*(data.as_ptr() as *const TPacketGCChangeSpeed) };
    if let Some(actor) = actor_mgr.get_actor_mut(packet.vid) {
        actor.mov_spd = packet.moving_speed as u32;
    }
    Ok(())
}
