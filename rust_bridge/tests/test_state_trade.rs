use m2_client_bridge::state::trade::{TradeState, TradeItem, TRADE_MAX_NUM};

#[test]
fn test_trade_initialization() {
    let state = TradeState::new();
    assert_eq!(state.is_open, false);
    assert_eq!(state.partner_vid, 0);
    assert_eq!(state.own_gold, 0);
    assert_eq!(state.partner_gold, 0);
    assert_eq!(state.is_own_locked, false);
    assert_eq!(state.is_partner_locked, false);

    for item in state.own_items.iter() {
        assert_eq!(*item, None);
    }
    for item in state.partner_items.iter() {
        assert_eq!(*item, None);
    }
}

#[test]
fn test_trade_open_close() {
    let mut state = TradeState::new();
    
    // Otwarcie z nieprawidlowym VID
    assert_eq!(state.open(0), Err("Nieprawidlowe VID partnera."));
    
    // Otwarcie z poprawnym VID
    assert_eq!(state.open(12345), Ok(()));
    assert_eq!(state.is_open, true);
    assert_eq!(state.partner_vid, 12345);

    // Otwarcie juz otwartego handlu
    assert_eq!(state.open(54321), Err("Handel jest juz otwarty."));

    // Zamkniecie handlu
    state.close();
    assert_eq!(state.is_open, false);
    assert_eq!(state.partner_vid, 0);
}

#[test]
fn test_manage_own_items() {
    let mut state = TradeState::new();
    state.open(111).unwrap();

    // Dodanie poprawnego przedmiotu
    assert_eq!(state.set_own_item(0, 5001, 10), Ok(()));
    assert_eq!(state.own_items[0], Some(TradeItem { vnum: 5001, count: 10 }));

    // Dodanie przedmiotu na zlym slocie
    assert_eq!(state.set_own_item(TRADE_MAX_NUM, 5002, 1), Err("Indeks slotu poza zakresem."));

    // Usuniecie przedmiotu
    assert_eq!(state.remove_own_item(0), Ok(()));
    assert_eq!(state.own_items[0], None);

    // Zamkniecie handlu usuwa przedmioty
    state.set_own_item(1, 5003, 5).unwrap();
    state.close();
    assert_eq!(state.own_items[1], None);
}

#[test]
fn test_manage_partner_items() {
    let mut state = TradeState::new();
    state.open(222).unwrap();

    // Dodanie poprawnego przedmiotu partnera
    assert_eq!(state.set_partner_item(5, 6001, 20), Ok(()));
    assert_eq!(state.partner_items[5], Some(TradeItem { vnum: 6001, count: 20 }));

    // Dodanie przedmiotu partnera na zlym slocie
    assert_eq!(state.set_partner_item(TRADE_MAX_NUM, 6002, 1), Err("Indeks slotu poza zakresem."));

    // Usuniecie przedmiotu partnera
    assert_eq!(state.remove_partner_item(5), Ok(()));
    assert_eq!(state.partner_items[5], None);
}

#[test]
fn test_manage_gold() {
    let mut state = TradeState::new();
    state.open(333).unwrap();

    assert_eq!(state.set_own_gold(100000), Ok(()));
    assert_eq!(state.own_gold, 100000);

    assert_eq!(state.set_partner_gold(50000), Ok(()));
    assert_eq!(state.partner_gold, 50000);
}

#[test]
fn test_locking() {
    let mut state = TradeState::new();
    state.open(444).unwrap();

    assert_eq!(state.lock_own(), Ok(()));
    assert_eq!(state.is_own_locked, true);

    // Zmiana zlota lub przedmiotu po blokadzie wlasnej powinna sie nie udac
    assert_eq!(state.set_own_gold(10), Err("Wlasna czesc handlu jest zablokowana."));
    assert_eq!(state.set_own_item(0, 100, 1), Err("Wlasna czesc handlu jest zablokowana."));
    assert_eq!(state.remove_own_item(0), Err("Wlasna czesc handlu jest zablokowana."));

    assert_eq!(state.unlock_own(), Ok(()));
    assert_eq!(state.is_own_locked, false);
    
    assert_eq!(state.set_own_gold(10), Ok(()));

    // To samo dla partnera
    assert_eq!(state.lock_partner(), Ok(()));
    assert_eq!(state.is_partner_locked, true);

    assert_eq!(state.set_partner_gold(20), Err("Czesc partnera jest zablokowana."));
    assert_eq!(state.set_partner_item(0, 100, 1), Err("Czesc partnera jest zablokowana."));
    assert_eq!(state.remove_partner_item(0), Err("Czesc partnera jest zablokowana."));

    assert_eq!(state.unlock_partner(), Ok(()));
    assert_eq!(state.is_partner_locked, false);
}

#[test]
fn test_not_open_errors() {
    let mut state = TradeState::new();

    // Proba wykonania akcji na nieotwartym handlu
    assert_eq!(state.set_own_item(0, 100, 1), Err("Handel nie jest otwarty."));
    assert_eq!(state.remove_own_item(0), Err("Handel nie jest otwarty."));
    assert_eq!(state.set_partner_item(0, 100, 1), Err("Handel nie jest otwarty."));
    assert_eq!(state.remove_partner_item(0), Err("Handel nie jest otwarty."));
    assert_eq!(state.set_own_gold(10), Err("Handel nie jest otwarty."));
    assert_eq!(state.set_partner_gold(10), Err("Handel nie jest otwarty."));
    assert_eq!(state.lock_own(), Err("Handel nie jest otwarty."));
    assert_eq!(state.unlock_own(), Err("Handel nie jest otwarty."));
    assert_eq!(state.lock_partner(), Err("Handel nie jest otwarty."));
    assert_eq!(state.unlock_partner(), Err("Handel nie jest otwarty."));
}
