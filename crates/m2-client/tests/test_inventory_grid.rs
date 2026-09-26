use m2_client::inventory_grid::{InventoryGrid, ItemSize};

#[test]
fn test_can_put_item_1x1() {
    let mut grid = InventoryGrid::new();
    // Powinno byc mozliwe wstawienie 1x1 na pusty poczatkowy slot
    assert!(grid.can_put_item(0, ItemSize::Size1x1 as u8));
    
    // Wstawienie i sprawdzenie
    assert!(grid.put_item(0, ItemSize::Size1x1 as u8).is_ok());
    
    // Proba wstawienia w to samo miejsce powinna zwrocic false
    assert!(!grid.can_put_item(0, ItemSize::Size1x1 as u8));
}

#[test]
fn test_can_put_item_1x3_and_collision() {
    let mut grid = InventoryGrid::new();
    
    // Wstawienie przedmiotu 1x3 na slot 0
    assert!(grid.can_put_item(0, ItemSize::Size1x3 as u8));
    assert!(grid.put_item(0, ItemSize::Size1x3 as u8).is_ok());
    
    // Przedmiot 1x3 zaczynajacy sie w slocie 0 zajmie sloty 0, 5, 10
    assert!(!grid.can_put_item(0, ItemSize::Size1x1 as u8));
    assert!(!grid.can_put_item(5, ItemSize::Size1x1 as u8));
    assert!(!grid.can_put_item(10, ItemSize::Size1x1 as u8));
    
    // Slot 1 powinien byc wolny
    assert!(grid.can_put_item(1, ItemSize::Size1x1 as u8));
}

#[test]
fn test_page_boundaries() {
    let grid = InventoryGrid::new();
    
    // Strona 0, ostatni wiersz (index wiersza 17). 
    // Przedmiot 1x1 na poczatku ostatniego wiersza (slot 85, bo 17 * 5 = 85).
    assert!(grid.can_put_item(85, ItemSize::Size1x1 as u8));
    
    // Proba wstawienia przedmiotu 1x2 w ostatnim wierszu (slot 85)
    // Powinno sie nie udac, poniewaz wychodzi poza pierwsza strone.
    assert!(!grid.can_put_item(85, ItemSize::Size1x2 as u8));
    
    // Proba wstawienia przedmiotu 1x3 na slot przedostatni wiersz (wiersz 16, slot 80)
    // Zajmuje 80, 85, 90. 90 to nowa strona, wiec blad.
    assert!(!grid.can_put_item(80, ItemSize::Size1x3 as u8));
    
    // Ale 1x2 na slocie 80 (wiersz 16) wejdzie (zajmie 80 i 85).
    assert!(grid.can_put_item(80, ItemSize::Size1x2 as u8));
}

#[test]
fn test_find_free_slot() {
    let mut grid = InventoryGrid::new();
    
    // Zapewnijmy zajecie kilku pierwszych slotow
    assert!(grid.put_item(0, ItemSize::Size1x1 as u8).is_ok());
    assert!(grid.put_item(1, ItemSize::Size1x1 as u8).is_ok());
    assert!(grid.put_item(2, ItemSize::Size1x1 as u8).is_ok());
    
    // Szukamy wolnego miejsca dla 1x1
    assert_eq!(grid.find_free_slot(ItemSize::Size1x1 as u8), Some(3));
    
    // Wstawmy 1x3 zaczynajac od slotu 3 (zajmie 3, 8, 13)
    assert!(grid.put_item(3, ItemSize::Size1x3 as u8).is_ok());
    
    // Szukamy wolnego miejsca dla 1x2
    // Sloty 0, 1, 2 sa zajete przez 1x1. Slot 3 przez 1x3.
    // Slot 4 powinien byc wolny, mozna tam wstawic 1x2 (zajmie 4, 9).
    assert_eq!(grid.find_free_slot(ItemSize::Size1x2 as u8), Some(4));
}

#[test]
fn test_remove_item() {
    let mut grid = InventoryGrid::new();
    
    assert!(grid.put_item(5, ItemSize::Size1x3 as u8).is_ok());
    assert!(!grid.can_put_item(5, ItemSize::Size1x1 as u8));
    assert!(!grid.can_put_item(10, ItemSize::Size1x1 as u8));
    assert!(!grid.can_put_item(15, ItemSize::Size1x1 as u8));
    
    grid.remove_item(5, ItemSize::Size1x3 as u8);
    
    assert!(grid.can_put_item(5, ItemSize::Size1x1 as u8));
    assert!(grid.can_put_item(10, ItemSize::Size1x1 as u8));
    assert!(grid.can_put_item(15, ItemSize::Size1x1 as u8));
}

#[test]
fn test_out_of_bounds() {
    let grid = InventoryGrid::new();
    
    // Total slots: 180. Slot 180 jest out of bounds (valid 0-179).
    assert!(!grid.can_put_item(180, ItemSize::Size1x1 as u8));
    
    // Invalid sizes
    assert!(!grid.can_put_item(0, 0));
    assert!(!grid.can_put_item(0, 4));
}
