use std::collections::HashMap;
use m2_client_bridge::net::gc::char_add::{TPacketGCCharacterAdd, TPacketGCCharacterAdd2, TPacketGCCharacterAdditionalInfo};
use m2_client_bridge::net::gc::char_del::TPacketGCCharacterDelete;
use m2_client_bridge::net::gc::char_add::CHR_EQUIPPART_NUM;

#[derive(Debug, Clone, PartialEq)]
pub struct NetworkActorData {
    pub vid: u32,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub rotation: f32,
    pub race: u16,
    pub b_type: u8,
    pub moving_speed: u8,
    pub attack_speed: u8,
    pub state_flag: u8,
    pub affect_flag: [u32; 2],
    pub parts: [u16; CHR_EQUIPPART_NUM],
    pub empire: u8,
    pub guild_id: u32,
    pub level: u32,
    pub alignment: i16,
    pub pk_mode: u8,
    pub mount_vnum: u32,
}

impl Default for NetworkActorData {
    fn default() -> Self {
        Self {
            vid: 0,
            name: String::new(),
            x: 0,
            y: 0,
            z: 0,
            rotation: 0.0,
            race: 0,
            b_type: 0,
            moving_speed: 0,
            attack_speed: 0,
            state_flag: 0,
            affect_flag: [0; 2],
            parts: [0; CHR_EQUIPPART_NUM],
            empire: 0,
            guild_id: 0,
            level: 0,
            alignment: 0,
            pk_mode: 0,
            mount_vnum: 0,
        }
    }
}

pub struct CharacterManager {
    actors: HashMap<u32, NetworkActorData>,
}

impl CharacterManager {
    pub fn new() -> Self {
        Self {
            actors: HashMap::new(),
        }
    }

    pub fn get_actor(&self, vid: u32) -> Option<&NetworkActorData> {
        self.actors.get(&vid)
    }

    pub fn handle_character_add(&mut self, packet: &TPacketGCCharacterAdd) {
        let vid = unsafe { std::ptr::addr_of!(packet.dwVID).read_unaligned() };
        let x = unsafe { std::ptr::addr_of!(packet.x).read_unaligned() };
        let y = unsafe { std::ptr::addr_of!(packet.y).read_unaligned() };
        let z = unsafe { std::ptr::addr_of!(packet.z).read_unaligned() };
        let rotation = unsafe { std::ptr::addr_of!(packet.angle).read_unaligned() };
        let race = unsafe { std::ptr::addr_of!(packet.wRaceNum).read_unaligned() };
        let b_type = unsafe { std::ptr::addr_of!(packet.bType).read_unaligned() };
        let moving_speed = unsafe { std::ptr::addr_of!(packet.bMovingSpeed).read_unaligned() };
        let attack_speed = unsafe { std::ptr::addr_of!(packet.bAttackSpeed).read_unaligned() };
        let state_flag = unsafe { std::ptr::addr_of!(packet.bStateFlag).read_unaligned() };
        let affect_flag = unsafe { std::ptr::addr_of!(packet.dwAffectFlag).read_unaligned() };

        let actor = NetworkActorData {
            vid,
            x,
            y,
            z,
            rotation,
            race,
            b_type,
            moving_speed,
            attack_speed,
            state_flag,
            affect_flag,
            ..Default::default()
        };

        self.actors.insert(vid, actor);
    }

    pub fn handle_character_add2(&mut self, packet: &TPacketGCCharacterAdd2) {
        let vid = unsafe { std::ptr::addr_of!(packet.dwVID).read_unaligned() };
        let x = unsafe { std::ptr::addr_of!(packet.x).read_unaligned() };
        let y = unsafe { std::ptr::addr_of!(packet.y).read_unaligned() };
        let z = unsafe { std::ptr::addr_of!(packet.z).read_unaligned() };
        let rotation = unsafe { std::ptr::addr_of!(packet.angle).read_unaligned() };
        let race = unsafe { std::ptr::addr_of!(packet.wRaceNum).read_unaligned() };
        let b_type = unsafe { std::ptr::addr_of!(packet.bType).read_unaligned() };
        let moving_speed = unsafe { std::ptr::addr_of!(packet.bMovingSpeed).read_unaligned() };
        let attack_speed = unsafe { std::ptr::addr_of!(packet.bAttackSpeed).read_unaligned() };
        let state_flag = unsafe { std::ptr::addr_of!(packet.bStateFlag).read_unaligned() };
        let affect_flag = unsafe { std::ptr::addr_of!(packet.dwAffectFlag).read_unaligned() };
        let parts = unsafe { std::ptr::addr_of!(packet.awPart).read_unaligned() };
        let empire = unsafe { std::ptr::addr_of!(packet.bEmpire).read_unaligned() };
        let guild_id = unsafe { std::ptr::addr_of!(packet.dwGuild).read_unaligned() };
        let alignment = unsafe { std::ptr::addr_of!(packet.sAlignment).read_unaligned() };
        let pk_mode = unsafe { std::ptr::addr_of!(packet.bPKMode).read_unaligned() };
        let mount_vnum = unsafe { std::ptr::addr_of!(packet.dwMountVnum).read_unaligned() };

        let name_bytes = unsafe { std::ptr::addr_of!(packet.name).read_unaligned() };
        let name_len = name_bytes.iter().position(|&c| c == 0).unwrap_or(name_bytes.len());
        let name = String::from_utf8_lossy(&name_bytes[..name_len]).into_owned();

        let actor = NetworkActorData {
            vid,
            name,
            x,
            y,
            z,
            rotation,
            race,
            b_type,
            moving_speed,
            attack_speed,
            state_flag,
            affect_flag,
            parts,
            empire,
            guild_id,
            alignment,
            pk_mode,
            mount_vnum,
            ..Default::default()
        };

        self.actors.insert(vid, actor);
    }

    pub fn handle_character_del(&mut self, packet: &TPacketGCCharacterDelete) {
        let vid = unsafe { std::ptr::addr_of!(packet.dwVID).read_unaligned() };
        self.actors.remove(&vid);
    }

    pub fn handle_character_additional_info(&mut self, packet: &TPacketGCCharacterAdditionalInfo) {
        let vid = unsafe { std::ptr::addr_of!(packet.dwVID).read_unaligned() };
        
        if let Some(actor) = self.actors.get_mut(&vid) {
            let parts = unsafe { std::ptr::addr_of!(packet.awPart).read_unaligned() };
            let empire = unsafe { std::ptr::addr_of!(packet.bEmpire).read_unaligned() };
            let guild_id = unsafe { std::ptr::addr_of!(packet.dwGuildID).read_unaligned() };
            let level = unsafe { std::ptr::addr_of!(packet.dwLevel).read_unaligned() };
            let alignment = unsafe { std::ptr::addr_of!(packet.sAlignment).read_unaligned() };
            let pk_mode = unsafe { std::ptr::addr_of!(packet.bPKMode).read_unaligned() };
            let mount_vnum = unsafe { std::ptr::addr_of!(packet.dwMountVnum).read_unaligned() };

            let name_bytes = unsafe { std::ptr::addr_of!(packet.name).read_unaligned() };
            let name_len = name_bytes.iter().position(|&c| c == 0).unwrap_or(name_bytes.len());
            let name = String::from_utf8_lossy(&name_bytes[..name_len]).into_owned();

            actor.name = name;
            actor.parts = parts;
            actor.empire = empire;
            actor.guild_id = guild_id;
            actor.level = level;
            actor.alignment = alignment;
            actor.pk_mode = pk_mode;
            actor.mount_vnum = mount_vnum;
        }
    }
}
