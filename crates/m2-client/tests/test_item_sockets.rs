#[path = "../src/item_sockets.rs"]
mod item_sockets;

use item_sockets::*;

#[test]
fn test_insert_valid_socket() {
    let mut item = ItemSockets::new();
    assert!(item.insert_socket(0, 28430)); // VNUM KD > 1000
    assert_eq!(item.sockets[0], 28430);
}

#[test]
fn test_insert_invalid_slot() {
    let mut item = ItemSockets::new();
    assert!(!item.insert_socket(3, 28430)); // index 3 is out of bounds (0, 1, 2)
}

#[test]
fn test_insert_already_occupied() {
    let mut item = ItemSockets::new();
    assert!(item.insert_socket(1, 28430));
    assert!(!item.insert_socket(1, 28431)); // slot 1 already has a socket
    assert_eq!(item.sockets[1], 28430);
}

#[test]
fn test_insert_invalid_vnum() {
    let mut item = ItemSockets::new();
    // Valid vnums are 1 (broken) or > 1000
    assert!(!item.insert_socket(0, 500)); // vnum 500 is invalid
    assert_eq!(item.sockets[0], SOCKET_EMPTY);
    
    assert!(!item.insert_socket(0, 0)); // 0 is empty, cannot insert empty
    assert_eq!(item.sockets[0], SOCKET_EMPTY);
    
    assert!(!item.insert_socket(0, 1000)); // 1000 is invalid
    assert_eq!(item.sockets[0], SOCKET_EMPTY);
}

#[test]
fn test_mark_broken_socket() {
    let mut item = ItemSockets::new();
    assert!(item.insert_socket(2, SOCKET_BROKEN)); // 1 is broken
    assert_eq!(item.sockets[2], SOCKET_BROKEN);
}

#[test]
fn test_clean_broken_sockets() {
    let mut item = ItemSockets::new();
    
    // Setup state
    assert!(item.insert_socket(0, 28430));
    assert!(item.insert_socket(1, SOCKET_BROKEN));
    assert!(item.insert_socket(2, SOCKET_BROKEN));
    
    // Clean
    let cleaned = item.clean_broken_sockets();
    assert!(cleaned);
    
    // Verify
    assert_eq!(item.sockets[0], 28430);
    assert_eq!(item.sockets[1], SOCKET_EMPTY);
    assert_eq!(item.sockets[2], SOCKET_EMPTY);
    
    // Try clean again
    let cleaned_again = item.clean_broken_sockets();
    assert!(!cleaned_again);
}

#[test]
fn test_default_state() {
    let item = ItemSockets::default();
    assert_eq!(item.sockets, [SOCKET_EMPTY, SOCKET_EMPTY, SOCKET_EMPTY]);
}
