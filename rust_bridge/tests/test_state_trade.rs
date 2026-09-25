use m2_client_bridge::state::trade::{
    m2_trade_is_locked, m2_trade_manager_free, m2_trade_manager_new, TradeManager,
    GLOBAL_TRADE_MANAGER,
};
use m2_client_bridge::state::inventory_item::InventoryItem;

#[test]
fn test_trade_cycle() {
    let mut manager = TradeManager::new();

    // Sprawdzenie stanu poczatkowego
    assert_eq!(manager.is_active, false);
    assert_eq!(manager.my_gold, 0);

    // Otwarcie handlu
    manager.start_trade();
    assert_eq!(manager.is_active, true);
    assert_eq!(manager.my_locked, false);

    // Dodanie itemu
    let item = InventoryItem::new(1001, 5001, 1);
    assert_eq!(manager.add_my_item(0, item.clone()), Ok(()));
    assert_eq!(manager.my_items[0], Some(item));

    // Wpisanie yang
    assert_eq!(manager.set_my_gold(100_000), Ok(()));
    assert_eq!(manager.my_gold, 100_000);

    // Akceptacja
    assert_eq!(manager.lock_trade(), Ok(()));
    assert_eq!(manager.my_locked, true);

    // Proba modyfikacji zablokowanego handlu
    assert_eq!(
        manager.set_my_gold(200_000),
        Err("Handel jest zablokowany, nie mozna zmienic zlota.")
    );

    let item2 = InventoryItem::new(1002, 5002, 1);
    assert_eq!(
        manager.add_my_item(1, item2),
        Err("Handel jest zablokowany, nie mozna dodac przedmiotu.")
    );

    // Zakonczenie (anulowanie)
    manager.cancel_trade();
    assert_eq!(manager.is_active, false);
    assert_eq!(manager.my_locked, false);
    assert_eq!(manager.my_gold, 0);
    assert_eq!(manager.my_items[0], None);
}

#[test]
fn test_partner_trade() {
    let mut manager = TradeManager::new();
    manager.start_trade();

    let item = InventoryItem::new(2001, 6001, 1);
    assert_eq!(manager.add_partner_item(5, item.clone()), Ok(()));
    assert_eq!(manager.partner_items[5], Some(item));

    assert_eq!(manager.set_partner_gold(50_000), Ok(()));
    assert_eq!(manager.partner_gold, 50_000);

    assert_eq!(manager.lock_partner_trade(), Ok(()));
    assert_eq!(manager.partner_locked, true);

    assert_eq!(
        manager.set_partner_gold(100_000),
        Err("Handel partnera jest zablokowany, nie mozna zmienic zlota.")
    );
}

#[test]
fn test_trade_ffi_cycle() {
    unsafe {
        // Przed inicjalizacja powinno byc false
        assert_eq!(m2_trade_is_locked(), false);

        // Tworzymy menedzera
        let ptr = m2_trade_manager_new();
        assert!(!ptr.is_null());
        
        let global_ptr = std::ptr::addr_of!(GLOBAL_TRADE_MANAGER).read();
        assert_eq!(global_ptr, ptr);

        // Po inicjalizacji dalej false (niezablokowane)
        assert_eq!(m2_trade_is_locked(), false);

        // Aktywujemy handel i blokujemy bezposrednio przez strukture
        (*ptr).start_trade();
        let _ = (*ptr).lock_trade();

        // Teraz FFI powinno zwrocic true
        assert_eq!(m2_trade_is_locked(), true);

        // Zwalniamy
        m2_trade_manager_free(ptr);
        
        // Zabezpieczenie przed panika z NULL zostalo obsluzone
        let global_ptr_after = std::ptr::addr_of!(GLOBAL_TRADE_MANAGER).read();
        assert_eq!(global_ptr_after, std::ptr::null_mut());
        assert_eq!(m2_trade_is_locked(), false);
    }
}