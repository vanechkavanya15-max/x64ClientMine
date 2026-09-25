use m2_client_bridge::state::safebox::{Item, SafeboxManager};

#[test]
fn test_open_safebox_success() {
    let mut manager = SafeboxManager::new();
    let vid = 1;
    let size = 45; // Standard safebox size
    manager.open_safebox(vid, size);
    
    // Safebox should be opened and empty
    for i in 0..size {
        assert_eq!(manager.get_item(vid, i), None);
    }
    assert_eq!(manager.get_gold(vid), 0);
}

#[test]
fn test_add_and_get_item_success() {
    let mut manager = SafeboxManager::new();
    let vid = 1;
    let size = 15;
    manager.open_safebox(vid, size);

    let item = Item { vnum: 27001, count: 200 }; // Red Potion
    assert_eq!(manager.add_item(vid, 5, item.clone()), Ok(()));

    let retrieved = manager.get_item(vid, 5);
    assert_eq!(retrieved, Some(item));
}

#[test]
fn test_remove_item_success() {
    let mut manager = SafeboxManager::new();
    let vid = 1;
    let size = 15;
    manager.open_safebox(vid, size);

    let item = Item { vnum: 19, count: 1 }; // Sword+9
    assert_eq!(manager.add_item(vid, 0, item), Ok(()));
    assert!(manager.get_item(vid, 0).is_some());

    assert_eq!(manager.remove_item(vid, 0), Ok(()));
    assert_eq!(manager.get_item(vid, 0), None);
}

#[test]
fn test_gold_transactions_success() {
    let mut manager = SafeboxManager::new();
    let vid = 2;
    manager.open_safebox(vid, 45);

    assert_eq!(manager.get_gold(vid), 0);

    assert_eq!(manager.add_gold(vid, 10000), Ok(()));
    assert_eq!(manager.get_gold(vid), 10000);

    assert_eq!(manager.sub_gold(vid, 2500), Ok(()));
    assert_eq!(manager.get_gold(vid), 7500);

    // Large transactions
    assert_eq!(manager.add_gold(vid, 2_000_000_000), Ok(()));
    assert_eq!(manager.get_gold(vid), 2_000_007_500);
}

#[test]
fn test_add_item_invalid_vid_fails() {
    let mut manager = SafeboxManager::new();
    let invalid_vid = 999;
    
    let item = Item { vnum: 27001, count: 200 };
    let result = manager.add_item(invalid_vid, 0, item);
    assert_eq!(result, Err("Magazyn dla podanego VID nie istnieje".to_string()));
}

#[test]
fn test_add_item_out_of_bounds_fails() {
    let mut manager = SafeboxManager::new();
    let vid = 1;
    let size = 5;
    manager.open_safebox(vid, size);

    let item = Item { vnum: 27001, count: 200 };
    // Try to add at index 5 (which is the 6th slot, out of bounds since size is 5)
    let result = manager.add_item(vid, 5, item);
    assert_eq!(result, Err("Index poza zakresem magazynu".to_string()));
}

#[test]
fn test_get_item_out_of_bounds_returns_none() {
    let mut manager = SafeboxManager::new();
    let vid = 1;
    let size = 15;
    manager.open_safebox(vid, size);

    // Get item at invalid index
    assert_eq!(manager.get_item(vid, 100), None);
}

#[test]
fn test_get_item_invalid_vid_returns_none() {
    let manager = SafeboxManager::new();
    assert_eq!(manager.get_item(999, 0), None);
}

#[test]
fn test_remove_item_out_of_bounds_fails() {
    let mut manager = SafeboxManager::new();
    let vid = 1;
    let size = 10;
    manager.open_safebox(vid, size);

    let result = manager.remove_item(vid, 10);
    assert_eq!(result, Err("Index poza zakresem magazynu".to_string()));
}

#[test]
fn test_sub_gold_insufficient_funds_fails() {
    let mut manager = SafeboxManager::new();
    let vid = 1;
    manager.open_safebox(vid, 45);

    manager.add_gold(vid, 100).unwrap();
    
    let result = manager.sub_gold(vid, 150);
    assert_eq!(result, Err("Niewystarczajaca ilosc zlota w magazynie".to_string()));
    
    // Balance shouldn't change
    assert_eq!(manager.get_gold(vid), 100);
}

#[test]
fn test_gold_invalid_vid_fails() {
    let mut manager = SafeboxManager::new();
    
    assert_eq!(manager.add_gold(999, 100), Err("Magazyn dla podanego VID nie istnieje".to_string()));
    assert_eq!(manager.sub_gold(999, 50), Err("Magazyn dla podanego VID nie istnieje".to_string()));
    assert_eq!(manager.get_gold(999), 0);
}
