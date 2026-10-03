#[path = "../src/granny_bone_hierarchy.rs"]
mod granny_bone_hierarchy;

use granny_bone_hierarchy::Skeleton;
use glam::{Mat4, Vec3};

#[test]
fn test_normal_hierarchy() {
    let mut skeleton = Skeleton::new();
    
    // Root bone
    let root_idx = skeleton.add_bone(None, Mat4::from_translation(Vec3::new(1.0, 0.0, 0.0))).unwrap();
    
    // Child bone
    let child_idx = skeleton.add_bone(Some(root_idx), Mat4::from_translation(Vec3::new(0.0, 1.0, 0.0))).unwrap();
    
    // Grandchild bone
    let grandchild_idx = skeleton.add_bone(Some(child_idx), Mat4::from_translation(Vec3::new(0.0, 0.0, 1.0))).unwrap();
    
    let globals = skeleton.calculate_global_transforms().unwrap();
    
    assert_eq!(globals[root_idx].to_cols_array(), Mat4::from_translation(Vec3::new(1.0, 0.0, 0.0)).to_cols_array());
    assert_eq!(globals[child_idx].to_cols_array(), Mat4::from_translation(Vec3::new(1.0, 1.0, 0.0)).to_cols_array());
    assert_eq!(globals[grandchild_idx].to_cols_array(), Mat4::from_translation(Vec3::new(1.0, 1.0, 1.0)).to_cols_array());
}

#[test]
fn test_exceeding_64_bones() {
    let mut skeleton = Skeleton::new();
    for _ in 0..64 {
        skeleton.add_bone(None, Mat4::IDENTITY).unwrap();
    }
    
    let result = skeleton.add_bone(None, Mat4::IDENTITY);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Max 64 bones allowed");
}

#[test]
fn test_cycle_detection() {
    let mut skeleton = Skeleton::new();
    
    // This is tricky because add_bone requires parent_idx to be < bone_count
    // We can simulate a cycle by modifying the bones array directly (simulating corrupted state from outside or bypassing add_bone checks somehow if it was public, but here we'll just modify it)
    skeleton.add_bone(None, Mat4::IDENTITY).unwrap();
    skeleton.add_bone(Some(0), Mat4::IDENTITY).unwrap();
    
    // Force a cycle
    skeleton.bones[0].parent_index = Some(1);
    
    let result = skeleton.calculate_global_transforms();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Cycle detected in bone hierarchy");
}

#[test]
fn test_invalid_parent_index() {
    let mut skeleton = Skeleton::new();
    let result = skeleton.add_bone(Some(99), Mat4::IDENTITY);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Parent index out of bounds");
}

#[test]
fn test_independent_bones() {
    let mut skeleton = Skeleton::new();
    let root1 = skeleton.add_bone(None, Mat4::from_translation(Vec3::new(10.0, 0.0, 0.0))).unwrap();
    let root2 = skeleton.add_bone(None, Mat4::from_translation(Vec3::new(0.0, 20.0, 0.0))).unwrap();
    
    let globals = skeleton.calculate_global_transforms().unwrap();
    assert_eq!(globals[root1].to_cols_array(), Mat4::from_translation(Vec3::new(10.0, 0.0, 0.0)).to_cols_array());
    assert_eq!(globals[root2].to_cols_array(), Mat4::from_translation(Vec3::new(0.0, 20.0, 0.0)).to_cols_array());
}
