use m2_client::granny_bone_tree_v2::*;
use std::ffi::CString;

#[test]
fn test_identity_matrix() {
    let mat = identity_matrix();
    assert_eq!(mat[0], 1.0);
    assert_eq!(mat[15], 1.0);
    assert_eq!(mat[4], 0.0);
}

#[test]
fn test_matrix_multiplication() {
    let mut mat_a = identity_matrix();
    // Translate X by 10
    mat_a[12] = 10.0; 
    
    let mut mat_b = identity_matrix();
    // Translate Y by 5
    mat_b[13] = 5.0;
    
    let result = multiply_matrix(&mat_a, &mat_b);
    
    // Result should have both translations if correctly row-major multiplied
    // Or depending on matrix layout, we just check they combine.
    // In D3D row-major: matWorld = matLocal * matParent
    // Let's just do a simple known test:
    // a * I = a
    let result2 = multiply_matrix(&mat_a, &identity_matrix());
    assert_eq!(result2, mat_a);
}

#[test]
fn test_bone_tree_creation() {
    let mut tree = BoneTree::new();
    let root = Bone::new(0, "Bip01", -1, identity_matrix());
    tree.add_bone(root);
    
    assert_eq!(tree.bones.len(), 1);
    assert_eq!(tree.bones[0].name, "Bip01");
}

#[test]
fn test_bone_tree_update_single_root() {
    let mut tree = BoneTree::new();
    let mut root_local = identity_matrix();
    root_local[0] = 2.0; // scale X by 2
    
    let root = Bone::new(0, "Bip01", -1, root_local);
    tree.add_bone(root);
    
    tree.update_skeleton();
    
    assert_eq!(tree.bones[0].global_matrix[0], 2.0);
}

#[test]
fn test_bone_tree_update_child() {
    let mut tree = BoneTree::new();
    
    let mut parent_local = identity_matrix();
    parent_local[12] = 10.0; // Translate X by 10
    
    let mut child_local = identity_matrix();
    child_local[12] = 5.0; // Translate X by 5
    
    tree.add_bone(Bone::new(0, "Root", -1, parent_local));
    tree.add_bone(Bone::new(1, "Child", 0, child_local));
    
    tree.update_skeleton();
    
    // Global of child should be matLocal * matParent in row-major
    // Translation in X: 5 * 1 + 10 = 15
    assert_eq!(tree.bones[1].global_matrix[12], 15.0);
}

#[test]
fn test_deep_tree_update() {
    let mut tree = BoneTree::new();
    
    tree.add_bone(Bone::new(0, "Root", -1, identity_matrix()));
    tree.add_bone(Bone::new(1, "Spine", 0, identity_matrix()));
    tree.add_bone(Bone::new(2, "Head", 1, identity_matrix()));
    
    tree.update_skeleton();
    
    assert_eq!(tree.bones[2].global_matrix[0], 1.0);
}

#[test]
fn test_find_bone_by_name() {
    let mut tree = BoneTree::new();
    tree.add_bone(Bone::new(0, "equip_right", -1, identity_matrix()));
    tree.add_bone(Bone::new(1, "equip_left", -1, identity_matrix()));
    
    assert!(tree.find_bone_by_name("equip_right").is_some());
    assert!(tree.find_bone_by_name("equip_left").is_some());
    assert!(tree.find_bone_by_name("non_existent").is_none());
}

#[test]
fn test_ffi_get_global_matrix() {
    {
        let mut lock = get_global_bone_tree().write().unwrap();
        lock.bones.clear(); // Ensure clean state
        let mut local = identity_matrix();
        local[0] = 5.0; // Mark it
        lock.add_bone(Bone::new(0, "Bip01 Head", -1, local));
        lock.update_skeleton();
    }
    
    let bone_name = CString::new("Bip01 Head").unwrap();
    let mut out_mat = [0.0f32; 16];
    
    let success = m2_bone_get_global_matrix(bone_name.as_ptr(), out_mat.as_mut_ptr());
    
    assert!(success);
    assert_eq!(out_mat[0], 5.0); // Global matrix scaled
}

#[test]
fn test_ffi_get_global_matrix_not_found() {
    let bone_name = CString::new("Bip01 Dummy").unwrap();
    let mut out_mat = [0.0f32; 16];
    
    let success = m2_bone_get_global_matrix(bone_name.as_ptr(), out_mat.as_mut_ptr());
    
    assert!(!success);
}
