#[path = "../src/exchange_security_session_v2.rs"]
mod exchange_security_session_v2;

use exchange_security_session_v2::{CPythonExchange, ExchangeError};

#[test]
fn test_exchange_initial_state() {
    let mut exchange = CPythonExchange::new();
    assert!(!exchange.is_trading());
    assert_eq!(exchange.get_elk_mode(), false);
    
    exchange.start();
    assert!(exchange.is_trading());
    
    exchange.end();
    assert!(!exchange.is_trading());
}

#[test]
fn test_exchange_can_modify_when_not_accepted() {
    let mut exchange = CPythonExchange::new();
    exchange.start();
    
    assert!(exchange.can_modify_exchange());
    
    assert_eq!(exchange.set_item_to_self(0, 1001, 1), Ok(()));
    assert_eq!(exchange.get_item_vnum_from_self(0), 1001);
    
    assert_eq!(exchange.set_elk_to_self(1000), Ok(()));
    assert_eq!(exchange.get_elk_from_self(), 1000);
}

#[test]
fn test_exchange_cannot_modify_when_self_accepted() {
    let mut exchange = CPythonExchange::new();
    exchange.start();
    
    exchange.set_accept_to_self(1);
    
    assert!(!exchange.can_modify_exchange());
    assert_eq!(exchange.set_item_to_self(0, 1001, 1), Err(ExchangeError::ExchangeIsLocked));
    assert_eq!(exchange.set_elk_to_self(1000), Err(ExchangeError::ExchangeIsLocked));
    assert_eq!(exchange.del_item_of_self(0), Err(ExchangeError::ExchangeIsLocked));
}

#[test]
fn test_exchange_cannot_modify_when_target_accepted() {
    let mut exchange = CPythonExchange::new();
    exchange.start();
    
    exchange.set_accept_to_target(1);
    
    assert!(!exchange.can_modify_exchange());
    assert_eq!(exchange.set_item_to_target(0, 1001, 1), Err(ExchangeError::ExchangeIsLocked));
    assert_eq!(exchange.set_elk_to_target(1000), Err(ExchangeError::ExchangeIsLocked));
    assert_eq!(exchange.del_item_of_target(0), Err(ExchangeError::ExchangeIsLocked));
}

#[test]
fn test_exchange_names() {
    let mut exchange = CPythonExchange::new();
    
    exchange.set_self_name("Player1");
    exchange.set_target_name("Player2");
    
    assert_eq!(exchange.get_name_from_self(), "Player1");
    assert_eq!(exchange.get_name_from_target(), "Player2");
}

#[test]
fn test_exchange_clear_resets_state() {
    let mut exchange = CPythonExchange::new();
    
    exchange.start();
    exchange.set_elk_mode(true);
    let _ = exchange.set_item_to_self(0, 1234, 1);
    exchange.set_accept_to_self(1);
    
    // Accept state means locked
    assert!(!exchange.can_modify_exchange());
    
    exchange.clear();
    
    // State is reset
    assert_eq!(exchange.get_item_vnum_from_self(0), 0);
    assert!(exchange.can_modify_exchange());
    assert!(!exchange.get_accept_from_self());
}
