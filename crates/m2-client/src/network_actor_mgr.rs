//! Modul `network_actor_mgr` — Menedzer synchronizacji sieciowej aktorow CNetworkActorManager
//! Wzorzec referencyjny C++: `source/UserInterface/NetworkActorManager.cpp`
//! Opis: Odwzorowanie CNetworkActorManager i SNetworkActorData: rejestr obiektow w zasiegu wzroku (AOI), plynna synchronizacja pozycji, spawnowanie i usuwanie poza zasiegiem.

use std::collections::HashMap;

pub const AFFECT_SHOW_ALWAYS: usize = 7;
pub const CHAR_STAGE_VIEW_BOUND: i32 = 20000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AffectFlagContainer {
    elements: [u8; 8],
}

impl AffectFlagContainer {
    pub fn new() -> Self {
        Self { elements: [0; 8] }
    }

    pub fn clear(&mut self) {
        self.elements.fill(0);
    }

    pub fn is_set(&self, pos: usize) -> bool {
        if pos >= 64 {
            return false;
        }
        let byte_idx = pos / 8;
        let bit_idx = pos % 8;
        (self.elements[byte_idx] & (1 << bit_idx)) != 0
    }

    pub fn set(&mut self, pos: usize, is_set: bool) {
        if pos >= 64 {
            return;
        }
        let byte_idx = pos / 8;
        let bit_idx = pos % 8;
        if is_set {
            self.elements[byte_idx] |= 1 << bit_idx;
        } else {
            self.elements[byte_idx] &= !(1 << bit_idx);
        }
    }

    pub fn copy_instance(&mut self, other: &Self) {
        self.elements.copy_from_slice(&other.elements);
    }
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

impl NetworkMoveActorData {
    pub fn new() -> Self {
        Self::default()
    }
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
    pub affect_flags: AffectFlagContainer,
}

impl NetworkUpdateActorData {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, Default)]
pub struct NetworkActorData {
    pub name: String,
    pub affect_flags: AffectFlagContainer,
    pub type_id: u8,
    pub vid: u32,
    pub state_flags: u32,
    pub empire_id: u32,
    pub race: u32,
    pub mov_spd: u32,
    pub atk_spd: u32,
    pub rot: f32,
    pub cur_x: i32,
    pub cur_y: i32,
    pub src_x: i32,
    pub src_y: i32,
    pub dst_x: i32,
    pub dst_y: i32,
    pub server_src_time: u32,
    pub client_src_time: u32,
    pub duration: u32,
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

impl NetworkActorData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.cur_x = x;
        self.src_x = x;
        self.dst_x = x;
        self.cur_y = y;
        self.src_y = y;
        self.dst_y = y;
    }

    pub fn set_dst_position(
        &mut self,
        server_time: u32,
        dst_x: i32,
        dst_y: i32,
        duration: u32,
        current_time: u32,
    ) {
        self.src_x = self.cur_x;
        self.src_y = self.cur_y;
        self.dst_x = dst_x;
        self.dst_y = dst_y;
        self.duration = duration;
        self.server_src_time = server_time;
        self.client_src_time = current_time;
    }

    pub fn update_position(&mut self, current_time: u32) {
        let elapsed_time = current_time.saturating_sub(self.client_src_time);
        if elapsed_time < self.duration && self.duration > 0 {
            let rate = (elapsed_time as f32) / (self.duration as f32);
            self.cur_x = (((self.dst_x - self.src_x) as f32) * rate) as i32 + self.src_x;
            self.cur_y = (((self.dst_y - self.src_y) as f32) * rate) as i32 + self.src_y;
        } else {
            self.cur_x = self.dst_x;
            self.cur_y = self.dst_y;
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct NetworkActorMgr {
    pub initialized: bool,
    pub main_vid: u32,
    pub main_pos_x: i32,
    pub main_pos_y: i32,
    pub net_actor_dict: HashMap<u32, NetworkActorData>,
}

impl NetworkActorMgr {
    pub fn new() -> Self {
        Self {
            initialized: true,
            main_vid: 0,
            main_pos_x: 0,
            main_pos_y: 0,
            net_actor_dict: HashMap::new(),
        }
    }

    pub fn destroy(&mut self) {
        self.net_actor_dict.clear();
        self.main_vid = 0;
        self.main_pos_x = 0;
        self.main_pos_y = 0;
    }

    pub fn set_main_actor_vid(&mut self, vid: u32) {
        self.main_vid = vid;
        self.main_pos_x = 0;
        self.main_pos_y = 0;
        self.net_actor_dict.clear();
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn is_main_actor_vid(&self, vid: u32) -> bool {
        self.main_vid == vid
    }

    pub fn is_visible_pos(&self, pos_x: i32, pos_y: i32) -> bool {
        let dx = (pos_x - self.main_pos_x) as f64;
        let dy = (pos_y - self.main_pos_y) as f64;
        let len = (dx * dx + dy * dy).sqrt() as i32;
        len < CHAR_STAGE_VIEW_BOUND
    }

    pub fn is_visible_actor(&self, data: &NetworkActorData) -> bool {
        if self.is_main_actor_vid(data.vid) {
            return true;
        }

        if data.affect_flags.is_set(AFFECT_SHOW_ALWAYS) {
            return true;
        }

        if self.is_visible_pos(data.cur_x, data.cur_y) {
            return true;
        }

        // TODO: is_wall check when map handling is fully available
        // if is_wall(data.race) { return true; }

        false
    }

    // TODO: Stub methods for CharacterManager. Since the exact interaction
    // isn't fully defined yet, these simply log/act as no-ops.
    fn __append_character_manager_actor(&self, _data: &NetworkActorData) {
        // Log/Trace stub
    }

    fn __remove_character_manager_actor(&self, _data: &NetworkActorData) {
        // Log/Trace stub
    }

    fn __remove_dynamic_actors(&mut self) {
        // Copy the keys (vids) to remove them from the dictionary later
        let keys: Vec<u32> = self.net_actor_dict.keys().copied().collect();
        for vid in keys {
            if !self.is_main_actor_vid(vid) {
                self.net_actor_dict.remove(&vid);
            }
        }
    }

    pub fn append_actor(&mut self, data: NetworkActorData) {
        if self.is_main_actor_vid(data.vid) {
            // bChangeMountStatus logic is stubbed
            let change_mount_status = false;

            if !change_mount_status {
                self.__remove_dynamic_actors();
            }
        }

        let vid = data.vid;
        self.net_actor_dict.insert(vid, data);

        // Fetch back to verify visibility
        if let Some(actor) = self.net_actor_dict.get(&vid) {
            if self.is_visible_actor(actor) {
                self.__append_character_manager_actor(actor);
            } else {
                // If appending failed or not visible, erase (matching C++)
                self.net_actor_dict.remove(&vid);
            }
        }
    }

    pub fn remove_actor(&mut self, vid: u32) {
        if let Some(actor) = self.net_actor_dict.remove(&vid) {
            self.__remove_character_manager_actor(&actor);
        }
    }

    pub fn update_actor(&mut self, data: &NetworkUpdateActorData) {
        if let Some(actor) = self.net_actor_dict.get_mut(&data.vid) {
            actor.affect_flags.copy_instance(&data.affect_flags);
            actor.guild_id = data.guild_id;
            actor.mov_spd = data.mov_spd;
            actor.atk_spd = data.atk_spd;
            actor.armor = data.armor;
            actor.weapon = data.weapon;
            actor.hair = data.hair;
            actor.alignment = data.alignment;
            actor.pk_mode = data.pk_mode;

            // TODO: In full implementation, find CharacterManager instance and update it here.
        }
    }

    pub fn move_actor(&mut self, data: &NetworkMoveActorData, current_time: u32) {
        if let Some(actor) = self.net_actor_dict.get_mut(&data.vid) {
            actor.set_dst_position(data.time, data.pos_x, data.pos_y, data.duration, current_time);
            actor.rot = data.rot;

            // TODO: In full implementation, push TCP state to CharacterManager instance.
        }
    }

    pub fn sync_actor(&mut self, vid: u32, pos_x: i32, pos_y: i32) {
        if let Some(actor) = self.net_actor_dict.get_mut(&vid) {
            actor.set_position(pos_x, pos_y);

            // TODO: In full implementation, sync pixel position in CharacterManager instance.
        }
    }

    pub fn set_actor_owner(&mut self, owner_vid: u32, victim_vid: u32) {
        if let Some(actor) = self.net_actor_dict.get_mut(&victim_vid) {
            actor.owner_vid = owner_vid;

            // TODO: In full implementation, set owner in CharacterManager instance.
        }
    }

    pub fn update(&mut self, current_time: u32) {
        // self.__update_main_actor() - update main_pos_x/y
        
        let mut to_append = Vec::new();

        for (_, actor) in self.net_actor_dict.iter_mut() {
            actor.update_position(current_time);
        }

        for (_, actor) in self.net_actor_dict.iter() {
            if self.is_visible_actor(actor) {
                to_append.push(actor.vid);
            }
        }

        for vid in to_append {
            if let Some(actor) = self.net_actor_dict.get(&vid) {
                self.__append_character_manager_actor(actor);
            }
        }
    }
}
