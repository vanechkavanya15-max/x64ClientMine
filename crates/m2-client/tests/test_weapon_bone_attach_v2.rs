use m2_client::weapon_bone_attach_v2::*;
use glam::Mat4;
use std::sync::{OnceLock, RwLock};

#[test]
fn test_weapon_hand_logic() {
    // SWORD: right hand only
    assert!(is_right_hand_weapon(WEAPON_SWORD, false));
    assert!(!is_left_hand_weapon(WEAPON_SWORD, false));

    // BOW: left hand only
    assert!(!is_right_hand_weapon(WEAPON_BOW, false));
    assert!(is_left_hand_weapon(WEAPON_BOW, false));

    // DAGGER: both hands
    assert!(is_right_hand_weapon(WEAPON_DAGGER, false));
    assert!(is_left_hand_weapon(WEAPON_DAGGER, false));

    // FAN: right hand only on foot, both hands on mount
    assert!(is_right_hand_weapon(WEAPON_FAN, false));
    assert!(!is_left_hand_weapon(WEAPON_FAN, false));
    assert!(is_right_hand_weapon(WEAPON_FAN, true));
    assert!(is_left_hand_weapon(WEAPON_FAN, true));
}

#[test]
fn test_weapon_attachment_world_matrix() {
    let mut attachment = WeaponAttachment::default();
    
    // Identity test
    let result = attachment.calculate_world_matrix();
    assert_eq!(result, Mat4::IDENTITY);
    
    // Translation test
    attachment.bone_matrix = Mat4::from_translation(glam::Vec3::new(1.0, 2.0, 3.0));
    attachment.offset_matrix = Mat4::from_translation(glam::Vec3::new(4.0, 5.0, 6.0));
    
    let result = attachment.calculate_world_matrix();
    let expected = Mat4::from_translation(glam::Vec3::new(5.0, 7.0, 9.0));
    assert_eq!(result, expected);
}

#[test]
fn test_weapon_bone_attach_equip() {
    let mut attach = WeaponBoneAttach::default();
    
    let bone_mat = Mat4::from_translation(glam::Vec3::new(1.0, 0.0, 0.0));
    let offset_mat = Mat4::from_translation(glam::Vec3::new(0.0, 1.0, 0.0));
    
    // Test dagger (dual wield)
    attach.equip_weapon(WEAPON_DAGGER, false, bone_mat, offset_mat, bone_mat, offset_mat);
    assert!(attach.right_hand.is_attached);
    assert!(attach.left_hand.is_attached);
    
    // Test sword (right hand only)
    attach.equip_weapon(WEAPON_SWORD, false, bone_mat, offset_mat, bone_mat, offset_mat);
    assert!(attach.right_hand.is_attached);
    assert!(!attach.left_hand.is_attached);
    
    // Test bow (left hand only)
    attach.equip_weapon(WEAPON_BOW, false, bone_mat, offset_mat, bone_mat, offset_mat);
    assert!(!attach.right_hand.is_attached);
    assert!(attach.left_hand.is_attached);
}

#[test]
fn test_ffi_m2_weapon_get_transform_null_pointer() {
    // Null pointer should return false
    let result = m2_weapon_get_transform(HAND_RIGHT, std::ptr::null_mut());
    assert!(!result);
}

#[test]
fn test_ffi_m2_weapon_get_transform_invalid_hand() {
    let mut out_mat = [0.0f32; 16];
    // Invalid hand index should return false
    let result = m2_weapon_get_transform(99, out_mat.as_mut_ptr());
    assert!(!result);
}

#[test]
fn test_ffi_m2_weapon_get_transform_valid() {
    // Initialize system
    init_weapon_attach_system();
    
    // Setup state
    if let Ok(mut state) = GLOBAL_WEAPON_ATTACH.get().unwrap().write() {
        let bone_mat = Mat4::from_translation(glam::Vec3::new(1.0, 0.0, 0.0));
        let offset_mat = Mat4::from_translation(glam::Vec3::new(0.0, 1.0, 0.0));
        state.equip_right(bone_mat, offset_mat);
    }
    
    let mut out_mat = [0.0f32; 16];
    
    // Read attached right hand
    let result = m2_weapon_get_transform(HAND_RIGHT, out_mat.as_mut_ptr());
    assert!(result);
    
    let expected = Mat4::from_translation(glam::Vec3::new(1.0, 1.0, 0.0)).to_cols_array();
    assert_eq!(out_mat, expected);
    
    // Read unattached left hand (should return false)
    let mut out_mat_left = [0.0f32; 16];
    let result_left = m2_weapon_get_transform(HAND_LEFT, out_mat_left.as_mut_ptr());
    assert!(!result_left);
}
