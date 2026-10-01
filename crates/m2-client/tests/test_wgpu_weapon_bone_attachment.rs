use glam::Mat4;
use m2_client::wgpu_weapon_bone_attachment::*;

#[test]
fn test_bone_names() {
    assert_eq!(AttachmentBone::EquipRight.name(), "equip_right");
    assert_eq!(AttachmentBone::EquipLeft.name(), "equip_left");
    assert_eq!(AttachmentBone::Head.name(), "Bip01 Head");
    assert_eq!(AttachmentBone::Spine2.name(), "Bip01 Spine2");
}

#[test]
fn test_weapon_hands() {
    // Sword
    assert!(is_right_hand_weapon(WeaponType::Sword, false));
    assert!(!is_left_hand_weapon(WeaponType::Sword, false));

    // Dagger
    assert!(is_right_hand_weapon(WeaponType::Dagger, false));
    assert!(is_left_hand_weapon(WeaponType::Dagger, false));

    // Bow
    assert!(!is_right_hand_weapon(WeaponType::Bow, false));
    assert!(is_left_hand_weapon(WeaponType::Bow, false));

    // Fan not mounting
    assert!(is_right_hand_weapon(WeaponType::Fan, false));
    assert!(!is_left_hand_weapon(WeaponType::Fan, false));

    // Fan mounting
    assert!(is_right_hand_weapon(WeaponType::Fan, true));
    assert!(is_left_hand_weapon(WeaponType::Fan, true));
}

#[test]
fn test_calculate_world_matrix() {
    let parent_mat = Mat4::from_translation(glam::Vec3::new(1.0, 2.0, 3.0));
    let offset_mat = Mat4::from_translation(glam::Vec3::new(0.5, 0.5, 0.5));

    let attachment = BoneAttachment::new(parent_mat, offset_mat);
    let world_mat = attachment.calculate_world_matrix();

    let expected = Mat4::from_translation(glam::Vec3::new(1.5, 2.5, 3.5));

    // Check elements
    for i in 0..16 {
        assert!((world_mat.to_cols_array()[i] - expected.to_cols_array()[i]).abs() < 1e-5);
    }
}

#[test]
fn test_bone_attachment_logic() {
    let parent = Mat4::IDENTITY;
    let offset = Mat4::from_translation(glam::Vec3::new(1.0, 0.0, 0.0));

    let attachment = BoneAttachment::new(parent, offset);
    assert_eq!(
        attachment.calculate_world_matrix().to_cols_array(),
        offset.to_cols_array()
    );
}

#[test]
fn test_equipment_attachment_bones() {
    // Sword -> Right Hand
    assert_eq!(
        get_weapon_attachment_bones(WeaponType::Sword, false),
        vec![AttachmentBone::EquipRight]
    );

    // Bow -> Left Hand
    assert_eq!(
        get_weapon_attachment_bones(WeaponType::Bow, false),
        vec![AttachmentBone::EquipLeft]
    );

    // Dagger -> Both Hands
    assert_eq!(
        get_weapon_attachment_bones(WeaponType::Dagger, false),
        vec![AttachmentBone::EquipRight, AttachmentBone::EquipLeft]
    );

    // Fan not mounting -> Right Hand
    assert_eq!(
        get_weapon_attachment_bones(WeaponType::Fan, false),
        vec![AttachmentBone::EquipRight]
    );

    // Fan mounting -> Both Hands
    assert_eq!(
        get_weapon_attachment_bones(WeaponType::Fan, true),
        vec![AttachmentBone::EquipRight, AttachmentBone::EquipLeft]
    );

    // Shield -> Left Hand
    assert_eq!(
        get_weapon_attachment_bones(WeaponType::Shield, false),
        vec![AttachmentBone::EquipLeft]
    );

    // Helmet -> Head
    assert_eq!(
        get_weapon_attachment_bones(WeaponType::Helmet, false),
        vec![AttachmentBone::Head]
    );

    // Sash -> Spine2
    assert_eq!(
        get_weapon_attachment_bones(WeaponType::Sash, false),
        vec![AttachmentBone::Spine2]
    );
}

#[test]
fn test_equipment_attachment_bones_full() {
    assert_eq!(
        get_weapon_attachment_bones(WeaponType::Mount, false),
        Vec::<AttachmentBone>::new()
    );
    assert_eq!(
        get_weapon_attachment_bones(WeaponType::None, false),
        Vec::<AttachmentBone>::new()
    );
}