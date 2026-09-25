use m2_client_bridge::state::dragon_soul::{DragonSoulManager, DRAGON_SOUL_MGR};
use m2_client_bridge::state::inventory_item::InventoryItem;

#[test]
fn test_ds_new() {
    let ds = DragonSoulManager::new();
    assert_eq!(ds.active_deck, 0);
    assert_eq!(DRAGON_SOUL_MGR, 0);
    
    for i in 0..6 {
        assert!(ds.get_ds_item(0, i).is_none());
        assert!(ds.get_ds_item(1, i).is_none());
    }
}

#[test]
fn test_ds_switch_deck() {
    let mut ds = DragonSoulManager::new();
    
    // Zmiana na poprawny deck
    assert!(ds.switch_deck(1).is_ok());
    assert_eq!(ds.active_deck, 1);
    
    // Zmiana na poprawny deck
    assert!(ds.switch_deck(0).is_ok());
    assert_eq!(ds.active_deck, 0);
    
    // Zmiana na niepoprawny deck
    assert!(ds.switch_deck(2).is_err());
    assert_eq!(ds.active_deck, 0); // Pozostaje bez zmian
}

#[test]
fn test_ds_set_item() {
    let mut ds = DragonSoulManager::new();
    let item = InventoryItem::new(1001, 50300, 1);
    
    assert!(ds.set_ds_item(0, 0, item.clone()).is_ok());
    
    let retrieved_item = ds.get_ds_item(0, 0).expect("Oczekiwano przedmiotu w slocie");
    assert_eq!(retrieved_item.vnum, 50300);
    assert_eq!(retrieved_item.vid, 1001);
}

#[test]
fn test_ds_get_item() {
    let mut ds = DragonSoulManager::new();
    let item = InventoryItem::new(1002, 50400, 1);
    
    ds.set_ds_item(1, 5, item).unwrap();
    
    let retrieved_item = ds.get_ds_item(1, 5);
    assert!(retrieved_item.is_some());
    assert_eq!(retrieved_item.unwrap().vnum, 50400);
    
    let empty_item = ds.get_ds_item(1, 4);
    assert!(empty_item.is_none());
}

#[test]
fn test_ds_remove_item() {
    let mut ds = DragonSoulManager::new();
    let item = InventoryItem::new(1003, 50500, 1);
    
    ds.set_ds_item(0, 2, item).unwrap();
    
    let removed_item = ds.remove_ds_item(0, 2);
    assert!(removed_item.is_some());
    assert_eq!(removed_item.unwrap().vnum, 50500);
    
    let check_empty = ds.get_ds_item(0, 2);
    assert!(check_empty.is_none());
}

#[test]
fn test_ds_out_of_bounds() {
    let mut ds = DragonSoulManager::new();
    let item = InventoryItem::new(1004, 50600, 1);
    
    // Przekroczenie decku
    assert!(ds.set_ds_item(2, 0, item.clone()).is_err());
    assert!(ds.get_ds_item(2, 0).is_none());
    assert!(ds.remove_ds_item(2, 0).is_none());
    
    // Przekroczenie slotu
    assert!(ds.set_ds_item(0, 6, item.clone()).is_err());
    assert!(ds.get_ds_item(0, 6).is_none());
    assert!(ds.remove_ds_item(0, 6).is_none());
}
