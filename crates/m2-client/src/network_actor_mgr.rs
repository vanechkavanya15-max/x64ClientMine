use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, PartialEq)]
pub struct AffectFlagContainer {
    pub flags: [u32; 2],
}

impl AffectFlagContainer {
    pub fn new() -> Self {
        Self { flags: [0; 2] }
    }
    
    pub fn clear(&mut self) {
        self.flags = [0; 2];
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NetworkActorData {
    pub name: String,
    pub affect_flags: AffectFlagContainer,
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
    pub cur_z: i32,
    pub src_x: i32,
    pub src_y: i32,
    pub src_z: i32,
    pub dst_x: i32,
    pub dst_y: i32,
    pub dst_z: i32,
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
        Self {
            name: String::new(),
            affect_flags: AffectFlagContainer::new(),
            b_type: 0,
            vid: 0,
            state_flags: 0,
            empire_id: 0,
            race: 0,
            mov_spd: 0,
            atk_spd: 0,
            rot: 0.0,
            cur_x: 0,
            cur_y: 0,
            cur_z: 0,
            src_x: 0,
            src_y: 0,
            src_z: 0,
            dst_x: 0,
            dst_y: 0,
            dst_z: 0,
            server_src_time: 0,
            client_src_time: 0,
            duration: 0,
            armor: 0,
            weapon: 0,
            hair: 0,
            owner_vid: 0,
            alignment: 0,
            pk_mode: 0,
            mount_vnum: 0,
            guild_id: 0,
            level: 0,
        }
    }

    pub fn set_dst_position(&mut self, server_time: u32, dst_x: i32, dst_y: i32, dst_z: i32, duration: u32) {
        self.src_x = self.cur_x;
        self.src_y = self.cur_y;
        self.src_z = self.cur_z;
        self.dst_x = dst_x;
        self.dst_y = dst_y;
        self.dst_z = dst_z;
        self.server_src_time = server_time;
        self.duration = duration;
    }

    pub fn set_position(&mut self, pos_x: i32, pos_y: i32, pos_z: i32) {
        self.cur_x = pos_x;
        self.cur_y = pos_y;
        self.cur_z = pos_z;
        self.src_x = pos_x;
        self.src_y = pos_y;
        self.src_z = pos_z;
        self.dst_x = pos_x;
        self.dst_y = pos_y;
        self.dst_z = pos_z;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NetworkMoveActorData {
    pub vid: u32,
    pub time: u32,
    pub pos_x: i32,
    pub pos_y: i32,
    pub pos_z: i32,
    pub rot: f32,
    pub func: u32,
    pub arg: u32,
    pub duration: u32,
}

impl NetworkMoveActorData {
    pub fn new() -> Self {
        Self {
            vid: 0,
            time: 0,
            pos_x: 0,
            pos_y: 0,
            pos_z: 0,
            rot: 0.0,
            func: 0,
            arg: 0,
            duration: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
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
        Self {
            vid: 0,
            guild_id: 0,
            armor: 0,
            weapon: 0,
            hair: 0,
            mov_spd: 0,
            atk_spd: 0,
            alignment: 0,
            pk_mode: 0,
            mount_vnum: 0,
            state_flags: 0,
            affect_flags: AffectFlagContainer::new(),
        }
    }
}

pub struct NetworkActorMgr {
    main_vid: u32,
    
    
    net_actor_dict: HashMap<u32, NetworkActorData>,
    spawn_queue: VecDeque<u32>,
}

impl NetworkActorMgr {
    pub fn new() -> Self {
        Self {
            main_vid: 0,
            
            
            net_actor_dict: HashMap::new(),
            spawn_queue: VecDeque::new(),
        }
    }

    pub fn set_main_actor_vid(&mut self, vid: u32) {
        self.main_vid = vid;
    }

    pub fn get_main_actor_vid(&self) -> u32 {
        self.main_vid
    }

    pub fn get_actor_data(&self, vid: u32) -> Option<&NetworkActorData> {
        self.net_actor_dict.get(&vid)
    }

    pub fn get_actor_mut(&mut self, vid: u32) -> Option<&mut NetworkActorData> {
        self.net_actor_dict.get_mut(&vid)
    }

    pub fn append_actor(&mut self, net_actor_data: NetworkActorData) {
        let vid = net_actor_data.vid;
        
        // C++:
        // if (__IsMainActorVID(c_rkNetActorData.m_dwVID)) ... // some mount logic
        // m_kNetActorDict[c_rkNetActorData.m_dwVID] = c_rkNetActorData;
        // if (__IsVisibleActor(rkNetActorData)) ... // append to CharacterManager
        
        self.net_actor_dict.insert(vid, net_actor_data);
        
        // Queue the spawn
        if !self.spawn_queue.contains(&vid) {
            self.spawn_queue.push_back(vid);
        }
    }

    pub fn remove_actor(&mut self, vid: u32) {
        if self.net_actor_dict.contains_key(&vid) {
            // C++ also does __RemoveCharacterManagerActor here
            self.net_actor_dict.remove(&vid);
            self.spawn_queue.retain(|&x| x != vid);
        }
    }

    pub fn update_actor(&mut self, net_update_actor_data: &NetworkUpdateActorData) {
        let vid = net_update_actor_data.vid;
        
        if let Some(actor_data) = self.net_actor_dict.get_mut(&vid) {
            actor_data.affect_flags = net_update_actor_data.affect_flags.clone();
            actor_data.guild_id = net_update_actor_data.guild_id;
            actor_data.mov_spd = net_update_actor_data.mov_spd;
            actor_data.atk_spd = net_update_actor_data.atk_spd;
            actor_data.armor = net_update_actor_data.armor;
            actor_data.weapon = net_update_actor_data.weapon;
            actor_data.hair = net_update_actor_data.hair;
            actor_data.alignment = net_update_actor_data.alignment;
            actor_data.pk_mode = net_update_actor_data.pk_mode;
            
            // C++ actually finds the instance and updates it directly too.
            // We just update the networked data here, the system using this
            // should listen to changes or we'd notify an event.
        }
    }

    pub fn move_actor(&mut self, net_move_actor_data: &NetworkMoveActorData) {
        let vid = net_move_actor_data.vid;
        
        if let Some(actor_data) = self.net_actor_dict.get_mut(&vid) {
            actor_data.set_dst_position(
                net_move_actor_data.time,
                net_move_actor_data.pos_x,
                net_move_actor_data.pos_y,
                net_move_actor_data.pos_z,
                net_move_actor_data.duration,
            );
            actor_data.rot = net_move_actor_data.rot;
            
            // C++ logic pushes TCP state to the instance.
        }
    }

    pub fn sync_actor(&mut self, vid: u32, pos_x: i32, pos_y: i32, pos_z: i32) {
        if let Some(actor_data) = self.net_actor_dict.get_mut(&vid) {
            actor_data.set_position(pos_x, pos_y, pos_z);
        }
    }

    pub fn set_actor_owner(&mut self, owner_vid: u32, victim_vid: u32) {
        if let Some(actor_data) = self.net_actor_dict.get_mut(&victim_vid) {
            actor_data.owner_vid = owner_vid;
        }
    }

    /// Process the spawn queue to prepare a limited number of actors per frame.
    /// Returns a list of `NetworkActorData` that should be actually spawned by the character manager.
    pub fn process_spawn_queue(&mut self, limit: usize) -> Vec<NetworkActorData> {
        let mut ready_to_spawn = Vec::new();
        let mut count = 0;

        while let Some(vid) = self.spawn_queue.pop_front() {
            if let Some(actor_data) = self.net_actor_dict.get(&vid) {
                // In C++ this is where __IsVisibleActor comes into play.
                // We'll assume the characters are ready to be instantiated.
                ready_to_spawn.push(actor_data.clone());
                count += 1;
                
                if count >= limit {
                    break;
                }
            }
        }

        ready_to_spawn
    }
}
