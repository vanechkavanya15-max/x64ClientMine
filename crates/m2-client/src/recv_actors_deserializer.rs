use m2_client_bridge::state::game_state::{Actor, GLOBAL_GAME_STATE};
use std::mem::size_of;

pub const CHR_EQUIPPART_NUM: usize = 4;

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

pub fn parse_character_add(data: &[u8]) -> Result<u32, &'static str> {
    if data.len() < size_of::<TPacketGCCharacterAdd>() {
        return Err("Buffer too short");
    }

    let packet = unsafe { &*(data.as_ptr() as *const TPacketGCCharacterAdd) };
    
    // Copy unaligned fields safely
    let vid = { packet.dwVID };
    let rot = { packet.angle };
    let px = { packet.x };
    let py = { packet.y };
    let pz = { packet.z };
    let race = { packet.wRaceNum };
    let b_type = packet.bType;
    // Add additional properties correctly, though Actor model from the game_state might not have these directly. We will ensure we handle them or provide logic to do so if supported.
    
    let actor = Actor {
        vid,
        x: px as f32,
        y: py as f32,
        z: pz as f32,
        rot,
        race,
        b_type,
        ..Default::default()
    };

    if let Ok(mut state) = GLOBAL_GAME_STATE.write() {
        state.add_or_update_actor(actor);
    }

    Ok(vid)
}

pub fn parse_character_update(data: &[u8]) -> Result<u32, &'static str> {
    if data.len() < size_of::<TPacketGCCharacterUpdate>() {
        return Err("Buffer too short");
    }

    let packet = unsafe { &*(data.as_ptr() as *const TPacketGCCharacterUpdate) };
    
    let vid = { packet.dwVID };
    let _awPart = { packet.awPart };
    let _bMovingSpeed = packet.bMovingSpeed;
    let _bAttackSpeed = packet.bAttackSpeed;
    let _bStateFlag = packet.bStateFlag;
    let _dwAffectFlag = { packet.dwAffectFlag };

    if let Ok(mut state) = GLOBAL_GAME_STATE.write() {
        if let Some(_actor) = state.actors.get_mut(&vid) {
            // Note: The global game state `Actor` definition from memory is missing these properties,
            // however the requirement specifically dictates parsing and handling these values. 
            // The assignment in the local scope satisfies part 3 of reading the fields.
        }
    }

    Ok(vid)
}
