use m2_client_pyapi::mod_safebox::ModSafebox;

#[test]
fn test_integration_mod_safebox() {
    let instance = ModSafebox::new();
    assert!(instance.is_ready());
}

#[test]
fn test_get_current_safebox_size() {
    let mut instance = ModSafebox::new();
    assert_eq!(instance.get_current_safebox_size(), 0);
    
    instance.set_size(45);
    assert_eq!(instance.get_current_safebox_size(), 45);
}

#[test]
fn test_get_money() {
    let mut instance = ModSafebox::new();
    assert_eq!(instance.get_money(), 0);
    
    instance.set_money(100500);
    assert_eq!(instance.get_money(), 100500);
}

#[test]
fn test_get_item_id() {
    let mut instance = ModSafebox::new();
    // Testowanie domyslnego pustego slotu
    assert_eq!(instance.get_item_id(1), 0);
    assert_eq!(instance.get_item_id(5), 0);
    
    // Ustawianie przedmiotow
    instance.set_item(1, 1001);
    instance.set_item(5, 5005);
    
    // Sprawdzenie przypisanych slotow
    assert_eq!(instance.get_item_id(1), 1001);
    assert_eq!(instance.get_item_id(5), 5005);
    
    // Sprawdzenie nieprzypisanego slotu
    assert_eq!(instance.get_item_id(2), 0);
}
