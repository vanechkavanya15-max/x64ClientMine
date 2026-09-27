use glam::Mat4;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AttachmentPoint {
    WeaponRight,
    WeaponLeft,
    Shield,
    Hair,
    Sash,
    Bow,
    Arrow,
}

impl AttachmentPoint {
    pub fn default_bone_name(&self) -> &'static str {
        match self {
            AttachmentPoint::WeaponRight => "equip_right",
            AttachmentPoint::WeaponLeft => "equip_left",
            AttachmentPoint::Shield => "equip_left",
            AttachmentPoint::Hair => "hair",
            AttachmentPoint::Sash => "spine",
            AttachmentPoint::Bow => "equip_left",
            AttachmentPoint::Arrow => "equip_right",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AttachmentItem {
    pub point: AttachmentPoint,
    pub bone_name: String,
    pub model_id: u32,
    pub offset_transform: Mat4,
}

impl AttachmentItem {
    pub fn new(point: AttachmentPoint, bone_name: String, model_id: u32, offset_transform: Mat4) -> Self {
        Self {
            point,
            bone_name,
            model_id,
            offset_transform,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct AttachmentManager {
    items: HashMap<AttachmentPoint, AttachmentItem>,
}

impl AttachmentManager {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn attach_item(&mut self, point: AttachmentPoint, item: AttachmentItem) {
        let mut attached_item = item;
        attached_item.point = point;
        self.items.insert(point, attached_item);
    }

    pub fn detach_item(&mut self, point: AttachmentPoint) -> Result<(), &'static str> {
        if self.items.remove(&point).is_some() {
            Ok(())
        } else {
            Err("Attachment point is empty")
        }
    }
    
    pub fn get_item(&self, point: AttachmentPoint) -> Option<&AttachmentItem> {
        self.items.get(&point)
    }

    pub fn compute_attached_world_matrix(item: &AttachmentItem, parent_bone_matrix: Mat4) -> Mat4 {
        parent_bone_matrix * item.offset_transform
    }
}
