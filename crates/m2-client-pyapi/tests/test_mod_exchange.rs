use m2_client_pyapi::mod_exchange::ModExchange;

#[test]
fn test_integration_mod_exchange_self() {
    let mut instance = ModExchange::new();
    assert!(instance.is_ready());

    assert_eq!(instance.get_elk_from_self(), 0);
    instance.set_elk_from_self(5000);
    assert_eq!(instance.get_elk_from_self(), 5000);

    assert_eq!(instance.is_accept_from_self(), false);
    instance.set_accept_from_self(true);
    assert_eq!(instance.is_accept_from_self(), true);

    assert_eq!(instance.get_item_vnum_from_self(3), 0);
    instance.set_item_vnum_from_self(3, 11299);
    assert_eq!(instance.get_item_vnum_from_self(3), 11299);
}

#[test]
fn test_integration_mod_exchange_target() {
    let mut instance = ModExchange::new();
    
    assert_eq!(instance.get_elk_from_target(), 0);
    instance.set_elk_from_target(123456);
    assert_eq!(instance.get_elk_from_target(), 123456);

    assert_eq!(instance.is_accept_from_target(), false);
    instance.set_accept_from_target(true);
    assert_eq!(instance.is_accept_from_target(), true);

    assert_eq!(instance.get_item_vnum_from_target(0), 0);
    instance.set_item_vnum_from_target(0, 19);
    assert_eq!(instance.get_item_vnum_from_target(0), 19);
}

#[test]
fn test_integration_mod_exchange_clear() {
    let mut instance = ModExchange::new();
    
    instance.set_elk_from_self(100);
    instance.set_accept_from_self(true);
    instance.set_item_vnum_from_self(1, 10);
    
    instance.set_elk_from_target(200);
    instance.set_accept_from_target(true);
    instance.set_item_vnum_from_target(2, 20);
    
    instance.clear();
    
    assert_eq!(instance.get_elk_from_self(), 0);
    assert_eq!(instance.is_accept_from_self(), false);
    assert_eq!(instance.get_item_vnum_from_self(1), 0);
    
    assert_eq!(instance.get_elk_from_target(), 0);
    assert_eq!(instance.is_accept_from_target(), false);
    assert_eq!(instance.get_item_vnum_from_target(2), 0);
}
