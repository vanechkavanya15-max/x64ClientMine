use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

/// Nazwa kosci do ktorej przypinane sa obiekty typu wlosy i nakrycia glowy.
pub const BONE_HEAD_NAME: &str = "Bip01 Head";

#[derive(Clone, Debug)]
pub struct HeadAttachState {
    pub hair_vnum: u32,
    pub head_vnum: u32,
    pub bone_transform: [f32; 16],
}

impl Default for HeadAttachState {
    fn default() -> Self {
        Self {
            hair_vnum: 0,
            head_vnum: 0,
            bone_transform: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }
}

impl HeadAttachState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_hair(&mut self, vnum: u32) {
        self.hair_vnum = vnum;
    }

    pub fn set_head(&mut self, vnum: u32) {
        self.head_vnum = vnum;
    }

    pub fn is_anything_attached(&self) -> bool {
        self.hair_vnum != 0 || self.head_vnum != 0
    }

    pub fn update_transform(&mut self, mat: &[f32; 16]) {
        self.bone_transform.copy_from_slice(mat);
    }
}

static HEAD_STATES: OnceLock<RwLock<HashMap<u32, HeadAttachState>>> = OnceLock::new();

pub fn get_head_states() -> &'static RwLock<HashMap<u32, HeadAttachState>> {
    HEAD_STATES.get_or_init(|| RwLock::new(HashMap::new()))
}

/// Pobiera aktualna macierz transformacji dla kosci "Bip01 Head" gracza (VID 0).
/// Wymagane przez specyfikacje zadania: `m2_head_get_transform(out_mat: *mut f32) -> bool`
#[no_mangle]
pub extern "C" fn m2_head_get_transform(out_mat: *mut f32) -> bool {
    if out_mat.is_null() {
        return false;
    }

    let states = get_head_states().read().unwrap();
    if let Some(state) = states.get(&0) {
        if state.is_anything_attached() {
            unsafe {
                std::ptr::copy_nonoverlapping(state.bone_transform.as_ptr(), out_mat, 16);
            }
            return true;
        }
    }

    false
}

#[no_mangle]
pub extern "C" fn m2_head_set_hair(vid: u32, vnum: u32) -> bool {
    let mut states = get_head_states().write().unwrap();
    let state = states.entry(vid).or_insert_with(HeadAttachState::new);
    state.set_hair(vnum);
    true
}

#[no_mangle]
pub extern "C" fn m2_head_set_head(vid: u32, vnum: u32) -> bool {
    let mut states = get_head_states().write().unwrap();
    let state = states.entry(vid).or_insert_with(HeadAttachState::new);
    state.set_head(vnum);
    true
}

#[no_mangle]
pub extern "C" fn m2_head_update_transform(vid: u32, transform: *const f32) -> bool {
    if transform.is_null() {
        return false;
    }

    let mut states = get_head_states().write().unwrap();
    let state = states.entry(vid).or_insert_with(HeadAttachState::new);
    let mat = unsafe { &*(transform as *const [f32; 16]) };
    state.update_transform(mat);
    true
}

#[no_mangle]
pub extern "C" fn m2_head_clear_state(vid: u32) {
    let mut states = get_head_states().write().unwrap();
    states.remove(&vid);
}
