use glam::Mat4;
use std::sync::{OnceLock, RwLock};

pub const WEAPON_SWORD: u32 = 0;
pub const WEAPON_DAGGER: u32 = 1;
pub const WEAPON_BOW: u32 = 2;
pub const WEAPON_TWO_HANDED: u32 = 3;
pub const WEAPON_BELL: u32 = 4;
pub const WEAPON_FAN: u32 = 5;
pub const WEAPON_ARROW: u32 = 6;
pub const WEAPON_NUM_TYPES: u32 = 7;
pub const WEAPON_NONE: u32 = 8;

pub const HAND_RIGHT: u8 = 0;
pub const HAND_LEFT: u8 = 1;

pub fn is_left_hand_weapon(weapon_type: u32, is_mounting: bool) -> bool {
    if weapon_type == WEAPON_DAGGER || (weapon_type == WEAPON_FAN && is_mounting) {
        true
    } else if weapon_type == WEAPON_BOW {
        true
    } else {
        false
    }
}

pub fn is_right_hand_weapon(weapon_type: u32, is_mounting: bool) -> bool {
    if weapon_type == WEAPON_DAGGER || (weapon_type == WEAPON_FAN && is_mounting) {
        true
    } else if weapon_type == WEAPON_BOW {
        false
    } else {
        true
    }
}

#[derive(Clone, Copy)]
pub struct WeaponAttachment {
    pub is_attached: bool,
    pub bone_matrix: Mat4,
    pub offset_matrix: Mat4,
}

impl Default for WeaponAttachment {
    fn default() -> Self {
        Self {
            is_attached: false,
            bone_matrix: Mat4::IDENTITY,
            offset_matrix: Mat4::IDENTITY,
        }
    }
}

impl WeaponAttachment {
    pub fn calculate_world_matrix(&self) -> Mat4 {
        self.bone_matrix * self.offset_matrix
    }
}

#[derive(Default)]
pub struct WeaponBoneAttach {
    pub right_hand: WeaponAttachment,
    pub left_hand: WeaponAttachment,
}

impl WeaponBoneAttach {
    pub fn equip_right(&mut self, bone_mat: Mat4, offset_mat: Mat4) {
        self.right_hand.is_attached = true;
        self.right_hand.bone_matrix = bone_mat;
        self.right_hand.offset_matrix = offset_mat;
    }

    pub fn equip_left(&mut self, bone_mat: Mat4, offset_mat: Mat4) {
        self.left_hand.is_attached = true;
        self.left_hand.bone_matrix = bone_mat;
        self.left_hand.offset_matrix = offset_mat;
    }
    
    pub fn equip_weapon(&mut self, weapon_type: u32, is_mounting: bool, right_bone: Mat4, right_offset: Mat4, left_bone: Mat4, left_offset: Mat4) {
        self.right_hand.is_attached = false;
        self.left_hand.is_attached = false;
        
        if is_right_hand_weapon(weapon_type, is_mounting) {
            self.equip_right(right_bone, right_offset);
        }
        if is_left_hand_weapon(weapon_type, is_mounting) {
            self.equip_left(left_bone, left_offset);
        }
    }
}

pub static GLOBAL_WEAPON_ATTACH: OnceLock<RwLock<WeaponBoneAttach>> = OnceLock::new();

pub fn init_weapon_attach_system() {
    GLOBAL_WEAPON_ATTACH.get_or_init(|| RwLock::new(WeaponBoneAttach::default()));
}

#[no_mangle]
pub extern "C" fn m2_weapon_get_transform(hand: u8, out_mat: *mut f32) -> bool {
    if out_mat.is_null() {
        return false;
    }

    let state_lock = GLOBAL_WEAPON_ATTACH.get_or_init(|| RwLock::new(WeaponBoneAttach::default()));
    let state = match state_lock.read() {
        Ok(guard) => guard,
        Err(_) => return false,
    };

    let attachment = match hand {
        HAND_RIGHT => &state.right_hand,
        HAND_LEFT => &state.left_hand,
        _ => return false,
    };

    if !attachment.is_attached {
        return false;
    }

    let world_mat = attachment.calculate_world_matrix();

    unsafe {
        let out_slice = std::slice::from_raw_parts_mut(out_mat, 16);
        out_slice.copy_from_slice(&world_mat.to_cols_array());
    }

    true
}
