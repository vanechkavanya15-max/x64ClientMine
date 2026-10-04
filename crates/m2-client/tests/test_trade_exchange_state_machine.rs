use m2_client::trade_exchange_state_machine::{ExchangeStateMachine, TradeState};

#[test]
fn test_exchange_state_machine_initialization() {
    let mut exchange = ExchangeStateMachine::new();

    assert_eq!(exchange.current_state, TradeState::Closed);
    assert_eq!(exchange.is_trading(), false);
    assert_eq!(exchange.get_elk_mode(), false);

    exchange.start();
    assert_eq!(exchange.is_trading(), true);
    assert_eq!(exchange.current_state, TradeState::Open);

    exchange.end();
    assert_eq!(exchange.is_trading(), false);
    assert_eq!(exchange.current_state, TradeState::Closed);
}

#[test]
fn test_exchange_names() {
    let mut exchange = ExchangeStateMachine::new();

    exchange.set_self_name("Player1");
    exchange.set_target_name("Player2");

    assert_eq!(exchange.get_name_from_self(), "Player1");
    assert_eq!(exchange.get_name_from_target(), "Player2");
}

#[test]
fn test_exchange_elk() {
    let mut exchange = ExchangeStateMachine::new();
    exchange.start();

    exchange.set_elk_to_self(1000);
    exchange.set_elk_to_target(2000);

    assert_eq!(exchange.get_elk_from_self(), 1000);
    assert_eq!(exchange.get_elk_from_target(), 2000);

    // Elk modification should transition state to ItemsAdded
    assert_eq!(exchange.current_state, TradeState::ItemsAdded);
}

#[test]
fn test_exchange_items() {
    let mut exchange = ExchangeStateMachine::new();
    exchange.start();

    exchange.set_item_to_self(0, 5001, 1);
    exchange.set_item_to_target(1, 5002, 5);

    assert_eq!(exchange.get_item_vnum_from_self(0), 5001);
    assert_eq!(exchange.get_item_count_from_self(0), 1);

    assert_eq!(exchange.get_item_vnum_from_target(1), 5002);
    assert_eq!(exchange.get_item_count_from_target(1), 5);

    assert_eq!(exchange.current_state, TradeState::ItemsAdded);
}

#[test]
fn test_exchange_item_sockets_and_attributes() {
    let mut exchange = ExchangeStateMachine::new();
    exchange.start();

    exchange.set_item_metin_socket_to_self(0, 1, 1234);
    assert_eq!(exchange.get_item_metin_socket_from_self(0, 1), 1234);

    exchange.set_item_attribute_to_target(2, 3, 5, 100);
    let attr = exchange.get_item_attribute_from_target(2, 3).unwrap();
    assert_eq!(attr.0, 5);
    assert_eq!(attr.1, 100);
}

#[test]
fn test_exchange_delete_item() {
    let mut exchange = ExchangeStateMachine::new();
    exchange.start();

    exchange.set_item_to_self(0, 5001, 1);
    assert_eq!(exchange.get_item_vnum_from_self(0), 5001);

    exchange.del_item_of_self(0);
    assert_eq!(exchange.get_item_vnum_from_self(0), 0);
    assert_eq!(exchange.get_item_count_from_self(0), 0);
}

#[test]
fn test_exchange_accept_and_finalize() {
    let mut exchange = ExchangeStateMachine::new();
    exchange.start();

    exchange.set_item_to_self(0, 5001, 1);
    exchange.set_item_to_target(0, 5002, 1);

    assert_eq!(exchange.current_state, TradeState::ItemsAdded);

    exchange.set_accept_to_self(true);
    assert_eq!(exchange.current_state, TradeState::ItemsAdded);

    exchange.set_accept_to_target(true);
    assert_eq!(exchange.current_state, TradeState::Accepted);

    exchange.finalize();
    assert_eq!(exchange.current_state, TradeState::Finalized);
    assert_eq!(exchange.is_trading(), false);
}

#[test]
fn test_exchange_item_change_resets_accept() {
    let mut exchange = ExchangeStateMachine::new();
    exchange.start();

    exchange.set_item_to_self(0, 5001, 1);
    
    exchange.set_accept_to_self(true);
    exchange.set_accept_to_target(true);
    assert_eq!(exchange.current_state, TradeState::Accepted);
    assert_eq!(exchange.get_accept_from_self(), true);
    assert_eq!(exchange.get_accept_from_target(), true);

    // One party changes an item
    exchange.set_item_to_self(1, 123, 1);

    // State should go back to ItemsAdded and both accept flags should be reset
    assert_eq!(exchange.current_state, TradeState::ItemsAdded);
    assert_eq!(exchange.get_accept_from_self(), false);
    assert_eq!(exchange.get_accept_from_target(), false);
}

#[test]
fn test_exchange_clear() {
    let mut exchange = ExchangeStateMachine::new();
    exchange.start();
    exchange.set_elk_mode(true);
    exchange.set_self_name("Test");
    exchange.set_item_to_self(0, 100, 1);

    exchange.clear();

    assert_eq!(exchange.get_name_from_self(), "");
    assert_eq!(exchange.get_item_vnum_from_self(0), 0);
    assert_eq!(exchange.current_state, TradeState::Closed);
    assert_eq!(exchange.is_trading(), true); // is_trading is not reset by clear
    assert_eq!(exchange.get_elk_mode(), true); // elk_mode is not reset by clear
}

#[test]
fn test_exchange_full_simulation() {
    let mut exchange = ExchangeStateMachine::new();
    
    // Both players open trade window
    exchange.start();
    assert_eq!(exchange.current_state, TradeState::Open);
    
    exchange.set_self_name("Alice");
    exchange.set_target_name("Bob");

    // Alice adds a sword
    exchange.set_item_to_self(0, 2799, 1);
    assert_eq!(exchange.current_state, TradeState::ItemsAdded);
    
    // Bob adds yang
    exchange.set_elk_to_target(50000);
    assert_eq!(exchange.current_state, TradeState::ItemsAdded);

    // Alice accepts
    exchange.set_accept_to_self(true);
    assert_eq!(exchange.current_state, TradeState::ItemsAdded);

    // Bob tries to scam by quickly changing yang
    exchange.set_elk_to_target(500);
    
    // Alice's accept must be reset
    assert_eq!(exchange.get_accept_from_self(), false);
    assert_eq!(exchange.current_state, TradeState::ItemsAdded);

    // Alice notices and doesn't accept. Bob puts back the right amount
    exchange.set_elk_to_target(50000);

    // Both accept
    exchange.set_accept_to_self(true);
    exchange.set_accept_to_target(true);
    assert_eq!(exchange.current_state, TradeState::Accepted);

    // Server finalizes
    exchange.finalize();
    assert_eq!(exchange.current_state, TradeState::Finalized);
    assert_eq!(exchange.is_trading(), false);
}

#[test]
fn test_exchange_elk_overflow() {
    let mut exchange = ExchangeStateMachine::new();
    exchange.start();
    
    // Use the max u64 value minus 100
    let max_val: u64 = u64::MAX - 100;
    exchange.set_elk_to_self(max_val);
    
    // Add 50, should succeed
    assert_eq!(exchange.add_elk_to_self(50), true);
    assert_eq!(exchange.get_elk_from_self(), u64::MAX - 50);
    
    // Add 100, should fail because u64::MAX - 50 + 100 > u64::MAX
    assert_eq!(exchange.add_elk_to_self(100), false);
    // Value shouldn't change after failed overflow attempt
    assert_eq!(exchange.get_elk_from_self(), u64::MAX - 50);
}
