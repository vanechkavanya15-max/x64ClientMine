use m2_client::exchange_system::{ExchangeSystem, ExchangeResult};

#[test]
fn test_exchange_start_end() {
    let mut system = ExchangeSystem::new();
    assert!(!system.is_trading());
    system.start();
    assert!(system.is_trading());
    
    // Testing finalize/cancel logic over simple end
    system.cancel();
    assert!(!system.is_trading());
}

#[test]
fn test_lock_on_accept() {
    let mut system = ExchangeSystem::new();
    system.start();
    
    system.set_item_to_self(0, 100, 1);
    system.set_elk_to_self(500);
    
    assert_eq!(system.get_item_vnum_from_self(0), 100);
    assert_eq!(system.get_elk_from_self(), 500);
    
    system.set_accept_to_self(true);
    
    // Attempting to modify locked trade
    let set_item_result = system.set_item_to_self(1, 200, 2);
    let set_elk_result = system.set_elk_to_self(1000);
    let del_item_result = system.del_item_of_self(0);
    
    assert_eq!(set_item_result, false);
    assert_eq!(set_elk_result, false);
    assert_eq!(del_item_result, false);
    
    // Verify state remained unchanged
    assert_eq!(system.get_item_vnum_from_self(1), 0);
    assert_eq!(system.get_elk_from_self(), 500);
    assert_eq!(system.get_item_vnum_from_self(0), 100);
}

#[test]
fn test_trade_finalization() {
    let mut system = ExchangeSystem::new();
    system.start();
    
    system.set_item_to_self(0, 123, 1);
    system.set_elk_to_self(100);
    
    system.set_item_to_target(0, 321, 2);
    system.set_elk_to_target(200);
    
    system.set_accept_to_self(true);
    system.set_accept_to_target(true);
    
    let result = system.finalize();
    
    if let ExchangeResult::Success { 
        self_received_items, self_received_counts, self_received_elk, 
        target_received_items, target_received_counts, target_received_elk 
    } = result {
        assert_eq!(self_received_items[0], 321);
        assert_eq!(self_received_counts[0], 2);
        assert_eq!(self_received_elk, 200);
        
        assert_eq!(target_received_items[0], 123);
        assert_eq!(target_received_counts[0], 1);
        assert_eq!(target_received_elk, 100);
    } else {
        panic!("Exchange did not finalize successfully!");
    }
    
    // System should be cleared after finalize
    assert!(!system.is_trading());
    assert_eq!(system.get_elk_from_self(), 0);
}

#[test]
fn test_trade_cancellation() {
    let mut system = ExchangeSystem::new();
    system.start();
    
    system.set_item_to_self(0, 555, 3);
    system.set_item_to_target(1, 777, 5);
    
    let result = system.cancel();
    
    if let ExchangeResult::Canceled {
        self_returned_items, self_returned_counts, 
        target_returned_items, target_returned_counts
    } = result {
        assert_eq!(self_returned_items[0], 555);
        assert_eq!(self_returned_counts[0], 3);
        assert_eq!(target_returned_items[1], 777);
        assert_eq!(target_returned_counts[1], 5);
    } else {
        panic!("Exchange did not cancel correctly!");
    }
    
    assert!(!system.is_trading());
    assert_eq!(system.get_item_vnum_from_self(0), 0);
}

#[test]
fn test_cannot_modify_if_not_trading() {
    let mut system = ExchangeSystem::new();
    
    let set_item_res = system.set_item_to_self(0, 10, 1);
    let set_elk_res = system.set_elk_to_self(100);
    
    assert_eq!(set_item_res, false);
    assert_eq!(set_elk_res, false);
}
