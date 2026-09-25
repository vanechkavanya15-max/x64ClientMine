use m2_client_bridge::state::mount::{
    m2_mount_get_vnum, m2_mount_is_mounted, m2_mount_set, MountState,
};

#[test]
fn test_mount_state_basic() {
    let mut ms = MountState::new();
    assert!(!ms.is_mounted);
    ms.mount(20101);
    assert!(ms.is_mounted);
    assert_eq!(ms.mount_vnum, 20101);
    ms.dismount();
    assert!(!ms.is_mounted);
}

#[test]
fn test_mount_ffi() {
    m2_mount_set(20104, true);
    assert!(m2_mount_is_mounted());
    assert_eq!(m2_mount_get_vnum(), 20104);
    m2_mount_set(0, false);
    assert!(!m2_mount_is_mounted());
}
