use std::sync::Mutex;
use m2_client::event_effect_attach_v2::{
    get_effect_manager, m2_motion_get_attached_effects, m2_motion_process_effect_event,
    m2_motion_register_bone, FfiAttachingEffect, FfiMotionEffectEventData, MotionEffectEventData,
};

static TEST_LOCK: Mutex<()> = Mutex::new(());

fn get_empty_ffi_data() -> FfiMotionEffectEventData {
    FfiMotionEffectEventData {
        type_id: 0,
        frame_time: 0,
        is_independent: 0,
        is_attaching: 0,
        is_following: 0,
        effect_index: 0,
        attaching_bone_name: [0; 32],
        effect_position_x: 0.0,
        effect_position_y: 0.0,
        effect_position_z: 0.0,
    }
}

fn set_bone_name(ffi_data: &mut FfiMotionEffectEventData, name: &str) {
    let bytes = name.as_bytes();
    for (i, &b) in bytes.iter().take(31).enumerate() {
        ffi_data.attaching_bone_name[i] = b;
    }
    ffi_data.attaching_bone_name[bytes.len().min(31)] = 0;
}

#[test]
fn test_independent_effect_generation() {
    let _lock = TEST_LOCK.lock().unwrap();
    let mut manager = get_effect_manager();
    let actor_vid = 1001;
    manager.clear_events(actor_vid);

    let data = MotionEffectEventData {
        is_independent: true,
        is_attaching: true, // Should be ignored if independent
        is_following: true, // Should be ignored if independent
        effect_index: 100,
        attaching_bone_name: "Bip01 Head".to_string(),
        effect_position: (1.0, 2.0, 3.0),
    };

    manager.process_event(actor_vid, data);
    drop(manager);

    let mut out_effects: [FfiAttachingEffect; 1] = unsafe { std::mem::zeroed() };
    // We test using FFI function because internal `actors` is private
    let count = m2_motion_get_attached_effects(actor_vid, out_effects.as_mut_ptr(), 1);
    
    assert_eq!(count, 1);
    let effect = &out_effects[0];
    assert_eq!(effect.is_independent, 1);
    assert_eq!(effect.is_attaching, 0);
    assert_eq!(effect.is_following, 0);
    assert_eq!(effect.bone_index, -1);
    assert_eq!(effect.effect_index, 100);
}

#[test]
fn test_attaching_and_following_with_bone() {
    let _lock = TEST_LOCK.lock().unwrap();
    let mut manager = get_effect_manager();
    let actor_vid = 1002;
    manager.clear_events(actor_vid);
    manager.register_bone("Bip01 Spine".to_string(), 5);

    let data = MotionEffectEventData {
        is_independent: false,
        is_attaching: true,
        is_following: true,
        effect_index: 200,
        attaching_bone_name: "Bip01 Spine".to_string(),
        effect_position: (0.0, 0.0, 0.0),
    };

    manager.process_event(actor_vid, data);
    drop(manager);

    let mut out_effects: [FfiAttachingEffect; 1] = unsafe { std::mem::zeroed() };
    let count = m2_motion_get_attached_effects(actor_vid, out_effects.as_mut_ptr(), 1);

    assert_eq!(count, 1);
    let effect = &out_effects[0];
    assert_eq!(effect.is_independent, 0);
    assert_eq!(effect.is_attaching, 1);
    assert_eq!(effect.is_following, 1);
    assert_eq!(effect.bone_index, 5);
}

#[test]
fn test_attaching_not_following_with_bone() {
    let _lock = TEST_LOCK.lock().unwrap();
    let mut manager = get_effect_manager();
    let actor_vid = 1003;
    manager.clear_events(actor_vid);
    manager.register_bone("Bip01 L Hand".to_string(), 10);

    let data = MotionEffectEventData {
        is_independent: false,
        is_attaching: true,
        is_following: false,
        effect_index: 300,
        attaching_bone_name: "Bip01 L Hand".to_string(),
        effect_position: (5.0, 0.0, 0.0),
    };

    manager.process_event(actor_vid, data);
    drop(manager);

    let mut out_effects: [FfiAttachingEffect; 1] = unsafe { std::mem::zeroed() };
    let count = m2_motion_get_attached_effects(actor_vid, out_effects.as_mut_ptr(), 1);

    assert_eq!(count, 1);
    let effect = &out_effects[0];
    assert_eq!(effect.is_independent, 0);
    assert_eq!(effect.is_attaching, 1);
    assert_eq!(effect.is_following, 0);
    assert_eq!(effect.bone_index, 10);
}

#[test]
fn test_attaching_missing_bone() {
    let _lock = TEST_LOCK.lock().unwrap();
    let mut manager = get_effect_manager();
    let actor_vid = 1004;
    manager.clear_events(actor_vid);

    let data = MotionEffectEventData {
        is_independent: false,
        is_attaching: true,
        is_following: false, // Not following
        effect_index: 400,
        attaching_bone_name: "NonExistentBone".to_string(),
        effect_position: (0.0, 0.0, 0.0),
    };

    manager.process_event(actor_vid, data);
    drop(manager);

    // If bone is missing and not following, effect shouldn't be added in C++
    let mut out_effects: [FfiAttachingEffect; 1] = unsafe { std::mem::zeroed() };
    let count = m2_motion_get_attached_effects(actor_vid, out_effects.as_mut_ptr(), 1);
    assert_eq!(count, 0);
}

#[test]
fn test_not_attaching() {
    let _lock = TEST_LOCK.lock().unwrap();
    let mut manager = get_effect_manager();
    let actor_vid = 1005;
    manager.clear_events(actor_vid);

    let data = MotionEffectEventData {
        is_independent: false,
        is_attaching: false,
        is_following: false,
        effect_index: 500,
        attaching_bone_name: "".to_string(),
        effect_position: (1.0, 1.0, 1.0),
    };

    manager.process_event(actor_vid, data);
    drop(manager);

    let mut out_effects: [FfiAttachingEffect; 1] = unsafe { std::mem::zeroed() };
    let count = m2_motion_get_attached_effects(actor_vid, out_effects.as_mut_ptr(), 1);

    assert_eq!(count, 1);
    let effect = &out_effects[0];
    assert_eq!(effect.is_independent, 0);
    assert_eq!(effect.is_attaching, 0);
    assert_eq!(effect.is_following, 0);
    assert_eq!(effect.bone_index, -1);
}

#[test]
fn test_ffi_process_and_get_effects() {
    let _lock = TEST_LOCK.lock().unwrap();
    let mut manager = get_effect_manager();
    let actor_vid = 1006;
    manager.clear_events(actor_vid);
    drop(manager);

    let mut event_data = get_empty_ffi_data();
    event_data.is_independent = 1; // BOOL
    event_data.effect_index = 600;
    event_data.effect_position_x = 10.0;
    
    assert!(m2_motion_process_effect_event(actor_vid, &event_data));

    let mut out_effects: [FfiAttachingEffect; 5] = unsafe { std::mem::zeroed() };
    
    let count = m2_motion_get_attached_effects(actor_vid, out_effects.as_mut_ptr(), 5);
    
    assert_eq!(count, 1);
    assert_eq!(out_effects[0].effect_index, 600);
    assert_eq!(out_effects[0].is_independent, 1);
    assert_eq!(out_effects[0].effect_position_x, 10.0);
}

#[test]
fn test_ffi_register_bone() {
    let _lock = TEST_LOCK.lock().unwrap();
    let bone_name = std::ffi::CString::new("Bip01 Pelvis").unwrap();
    assert!(m2_motion_register_bone(bone_name.as_ptr(), 2));

    let manager = get_effect_manager();
    assert_eq!(manager.get_bone_index("Bip01 Pelvis"), Some(2));
}

#[test]
fn test_ffi_get_effects_limits() {
    let _lock = TEST_LOCK.lock().unwrap();
    let mut manager = get_effect_manager();
    let actor_vid = 1007;
    manager.clear_events(actor_vid);
    
    for i in 0..5 {
        manager.process_event(actor_vid, MotionEffectEventData {
            is_independent: true,
            is_attaching: false,
            is_following: false,
            effect_index: i as u32,
            attaching_bone_name: "".to_string(),
            effect_position: (0.0, 0.0, 0.0),
        });
    }
    drop(manager);

    let mut out_effects: [FfiAttachingEffect; 3] = unsafe { std::mem::zeroed() };
    
    // Should get first 3 effects out of 5
    let count = m2_motion_get_attached_effects(actor_vid, out_effects.as_mut_ptr(), 3);
    
    assert_eq!(count, 3);
    assert_eq!(out_effects[0].effect_index, 0);
    assert_eq!(out_effects[2].effect_index, 2);
}
