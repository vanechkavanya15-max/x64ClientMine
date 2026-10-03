#[path = "../src/weapon_bone_attachment_mgr.rs"]
mod weapon_bone_attachment_mgr;

use weapon_bone_attachment_mgr::*;
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
