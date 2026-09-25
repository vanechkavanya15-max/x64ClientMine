use m2_client_bridge::state::inventory::{Inventory, InventoryError};

#[test]
fn test_inventory_new() {
    let inv = Inventory::new();
    // Testowanie domyslnego pustego stanu ekwipunku
    for i in 0..180 {
        assert_eq!(inv.get(i), Ok(0));
    }
}

#[test]
fn test_inventory_get_set() {
    let mut inv = Inventory::new();
    
    // Ustawienie i pobranie prawidlowego VID
    assert_eq!(inv.set(0, 100), Ok(()));
    assert_eq!(inv.get(0), Ok(100));

    // Zmiana i sprawdzenie innego slota
    assert_eq!(inv.set(179, 200), Ok(()));
    assert_eq!(inv.get(179), Ok(200));

    // Resetowanie (czyszczenie slota)
    assert_eq!(inv.set(0, 0), Ok(()));
    assert_eq!(inv.get(0), Ok(0));
}

#[test]
fn test_inventory_out_of_bounds() {
    let mut inv = Inventory::new();

    // Sprawdzenie get poza zasiegiem
    assert_eq!(inv.get(180), Err(InventoryError::InvalidIndex));
    assert_eq!(inv.get(999), Err(InventoryError::InvalidIndex));

    // Sprawdzenie set poza zasiegiem
    assert_eq!(inv.set(180, 100), Err(InventoryError::InvalidIndex));
    assert_eq!(inv.set(999, 100), Err(InventoryError::InvalidIndex));

    // Sprawdzenie swap z pierwszym zlym indeksem
    assert_eq!(inv.swap(180, 0), Err(InventoryError::InvalidIndex));

    // Sprawdzenie swap z drugim zlym indeksem
    assert_eq!(inv.swap(0, 180), Err(InventoryError::InvalidIndex));

    // Sprawdzenie swap z oboma zlymi indeksami
    assert_eq!(inv.swap(180, 181), Err(InventoryError::InvalidIndex));
}

#[test]
fn test_inventory_swap() {
    let mut inv = Inventory::new();
    
    inv.set(10, 500).unwrap();
    inv.set(20, 600).unwrap();

    // Wymiana przedmiotow
    assert_eq!(inv.swap(10, 20), Ok(()));

    // Sprawdzenie czy pozycje sie zmienily
    assert_eq!(inv.get(10), Ok(600));
    assert_eq!(inv.get(20), Ok(500));

    // Wymiana ze soba samym - nie powinno zmienic niczego, nie powinno zglosic bledu
    assert_eq!(inv.swap(10, 10), Ok(()));
    assert_eq!(inv.get(10), Ok(600));
}

#[test]
fn test_inventory_find_empty() {
    let mut inv = Inventory::new();
    
    // Na poczatku slot 0 jest pusty
    assert_eq!(inv.find_empty(), Some(0));

    // Po zajeciu slota 0, slot 1 powinien byc piewszym pustym
    inv.set(0, 10).unwrap();
    assert_eq!(inv.find_empty(), Some(1));

    // Po zajeciu wszystkich slotow, funkcja powinna zwrocic None
    for i in 0..180 {
        inv.set(i, 999).unwrap();
    }
    assert_eq!(inv.find_empty(), None);

    // Po zwolnieniu wybranego slota (np. 50), funkcja powinna go zwrocic
    inv.set(50, 0).unwrap();
    assert_eq!(inv.find_empty(), Some(50));
}
