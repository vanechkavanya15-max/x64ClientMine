use m2_client_bridge::state::special_storage::{
    m2_special_storage_count, m2_special_storage_free, m2_special_storage_new, SpecialStorage,
    StorageCategory, SPECIAL_STORAGE,
};
use m2_client_bridge::state::inventory_item::InventoryItem;

#[test]
fn test_special_storage_creation() {
    let storage = SpecialStorage::new();
    assert_eq!(storage.get_count_for_category(StorageCategory::UpgradeItems), 0);
    assert_eq!(storage.get_count_for_category(StorageCategory::BookItems), 0);
    assert_eq!(storage.get_count_for_category(StorageCategory::StoneItems), 0);
    assert_eq!(SPECIAL_STORAGE, 0);
}

#[test]
fn test_special_storage_add_item() {
    let mut storage = SpecialStorage::new();

    let item1 = InventoryItem::new(1, 100, 1);
    let item2 = InventoryItem::new(2, 200, 2);
    let item3 = InventoryItem::new(3, 300, 3);

    storage.add_item(StorageCategory::UpgradeItems, 0, item1.clone());
    storage.add_item(StorageCategory::UpgradeItems, 1, item2.clone());

    storage.add_item(StorageCategory::BookItems, 0, item3.clone());

    assert_eq!(storage.get_count_for_category(StorageCategory::UpgradeItems), 2);
    assert_eq!(storage.get_count_for_category(StorageCategory::BookItems), 1);
    assert_eq!(storage.get_count_for_category(StorageCategory::StoneItems), 0);
}

#[test]
fn test_special_storage_ffi_count() {
    // Poniewaz count dziala z GLOBAL_STORAGE,
    // ffi count nie powinnien panikowac jezeli podamy wlasciwa / niewlasciwa wartosc
    let count = unsafe { m2_special_storage_count(0) };
    assert_eq!(count, 0); // Oczekujemy 0, bo GLOBAL_STORAGE = None

    let count_invalid = unsafe { m2_special_storage_count(99) };
    assert_eq!(count_invalid, 0);
}

#[test]
fn test_special_storage_ffi_free() {
    // Sprawdza czy utworzenie i zwolnienie pamieci za pomoca FFI przechodzi pomyslnie i nie panikuje
    let ptr = unsafe { m2_special_storage_new() };
    assert!(!ptr.is_null());

    unsafe {
        m2_special_storage_free(ptr);
    }
}
