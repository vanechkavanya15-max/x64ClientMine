#[path = "../src/actor_pvp_keys.rs"]
mod actor_pvp_keys;

use actor_pvp_keys::{ActorPvpManager, PVPKey};

#[test]
fn test_pvp_key_creation() {
    let key = PVPKey::new(100, 200);
    assert_eq!(key.vid_src, 100);
    assert_eq!(key.vid_dst, 200);
}

#[test]
fn test_insert_and_check_pvp_active() {
    let mut manager = ActorPvpManager::new();
    
    // Initially not active
    assert!(!manager.is_pvp_active(100, 200));
    assert!(!manager.is_pvp_active(200, 100));

    // Insert key
    manager.insert_pvp_key(100, 200);

    // Bidirectional checking should work
    assert!(manager.is_pvp_active(100, 200));
    assert!(manager.is_pvp_active(200, 100));

    // Other characters should not be active
    assert!(!manager.is_pvp_active(100, 300));
    assert!(!manager.is_pvp_active(200, 300));
}

#[test]
fn test_remove_pvp_key() {
    let mut manager = ActorPvpManager::new();
    
    manager.insert_pvp_key(100, 200);
    assert!(manager.is_pvp_active(100, 200));

    // Remove key
    manager.remove_pvp_key(100, 200);

    // Should no longer be active
    assert!(!manager.is_pvp_active(100, 200));
    assert!(!manager.is_pvp_active(200, 100));
}

#[test]
fn test_remove_nonexistent_pvp_key() {
    let mut manager = ActorPvpManager::new();
    
    manager.insert_pvp_key(100, 200);
    assert!(manager.is_pvp_active(100, 200));

    // Remove key with reversed order (should not work, as insertion order matters for exact removal)
    manager.remove_pvp_key(200, 100);

    // Should still be active, since (100, 200) was inserted
    assert!(manager.is_pvp_active(100, 200));
}

#[test]
fn test_self_pvp() {
    let mut manager = ActorPvpManager::new();
    
    manager.insert_pvp_key(100, 100);

    assert!(manager.is_pvp_active(100, 100));

    manager.remove_pvp_key(100, 100);

    assert!(!manager.is_pvp_active(100, 100));
}

#[test]
fn test_multiple_pvp_keys() {
    let mut manager = ActorPvpManager::new();

    manager.insert_pvp_key(100, 200);
    manager.insert_pvp_key(100, 300);
    manager.insert_pvp_key(400, 500);

    assert!(manager.is_pvp_active(100, 200));
    assert!(manager.is_pvp_active(300, 100));
    assert!(manager.is_pvp_active(500, 400));

    assert!(!manager.is_pvp_active(200, 300));
    assert!(!manager.is_pvp_active(100, 400));
}
