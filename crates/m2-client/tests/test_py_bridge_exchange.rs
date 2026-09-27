use m2_client::py_bridge_exchange::{
    m2_py_exchange_get_accept_from_target, m2_py_exchange_get_elk_from_target,
    m2_py_exchange_get_item_count_from_target, m2_py_exchange_get_item_vnum_from_target,
    m2_py_exchange_init_trading,
};
use m2_client::recv_exchange_logic_v2::{get_exchange_logic, ExchangeData, ExchangeLogic};
use serial_test::serial;

fn setup_test_state() {
    let mut logic = get_exchange_logic().write().unwrap();
    *logic = ExchangeLogic::new();
}

#[test]
#[serial]
fn test_init_trading() {
    setup_test_state();
    assert!(m2_py_exchange_init_trading());
}

#[test]
#[serial]
fn test_get_elk_from_target() {
    setup_test_state();
    {
        let mut logic = get_exchange_logic().write().unwrap();
        logic.target_data.elk = 123456;
    }
    assert_eq!(m2_py_exchange_get_elk_from_target(), 123456);
}

#[test]
#[serial]
fn test_get_item_vnum_from_target_valid_and_invalid_slot() {
    setup_test_state();
    {
        let mut logic = get_exchange_logic().write().unwrap();
        logic.target_data.item_vnum[0] = 1001;
        logic.target_data.item_vnum[5] = 9999;
        logic.target_data.item_vnum[11] = 5005;
    }
    
    // Valid slots
    assert_eq!(m2_py_exchange_get_item_vnum_from_target(0), 1001);
    assert_eq!(m2_py_exchange_get_item_vnum_from_target(5), 9999);
    assert_eq!(m2_py_exchange_get_item_vnum_from_target(11), 5005);
    
    // Invalid/Out-of-bounds slots
    assert_eq!(m2_py_exchange_get_item_vnum_from_target(12), 0);
    assert_eq!(m2_py_exchange_get_item_vnum_from_target(255), 0);
}

#[test]
#[serial]
fn test_get_item_count_from_target_valid_and_invalid_slot() {
    setup_test_state();
    {
        let mut logic = get_exchange_logic().write().unwrap();
        logic.target_data.item_count[0] = 1;
        logic.target_data.item_count[3] = 200;
        logic.target_data.item_count[11] = 50;
    }
    
    // Valid slots
    assert_eq!(m2_py_exchange_get_item_count_from_target(0), 1);
    assert_eq!(m2_py_exchange_get_item_count_from_target(3), 200);
    assert_eq!(m2_py_exchange_get_item_count_from_target(11), 50);
    
    // Invalid/Out-of-bounds slots
    assert_eq!(m2_py_exchange_get_item_count_from_target(12), 0);
    assert_eq!(m2_py_exchange_get_item_count_from_target(255), 0);
}

#[test]
#[serial]
fn test_get_accept_from_target() {
    setup_test_state();
    
    // Initially not accepted
    assert!(!m2_py_exchange_get_accept_from_target());
    
    // Set accepted
    {
        let mut logic = get_exchange_logic().write().unwrap();
        logic.target_data.accept = 1;
    }
    assert!(m2_py_exchange_get_accept_from_target());
    
    // Unset accepted
    {
        let mut logic = get_exchange_logic().write().unwrap();
        logic.target_data.accept = 0;
    }
    assert!(!m2_py_exchange_get_accept_from_target());
}
