//! Modul `phase_game_actor` — Obsluga pakietow pojawiania sie, ruchu i aktualizacji postaci
//! Wzorzec referencyjny C++: `source/UserInterface/PythonNetworkStreamPhaseGameActor.cpp`
//! Opis: Odwzorowanie RecvCharacterAppendPacket, RecvCharacterAdditionalInfo, RecvCharacterUpdatePacket, RecvCharacterDeletePacket, RecvCharacterMovePacket.

use std::collections::HashMap;

pub const CHARACTER_NAME_MAX_LEN: usize = 24;
pub const CHR_EQUIPPART_NUM: usize = 4;
pub const CHR_EQUIPPART_ARMOR: usize = 0;
pub const CHR_EQUIPPART_WEAPON: usize = 1;
pub const CHR_EQUIPPART_HEAD: usize = 2;
pub const CHR_EQUIPPART_HAIR: usize = 3;

pub const TYPE_PC: u8 = 0;
pub const TYPE_NPC: u8 = 1;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C, packed)]
pub struct PacketGcCharacterAdd {
    pub header: u8,
    pub vid: u32,
    pub angle: f32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub b_type: u8,
    pub w_race_num: u16,
    pub b_moving_speed: u8,
    pub b_attack_speed: u8,
    pub b_state_flag: u8,
    pub dw_affect_flag: [u32; 2],
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C, packed)]
pub struct PacketGcCharacterAdd2 {
    pub header: u8,
    pub vid: u32,
    pub name: [u8; CHARACTER_NAME_MAX_LEN + 1],
    pub angle: f32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub b_type: u8,
    pub w_race_num: u16,
    pub aw_part: [u16; CHR_EQUIPPART_NUM],
    pub b_moving_speed: u8,
    pub b_attack_speed: u8,
    pub b_state_flag: u8,
    pub dw_affect_flag: [u32; 2],
    pub b_empire: u8,
    pub dw_guild: u32,
    pub s_alignment: i16,
    pub b_pk_mode: u8,
    pub dw_mount_vnum: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C, packed)]
pub struct PacketGcCharacterAdditionalInfo {
    pub header: u8,
    pub vid: u32,
    pub name: [u8; CHARACTER_NAME_MAX_LEN + 1],
    pub aw_part: [u16; CHR_EQUIPPART_NUM],
    pub b_empire: u8,
    pub dw_guild_id: u32,
    pub dw_level: u32,
    pub s_alignment: i16,
    pub b_pk_mode: u8,
    pub dw_mount_vnum: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C, packed)]
pub struct PacketGcCharacterUpdate {
    pub header: u8,
    pub vid: u32,
    pub aw_part: [u16; CHR_EQUIPPART_NUM],
    pub b_moving_speed: u8,
    pub b_attack_speed: u8,
    pub b_state_flag: u8,
    pub dw_affect_flag: [u32; 2],
    pub dw_guild_id: u32,
    pub s_alignment: i16,
    pub b_pk_mode: u8,
    pub dw_mount_vnum: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C, packed)]
pub struct PacketGcCharacterUpdate2 {
    pub header: u8,
    pub vid: u32,
    pub aw_part: [u16; CHR_EQUIPPART_NUM],
    pub b_moving_speed: u8,
    pub b_attack_speed: u8,
    pub b_state_flag: u8,
    pub dw_affect_flag: [u32; 2],
    pub dw_guild_id: u32,
    pub s_alignment: i16,
    pub b_pk_mode: u8,
    pub dw_mount_vnum: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C, packed)]
pub struct PacketGcCharacterDelete {
    pub header: u8,
    pub vid: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C, packed)]
pub struct PacketGcMove {
    pub header: u8,
    pub b_func: u8,
    pub b_arg: u8,
    pub b_rot: u8,
    pub vid: u32,
    pub x: i32,
    pub y: i32,
    pub dw_time: u32,
    pub dw_duration: u32,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NetworkActorData {
    pub name: String,
    pub affect_flags: [u32; 2],
    pub b_type: u8,
    pub vid: u32,
    pub state_flags: u32,
    pub empire_id: u32,
    pub race: u32,
    pub mov_spd: u32,
    pub atk_spd: u32,
    pub rot: f32,
    pub cur_x: i32,
    pub cur_y: i32,
    pub armor: u32,
    pub weapon: u32,
    pub hair: u32,
    pub owner_vid: u32,
    pub alignment: i16,
    pub pk_mode: u8,
    pub mount_vnum: u32,
    pub guild_id: u32,
    pub level: u32,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NetworkMoveActorData {
    pub vid: u32,
    pub time: u32,
    pub pos_x: i32,
    pub pos_y: i32,
    pub rot: f32,
    pub func: u32,
    pub arg: u32,
    pub duration: u32,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NetworkUpdateActorData {
    pub vid: u32,
    pub guild_id: u32,
    pub armor: u32,
    pub weapon: u32,
    pub hair: u32,
    pub mov_spd: u32,
    pub atk_spd: u32,
    pub alignment: i16,
    pub pk_mode: u8,
    pub mount_vnum: u32,
    pub state_flags: u32,
    pub affect_flags: [u32; 2],
}

// Funkcje pomocnicze
fn is_invisible_race(race_num: u16) -> bool {
    matches!(race_num, 20025 | 20038 | 20039)
}

fn parse_c_string(bytes: &[u8]) -> String {
    let null_pos = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..null_pos]).into_owned()
}

#[derive(Debug, Clone, Default)]
pub struct PhaseGameActor {
    pub initialized: bool,
    pub actors: HashMap<u32, NetworkActorData>,
    pub main_vid: u32,
    // Przechowuje tymczasowe dane aktora przed otrzymaniem dodatkowych informacji
    pub temp_actor_data: Option<NetworkActorData>,
}

impl PhaseGameActor {
    pub fn new() -> Self {
        Self {
            initialized: true,
            actors: HashMap::new(),
            main_vid: 0,
            temp_actor_data: None,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn set_main_vid(&mut self, vid: u32) {
        self.main_vid = vid;
    }

    pub fn get_actor(&self, vid: u32) -> Option<&NetworkActorData> {
        self.actors.get(&vid)
    }

    pub fn recv_character_append_packet(&mut self, packet: &PacketGcCharacterAdd) -> bool {
        let vid = unsafe { std::ptr::addr_of!(packet.vid).read_unaligned() };
        let b_type = unsafe { std::ptr::addr_of!(packet.b_type).read_unaligned() };
        let w_race_num = unsafe { std::ptr::addr_of!(packet.w_race_num).read_unaligned() };
        
        let mut actor = NetworkActorData::default();
        actor.b_type = b_type;
        actor.mov_spd = unsafe { std::ptr::addr_of!(packet.b_moving_speed).read_unaligned() } as u32;
        actor.atk_spd = unsafe { std::ptr::addr_of!(packet.b_attack_speed).read_unaligned() } as u32;
        actor.race = w_race_num as u32;
        actor.state_flags = unsafe { std::ptr::addr_of!(packet.b_state_flag).read_unaligned() } as u32;
        actor.vid = vid;
        actor.rot = unsafe { std::ptr::addr_of!(packet.angle).read_unaligned() };
        actor.name = String::new();
        
        actor.affect_flags[0] = unsafe { std::ptr::addr_of!(packet.dw_affect_flag[0]).read_unaligned() };
        actor.affect_flags[1] = unsafe { std::ptr::addr_of!(packet.dw_affect_flag[1]).read_unaligned() };
        
        actor.cur_x = unsafe { std::ptr::addr_of!(packet.x).read_unaligned() };
        actor.cur_y = unsafe { std::ptr::addr_of!(packet.y).read_unaligned() };
        
        // Pola domyslne
        actor.alignment = 0;
        actor.pk_mode = 0;
        actor.guild_id = 0;
        actor.empire_id = 0;
        actor.armor = 0;
        actor.weapon = 0;
        actor.hair = 0;
        actor.mount_vnum = 0;
        actor.level = 0;

        if b_type != TYPE_PC && b_type != TYPE_NPC {
            // NPC / potwory - dodajemy od razu
            self.actors.insert(actor.vid, actor);
        } else {
            // Gracze - zapisujemy do otrzymania AdditionalInfo
            self.temp_actor_data = Some(actor);
        }

        true
    }

    pub fn recv_character_additional_info(&mut self, packet: &PacketGcCharacterAdditionalInfo) -> bool {
        let mut actor = match self.temp_actor_data.take() {
            Some(a) => a,
            None => return false,
        };

        if is_invisible_race(actor.race as u16) {
            return true;
        }
        
        let vid = unsafe { std::ptr::addr_of!(packet.vid).read_unaligned() };

        if actor.vid == vid {
            let name_bytes = unsafe { std::ptr::addr_of!(packet.name).read_unaligned() };
            actor.name = parse_c_string(&name_bytes);
            actor.guild_id = unsafe { std::ptr::addr_of!(packet.dw_guild_id).read_unaligned() };
            actor.level = unsafe { std::ptr::addr_of!(packet.dw_level).read_unaligned() };
            actor.alignment = unsafe { std::ptr::addr_of!(packet.s_alignment).read_unaligned() };
            actor.pk_mode = unsafe { std::ptr::addr_of!(packet.b_pk_mode).read_unaligned() };
            actor.empire_id = unsafe { std::ptr::addr_of!(packet.b_empire).read_unaligned() } as u32;
            
            let aw_part_armor = unsafe { std::ptr::addr_of!(packet.aw_part[CHR_EQUIPPART_ARMOR]).read_unaligned() };
            let aw_part_weapon = unsafe { std::ptr::addr_of!(packet.aw_part[CHR_EQUIPPART_WEAPON]).read_unaligned() };
            let aw_part_hair = unsafe { std::ptr::addr_of!(packet.aw_part[CHR_EQUIPPART_HAIR]).read_unaligned() };
            
            actor.armor = aw_part_armor as u32;
            actor.weapon = aw_part_weapon as u32;
            actor.hair = aw_part_hair as u32;
            actor.mount_vnum = unsafe { std::ptr::addr_of!(packet.dw_mount_vnum).read_unaligned() };

            self.actors.insert(actor.vid, actor);
        } else {
            // Blad, przywracamy
            self.temp_actor_data = Some(actor);
        }

        true
    }

    pub fn recv_character_append_packet_new(&mut self, packet: &PacketGcCharacterAdd2) -> bool {
        let w_race_num = unsafe { std::ptr::addr_of!(packet.w_race_num).read_unaligned() };
        if is_invisible_race(w_race_num) {
            return true;
        }

        let mut actor = NetworkActorData::default();
        actor.level = 0;
        actor.b_type = unsafe { std::ptr::addr_of!(packet.b_type).read_unaligned() };
        actor.guild_id = unsafe { std::ptr::addr_of!(packet.dw_guild).read_unaligned() };
        actor.empire_id = unsafe { std::ptr::addr_of!(packet.b_empire).read_unaligned() } as u32;
        actor.mov_spd = unsafe { std::ptr::addr_of!(packet.b_moving_speed).read_unaligned() } as u32;
        actor.atk_spd = unsafe { std::ptr::addr_of!(packet.b_attack_speed).read_unaligned() } as u32;
        actor.race = w_race_num as u32;
        
        let aw_part_armor = unsafe { std::ptr::addr_of!(packet.aw_part[CHR_EQUIPPART_ARMOR]).read_unaligned() };
        let aw_part_weapon = unsafe { std::ptr::addr_of!(packet.aw_part[CHR_EQUIPPART_WEAPON]).read_unaligned() };
        let aw_part_hair = unsafe { std::ptr::addr_of!(packet.aw_part[CHR_EQUIPPART_HAIR]).read_unaligned() };
        actor.armor = aw_part_armor as u32;
        actor.weapon = aw_part_weapon as u32;
        actor.hair = aw_part_hair as u32;
        
        actor.state_flags = unsafe { std::ptr::addr_of!(packet.b_state_flag).read_unaligned() } as u32;
        actor.vid = unsafe { std::ptr::addr_of!(packet.vid).read_unaligned() };
        actor.mount_vnum = unsafe { std::ptr::addr_of!(packet.dw_mount_vnum).read_unaligned() };
        actor.rot = unsafe { std::ptr::addr_of!(packet.angle).read_unaligned() };
        
        actor.affect_flags[0] = unsafe { std::ptr::addr_of!(packet.dw_affect_flag[0]).read_unaligned() };
        actor.affect_flags[1] = unsafe { std::ptr::addr_of!(packet.dw_affect_flag[1]).read_unaligned() };
        
        actor.cur_x = unsafe { std::ptr::addr_of!(packet.x).read_unaligned() };
        actor.cur_y = unsafe { std::ptr::addr_of!(packet.y).read_unaligned() };
        actor.alignment = unsafe { std::ptr::addr_of!(packet.s_alignment).read_unaligned() };
        actor.pk_mode = unsafe { std::ptr::addr_of!(packet.b_pk_mode).read_unaligned() };
        
        let name_bytes = unsafe { std::ptr::addr_of!(packet.name).read_unaligned() };
        actor.name = parse_c_string(&name_bytes);

        self.actors.insert(actor.vid, actor);
        true
    }

    pub fn recv_character_update_packet(&mut self, packet: &PacketGcCharacterUpdate) -> bool {
        let mut update = NetworkUpdateActorData::default();
        update.guild_id = unsafe { std::ptr::addr_of!(packet.dw_guild_id).read_unaligned() };
        update.mov_spd = unsafe { std::ptr::addr_of!(packet.b_moving_speed).read_unaligned() } as u32;
        update.atk_spd = unsafe { std::ptr::addr_of!(packet.b_attack_speed).read_unaligned() } as u32;
        
        let aw_part_armor = unsafe { std::ptr::addr_of!(packet.aw_part[CHR_EQUIPPART_ARMOR]).read_unaligned() };
        let aw_part_weapon = unsafe { std::ptr::addr_of!(packet.aw_part[CHR_EQUIPPART_WEAPON]).read_unaligned() };
        let aw_part_hair = unsafe { std::ptr::addr_of!(packet.aw_part[CHR_EQUIPPART_HAIR]).read_unaligned() };
        update.armor = aw_part_armor as u32;
        update.weapon = aw_part_weapon as u32;
        update.hair = aw_part_hair as u32;
        
        update.vid = unsafe { std::ptr::addr_of!(packet.vid).read_unaligned() };
        update.affect_flags[0] = unsafe { std::ptr::addr_of!(packet.dw_affect_flag[0]).read_unaligned() };
        update.affect_flags[1] = unsafe { std::ptr::addr_of!(packet.dw_affect_flag[1]).read_unaligned() };
        update.alignment = unsafe { std::ptr::addr_of!(packet.s_alignment).read_unaligned() };
        update.pk_mode = unsafe { std::ptr::addr_of!(packet.b_pk_mode).read_unaligned() };
        update.state_flags = unsafe { std::ptr::addr_of!(packet.b_state_flag).read_unaligned() } as u32;
        update.mount_vnum = unsafe { std::ptr::addr_of!(packet.dw_mount_vnum).read_unaligned() };

        if let Some(actor) = self.actors.get_mut(&update.vid) {
            actor.guild_id = update.guild_id;
            actor.mov_spd = update.mov_spd;
            actor.atk_spd = update.atk_spd;
            actor.armor = update.armor;
            actor.weapon = update.weapon;
            actor.hair = update.hair;
            actor.affect_flags = update.affect_flags;
            actor.alignment = update.alignment;
            actor.pk_mode = update.pk_mode;
            actor.state_flags = update.state_flags;
            actor.mount_vnum = update.mount_vnum;
        }

        true
    }

    pub fn recv_character_update_packet_new(&mut self, packet: &PacketGcCharacterUpdate2) -> bool {
        // Logika 1:1 identyczna z recv_character_update_packet na podstawie C++
        let mut update = NetworkUpdateActorData::default();
        update.guild_id = unsafe { std::ptr::addr_of!(packet.dw_guild_id).read_unaligned() };
        update.mov_spd = unsafe { std::ptr::addr_of!(packet.b_moving_speed).read_unaligned() } as u32;
        update.atk_spd = unsafe { std::ptr::addr_of!(packet.b_attack_speed).read_unaligned() } as u32;
        
        let aw_part_armor = unsafe { std::ptr::addr_of!(packet.aw_part[CHR_EQUIPPART_ARMOR]).read_unaligned() };
        let aw_part_weapon = unsafe { std::ptr::addr_of!(packet.aw_part[CHR_EQUIPPART_WEAPON]).read_unaligned() };
        let aw_part_hair = unsafe { std::ptr::addr_of!(packet.aw_part[CHR_EQUIPPART_HAIR]).read_unaligned() };
        update.armor = aw_part_armor as u32;
        update.weapon = aw_part_weapon as u32;
        update.hair = aw_part_hair as u32;
        
        update.vid = unsafe { std::ptr::addr_of!(packet.vid).read_unaligned() };
        update.affect_flags[0] = unsafe { std::ptr::addr_of!(packet.dw_affect_flag[0]).read_unaligned() };
        update.affect_flags[1] = unsafe { std::ptr::addr_of!(packet.dw_affect_flag[1]).read_unaligned() };
        update.alignment = unsafe { std::ptr::addr_of!(packet.s_alignment).read_unaligned() };
        update.pk_mode = unsafe { std::ptr::addr_of!(packet.b_pk_mode).read_unaligned() };
        update.state_flags = unsafe { std::ptr::addr_of!(packet.b_state_flag).read_unaligned() } as u32;
        update.mount_vnum = unsafe { std::ptr::addr_of!(packet.dw_mount_vnum).read_unaligned() };

        if let Some(actor) = self.actors.get_mut(&update.vid) {
            actor.guild_id = update.guild_id;
            actor.mov_spd = update.mov_spd;
            actor.atk_spd = update.atk_spd;
            actor.armor = update.armor;
            actor.weapon = update.weapon;
            actor.hair = update.hair;
            actor.affect_flags = update.affect_flags;
            actor.alignment = update.alignment;
            actor.pk_mode = update.pk_mode;
            actor.state_flags = update.state_flags;
            actor.mount_vnum = update.mount_vnum;
        }

        true
    }

    pub fn recv_character_delete_packet(&mut self, packet: &PacketGcCharacterDelete) -> bool {
        let vid = unsafe { std::ptr::addr_of!(packet.vid).read_unaligned() };
        self.actors.remove(&vid);
        true
    }

    pub fn recv_character_move_packet(&mut self, packet: &PacketGcMove) -> bool {
        let vid = unsafe { std::ptr::addr_of!(packet.vid).read_unaligned() };
        
        let mut move_data = NetworkMoveActorData::default();
        move_data.vid = vid;
        move_data.arg = unsafe { std::ptr::addr_of!(packet.b_arg).read_unaligned() } as u32;
        move_data.func = unsafe { std::ptr::addr_of!(packet.b_func).read_unaligned() } as u32;
        move_data.time = unsafe { std::ptr::addr_of!(packet.dw_time).read_unaligned() };
        move_data.rot = unsafe { std::ptr::addr_of!(packet.b_rot).read_unaligned() } as f32 * 5.0; // z C++: bRot*5.0f
        move_data.pos_x = unsafe { std::ptr::addr_of!(packet.x).read_unaligned() };
        move_data.pos_y = unsafe { std::ptr::addr_of!(packet.y).read_unaligned() };
        move_data.duration = unsafe { std::ptr::addr_of!(packet.dw_duration).read_unaligned() };

        if let Some(actor) = self.actors.get_mut(&vid) {
            actor.cur_x = move_data.pos_x;
            actor.cur_y = move_data.pos_y;
            actor.rot = move_data.rot;
            // Tutaj normalnie bylbysmy aktualizowali menedzera aktorow (NetworkActorManager), ale
            // dla celow PhaseGameActor mapujemy same dane
        }

        true
    }
}
