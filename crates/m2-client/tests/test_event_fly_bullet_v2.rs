use m2_client::event_fly_bullet_v2::*;
use std::ffi::CString;

#[test]
fn test_fly_event_data_new() {
    let data = FlyEventData::new();
    assert_eq!(data.is_attaching, false);
    assert_eq!(data.attaching_bone_name, "");
    assert_eq!(data.fly_position, [0.0, 0.0, 0.0]);
    assert_eq!(data.fly_index, 0);
    assert_eq!(data.fly_file_name, "");
}

#[test]
fn test_actor_instance_no_target() {
    let actor = CActorInstance::new(1);
    let data = FlyEventData::new();
    let result = actor.process_motion_event_fly(&data);
    assert!(result.is_none());
}

#[test]
fn test_actor_instance_with_target_no_attach() {
    let mut actor = CActorInstance::new(1);
    actor.x = 100.0;
    actor.y = 200.0;
    actor.z = 300.0;
    actor.has_target = true;

    let mut data = FlyEventData::new();
    data.fly_position = [1.0, 2.0, 3.0];

    let result = actor.process_motion_event_fly(&data);
    assert!(result.is_some());
    let pos = result.unwrap();
    assert_eq!(pos, [101.0, 202.0, 303.0]);
}

#[test]
fn test_actor_instance_with_target_and_attach() {
    let mut actor = CActorInstance::new(1);
    actor.x = 100.0;
    actor.y = 200.0;
    actor.z = 300.0;
    actor.has_target = true;
    
    let mut mat = [0.0; 16];
    mat[0] = 1.0; mat[5] = 1.0; mat[10] = 1.0; mat[15] = 1.0;
    mat[12] = 5.0; // _41
    mat[13] = 10.0; // _42
    mat[14] = 15.0; // _43
    actor.set_bone_matrix("Bip01 R Hand", mat);

    let mut data = FlyEventData::new();
    data.fly_position = [1.0, 2.0, 3.0];
    data.is_attaching = true;
    data.attaching_bone_name = "Bip01 R Hand".to_string();

    let result = actor.process_motion_event_fly(&data);
    assert!(result.is_some());
    let pos = result.unwrap();
    // actor pos (100, 200, 300) + fly pos (1, 2, 3) + bone matrix (5, 10, 15)
    assert_eq!(pos, [106.0, 212.0, 318.0]);
}

#[test]
fn test_actor_instance_with_target_attach_invalid_bone() {
    let mut actor = CActorInstance::new(1);
    actor.x = 100.0;
    actor.y = 200.0;
    actor.z = 300.0;
    actor.has_target = true;

    let mut data = FlyEventData::new();
    data.fly_position = [1.0, 2.0, 3.0];
    data.is_attaching = true;
    data.attaching_bone_name = "InvalidBone".to_string();

    let result = actor.process_motion_event_fly(&data);
    assert!(result.is_some());
    let pos = result.unwrap();
    // Zly bone, brak offsetu
    assert_eq!(pos, [101.0, 202.0, 303.0]);
}

#[test]
fn test_ffi_m2_motion_get_fly_start_pos() {
    let vid = 10;
    
    m2_motion_set_actor_position(vid, 100.0, 200.0, 300.0);
    m2_motion_set_actor_target(vid, true);
    
    let mut mat = [0.0; 16];
    mat[0] = 1.0; mat[5] = 1.0; mat[10] = 1.0; mat[15] = 1.0;
    mat[12] = 5.0;
    mat[13] = 10.0;
    mat[14] = 15.0;
    
    let bone_name = CString::new("equip_right").unwrap();
    m2_motion_set_bone_matrix(vid, bone_name.as_ptr(), mat.as_ptr());

    let mut out_pos: [f32; 3] = [0.0; 3];
    let result = m2_motion_get_fly_start_pos(
        vid,
        true,
        bone_name.as_ptr(),
        1.0, 2.0, 3.0,
        out_pos.as_mut_ptr()
    );
    
    assert!(result);
    // 100 + 1 + 5, 200 + 2 + 10, 300 + 3 + 15
    assert_eq!(out_pos, [106.0, 212.0, 318.0]);
}

#[test]
fn test_ffi_m2_motion_get_fly_start_pos_null_ptr() {
    let bone_name = CString::new("equip_right").unwrap();
    let result = m2_motion_get_fly_start_pos(
        1,
        true,
        bone_name.as_ptr(),
        1.0, 2.0, 3.0,
        std::ptr::null_mut()
    );
    assert!(!result);
}

#[test]
fn test_ffi_m2_motion_get_fly_start_pos_invalid_vid() {
    let bone_name = CString::new("equip_right").unwrap();
    let mut out_pos: [f32; 3] = [0.0; 3];
    let result = m2_motion_get_fly_start_pos(
        999, // nieistniejacy vid
        true,
        bone_name.as_ptr(),
        1.0, 2.0, 3.0,
        out_pos.as_mut_ptr()
    );
    assert!(!result);
}
