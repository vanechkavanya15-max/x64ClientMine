use std::sync::{OnceLock, Mutex};
use m2_client::net_exchange_packets::*;

static TEST_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

#[test]
fn test_exchange_flow_success() {
    let _guard = TEST_MUTEX.get_or_init(|| Mutex::new(())).lock().unwrap();
    
    // 1. Start exchange
    let started = send_exchange_start(1234);
    assert!(started, "Failed to send exchange start packet");
    
    // 2. Add an item
    let item_added = send_exchange_item_add(5, 0);
    assert!(item_added, "Failed to send exchange item add packet");
    
    // 3. Add Elk (Yang)
    let elk_added = send_exchange_elk_add(1000000);
    assert!(elk_added, "Failed to send exchange elk add packet");
    
    // 4. Accept Exchange
    let accepted = send_exchange_accept();
    assert!(accepted, "Failed to send exchange accept packet");
}

#[test]
fn test_exchange_flow_cancellation() {
    let _guard = TEST_MUTEX.get_or_init(|| Mutex::new(())).lock().unwrap();
    
    // 1. Start exchange
    assert!(send_exchange_start(1235));
    
    // 2. Add item
    assert!(send_exchange_item_add(6, 1));
    
    // 3. Cancel item
    assert!(send_exchange_item_del(1));
    
    // 4. Exit
    assert!(send_exchange_exit());
}

#[test]
fn test_send_exchange_elk_add_negative_overflow() {
    let _guard = TEST_MUTEX.get_or_init(|| Mutex::new(())).lock().unwrap();
    
    // Simulate negative Yang input checking cast logic:
    // Any value that casts to negative i32 must be rejected.
    assert_eq!(send_exchange_elk_add(u32::MAX), false, "Should reject u32::MAX");
    assert_eq!(send_exchange_elk_add(i32::MAX as u32 + 1), false, "Should reject i32 overflow boundary");
    
    // Valid values
    assert_eq!(send_exchange_elk_add(0), true, "Should allow 0");
    assert_eq!(send_exchange_elk_add(100), true, "Should allow 100");
}

#[test]
fn test_send_exchange_invalid_slots() {
    let _guard = TEST_MUTEX.get_or_init(|| Mutex::new(())).lock().unwrap();
    
    // Test invalid target slots based on EXCHANGE_ITEM_MAX_NUM (which is 12)
    assert_eq!(send_exchange_item_add(0, 12), false, "Should reject slot 12 (out of bounds)");
    assert_eq!(send_exchange_item_add(0, u8::MAX), false, "Should reject slot u8::MAX");
    assert_eq!(send_exchange_item_del(13), false, "Should reject del for slot 13");
    
    // Test valid boundary
    assert_eq!(send_exchange_item_add(0, 11), true, "Should accept slot 11 (max valid index)");
    assert_eq!(send_exchange_item_del(11), true, "Should accept del for slot 11");
}
