use m2_client::granny_attachment_system::{AttachmentItem, AttachmentManager, AttachmentPoint};
use glam::Mat4;

#[test]
fn test_default_bone_names() {
    assert_eq!(AttachmentPoint::WeaponRight.default_bone_name(), "equip_right");
    assert_eq!(AttachmentPoint::WeaponLeft.default_bone_name(), "equip_left");
    assert_eq!(AttachmentPoint::Shield.default_bone_name(), "equip_left");
    assert_eq!(AttachmentPoint::Hair.default_bone_name(), "hair");
    assert_eq!(AttachmentPoint::Sash.default_bone_name(), "spine");
    assert_eq!(AttachmentPoint::Bow.default_bone_name(), "equip_left");
    assert_eq!(AttachmentPoint::Arrow.default_bone_name(), "equip_right");
}

#[test]
fn test_attach_and_detach_item() {
    let mut manager = AttachmentManager::new();
    let item = AttachmentItem::new(
        AttachmentPoint::WeaponRight,
        AttachmentPoint::WeaponRight.default_bone_name().to_string(),
        100,
        Mat4::IDENTITY,
    );
    
    manager.attach_item(AttachmentPoint::WeaponRight, item.clone());
    assert_eq!(manager.get_item(AttachmentPoint::WeaponRight), Some(&item));
    
    assert_eq!(manager.detach_item(AttachmentPoint::WeaponRight), Ok(()));
    assert_eq!(manager.get_item(AttachmentPoint::WeaponRight), None);
}

#[test]
fn test_compute_attached_world_matrix() {
    let offset = Mat4::from_translation(glam::Vec3::new(1.0, 2.0, 3.0));
    let item = AttachmentItem::new(
        AttachmentPoint::WeaponRight,
        AttachmentPoint::WeaponRight.default_bone_name().to_string(),
        100,
        offset,
    );
    
    let parent_matrix = Mat4::from_translation(glam::Vec3::new(10.0, 20.0, 30.0));
    
    let world_matrix = AttachmentManager::compute_attached_world_matrix(&item, parent_matrix);
    let expected_matrix = parent_matrix * offset;
    
    assert_eq!(world_matrix, expected_matrix);
}

#[test]
fn test_multiple_attachments() {
    let mut manager = AttachmentManager::new();
    
    let weapon = AttachmentItem::new(
        AttachmentPoint::WeaponRight,
        AttachmentPoint::WeaponRight.default_bone_name().to_string(),
        100,
        Mat4::IDENTITY,
    );
    let shield = AttachmentItem::new(
        AttachmentPoint::Shield,
        AttachmentPoint::Shield.default_bone_name().to_string(),
        200,
        Mat4::IDENTITY,
    );
    
    manager.attach_item(AttachmentPoint::WeaponRight, weapon.clone());
    manager.attach_item(AttachmentPoint::Shield, shield.clone());
    
    assert_eq!(manager.get_item(AttachmentPoint::WeaponRight), Some(&weapon));
    assert_eq!(manager.get_item(AttachmentPoint::Shield), Some(&shield));
}

#[test]
fn test_reassign_attachment_point() {
    let mut manager = AttachmentManager::new();
    
    let weapon = AttachmentItem::new(
        AttachmentPoint::WeaponRight,
        AttachmentPoint::WeaponRight.default_bone_name().to_string(),
        100,
        Mat4::IDENTITY,
    );
    
    // Attach weapon to shield slot (for testing purposes)
    manager.attach_item(AttachmentPoint::Shield, weapon.clone());
    
    let attached = manager.get_item(AttachmentPoint::Shield).unwrap();
    assert_eq!(attached.point, AttachmentPoint::Shield);
    assert_eq!(attached.model_id, 100);
}
