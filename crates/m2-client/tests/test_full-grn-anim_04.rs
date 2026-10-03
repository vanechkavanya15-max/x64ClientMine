#[path = "../src/full-grn-anim_04.rs"]
mod full_grn_anim_04;

use full_grn_anim_04::weapon_bone_attachment_mgr::*;
use glam::Mat4;

#[test]
fn test_equip_sword() {
    let mut mgr = WeaponBoneAttachmentMgr::new();
    let right_bone = Mat4::from_translation(glam::Vec3::new(1.0, 0.0, 0.0));
    let right_offset = Mat4::from_translation(glam::Vec3::new(0.5, 0.0, 0.0));
    let left_bone = Mat4::IDENTITY;
    let left_offset = Mat4::IDENTITY;

    mgr.attach_weapon(WEAPON_SWORD, false, right_bone, right_offset, left_bone, left_offset);

    assert!(mgr.right_hand.is_attached);
    assert!(!mgr.left_hand.is_attached);

    let transform = mgr.get_transform(HAND_RIGHT).unwrap();
    let expected = right_bone * right_offset;
    assert_eq!(transform, expected);
}

#[test]
fn test_equip_dagger() {
    let mut mgr = WeaponBoneAttachmentMgr::new();
    let right_bone = Mat4::from_translation(glam::Vec3::new(1.0, 0.0, 0.0));
    let right_offset = Mat4::from_translation(glam::Vec3::new(0.5, 0.0, 0.0));
    let left_bone = Mat4::from_translation(glam::Vec3::new(-1.0, 0.0, 0.0));
    let left_offset = Mat4::from_translation(glam::Vec3::new(-0.5, 0.0, 0.0));

    mgr.attach_weapon(WEAPON_DAGGER, false, right_bone, right_offset, left_bone, left_offset);

    assert!(mgr.right_hand.is_attached);
    assert!(mgr.left_hand.is_attached);

    let right_transform = mgr.get_transform(HAND_RIGHT).unwrap();
    let expected_right = right_bone * right_offset;
    assert_eq!(right_transform, expected_right);

    let left_transform = mgr.get_transform(HAND_LEFT).unwrap();
    let expected_left = left_bone * left_offset;
    assert_eq!(left_transform, expected_left);
}

#[test]
fn test_equip_bow() {
    let mut mgr = WeaponBoneAttachmentMgr::new();
    let right_bone = Mat4::IDENTITY;
    let right_offset = Mat4::IDENTITY;
    let left_bone = Mat4::from_translation(glam::Vec3::new(-1.0, 0.0, 0.0));
    let left_offset = Mat4::from_translation(glam::Vec3::new(-0.5, 0.0, 0.0));

    mgr.attach_weapon(WEAPON_BOW, false, right_bone, right_offset, left_bone, left_offset);

    assert!(!mgr.right_hand.is_attached);
    assert!(mgr.left_hand.is_attached);

    let left_transform = mgr.get_transform(HAND_LEFT).unwrap();
    let expected_left = left_bone * left_offset;
    assert_eq!(left_transform, expected_left);
}

#[test]
fn test_equip_fan_mounting() {
    let mut mgr = WeaponBoneAttachmentMgr::new();
    let right_bone = Mat4::from_translation(glam::Vec3::new(1.0, 0.0, 0.0));
    let right_offset = Mat4::from_translation(glam::Vec3::new(0.5, 0.0, 0.0));
    let left_bone = Mat4::from_translation(glam::Vec3::new(-1.0, 0.0, 0.0));
    let left_offset = Mat4::from_translation(glam::Vec3::new(-0.5, 0.0, 0.0));

    mgr.attach_weapon(WEAPON_FAN, true, right_bone, right_offset, left_bone, left_offset);

    assert!(mgr.right_hand.is_attached);
    assert!(mgr.left_hand.is_attached);

    let right_transform = mgr.get_transform(HAND_RIGHT).unwrap();
    let expected_right = right_bone * right_offset;
    assert_eq!(right_transform, expected_right);

    let left_transform = mgr.get_transform(HAND_LEFT).unwrap();
    let expected_left = left_bone * left_offset;
    assert_eq!(left_transform, expected_left);
}

#[test]
fn test_unequip() {
    let mut mgr = WeaponBoneAttachmentMgr::new();
    let right_bone = Mat4::IDENTITY;
    let right_offset = Mat4::IDENTITY;
    let left_bone = Mat4::IDENTITY;
    let left_offset = Mat4::IDENTITY;

    mgr.attach_weapon(WEAPON_SWORD, false, right_bone, right_offset, left_bone, left_offset);
    assert!(mgr.right_hand.is_attached);

    mgr.unequip();
    assert!(!mgr.right_hand.is_attached);
    assert!(!mgr.left_hand.is_attached);
    assert!(mgr.get_transform(HAND_RIGHT).is_none());
    assert!(mgr.get_transform(HAND_LEFT).is_none());
}
