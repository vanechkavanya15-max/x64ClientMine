use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PvpMode {
    None = 0,
    Agree = 1,
    Fight = 2,
    Revenge = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum DuelMode {
    None = 0,
    CannotAttack = 1,
    Start = 2,
}

/// Helper method to create a unified key for a PVP pair.
/// Replicates `CInstanceBase::__GetPVPKey(DWORD dwVIDSrc, DWORD dwVIDDst)` logic
pub fn get_pvp_key(vid_src: u32, vid_dst: u32) -> u32 {
    let (vid1, vid2) = if vid_src > vid_dst {
        (vid_dst, vid_src)
    } else {
        (vid_src, vid_dst)
    };

    let arr = [vid1, vid2];
    let mut h: u32 = 0;
    
    // We recreate the FNV-1-like string hashing from C++ (multiply by 16777619 and XOR)
    let bytes: [u8; 8] = unsafe { std::mem::transmute(arr) };
    
    for b in bytes {
        h = h.wrapping_mul(16777619);
        h ^= b as u32;
    }

    h
}

#[derive(Debug, Clone, Copy)]
pub struct PixelPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl PixelPosition {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

pub trait TerrainAttributeProvider {
    fn is_attr_on(&self, x: f32, y: f32, attr: u32) -> bool;
}

pub const ATTRIBUTE_BLOCK: u32 = 1 << 0;
pub const ATTRIBUTE_WATER: u32 = 1 << 1;
pub const ATTRIBUTE_BANPK: u32 = 1 << 2; // Safe zone attribute

pub struct PvpDuelStateMachine {
    duel_mode: DuelMode,
    pvp_keys: HashSet<u32>,
    pvp_ready_keys: HashSet<u32>,
    duel_keys: HashSet<u32>,
    gvg_keys: HashSet<u32>,
}

impl PvpDuelStateMachine {
    pub fn new() -> Self {
        Self {
            duel_mode: DuelMode::None,
            pvp_keys: HashSet::new(),
            pvp_ready_keys: HashSet::new(),
            duel_keys: HashSet::new(),
            gvg_keys: HashSet::new(),
        }
    }

    pub fn set_duel_mode(&mut self, mode: DuelMode) {
        self.duel_mode = mode;
    }

    pub fn get_duel_mode(&self) -> DuelMode {
        self.duel_mode
    }
    
    pub fn clear_pvp_key_system(&mut self) {
        self.pvp_keys.clear();
        self.pvp_ready_keys.clear();
        self.duel_keys.clear();
        self.gvg_keys.clear();
    }

    pub fn insert_pvp_key(&mut self, src_vid: u32, dst_vid: u32) {
        let key = get_pvp_key(src_vid, dst_vid);
        self.pvp_keys.insert(key);
    }

    pub fn remove_pvp_key(&mut self, src_vid: u32, dst_vid: u32) {
        let key = get_pvp_key(src_vid, dst_vid);
        self.pvp_keys.remove(&key);
    }

    pub fn insert_pvp_ready_key(&mut self, src_vid: u32, dst_vid: u32) {
        let key = get_pvp_key(src_vid, dst_vid);
        self.pvp_ready_keys.insert(key);
    }
    
    pub fn insert_duel_key(&mut self, src_vid: u32, dst_vid: u32) {
        let key = get_pvp_key(src_vid, dst_vid);
        self.duel_keys.insert(key);
    }

    pub fn remove_duel_key(&mut self, src_vid: u32, dst_vid: u32) {
        let key = get_pvp_key(src_vid, dst_vid);
        self.duel_keys.remove(&key);
    }

    pub fn insert_gvg_key(&mut self, src_guild: u32, dst_guild: u32) {
        let key = get_pvp_key(src_guild, dst_guild);
        self.gvg_keys.insert(key);
    }

    pub fn remove_gvg_key(&mut self, src_guild: u32, dst_guild: u32) {
        let key = get_pvp_key(src_guild, dst_guild);
        self.gvg_keys.remove(&key);
    }

    pub fn find_pvp_key(&self, src_vid: u32, dst_vid: u32) -> bool {
        let key = get_pvp_key(src_vid, dst_vid);
        self.pvp_keys.contains(&key)
    }

    pub fn find_pvp_ready_key(&self, src_vid: u32, dst_vid: u32) -> bool {
        let key = get_pvp_key(src_vid, dst_vid);
        self.pvp_ready_keys.contains(&key)
    }

    pub fn find_duel_key(&self, src_vid: u32, dst_vid: u32) -> bool {
        let key = get_pvp_key(src_vid, dst_vid);
        self.duel_keys.contains(&key)
    }

    pub fn find_gvg_key(&self, src_guild: u32, dst_guild: u32) -> bool {
        let key = get_pvp_key(src_guild, dst_guild);
        self.gvg_keys.contains(&key)
    }
    
    pub fn is_pvp_instance(&self, self_vid: u32, self_guild: u32, target_vid: u32, target_guild: u32) -> bool {
        if self.duel_mode != DuelMode::None {
            return true;
        }

        self.find_pvp_key(self_vid, target_vid) || self.find_gvg_key(self_guild, target_guild)
    }

    pub fn is_in_safe<T: TerrainAttributeProvider>(&self, pos: PixelPosition, terrain: &T) -> bool {
        terrain.is_attr_on(pos.x, pos.y, ATTRIBUTE_BANPK)
    }
    
    pub fn check_attacking<T: TerrainAttributeProvider>(&self, self_pos: PixelPosition, target_pos: PixelPosition, terrain: &T) -> bool {
        if self.is_in_safe(self_pos, terrain) {
            return false;
        }
        
        if self.is_in_safe(target_pos, terrain) {
            return false;
        }
        
        true
    }
    
    // Simulates CPythonNetworkStreamPhaseGame::RecvPVPPacket handling
    pub fn handle_pvp_packet_mode(&mut self, src_vid: u32, dst_vid: u32, mode: PvpMode) {
        match mode {
            PvpMode::Agree => {
                self.remove_pvp_key(src_vid, dst_vid);
            }
            PvpMode::Revenge => {
                self.remove_pvp_key(src_vid, dst_vid);
            }
            PvpMode::Fight => {
                self.insert_pvp_key(src_vid, dst_vid);
            }
            PvpMode::None => {
                self.remove_pvp_key(src_vid, dst_vid);
            }
        }
    }
}
