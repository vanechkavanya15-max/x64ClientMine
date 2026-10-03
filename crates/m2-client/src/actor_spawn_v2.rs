use std::ffi::CStr;
use std::mem::size_of;
use crate::terrain_ffi::m2_terrain_get_height;
use crate::network_actor_mgr::{NetworkActorData, NetworkActorMgr};
use std::sync::{RwLock, OnceLock};

pub fn global_to_local(world_x: i32, world_y: i32, map_base_x: i32, map_base_y: i32) -> (f32, f32) {
    let local_x = (world_x - map_base_x) as f32;
    let local_y = (world_y - map_base_y) as f32;
    (local_x, local_y)
}

#[derive(Debug, Clone, PartialEq)]
pub struct NetworkActorDataV2 {
    pub vid: u32,
    pub name: String,
    
    pub world_x: i32,
    pub world_y: i32,
    pub local_x: f32,
    pub local_y: f32,
    pub local_z: f32,
    pub rot: f32,
    
    pub race: u16,
    pub actor_type: u8,
    pub moving_speed: u8,
    pub attack_speed: u8,
    
    pub state_flags: u8,
    pub affect_flags: [u32; 2],
    pub alignment: i16,
    pub pk_mode: u8,
    
    pub guild_id: u32,
    pub empire_id: u8,
    pub mount_vnum: u32,
    
    pub parts: [u32; 6], // armor, weapon, head, weapon_left, hair, sash
}

impl NetworkActorDataV2 {
    pub fn new() -> Self {
        Self {
            vid: 0,
            name: String::new(),
            world_x: 0,
            world_y: 0,
            local_x: 0.0,
            local_y: 0.0,
            local_z: 0.0,
            rot: 0.0,
            race: 0,
            actor_type: 0,
            moving_speed: 0,
            attack_speed: 0,
            state_flags: 0,
            affect_flags: [0; 2],
            alignment: 0,
            pk_mode: 0,
            guild_id: 0,
            empire_id: 0,
            mount_vnum: 0,
            parts: [0; 6],
        }
    }

    pub fn set_position(&mut self, world_x: i32, world_y: i32, map_base_x: i32, map_base_y: i32) {
        self.world_x = world_x;
        self.world_y = world_y;
        
        let (lx, ly) = global_to_local(world_x, world_y, map_base_x, map_base_y);
        self.local_x = lx;
        self.local_y = ly;
        self.local_z = m2_terrain_get_height(lx, ly);
    }
}

pub const CHARACTER_NAME_MAX_LEN: usize = 24;
pub const CHR_EQUIPPART_NUM: usize = 4;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
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
#[derive(Debug, Clone, Copy)]
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
#[derive(Debug, Clone, Copy)]
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

pub const HEADER_GC_CHARACTER_ADD: u16 = 0x0205;
pub const HEADER_GC_CHARACTER_ADD2: u16 = 0x0206;
pub const HEADER_GC_CHARACTER_ADDITIONAL_INFO: u16 = 0x0207;

pub fn parse_character_add_v2(
    data: &[u8],
    map_base_x: i32,
    map_base_y: i32,
) -> Result<NetworkActorDataV2, &'static str> {
    if data.len() < 2 {
        return Err("Buffer too short for header");
    }

    let header = unsafe { *(data.as_ptr() as *const u16) };
    let mut actor = NetworkActorDataV2::new();

    if header == HEADER_GC_CHARACTER_ADD {
        if data.len() < size_of::<TPacketGCCharacterAdd>() {
            return Err("Buffer too short for Add");
        }
        let packet = unsafe { &*(data.as_ptr() as *const TPacketGCCharacterAdd) };
        actor.vid = packet.dwVID;
        actor.rot = packet.angle;
        actor.actor_type = packet.bType;
        actor.race = packet.wRaceNum;
        actor.moving_speed = packet.bMovingSpeed;
        actor.attack_speed = packet.bAttackSpeed;
        actor.state_flags = packet.bStateFlag;
        
        let flags = packet.dwAffectFlag;
        actor.affect_flags[0] = flags[0];
        actor.affect_flags[1] = flags[1];
        
        actor.set_position(packet.x, packet.y, map_base_x, map_base_y);
    } else if header == HEADER_GC_CHARACTER_ADD2 {
        if data.len() < size_of::<TPacketGCCharacterAdd2>() {
            return Err("Buffer too short for Add2");
        }
        let packet = unsafe { &*(data.as_ptr() as *const TPacketGCCharacterAdd2) };
        actor.vid = packet.dwVID;
        
        let p_name = packet.name;
        if let Ok(c_str) = CStr::from_bytes_until_nul(&p_name) {
            actor.name = c_str.to_string_lossy().into_owned();
        } else {
            let mut len = 0;
            while len <= CHARACTER_NAME_MAX_LEN && p_name[len] != 0 {
                len += 1;
            }
            actor.name = String::from_utf8_lossy(&p_name[..len]).into_owned();
        }

        actor.rot = packet.angle;
        actor.actor_type = packet.bType;
        actor.race = packet.wRaceNum;
        actor.moving_speed = packet.bMovingSpeed;
        actor.attack_speed = packet.bAttackSpeed;
        actor.state_flags = packet.bStateFlag;
        
        let flags = packet.dwAffectFlag;
        actor.affect_flags[0] = flags[0];
        actor.affect_flags[1] = flags[1];
        
        let parts = packet.awPart;
        actor.parts[0] = parts[0] as u32;
        actor.parts[1] = parts[1] as u32;
        actor.parts[2] = parts[2] as u32;
        actor.parts[3] = parts[3] as u32;

        actor.set_position(packet.x, packet.y, map_base_x, map_base_y);
    } else if header == HEADER_GC_CHARACTER_ADDITIONAL_INFO {
        if data.len() < size_of::<TPacketGCCharacterAdditionalInfo>() {
            return Err("Buffer too short for Additional Info");
        }
        let packet = unsafe { &*(data.as_ptr() as *const TPacketGCCharacterAdditionalInfo) };
        actor.vid = packet.dwVID;
        
        let p_name = packet.name;
        if let Ok(c_str) = CStr::from_bytes_until_nul(&p_name) {
            actor.name = c_str.to_string_lossy().into_owned();
        } else {
            let mut len = 0;
            while len <= CHARACTER_NAME_MAX_LEN && p_name[len] != 0 {
                len += 1;
            }
            actor.name = String::from_utf8_lossy(&p_name[..len]).into_owned();
        }
        
        actor.empire_id = packet.bEmpire;
        actor.guild_id = packet.dwGuildID;
        actor.alignment = packet.sAlignment;
        actor.pk_mode = packet.bPKMode;
        actor.mount_vnum = packet.dwMountVnum;
        
        let parts = packet.awPart;
        actor.parts[0] = parts[0] as u32;
        actor.parts[1] = parts[1] as u32;
        actor.parts[2] = parts[2] as u32;
        actor.parts[3] = parts[3] as u32;
    } else {
        return Err("Unknown packet header");
    }

    Ok(actor)
}

pub fn get_network_actor_mgr() -> &'static RwLock<NetworkActorMgr> {
    static MGR: OnceLock<RwLock<NetworkActorMgr>> = OnceLock::new();
    MGR.get_or_init(|| RwLock::new(NetworkActorMgr::new()))
}

#[no_mangle]
pub extern "C" fn m2_actor_spawn_full(
    data_ptr: *const u8,
    data_len: usize,
    map_base_x: i32,
    map_base_y: i32,
    out_vid: *mut u32
) -> bool {
    if data_ptr.is_null() || data_len == 0 {
        return false;
    }
    let slice = unsafe { std::slice::from_raw_parts(data_ptr, data_len) };
    if let Ok(v2) = parse_character_add_v2(slice, map_base_x, map_base_y) {
        if !out_vid.is_null() {
            unsafe { *out_vid = v2.vid };
        }
        
        let mut actor = NetworkActorData::new();
        actor.vid = v2.vid;
        actor.name = v2.name.clone();
        actor.cur_x = v2.world_x;
        actor.cur_y = v2.world_y;
        actor.src_x = v2.world_x;
        actor.src_y = v2.world_y;
        actor.dst_x = v2.world_x;
        actor.dst_y = v2.world_y;
        actor.rot = v2.rot;
        actor.race = v2.race as u32;
        actor.b_type = v2.actor_type;
        actor.mov_spd = v2.moving_speed as u32;
        actor.atk_spd = v2.attack_speed as u32;
        actor.state_flags = v2.state_flags as u32;
        actor.affect_flags.flags[0] = v2.affect_flags[0];
        actor.affect_flags.flags[1] = v2.affect_flags[1];
        actor.alignment = v2.alignment;
        actor.pk_mode = v2.pk_mode;
        actor.guild_id = v2.guild_id;
        actor.empire_id = v2.empire_id as u32;
        actor.mount_vnum = v2.mount_vnum;
        actor.armor = v2.parts[0];
        actor.weapon = v2.parts[1];
        actor.hair = v2.parts[3];

        if let Ok(mut mgr) = get_network_actor_mgr().write() {
            let header = unsafe { *(slice.as_ptr() as *const u16) };
            if header == HEADER_GC_CHARACTER_ADDITIONAL_INFO {
                if let Some(existing) = mgr.get_actor_data(v2.vid) {
                    actor.cur_x = existing.cur_x;
                    actor.cur_y = existing.cur_y;
                    actor.src_x = existing.src_x;
                    actor.src_y = existing.src_y;
                    actor.dst_x = existing.dst_x;
                    actor.dst_y = existing.dst_y;
                    actor.rot = existing.rot;
                    actor.race = existing.race;
                    actor.b_type = existing.b_type;
                    actor.mov_spd = existing.mov_spd;
                    actor.atk_spd = existing.atk_spd;
                    actor.state_flags = existing.state_flags;
                    actor.affect_flags = existing.affect_flags.clone();
                }
            }
            mgr.append_actor(actor);
        }
        
        return true;
    }
    false
}
