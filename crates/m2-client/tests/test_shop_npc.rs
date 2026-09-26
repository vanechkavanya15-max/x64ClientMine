#[path = "../src/shop_npc.rs"]
mod shop_npc;

use shop_npc::{ShopItemInfo, ShopError, ShopNpc};

#[test]
fn test_buy_item_success() {
    let mut npc = ShopNpc::new();
    npc.shop_items.push(ShopItemInfo {
        slot: 1,
        vnum: 100,
        count: 1,
        price: 500,
    });

    let mut inventory = vec![None, None, None];
    
    let result = npc.buy_item(1, 1000, &mut inventory);
    
    assert_eq!(result, Ok(500)); // Remaining gold
    assert_eq!(inventory, vec![Some(100), None, None]);
}

#[test]
fn test_buy_item_not_enough_gold() {
    let mut npc = ShopNpc::new();
    npc.shop_items.push(ShopItemInfo {
        slot: 1,
        vnum: 100,
        count: 1,
        price: 500,
    });

    let mut inventory = vec![None, None, None];
    
    let result = npc.buy_item(1, 300, &mut inventory);
    
    assert_eq!(result, Err(ShopError::NotEnoughGold));
    assert_eq!(inventory, vec![None, None, None]);
}

#[test]
fn test_buy_item_inventory_full() {
    let mut npc = ShopNpc::new();
    npc.shop_items.push(ShopItemInfo {
        slot: 1,
        vnum: 100,
        count: 1,
        price: 500,
    });

    let mut inventory = vec![Some(10), Some(20), Some(30)];
    
    let result = npc.buy_item(1, 1000, &mut inventory);
    
    assert_eq!(result, Err(ShopError::InventoryFull));
    assert_eq!(inventory, vec![Some(10), Some(20), Some(30)]);
}

#[test]
fn test_buy_item_invalid_slot() {
    let mut npc = ShopNpc::new();
    npc.shop_items.push(ShopItemInfo {
        slot: 1,
        vnum: 100,
        count: 1,
        price: 500,
    });

    let mut inventory = vec![None, None, None];
    
    let result = npc.buy_item(99, 1000, &mut inventory);
    
    assert_eq!(result, Err(ShopError::InvalidSlot));
}

#[test]
fn test_sell_item_standard() {
    let mut npc = ShopNpc::new();
    npc.inventory = vec![Some(100)];
    npc.item_sell_prices.insert(100, 1000);
    // flag not set for 1gold per count
    npc.item_flags.insert(100, 0);

    // standard sell: price = count * sell_price
    // final: (2 * 1000) / 5 = 400
    let value = npc.sell_item(0, 2);
    assert_eq!(value, 400);
}

#[test]
fn test_sell_item_count_per_1gold() {
    let mut npc = ShopNpc::new();
    npc.inventory = vec![Some(100)];
    npc.item_sell_prices.insert(100, 10);
    // Set ITEM_FLAG_COUNT_PER_1GOLD (1 << 3)
    npc.item_flags.insert(100, 1 << 3);

    // count_per_1gold sell: price = count / sell_price
    // final: (5000 / 10) / 5 = 100
    let value = npc.sell_item(0, 5000);
    assert_eq!(value, 100);
}

#[test]
fn test_sell_item_invalid_slot() {
    let npc = ShopNpc::new();
    let value = npc.sell_item(0, 1);
    assert_eq!(value, 0);
}

#[test]
fn test_sell_item_empty_slot() {
    let mut npc = ShopNpc::new();
    npc.inventory = vec![None];
    let value = npc.sell_item(0, 1);
    assert_eq!(value, 0);
}
