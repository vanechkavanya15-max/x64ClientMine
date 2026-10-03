use glam::Mat4;
use std::sync::{OnceLock, RwLock};

use crate::sash_wing_physics_v2::m2_sash_get_transform;

#[derive(Clone, Copy)]
pub struct SashAttachment {
    pub is_attached: bool,
    pub spine_bone_matrix: Mat4,
    pub local_offset_matrix: Mat4,
}

impl Default for SashAttachment {
    fn default() -> Self {
        Self {
            is_attached: false,
            spine_bone_matrix: Mat4::IDENTITY,
            local_offset_matrix: Mat4::IDENTITY,
        }
    }
}

impl SashAttachment {
    pub fn calculate_world_matrix(&self, physics_matrix: Mat4) -> Mat4 {
        // The order should typically be: spine_bone * physics * local_offset
        self.spine_bone_matrix * physics_matrix * self.local_offset_matrix
    }
}

#[derive(Default)]
pub struct SashWingAttachSystem {
    pub sash: SashAttachment,
}

impl SashWingAttachSystem {
    pub fn equip_sash(&mut self, spine_bone: Mat4, local_offset: Mat4) {
        self.sash.is_attached = true;
        self.sash.spine_bone_matrix = spine_bone;
        self.sash.local_offset_matrix = local_offset;
    }

    pub fn unequip_sash(&mut self) {
        self.sash.is_attached = false;
    }
}

pub static GLOBAL_SASH_ATTACH: OnceLock<RwLock<SashWingAttachSystem>> = OnceLock::new();

#[no_mangle]
pub extern "C" fn m2_sash_wing_equip(spine_bone_mat: *const f32, local_offset_mat: *const f32) -> bool {
    if spine_bone_mat.is_null() || local_offset_mat.is_null() {
        return false;
    }

    let spine_bone = unsafe { Mat4::from_cols_array(&*(spine_bone_mat as *const [f32; 16])) };
    let local_offset = unsafe { Mat4::from_cols_array(&*(local_offset_mat as *const [f32; 16])) };

    let state_lock = GLOBAL_SASH_ATTACH.get_or_init(|| RwLock::new(SashWingAttachSystem::default()));
    if let Ok(mut state) = state_lock.write() {
        state.equip_sash(spine_bone, local_offset);
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn m2_sash_wing_unequip() -> bool {
    let state_lock = GLOBAL_SASH_ATTACH.get_or_init(|| RwLock::new(SashWingAttachSystem::default()));
    if let Ok(mut state) = state_lock.write() {
        state.unequip_sash();
        true
    } else {
        false
    }
}

pub fn init_sash_attach_system() {
    GLOBAL_SASH_ATTACH.get_or_init(|| RwLock::new(SashWingAttachSystem::default()));
}

#[no_mangle]
pub extern "C" fn m2_sash_wing_get_transform(out_mat: *mut f32) -> bool {
    if out_mat.is_null() {
        return false;
    }

    let state_lock = GLOBAL_SASH_ATTACH.get_or_init(|| RwLock::new(SashWingAttachSystem::default()));
    let state = match state_lock.read() {
        Ok(guard) => guard,
        Err(_) => return false,
    };

    if !state.sash.is_attached {
        return false;
    }

    let mut physics_mat_array = [0.0f32; 16];
    let mut physics_matrix = Mat4::IDENTITY;

    if m2_sash_get_transform(physics_mat_array.as_mut_ptr()) {
        physics_matrix = Mat4::from_cols_array(&physics_mat_array);
    }

    let world_mat = state.sash.calculate_world_matrix(physics_matrix);

    unsafe {
        let out_slice = std::slice::from_raw_parts_mut(out_mat, 16);
        out_slice.copy_from_slice(&world_mat.to_cols_array());
    }

    true
}
