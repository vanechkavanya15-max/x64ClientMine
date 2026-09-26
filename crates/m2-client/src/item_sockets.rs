pub const SOCKET_EMPTY: i32 = 0;
pub const SOCKET_BROKEN: i32 = 1;

pub struct ItemSockets {
    pub sockets: [i32; 3],
}

impl ItemSockets {
    pub fn new() -> Self {
        Self {
            sockets: [SOCKET_EMPTY; 3],
        }
    }

    pub fn insert_socket(&mut self, slot_idx: usize, socket_vnum: i32) -> bool {
        if slot_idx >= self.sockets.len() {
            return false;
        }
        if self.sockets[slot_idx] != SOCKET_EMPTY {
            return false;
        }
        if socket_vnum != SOCKET_BROKEN && socket_vnum <= 1000 {
            return false;
        }
        
        self.sockets[slot_idx] = socket_vnum;
        true
    }

    pub fn clean_broken_sockets(&mut self) -> bool {
        let mut cleaned = false;
        for i in 0..self.sockets.len() {
            if self.sockets[i] == SOCKET_BROKEN {
                self.sockets[i] = SOCKET_EMPTY;
                cleaned = true;
            }
        }
        cleaned
    }
}

impl Default for ItemSockets {
    fn default() -> Self {
        Self::new()
    }
}
