use glam::Mat4;

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
pub struct WeaponBoneAttachmentMgr {
    pub right_hand: WeaponAttachment,
    pub left_hand: WeaponAttachment,
}

impl WeaponBoneAttachmentMgr {
    pub fn new() -> Self {
        Self::default()
    }

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

    pub fn unequip(&mut self) {
        self.right_hand.is_attached = false;
        self.left_hand.is_attached = false;
    }
    
    pub fn attach_weapon(&mut self, weapon_type: u32, is_mounting: bool, right_bone: Mat4, right_offset: Mat4, left_bone: Mat4, left_offset: Mat4) {
        self.unequip();
        
        if is_right_hand_weapon(weapon_type, is_mounting) {
            self.equip_right(right_bone, right_offset);
        }
        if is_left_hand_weapon(weapon_type, is_mounting) {
            self.equip_left(left_bone, left_offset);
        }
    }
    
    pub fn get_transform(&self, hand: u8) -> Option<Mat4> {
        match hand {
            HAND_RIGHT => {
                if self.right_hand.is_attached {
                    Some(self.right_hand.calculate_world_matrix())
                } else {
                    None
                }
            },
            HAND_LEFT => {
                if self.left_hand.is_attached {
                    Some(self.left_hand.calculate_world_matrix())
                } else {
                    None
                }
            },
            _ => None,
        }
    }
}
