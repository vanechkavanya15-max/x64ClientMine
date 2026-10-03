use glam::{Mat4, Vec3};
use std::sync::Mutex;
use std::sync::OnceLock;

use m2_client::sash_wing_attachment_system::{
    init_sash_attach_system, m2_sash_wing_equip, m2_sash_wing_unequip, m2_sash_wing_get_transform, GLOBAL_SASH_ATTACH
};
use m2_client::sash_wing_physics_v2::{m2_sash_update, test_reset_sash_state};

static TEST_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

fn get_test_mutex() -> &'static Mutex<()> {
    TEST_MUTEX.get_or_init(|| Mutex::new(()))
}

fn reset_attachment_state() {
    init_sash_attach_system();
    let mut state = GLOBAL_SASH_ATTACH.get().unwrap().write().unwrap();
    state.unequip_sash();
    state.sash.spine_bone_matrix = Mat4::IDENTITY;
    state.sash.local_offset_matrix = Mat4::IDENTITY;
}

#[test]
fn test_init_sash_attach_system() {
    let _guard = get_test_mutex().lock().unwrap();
    reset_attachment_state();
    let state_lock = GLOBAL_SASH_ATTACH.get().unwrap();
    let state = state_lock.read().unwrap();
    assert!(!state.sash.is_attached);
    assert_eq!(state.sash.spine_bone_matrix, Mat4::IDENTITY);
    assert_eq!(state.sash.local_offset_matrix, Mat4::IDENTITY);
}

#[test]
fn test_equip_sash() {
    let _guard = get_test_mutex().lock().unwrap();
    reset_attachment_state();
    let spine_bone = Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0));
    let local_offset = Mat4::from_translation(Vec3::new(4.0, 5.0, 6.0));
    
    let result = m2_sash_wing_equip(
        spine_bone.to_cols_array().as_ptr(),
        local_offset.to_cols_array().as_ptr()
    );
    assert!(result);
    
    let state = GLOBAL_SASH_ATTACH.get().unwrap().read().unwrap();
    assert!(state.sash.is_attached);
    assert_eq!(state.sash.spine_bone_matrix, spine_bone);
    assert_eq!(state.sash.local_offset_matrix, local_offset);
}

#[test]
fn test_unequip_sash() {
    let _guard = get_test_mutex().lock().unwrap();
    reset_attachment_state();
    
    m2_sash_wing_equip(Mat4::IDENTITY.to_cols_array().as_ptr(), Mat4::IDENTITY.to_cols_array().as_ptr());
    
    {
        let state = GLOBAL_SASH_ATTACH.get().unwrap().read().unwrap();
        assert!(state.sash.is_attached);
    }
    
    let result = m2_sash_wing_unequip();
    assert!(result);
    
    let state = GLOBAL_SASH_ATTACH.get().unwrap().read().unwrap();
    assert!(!state.sash.is_attached);
}

#[test]
fn test_m2_sash_wing_get_transform_unequipped() {
    let _guard = get_test_mutex().lock().unwrap();
    reset_attachment_state();
    
    let mut out_mat = [0.0f32; 16];
    let result = m2_sash_wing_get_transform(out_mat.as_mut_ptr());
    assert!(!result);
}

#[test]
fn test_m2_sash_wing_get_transform_equipped_with_physics() {
    let _guard = get_test_mutex().lock().unwrap();
    reset_attachment_state();
    test_reset_sash_state();
    
    let spine_bone = Mat4::from_translation(Vec3::new(0.0, 10.0, 0.0));
    let local_offset = Mat4::from_translation(Vec3::new(0.0, 0.0, -5.0));
    
    m2_sash_wing_equip(
        spine_bone.to_cols_array().as_ptr(),
        local_offset.to_cols_array().as_ptr()
    );

    // Simulate physics movement to generate some pitch/roll
    m2_sash_update(0.0, 0.0, 0.0, 0.0, 16);
    m2_sash_update(10.0, 0.0, 0.0, 0.0, 16); // Move fast to generate accel
    
    let mut out_mat = [0.0f32; 16];
    let result = m2_sash_wing_get_transform(out_mat.as_mut_ptr());
    assert!(result);
    
    let final_mat = Mat4::from_cols_array(&out_mat);
    assert_ne!(final_mat, Mat4::IDENTITY);
    
    // Check if translation is affected by spine bone and local offset correctly
    // The exact matrix depends on the physics state, but we can verify it's not identity
    // and that the calculation executed without crashing
}

#[test]
fn test_m2_sash_wing_get_transform_null_ptr() {
    let result = m2_sash_wing_get_transform(std::ptr::null_mut());
    assert!(!result);
}
