use m2_client::offline_shop_catalog_engine::{
    CatalogItem, ItemCategory, OfflineShopCatalogEngine, SearchFilters, SortCriteria, SortOrder,
};
use m2_client::net_shop_private_packets::{HEADER_CG_SHOP, SHOP_SUB_BUY, TPacketCGShop};
use std::mem::size_of;

#[test]
fn test_catalog_filtering() {
    let mut engine = OfflineShopCatalogEngine::new();

    engine.add_item(CatalogItem {
        item_id: 1,
        shop_id: 10,
        name: "Sword+9".to_string(),
        category: ItemCategory::Weapon,
        level: 30,
        price: 15000,
        count: 1,
        pos: 0,
    });
    engine.add_item(CatalogItem {
        item_id: 2,
        shop_id: 11,
        name: "Armor+9".to_string(),
        category: ItemCategory::Armor,
        level: 34,
        price: 25000,
        count: 1,
        pos: 1,
    });
    engine.add_item(CatalogItem {
        item_id: 3,
        shop_id: 12,
        name: "Potion".to_string(),
        category: ItemCategory::Other,
        level: 1,
        price: 500,
        count: 200,
        pos: 2,
    });

    let mut filters = SearchFilters::default();
    
    // Filter by Category
    filters.category = Some(ItemCategory::Armor);
    let results = engine.search(&filters, None, None);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Armor+9");

    // Filter by Name Substring
    filters.category = None;
    filters.name_substring = Some("SWORD".to_string());
    let results = engine.search(&filters, None, None);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Sword+9");

    // Filter by Price Range
    filters.name_substring = None;
    filters.min_price = Some(10000);
    filters.max_price = Some(20000);
    let results = engine.search(&filters, None, None);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].price, 15000);

    // Filter by Level Range
    filters.min_price = None;
    filters.max_price = None;
    filters.min_level = Some(30);
    filters.max_level = Some(35);
    let results = engine.search(&filters, None, None);
    assert_eq!(results.len(), 2);
}

#[test]
fn test_catalog_sorting() {
    let mut engine = OfflineShopCatalogEngine::new();

    engine.add_item(CatalogItem {
        item_id: 1,
        shop_id: 1,
        name: "A".to_string(),
        category: ItemCategory::Other,
        level: 10,
        price: 300,
        count: 50,
        pos: 0,
    });
    engine.add_item(CatalogItem {
        item_id: 2,
        shop_id: 1,
        name: "B".to_string(),
        category: ItemCategory::Other,
        level: 5,
        price: 500,
        count: 10,
        pos: 1,
    });
    engine.add_item(CatalogItem {
        item_id: 3,
        shop_id: 1,
        name: "C".to_string(),
        category: ItemCategory::Other,
        level: 20,
        price: 100,
        count: 100,
        pos: 2,
    });

    let filters = SearchFilters::default();

    // Sort by Price Ascending
    let results = engine.search(&filters, Some(SortCriteria::Price), Some(SortOrder::Ascending));
    assert_eq!(results[0].price, 100);
    assert_eq!(results[1].price, 300);
    assert_eq!(results[2].price, 500);

    // Sort by Level Descending
    let results = engine.search(&filters, Some(SortCriteria::Level), Some(SortOrder::Descending));
    assert_eq!(results[0].level, 20);
    assert_eq!(results[1].level, 10);
    assert_eq!(results[2].level, 5);
    
    // Sort by Count Ascending
    let results = engine.search(&filters, Some(SortCriteria::Count), Some(SortOrder::Ascending));
    assert_eq!(results[0].count, 10);
    assert_eq!(results[1].count, 50);
    assert_eq!(results[2].count, 100);
}

#[test]
fn test_purchase_packet_generation() {
    let mut engine = OfflineShopCatalogEngine::new();

    engine.add_item(CatalogItem {
        item_id: 1234,
        shop_id: 5678,
        name: "Sword".to_string(),
        category: ItemCategory::Weapon,
        level: 1,
        price: 100,
        count: 1,
        pos: 42,
    });

    let packet = engine.generate_purchase_packet(5678, 1234);
    assert!(packet.is_some());
    
    let bytes = packet.unwrap();
    let expected_len = size_of::<TPacketCGShop>() + 2;
    assert_eq!(bytes.len(), expected_len);
    
    // Header should be 0x0801 (little endian)
    let header_bytes = &bytes[0..2];
    assert_eq!(header_bytes, &HEADER_CG_SHOP.to_ne_bytes());
    
    // Subheader should be 1
    let subheader_pos = 4;
    assert_eq!(bytes[subheader_pos], SHOP_SUB_BUY);
    
    // Count should be 1
    assert_eq!(bytes[5], 1);
    
    // Pos should be 42
    assert_eq!(bytes[6], 42);
}

#[test]
fn test_catalog_large_dataset() {
    let mut engine = OfflineShopCatalogEngine::new();

    // Generate exactly 1000 items
    for i in 0..1000 {
        let category = match i % 5 {
            0 => ItemCategory::Weapon,
            1 => ItemCategory::Armor,
            2 => ItemCategory::Jewelry,
            3 => ItemCategory::Upgrade,
            _ => ItemCategory::Other,
        };

        engine.add_item(CatalogItem {
            item_id: i,
            shop_id: i % 10,
            name: format!("Item_{}", i),
            category,
            level: i % 100, // Levels 0-99
            price: (i as u64) * 100, // Price 0, 100, 200, ... 99900
            count: i % 200,
            pos: (i % 40) as u8,
        });
    }

    // Test filtering on the 1000 items
    let mut filters = SearchFilters::default();
    
    // All Weapons
    filters.category = Some(ItemCategory::Weapon);
    let results = engine.search(&filters, None, None);
    assert_eq!(results.len(), 200); // 1000 / 5 = 200

    // Weapons with price between 1000 and 5000
    filters.min_price = Some(1000);
    filters.max_price = Some(5000);
    let results = engine.search(&filters, None, None);
    
    // i % 5 == 0 && i * 100 >= 1000 && i * 100 <= 5000
    // => i % 5 == 0 && i >= 10 && i <= 50
    // => 10, 15, 20, 25, 30, 35, 40, 45, 50 (9 items)
    assert_eq!(results.len(), 9);

    // Sorting the filtered results
    let sorted_results = engine.search(&filters, Some(SortCriteria::Price), Some(SortOrder::Descending));
    assert_eq!(sorted_results.len(), 9);
    assert_eq!(sorted_results[0].price, 5000);
    assert_eq!(sorted_results[8].price, 1000);
}
