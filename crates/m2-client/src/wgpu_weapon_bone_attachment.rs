use glam::Mat4;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponType {
    Sword = 0,
    Dagger = 1,
    Bow = 2,
    TwoHanded = 3,
    Bell = 4,
    Fan = 5,
    Arrow = 6,
    Mount = 7,
    Shield = 8,
    Hair = 9,
    Helmet = 10,
    Sash = 11,
    None = 99,
}

pub fn is_left_hand_weapon(weapon_type: WeaponType, is_mounting: bool) -> bool {
    match weapon_type {
        WeaponType::Dagger => true,
        WeaponType::Fan if is_mounting => true,
        WeaponType::Bow => true,
        _ => false,
    }
}

pub fn is_right_hand_weapon(weapon_type: WeaponType, is_mounting: bool) -> bool {
    match weapon_type {
        WeaponType::Dagger => true,
        WeaponType::Fan if is_mounting => true,
        WeaponType::Bow => false,
        WeaponType::Shield => false,
        WeaponType::Hair => false,
        WeaponType::Helmet => false,
        WeaponType::Sash => false,
        WeaponType::Mount => false,
        WeaponType::None => false,
        _ => true,
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttachmentBone {
    EquipRight, // 'equip_right'
    EquipLeft,  // 'equip_left'
    Head,       // 'Bip01 Head'
    Spine2,     // 'Bip01 Spine2'
}

impl AttachmentBone {
    pub fn name(&self) -> &'static str {
        match self {
            AttachmentBone::EquipRight => "equip_right",
            AttachmentBone::EquipLeft => "equip_left",
            AttachmentBone::Head => "Bip01 Head",
            AttachmentBone::Spine2 => "Bip01 Spine2",
        }
    }
}

pub fn get_weapon_attachment_bones(
    weapon_type: WeaponType,
    is_mounting: bool,
) -> Vec<AttachmentBone> {
    let mut bones = Vec::new();

    match weapon_type {
        // Sword, TwoHanded, Bell, Arrow -> Right Hand
        WeaponType::Sword | WeaponType::TwoHanded | WeaponType::Bell | WeaponType::Arrow => {
            bones.push(AttachmentBone::EquipRight);
        }

        // Bow -> Left Hand
        WeaponType::Bow => {
            bones.push(AttachmentBone::EquipLeft);
        }

        // Dagger -> Both Hands
        WeaponType::Dagger => {
            bones.push(AttachmentBone::EquipRight);
            bones.push(AttachmentBone::EquipLeft);
        }

        // Fan -> Left hand usually, but Right hand if mounting
        WeaponType::Fan => {
            if is_mounting {
                bones.push(AttachmentBone::EquipRight);
                bones.push(AttachmentBone::EquipLeft);
            } else {
                bones.push(AttachmentBone::EquipRight);
            }
        }

        // Shield -> Left Hand
        WeaponType::Shield => {
            bones.push(AttachmentBone::EquipLeft);
        }

        // Hair, Helmet -> Head
        WeaponType::Hair | WeaponType::Helmet => {
            bones.push(AttachmentBone::Head);
        }

        // Sash -> Spine2
        WeaponType::Sash => {
            bones.push(AttachmentBone::Spine2);
        }

        WeaponType::Mount | WeaponType::None => {}
    }

    bones
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoneAttachment {
    pub parent_bone_matrix: Mat4,
    pub socket_offset_matrix: Mat4,
}

impl BoneAttachment {
    pub fn new(parent_bone_matrix: Mat4, socket_offset_matrix: Mat4) -> Self {
        Self {
            parent_bone_matrix,
            socket_offset_matrix,
        }
    }

    /// Calculates the final world matrix of the attached object.
    /// WorldMatrix = ParentBoneMatrix * SocketOffsetMatrix
    pub fn calculate_world_matrix(&self) -> Mat4 {
        self.parent_bone_matrix * self.socket_offset_matrix
    }
}