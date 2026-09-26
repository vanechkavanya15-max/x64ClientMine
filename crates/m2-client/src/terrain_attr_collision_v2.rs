use std::sync::{Mutex, OnceLock};
use m2_client_bridge::ffi_guard::safe_ffi_call;
use std::slice;

pub const FLAG_BLOCKED: u8 = 1 << 0;
pub const FLAG_WATER: u8 = 1 << 1;
pub const FLAG_SAFE_ZONE: u8 = 1 << 2;
pub const FLAG_CANT_BUILD: u8 = 1 << 3;

pub const TERRAIN_SECTOR_SIZE: usize = 128;
pub const EXPECTED_GRID_LEN: usize = TERRAIN_SECTOR_SIZE * TERRAIN_SECTOR_SIZE;

pub struct TerrainSectorAttr {
    attr_grid: Vec<u8>,
}

impl TerrainSectorAttr {
    pub fn new(attr_grid: Vec<u8>) -> Result<Self, &'static str> {
        if attr_grid.len() != EXPECTED_GRID_LEN {
            return Err("Invalid grid size for TerrainSectorAttr (must be 128x128)");
        }
        Ok(Self { attr_grid })
    }

    pub fn get_attribute(&self, x: usize, y: usize) -> u8 {
        if x >= TERRAIN_SECTOR_SIZE || y >= TERRAIN_SECTOR_SIZE {
            return FLAG_BLOCKED; // traktujemy out of bounds jako sciane
        }
        self.attr_grid[y * TERRAIN_SECTOR_SIZE + x]
    }

    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        let attr = self.get_attribute(x, y);
        (attr & (FLAG_BLOCKED | FLAG_WATER)) == 0
    }
}

static TERRAIN_ATTR_STATE: OnceLock<Mutex<Option<TerrainSectorAttr>>> = OnceLock::new();

pub fn get_terrain_attr_state() -> &'static Mutex<Option<TerrainSectorAttr>> {
    TERRAIN_ATTR_STATE.get_or_init(|| Mutex::new(None))
}

#[no_mangle]
pub extern "C" fn m2_terrain_get_attr(x: f32, y: f32) -> u8 {
    safe_ffi_call(FLAG_BLOCKED, || {
        if x < 0.0 || y < 0.0 {
            return FLAG_BLOCKED;
        }
        let px = x.floor() as usize;
        let py = y.floor() as usize;

        if let Ok(guard) = get_terrain_attr_state().lock() {
            if let Some(state) = &*guard {
                return state.get_attribute(px, py);
            }
        }
        FLAG_BLOCKED
    })
}

#[no_mangle]
pub extern "C" fn m2_terrain_is_walkable_v2(x: f32, y: f32) -> bool {
    safe_ffi_call(false, || {
        if x < 0.0 || y < 0.0 {
            return false;
        }
        let px = x.floor() as usize;
        let py = y.floor() as usize;

        if let Ok(guard) = get_terrain_attr_state().lock() {
            if let Some(state) = &*guard {
                return state.is_walkable(px, py);
            }
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_terrain_load_attr_grid(data_ptr: *const u8, len: usize) -> bool {
    safe_ffi_call(false, || {
        if data_ptr.is_null() || len != EXPECTED_GRID_LEN {
            return false;
        }

        let slice = unsafe { slice::from_raw_parts(data_ptr, len) };
        let mut vec = Vec::with_capacity(slice.len());
        vec.extend_from_slice(slice);

        if let Ok(map) = TerrainSectorAttr::new(vec) {
            if let Ok(mut guard) = get_terrain_attr_state().lock() {
                *guard = Some(map);
                return true;
            }
        }
        false
    })
}
