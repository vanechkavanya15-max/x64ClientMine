//! Equipment gem socket engine (Kamienie Dusz): inserting Metin stones, broken shard failure handling, and socket cleaning.

use std::sync::{Mutex, OnceLock};
use m2_client_bridge::ffi_guard::safe_ffi_call;

pub const ITEM_SOCKET_MAX_NUM: usize = 3;
pub const ITEM_BROKEN_METIN_VNUM: u32 = 28960; // Broken stone shard

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketInsertionResult {
    Success,
    BrokenShard,
    SocketAlreadyFilled,
    InvalidSocketIndex,
}

#[derive(Debug, Clone)]
pub struct EquipmentSocketState {
    pub sockets: [u32; ITEM_SOCKET_MAX_NUM],
}

impl Default for EquipmentSocketState {
    fn default() -> Self {
        Self {
            sockets: [0; ITEM_SOCKET_MAX_NUM],
        }
    }
}

impl EquipmentSocketState {
    pub fn clear(&mut self) {
        self.sockets = [0; ITEM_SOCKET_MAX_NUM];
    }

    pub fn insert_metin(
        &mut self,
        socket_idx: usize,
        metin_vnum: u32,
        success_rate_pct: u8,
        seed: u64,
    ) -> SocketInsertionResult {
        if socket_idx >= ITEM_SOCKET_MAX_NUM {
            return SocketInsertionResult::InvalidSocketIndex;
        }
        if self.sockets[socket_idx] != 0 {
            return SocketInsertionResult::SocketAlreadyFilled;
        }

        // Deterministic or pseudo-random roll: 0..99
        let roll = (seed % 100) as u8;
        if roll < success_rate_pct.min(100) {
            self.sockets[socket_idx] = metin_vnum;
            SocketInsertionResult::Success
        } else {
            self.sockets[socket_idx] = ITEM_BROKEN_METIN_VNUM;
            SocketInsertionResult::BrokenShard
        }
    }

    pub fn clean_shards(&mut self) -> u8 {
        let mut cleaned = 0;
        for s in &mut self.sockets {
            if *s == ITEM_BROKEN_METIN_VNUM {
                *s = 0;
                cleaned += 1;
            }
        }
        cleaned
    }

    pub fn get_socket(&self, socket_idx: usize) -> u32 {
        if socket_idx < ITEM_SOCKET_MAX_NUM {
            self.sockets[socket_idx]
        } else {
            0
        }
    }
}

static SOCKET_STATE: OnceLock<Mutex<EquipmentSocketState>> = OnceLock::new();

pub fn get_equipment_socket_state() -> &'static Mutex<EquipmentSocketState> {
    SOCKET_STATE.get_or_init(|| Mutex::new(EquipmentSocketState::default()))
}

// C-FFI exports
#[no_mangle]
pub extern "C" fn m2_equipment_socket_insert_metin(
    socket_idx: u8,
    metin_vnum: u32,
    success_pct: u8,
) -> u8 {
    safe_ffi_call(0, || {
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(42);

        let mut state = get_equipment_socket_state().lock().unwrap();
        match state.insert_metin(socket_idx as usize, metin_vnum, success_pct, seed) {
            SocketInsertionResult::Success => 1,
            SocketInsertionResult::BrokenShard => 2,
            SocketInsertionResult::SocketAlreadyFilled => 3,
            SocketInsertionResult::InvalidSocketIndex => 0,
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_equipment_socket_get(socket_idx: u8) -> u32 {
    safe_ffi_call(0, || {
        let state = get_equipment_socket_state().lock().unwrap();
        state.get_socket(socket_idx as usize)
    })
}

#[no_mangle]
pub extern "C" fn m2_equipment_socket_set(socket_idx: u8, vnum: u32) -> bool {
    safe_ffi_call(false, || {
        let mut state = get_equipment_socket_state().lock().unwrap();
        if (socket_idx as usize) < ITEM_SOCKET_MAX_NUM {
            state.sockets[socket_idx as usize] = vnum;
            true
        } else {
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_equipment_socket_clean_shards() -> u8 {
    safe_ffi_call(0, || {
        let mut state = get_equipment_socket_state().lock().unwrap();
        state.clean_shards()
    })
}

#[no_mangle]
pub extern "C" fn m2_equipment_socket_clear() {
    safe_ffi_call((), || {
        let mut state = get_equipment_socket_state().lock().unwrap();
        state.clear();
    })
}
