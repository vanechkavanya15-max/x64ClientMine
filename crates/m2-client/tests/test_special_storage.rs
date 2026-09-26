#[path = "../src/special_storage.rs"]
mod special_storage;

use special_storage::{Item, ItemCategory, SpecialStorage, SPECIAL_STORAGE_SIZE};

#[test]
fn test_special_storage_initialization() {
    let storage = SpecialStorage::new();

    // Sprawdzenie czy poczatkowo jest maksimum wolnego miejsca we wszystkich kategoriach
    assert_eq!(storage.get_free_space(ItemCategory::SkillBook), SPECIAL_STORAGE_SIZE);
    assert_eq!(storage.get_free_space(ItemCategory::UpgradeItems), SPECIAL_STORAGE_SIZE);
    assert_eq!(storage.get_free_space(ItemCategory::Stone), SPECIAL_STORAGE_SIZE);
}

#[test]
fn test_auto_sort_skill_book() {
    let mut storage = SpecialStorage::new();
    let book = Item { category: ItemCategory::SkillBook };

    // Dodawanie pierwszej ksiegi
    let result = storage.auto_sort_item(book);
    assert_eq!(result, Ok(0)); // Powinna trafic do slotu 0

    // Sprawdzenie czy miejsce sie zmniejszylo w dobrej kategorii
    assert_eq!(storage.get_free_space(ItemCategory::SkillBook), SPECIAL_STORAGE_SIZE - 1);
    
    // Sprawdzenie ze inne kategorie nie ulegly zmianie
    assert_eq!(storage.get_free_space(ItemCategory::UpgradeItems), SPECIAL_STORAGE_SIZE);
    assert_eq!(storage.get_free_space(ItemCategory::Stone), SPECIAL_STORAGE_SIZE);
}

#[test]
fn test_auto_sort_upgrade_items() {
    let mut storage = SpecialStorage::new();
    let upgrade_item = Item { category: ItemCategory::UpgradeItems };

    let result = storage.auto_sort_item(upgrade_item);
    assert_eq!(result, Ok(0)); 

    assert_eq!(storage.get_free_space(ItemCategory::SkillBook), SPECIAL_STORAGE_SIZE);
    assert_eq!(storage.get_free_space(ItemCategory::UpgradeItems), SPECIAL_STORAGE_SIZE - 1);
    assert_eq!(storage.get_free_space(ItemCategory::Stone), SPECIAL_STORAGE_SIZE);
}

#[test]
fn test_auto_sort_stone() {
    let mut storage = SpecialStorage::new();
    let stone = Item { category: ItemCategory::Stone };

    let result = storage.auto_sort_item(stone);
    assert_eq!(result, Ok(0));

    assert_eq!(storage.get_free_space(ItemCategory::SkillBook), SPECIAL_STORAGE_SIZE);
    assert_eq!(storage.get_free_space(ItemCategory::UpgradeItems), SPECIAL_STORAGE_SIZE);
    assert_eq!(storage.get_free_space(ItemCategory::Stone), SPECIAL_STORAGE_SIZE - 1);
}

#[test]
fn test_storage_full_edge_case() {
    let mut storage = SpecialStorage::new();
    let book = Item { category: ItemCategory::SkillBook };

    // Wypelnienie calej kategorii
    for i in 0..SPECIAL_STORAGE_SIZE {
        let result = storage.auto_sort_item(book);
        assert_eq!(result, Ok(i));
    }

    // Proba dodania kolejnego przedmiotu poza limitem
    let result = storage.auto_sort_item(book);
    assert_eq!(result, Err("Brak wolnego miejsca w magazynie specjalnym"));
    
    // Sprawdzenie czy miejsca nie ma
    assert_eq!(storage.get_free_space(ItemCategory::SkillBook), 0);
}

#[test]
fn test_put_and_remove_item() {
    let mut storage = SpecialStorage::new();
    let stone = Item { category: ItemCategory::Stone };

    // Umieszczenie reczne w slocie 10
    let put_result = storage.put_item(ItemCategory::Stone, 10, stone);
    assert_eq!(put_result, Ok(()));
    
    // Proba umieszczenia w zajetym slocie
    let put_fail = storage.put_item(ItemCategory::Stone, 10, stone);
    assert_eq!(put_fail, Err("Slot jest juz zajety"));

    // Sprawdzenie wolnego miejsca
    assert_eq!(storage.get_free_space(ItemCategory::Stone), SPECIAL_STORAGE_SIZE - 1);

    // Usuniecie przedmiotu ze slotu 10
    let removed_item = storage.remove_item(ItemCategory::Stone, 10);
    assert!(removed_item.is_some());

    // Sprawdzenie wolnego miejsca
    assert_eq!(storage.get_free_space(ItemCategory::Stone), SPECIAL_STORAGE_SIZE);
}

#[test]
fn test_index_bound_checking() {
    let mut storage = SpecialStorage::new();
    let book = Item { category: ItemCategory::SkillBook };

    // Proba dodania do slotu poza zakresem (45)
    let put_fail = storage.put_item(ItemCategory::SkillBook, SPECIAL_STORAGE_SIZE, book);
    assert_eq!(put_fail, Err("Index poza zakresem"));

    // Proba usuniecia ze slotu poza zakresem
    let removed_item = storage.remove_item(ItemCategory::SkillBook, SPECIAL_STORAGE_SIZE);
    assert!(removed_item.is_none());
}

#[test]
fn test_remove_from_empty_slot() {
    let mut storage = SpecialStorage::new();
    
    // Usuniecie z pustego slotu w limicie
    let removed_item = storage.remove_item(ItemCategory::UpgradeItems, 5);
    assert!(removed_item.is_none());
}
