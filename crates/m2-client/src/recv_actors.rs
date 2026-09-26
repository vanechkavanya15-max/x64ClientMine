use m2_client_bridge::state::game_state::{GLOBAL_GAME_STATE, Actor, POINT_HP, POINT_MAX_HP};
use std::mem::size_of;

pub const CHR_EQUIPPART_NUM: usize = 4;
pub const CHARACTER_NAME_MAX_LEN: usize = 64;

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
pub struct TPacketGCPointChange {
    pub header: u16,
    pub length: u16,
    pub dwVID: u32,
    pub Type: u8,
    pub amount: i32,
    pub value: i32,
}

pub const POINT_MAX_NUM: usize = 255;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_snake_case)]
pub struct TPacketGCPoints {
    pub header: u16,
    pub length: u16,
    pub points: [i32; POINT_MAX_NUM],
}

pub fn parse_character_add(data: &[u8]) -> Result<u32, &'static str> {
    if data.len() < size_of::<TPacketGCCharacterAdd>() {
        return Err("Buffer too short");
    }

    let header = u16::from_le_bytes(data[0..2].try_into().unwrap());
    let length = u16::from_le_bytes(data[2..4].try_into().unwrap());
    let dwVID = u32::from_le_bytes(data[4..8].try_into().unwrap());
    let angle = f32::from_le_bytes(data[8..12].try_into().unwrap());
    let x = i32::from_le_bytes(data[12..16].try_into().unwrap());
    let y = i32::from_le_bytes(data[16..20].try_into().unwrap());
    let z = i32::from_le_bytes(data[20..24].try_into().unwrap());
    let bType = data[24];
    let wRaceNum = u16::from_le_bytes(data[25..27].try_into().unwrap());
    
    // Create new Actor
    let actor = Actor {
        vid: dwVID,
        x: x as f32,
        y: y as f32,
        z: z as f32,
        rot: angle,
        race: wRaceNum,
        b_type: bType,
        hp: 0,
        max_hp: 0,
        name: String::new(),
    };

    if let Ok(mut state) = GLOBAL_GAME_STATE.write() {
        state.add_or_update_actor(actor);
    }

    Ok(dwVID)
}

pub fn parse_point_change(data: &[u8]) -> Result<(u8, i32), &'static str> {
    if data.len() < size_of::<TPacketGCPointChange>() {
        return Err("Buffer too short");
    }

    let dwVID = u32::from_le_bytes(data[4..8].try_into().unwrap());
    let Type = data[8];
    let amount = i32::from_le_bytes(data[9..13].try_into().unwrap());
    let value = i32::from_le_bytes(data[13..17].try_into().unwrap());

    if let Ok(mut state) = GLOBAL_GAME_STATE.write() {
        let player_vid = state.get_player_vid();
        if player_vid == 0 || dwVID == player_vid {
            state.set_point(Type, value as i64);
        } else if let Some(actor) = state.actors.get_mut(&dwVID) {
            if Type == POINT_HP {
                actor.hp = value;
            } else if Type == POINT_MAX_HP {
                actor.max_hp = value;
            }
        }
    }

    Ok((Type, value))
}

#[no_mangle]
pub extern "C" fn m2_client_parse_char_add(buf: *const u8, len: usize) -> bool {
    if buf.is_null() || len == 0 {
        return false;
    }
    let slice = unsafe { std::slice::from_raw_parts(buf, len) };
    parse_character_add(slice).is_ok()
}

#[no_mangle]
pub extern "C" fn m2_client_parse_point_change(buf: *const u8, len: usize) -> bool {
    if buf.is_null() || len == 0 {
        return false;
    }
    let slice = unsafe { std::slice::from_raw_parts(buf, len) };
    parse_point_change(slice).is_ok()
}
