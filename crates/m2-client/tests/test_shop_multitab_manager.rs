#[path = "../src/shop_multitab_manager.rs"]
mod shop_multitab_manager;

use shop_multitab_manager::{
    ShopMultitabManager, TItemPos, SHOP_COIN_TYPE_GOLD, SHOP_COIN_TYPE_SECONDARY_COIN,
    SHOP_TAB_COUNT_MAX,
};

#[test]
fn test_shop_multitab_manager_tabs() {
    let mut manager = ShopMultitabManager::new();
    assert_eq!(manager.get_tab_count(), 1);

    manager.set_tab_count(2);
    assert_eq!(manager.get_tab_count(), 2);

    manager.set_tab_count(10);
    assert_eq!(manager.get_tab_count(), SHOP_TAB_COUNT_MAX);
}

#[test]
fn test_private_shop_stock() {
    let mut manager = ShopMultitabManager::new();
    
    let pos1 = TItemPos { window_type: 1, cell: 0 };
    let pos2 = TItemPos { window_type: 1, cell: 1 };
    
    manager.add_private_shop_item_stock(pos1, 0, 1000, SHOP_COIN_TYPE_GOLD);
    manager.add_private_shop_item_stock(pos2, 1, 5, SHOP_COIN_TYPE_SECONDARY_COIN);
    
    assert_eq!(
        manager.get_private_shop_item_price_ex(pos1),
        Some((1000, SHOP_COIN_TYPE_GOLD))
    );
    assert_eq!(
        manager.get_private_shop_item_price_ex(pos2),
        Some((5, SHOP_COIN_TYPE_SECONDARY_COIN))
    );
    
    manager.del_private_shop_item_stock(pos1);
    assert_eq!(manager.get_private_shop_item_price_ex(pos1), None);
    
    manager.clear_private_shop_stock();
    assert_eq!(manager.get_private_shop_item_price_ex(pos2), None);
}

#[test]
fn test_calculate_total_price() {
    let mut manager = ShopMultitabManager::new();
    
    manager.add_private_shop_item_stock(
        TItemPos { window_type: 1, cell: 0 },
        0,
        1500,
        SHOP_COIN_TYPE_GOLD,
    );
    manager.add_private_shop_item_stock(
        TItemPos { window_type: 1, cell: 1 },
        1,
        2500,
        SHOP_COIN_TYPE_GOLD,
    );
    manager.add_private_shop_item_stock(
        TItemPos { window_type: 1, cell: 2 },
        2,
        10,
        SHOP_COIN_TYPE_SECONDARY_COIN,
    );
    manager.add_private_shop_item_stock(
        TItemPos { window_type: 1, cell: 3 },
        3,
        20,
        SHOP_COIN_TYPE_SECONDARY_COIN,
    );
    
    let (total_gold, total_sec) = manager.calculate_total_price();
    assert_eq!(total_gold, 4000);
    assert_eq!(total_sec, 30);
}

#[test]
fn test_build_private_shop() {
    let mut manager = ShopMultitabManager::new();
    
    manager.add_private_shop_item_stock(
        TItemPos { window_type: 1, cell: 5 },
        2, // display pos 2
        10,
        SHOP_COIN_TYPE_GOLD,
    );
    manager.add_private_shop_item_stock(
        TItemPos { window_type: 1, cell: 6 },
        0, // display pos 0
        20,
        SHOP_COIN_TYPE_GOLD,
    );
    manager.add_private_shop_item_stock(
        TItemPos { window_type: 1, cell: 7 },
        1, // display pos 1
        30,
        SHOP_COIN_TYPE_GOLD,
    );
    
    let items = manager.build_private_shop("TestShop");
    assert_eq!(items.len(), 3);
    assert_eq!(items[0].display_pos, 0);
    assert_eq!(items[1].display_pos, 1);
    assert_eq!(items[2].display_pos, 2);
    
    assert_eq!(items[0].price, 20);
    assert_eq!(items[1].price, 30);
    assert_eq!(items[2].price, 10);
}
