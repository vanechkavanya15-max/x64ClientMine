use glam::Mat4;
use std::collections::HashMap;

pub const BONE_EQUIP_RIGHT: &str = "equip_right";
pub const BONE_EQUIP_LEFT: &str = "equip_left";
pub const BONE_BIP01_HEAD: &str = "Bip01 Head";
pub const BONE_BIP01_SPINE2: &str = "Bip01 Spine2";

#[derive(Debug, Clone)]
pub struct Attachment {
    pub bone_name: String,
    pub child_model_id: u32,
    pub offset: Mat4,
}

#[derive(Debug, Default)]
pub struct GrannyBoneAttachmentHierarchy {
    bone_matrices: HashMap<String, Mat4>,
    attachments: HashMap<u32, Attachment>,
    next_slot_id: u32,
}

impl GrannyBoneAttachmentHierarchy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn attach_model(&mut self, bone_name: &str, child_model_id: u32, offset: Mat4) -> u32 {
        let slot = self.next_slot_id;
        self.next_slot_id += 1;

        self.attachments.insert(
            slot,
            Attachment {
                bone_name: bone_name.to_string(),
                child_model_id,
                offset,
            },
        );

        slot
    }

    pub fn detach_model(&mut self, slot: u32) -> bool {
        self.attachments.remove(&slot).is_some()
    }

    pub fn get_bone_world_matrix(&self, bone_name: &str) -> Option<Mat4> {
        self.bone_matrices.get(bone_name).copied()
    }

    pub fn update_bone_matrix(&mut self, bone_name: &str, matrix: Mat4) {
        self.bone_matrices.insert(bone_name.to_string(), matrix);
    }

    pub fn get_attachment_world_matrix(&self, slot: u32) -> Option<Mat4> {
        if let Some(attachment) = self.attachments.get(&slot) {
            if let Some(bone_matrix) = self.get_bone_world_matrix(&attachment.bone_name) {
                return Some(bone_matrix * attachment.offset);
            }
        }
        None
    }

    pub fn get_attachment(&self, slot: u32) -> Option<&Attachment> {
        self.attachments.get(&slot)
    }
}
