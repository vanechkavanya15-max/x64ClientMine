#[path = "../src/inventory_grid.rs"]
mod inventory_grid;

use inventory_grid::{InventoryGrid, ItemSize, ItemData};

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
    
    // Strona 0, ostatni wiersz (index wiersza 8, slot 40, bo 8 * 5 = 40).
    // Przedmiot 1x1 na poczatku ostatniego wiersza miesci sie na stronie 0.
    assert!(grid.can_put_item(40, ItemSize::Size1x1 as u8));
    
    // Proba wstawienia przedmiotu 1x2 w ostatnim wierszu (slot 40).
    // Powinno sie NIE udac, poniewaz wychodzi poza strone 0 (8 + 2 = 10 > 9).
    assert!(!grid.can_put_item(40, ItemSize::Size1x2 as u8));
    
    // Proba wstawienia przedmiotu 1x3 na przedostatni wiersz strony 0 (wiersz 7, slot 35).
    // 7 + 3 = 10 > 9, czyli zajeloby 35, 40 i 45 (slot 0 na stronie 1) - zabronione!
    assert!(!grid.can_put_item(35, ItemSize::Size1x3 as u8));
    
    // Ale 1x2 na slocie 35 (wiersz 7) wejdzie legalnie (zajmie 35 i 40 w obrebie strony 0).
    assert!(grid.can_put_item(35, ItemSize::Size1x2 as u8));

    // Test dla poczatku strony 1 (slot 45)
    assert!(grid.can_put_item(45, ItemSize::Size1x3 as u8)); // zajmie 45, 50, 55 na stronie 1
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

#[test]
fn test_four_pages() {
    let mut grid = InventoryGrid::new();
    
    // 4 pages, each 45 slots. Let's test the start of each page.
    let item1 = ItemData { vnum: 10, count: 1, flags: 0, anti_flags: 0 };
    let item2 = ItemData { vnum: 20, count: 1, flags: 0, anti_flags: 0 };
    let item3 = ItemData { vnum: 30, count: 1, flags: 0, anti_flags: 0 };
    let item4 = ItemData { vnum: 40, count: 1, flags: 0, anti_flags: 0 };
    
    assert!(grid.set_item(0, item1)); // Page 1
    assert!(grid.set_item(45, item2)); // Page 2
    assert!(grid.set_item(90, item3)); // Page 3
    assert!(grid.set_item(135, item4)); // Page 4
    
    assert_eq!(grid.get_item(0).unwrap().vnum, 10);
    assert_eq!(grid.get_item(45).unwrap().vnum, 20);
    assert_eq!(grid.get_item(90).unwrap().vnum, 30);
    assert_eq!(grid.get_item(135).unwrap().vnum, 40);
    
    // 179 is the last slot (page 4, slot 44)
    assert!(grid.set_item(179, item1));
    assert_eq!(grid.get_item(179).unwrap().vnum, 10);
    
    // 180 is out of bounds
    assert!(!grid.set_item(180, item1));
}

#[test]
fn test_swap_items() {
    let mut grid = InventoryGrid::new();
    let item1 = ItemData { vnum: 100, count: 5, flags: 0, anti_flags: 0 };
    let item2 = ItemData { vnum: 200, count: 10, flags: 0, anti_flags: 0 };
    
    grid.set_item(5, item1);
    grid.set_item(10, item2);
    
    assert!(grid.swap_items(5, 10));
    
    assert_eq!(grid.get_item(5).unwrap().vnum, 200);
    assert_eq!(grid.get_item(10).unwrap().vnum, 100);
    
    // Swap with empty
    assert!(grid.swap_items(5, 15));
    assert!(grid.get_item(5).is_none());
    assert_eq!(grid.get_item(15).unwrap().vnum, 200);
}

#[test]
fn test_split_stack() {
    let mut grid = InventoryGrid::new();
    let item = ItemData { vnum: 500, count: 20, flags: 0, anti_flags: 0 };
    
    grid.set_item(0, item);
    
    // Valid split
    assert!(grid.split_stack(0, 1, 5));
    assert_eq!(grid.get_item(0).unwrap().count, 15);
    assert_eq!(grid.get_item(1).unwrap().count, 5);
    assert_eq!(grid.get_item(1).unwrap().vnum, 500);
    
    // Try to split more than available
    assert!(!grid.split_stack(0, 2, 20));
    
    // Try to split exactly available (should fail, standard behavior is swap/move for full count)
    assert!(!grid.split_stack(0, 2, 15));
    
    // Try to split 0
    assert!(!grid.split_stack(0, 2, 0));
    
    // Try to split to an occupied slot
    let item2 = ItemData { vnum: 600, count: 1, flags: 0, anti_flags: 0 };
    grid.set_item(2, item2);
    assert!(!grid.split_stack(0, 2, 5));
}
