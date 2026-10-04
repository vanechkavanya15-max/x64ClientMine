use glam::{Mat4, Vec3, Quat};
use m2_client::granny_bone_attachment_hierarchy::*;

#[test]
fn test_bone_names_constants() {
    assert_eq!(BONE_EQUIP_RIGHT, "equip_right");
    assert_eq!(BONE_EQUIP_LEFT, "equip_left");
    assert_eq!(BONE_BIP01_HEAD, "Bip01 Head");
    assert_eq!(BONE_BIP01_SPINE2, "Bip01 Spine2");
}

#[test]
fn test_attach_and_detach_model() {
    let mut hierarchy = GrannyBoneAttachmentHierarchy::new();
    
    let offset = Mat4::from_translation(Vec3::new(1.0, 0.0, 0.0));
    let slot1 = hierarchy.attach_model(BONE_EQUIP_RIGHT, 100, offset);
    let slot2 = hierarchy.attach_model(BONE_BIP01_HEAD, 200, offset);
    
    assert_ne!(slot1, slot2);
    
    let attachment1 = hierarchy.get_attachment(slot1).unwrap();
    assert_eq!(attachment1.bone_name, BONE_EQUIP_RIGHT);
    assert_eq!(attachment1.child_model_id, 100);
    assert_eq!(attachment1.offset, offset);
    
    // Detach slot 1
    assert!(hierarchy.detach_model(slot1));
    assert!(hierarchy.get_attachment(slot1).is_none());
    
    // Detach slot 2
    assert!(hierarchy.detach_model(slot2));
    assert!(hierarchy.get_attachment(slot2).is_none());
    
    // Attempt detach invalid slot
    assert!(!hierarchy.detach_model(9999));
}

#[test]
fn test_bone_world_matrix() {
    let mut hierarchy = GrannyBoneAttachmentHierarchy::new();
    
    let mat = Mat4::from_translation(Vec3::new(10.0, 20.0, 30.0));
    hierarchy.update_bone_matrix(BONE_EQUIP_LEFT, mat);
    
    let retrieved_mat = hierarchy.get_bone_world_matrix(BONE_EQUIP_LEFT).unwrap();
    assert_eq!(retrieved_mat, mat);
    
    // Missing bone
    assert!(hierarchy.get_bone_world_matrix("non_existent").is_none());
}

#[test]
fn test_attachment_world_matrix_calculation() {
    let mut hierarchy = GrannyBoneAttachmentHierarchy::new();
    
    // 1. Parent bone matrix (e.g., Spine2)
    let parent_pos = Vec3::new(0.0, 100.0, 0.0);
    // 90 degrees rotation around Y axis
    let parent_rot = Quat::from_rotation_y(std::f32::consts::PI / 2.0);
    let parent_matrix = Mat4::from_rotation_translation(parent_rot, parent_pos);
    
    hierarchy.update_bone_matrix(BONE_BIP01_SPINE2, parent_matrix);
    
    // 2. Child local offset matrix
    let child_local_pos = Vec3::new(10.0, 0.0, 0.0);
    let child_local_matrix = Mat4::from_translation(child_local_pos);
    
    // Attach
    let slot = hierarchy.attach_model(BONE_BIP01_SPINE2, 300, child_local_matrix);
    
    // 3. Get expected calculated matrix
    // WorldMatrix = ParentBoneMatrix * ChildLocalOffsetMatrix
    let calculated_world_matrix = hierarchy.get_attachment_world_matrix(slot).unwrap();
    
    // We expect the translation to be transformed by the parent's matrix
    // (0, 100, 0) + rotate_Y_90(10, 0, 0) = (0, 100, 0) + (0, 0, -10) = (0, 100, -10)
    let expected_matrix = parent_matrix * child_local_matrix;
    
    // Compare 4x4 matrices elements directly with a small epsilon
    for i in 0..16 {
        assert!(
            (calculated_world_matrix.to_cols_array()[i] - expected_matrix.to_cols_array()[i]).abs() < 1e-5
        );
    }
}

#[test]
fn test_get_attachment_world_matrix_missing_bone() {
    let mut hierarchy = GrannyBoneAttachmentHierarchy::new();
    let offset = Mat4::IDENTITY;
    
    // Attach to a bone that hasn't been updated with a matrix yet
    let slot = hierarchy.attach_model("unknown_bone", 123, offset);
    
    // Should return None because the parent bone matrix isn't present
    assert!(hierarchy.get_attachment_world_matrix(slot).is_none());
}
