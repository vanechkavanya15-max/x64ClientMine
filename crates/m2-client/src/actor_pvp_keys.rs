use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PVPKey {
    pub vid_src: u32,
    pub vid_dst: u32,
}

impl PVPKey {
    pub fn new(vid_src: u32, vid_dst: u32) -> Self {
        Self { vid_src, vid_dst }
    }
}

pub struct ActorPvpManager {
    active_pvp_keys: HashSet<(u32, u32)>,
}

impl ActorPvpManager {
    pub fn new() -> Self {
        Self {
            active_pvp_keys: HashSet::new(),
        }
    }

    pub fn insert_pvp_key(&mut self, src: u32, dst: u32) {
        self.active_pvp_keys.insert((src, dst));
    }

    pub fn remove_pvp_key(&mut self, src: u32, dst: u32) {
        self.active_pvp_keys.remove(&(src, dst));
    }

    pub fn is_pvp_active(&self, vid1: u32, vid2: u32) -> bool {
        self.active_pvp_keys.contains(&(vid1, vid2)) || self.active_pvp_keys.contains(&(vid2, vid1))
    }
}
