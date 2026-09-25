use m2_client_bridge::state::messenger::{
    m2_messenger_add_friend, m2_messenger_count, m2_messenger_is_friend_online, MessengerManager,
};
use std::ffi::CString;

#[test]
fn test_messenger_manager_logic() {
    let mut mm = MessengerManager::new();
    assert_eq!(mm.count(), 0);
    mm.add_friend("Janek");
    assert_eq!(mm.count(), 1);
    assert!(!mm.is_friend_online("Janek"));
    mm.set_online("janek", true);
    assert!(mm.is_friend_online("JANEK"));
    mm.remove_friend("Janek");
    assert_eq!(mm.count(), 0);
}

#[test]
fn test_messenger_ffi() {
    unsafe {
        let name = CString::new("Kowalski").unwrap();
        assert!(m2_messenger_add_friend(name.as_ptr()));
        assert_eq!(m2_messenger_count() >= 1, true);
        assert_eq!(m2_messenger_is_friend_online(name.as_ptr()), false);
        assert_eq!(m2_messenger_add_friend(std::ptr::null()), false);
    }
}
