// Testy okna paska szybkiego dostepu (QuickSlot) dla Metin2.

use m2_client::ui_window_quickslot::{QuickSlotWindow, TQuickSlot, SLOT_TYPE_INVENTORY, SLOT_TYPE_SKILL, QUICKSLOT_MAX_NUM_SERVER};

#[test]
fn test_quickslot_initialization() {
    let window = QuickSlotWindow::new();
    
    // Sprawdz czy strona poczatkowa to 0
    assert_eq!(window.active_page, 0);
    
    // Sprawdz czy wszystkie sloty sa puste
    for slot in window.slots.iter() {
        assert_eq!(slot.slot_type, 0);
        let pos = slot.slot_position;
        assert_eq!(pos, 0);
    }
}

#[test]
fn test_set_page() {
    let mut window = QuickSlotWindow::new();
    
    // Zmiana na prawidlowe strony (0-3)
    window.set_page(1);
    assert_eq!(window.active_page, 1);
    
    window.set_page(3);
    assert_eq!(window.active_page, 3);
    
    // Sprawdzanie granic z zawijaniem
    window.set_page(4); // np. klikniecie na strzalke 'dalej' za bardzo
    assert_eq!(window.active_page, 0); // Zakladajac modulo
    
    window.set_page(5);
    assert_eq!(window.active_page, 1);

    window.set_page(-1);
    assert_eq!(window.active_page, 3);
}

#[test]
fn test_get_global_index() {
    let mut window = QuickSlotWindow::new();
    
    // Strona 0, klawisze 1-4, F1-F4 -> local 0..7 -> global 0..7
    window.set_page(0);
    assert_eq!(window.get_global_index(0), 0);
    assert_eq!(window.get_global_index(7), 7);
    
    // Strona 1, klawisze 1-4, F1-F4 -> local 0..7 -> global 8..15
    window.set_page(1);
    assert_eq!(window.get_global_index(0), 8);
    assert_eq!(window.get_global_index(7), 15);
    
    // Strona 3 -> global 24..31
    window.set_page(3);
    assert_eq!(window.get_global_index(0), 24);
    assert_eq!(window.get_global_index(7), 31);
}

#[test]
fn test_add_and_use_quickslot() {
    let mut window = QuickSlotWindow::new();
    window.set_page(0); // Strona pierwsza
    
    // Przypisanie przedmiotu na pozycje lokalna 0 (pod klawiszem '1')
    window.add_quickslot(0, SLOT_TYPE_INVENTORY, 150);
    
    // Oczekujemy, ze pod globalnym slotem 0 znajduje sie item o pozycji 150
    assert_eq!(window.slots[0].slot_type, SLOT_TYPE_INVENTORY);
    let pos0 = window.slots[0].slot_position;
    assert_eq!(pos0, 150);
    
    // Symulacja uzycia przypisanego slota klawiszem (np. wcisniecie '1')
    let result = window.on_key_pressed(0x31, false);
    assert!(result.is_some());
    
    let used_slot = result.unwrap();
    assert_eq!(used_slot.slot_type, SLOT_TYPE_INVENTORY);
    let pos_used = used_slot.slot_position;
    assert_eq!(pos_used, 150);
}

#[test]
fn test_shift_number_changes_page() {
    let mut window = QuickSlotWindow::new();
    window.set_page(0);

    // Klawisz '2' bez shifta uzywa przedmiotu
    let result = window.on_key_pressed(0x32, false);
    assert!(result.is_none()); // Nic nie ma na tym slocie
    assert_eq!(window.active_page, 0); // Strona pozostaje niezmieniona
    
    // Klawisz '2' z shiftem (0x32, is_shift = true) zmienia strone
    let result2 = window.on_key_pressed(0x32, true);
    assert!(result2.is_none());
    assert_eq!(window.active_page, 1);
    
    // Klawisz '4' z shiftem (0x34)
    let _ = window.on_key_pressed(0x34, true);
    assert_eq!(window.active_page, 3);
}

#[test]
fn test_remove_quickslot() {
    let mut window = QuickSlotWindow::new();
    
    // Dodajemy element
    window.add_quickslot(2, SLOT_TYPE_SKILL, 10); // Dodane pod globalne index 2
    assert_eq!(window.slots[2].slot_type, SLOT_TYPE_SKILL);
    
    // Usuwamy element
    window.remove_quickslot(2);
    assert_eq!(window.slots[2].slot_type, 0);
    
    // Uzycie powinno zwrocic None
    let result = window.on_key_pressed(0x33, false); // Klawisz '3' -> lokalny index 2
    assert!(result.is_none());
}

#[test]
fn test_swap_quickslot() {
    let mut window = QuickSlotWindow::new();
    window.set_page(0);
    
    window.add_quickslot(0, SLOT_TYPE_INVENTORY, 100);
    window.add_quickslot(1, SLOT_TYPE_SKILL, 10);
    
    // Zamiana
    window.swap_quickslot(0, 1);
    
    assert_eq!(window.slots[0].slot_type, SLOT_TYPE_SKILL);
    let pos0 = window.slots[0].slot_position;
    assert_eq!(pos0, 10);
    
    assert_eq!(window.slots[1].slot_type, SLOT_TYPE_INVENTORY);
    let pos1 = window.slots[1].slot_position;
    assert_eq!(pos1, 100);
}
