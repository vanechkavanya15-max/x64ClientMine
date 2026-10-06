#[path = "../src/shop_transaction_processor_v2.rs"]
mod shop_transaction_processor_v2;

use shop_transaction_processor_v2::{
    INVENTORY_MAX_NUM, ShopCartItem, ShopCoinType, ShopTransactionError, ShopTransactionProcessor,
};

#[test]
fn test_build_shop_price() {
    let mut processor = ShopTransactionProcessor::new();

    processor.add_item(ShopCartItem {
        vnum: 10,
        price: 500,
        count: 1,
        item_size: 1,
        coin_type: ShopCoinType::Gold,
    }).unwrap();

    processor.add_item(ShopCartItem {
        vnum: 20,
        price: 1500,
        count: 1,
        item_size: 2,
        coin_type: ShopCoinType::Gold,
    }).unwrap();

    processor.add_item(ShopCartItem {
        vnum: 30,
        price: 50,
        count: 1,
        item_size: 1,
        coin_type: ShopCoinType::SecondaryCoin,
    }).unwrap();

    assert_eq!(processor.build_shop_price(ShopCoinType::Gold), 2000);
    assert_eq!(processor.build_shop_price(ShopCoinType::SecondaryCoin), 50);
}

#[test]
fn test_can_buy_item_gold_and_secondary_coin() {
    let inventory = [None; INVENTORY_MAX_NUM];

    let item_gold = ShopCartItem {
        vnum: 10,
        price: 1000,
        count: 1,
        item_size: 1,
        coin_type: ShopCoinType::Gold,
    };

    assert_eq!(
        ShopTransactionProcessor::can_buy_item(900, 0, &inventory, &item_gold),
        Err(ShopTransactionError::NotEnoughGold)
    );
    assert_eq!(
        ShopTransactionProcessor::can_buy_item(1000, 0, &inventory, &item_gold),
        Ok(())
    );

    let item_secondary = ShopCartItem {
        vnum: 20,
        price: 50,
        count: 1,
        item_size: 1,
        coin_type: ShopCoinType::SecondaryCoin,
    };

    assert_eq!(
        ShopTransactionProcessor::can_buy_item(0, 40, &inventory, &item_secondary),
        Err(ShopTransactionError::NotEnoughSecondaryCoin)
    );
    assert_eq!(
        ShopTransactionProcessor::can_buy_item(0, 50, &inventory, &item_secondary),
        Ok(())
    );
}

#[test]
fn test_can_buy_items_inventory_full() {
    let mut processor = ShopTransactionProcessor::new();

    // Fill the cart with 30 size-3 items. This requires 90 slots total.
    for _ in 0..30 {
        processor.add_item(ShopCartItem {
            vnum: 10,
            price: 10,
            count: 1,
            item_size: 3,
            coin_type: ShopCoinType::Gold,
        }).unwrap();
    }

    let mut inventory = [None; INVENTORY_MAX_NUM];

    // Buying should succeed since 30 * 3 = 90 slots are exactly available.
    assert_eq!(processor.can_buy_items(1000, 0, &inventory), Ok(()));

    // Fill just ONE slot in the inventory, making it impossible to fit 30 size-3 items.
    inventory[0] = Some(999);
    assert_eq!(
        processor.can_buy_items(1000, 0, &inventory),
        Err(ShopTransactionError::InventoryFull)
    );
}

#[test]
fn test_cart_size_limit() {
    let mut processor = ShopTransactionProcessor::new();

    for _ in 0..40 {
        assert_eq!(
            processor.add_item(ShopCartItem {
                vnum: 10,
                price: 10,
                count: 1,
                item_size: 1,
                coin_type: ShopCoinType::Gold,
            }),
            Ok(())
        );
    }

    // 41st item should fail
    assert_eq!(
        processor.add_item(ShopCartItem {
            vnum: 10,
            price: 10,
            count: 1,
            item_size: 1,
            coin_type: ShopCoinType::Gold,
        }),
        Err(ShopTransactionError::CartFull)
    );
}

#[test]
fn test_clear_cart() {
    let mut processor = ShopTransactionProcessor::new();

    processor.add_item(ShopCartItem {
        vnum: 10,
        price: 10,
        count: 1,
        item_size: 1,
        coin_type: ShopCoinType::Gold,
    }).unwrap();

    assert_eq!(processor.cart.len(), 1);
    processor.clear();
    assert_eq!(processor.cart.len(), 0);
    assert_eq!(processor.build_shop_price(ShopCoinType::Gold), 0);
}

#[test]
fn test_inventory_grid_edge_cases() {
    let inventory = [None; INVENTORY_MAX_NUM];

    // Invalid item size 0
    let mut processor = ShopTransactionProcessor::new();
    assert_eq!(
        processor.add_item(ShopCartItem {
            vnum: 10,
            price: 10,
            count: 1,
            item_size: 0,
            coin_type: ShopCoinType::Gold,
        }),
        Err(ShopTransactionError::InvalidItemSize)
    );

    // Invalid item size 4
    assert_eq!(
        processor.add_item(ShopCartItem {
            vnum: 10,
            price: 10,
            count: 1,
            item_size: 4,
            coin_type: ShopCoinType::Gold,
        }),
        Err(ShopTransactionError::InvalidItemSize)
    );
    
    // Simulate fragmented inventory
    let mut fragmented = [None; INVENTORY_MAX_NUM];
    // Fill every other slot
    for i in 0..INVENTORY_MAX_NUM {
        if i % 2 == 0 {
            fragmented[i] = Some(1);
        }
    }
    
    // A size 2 item needs 2 consecutive vertical slots.
    // In a 5-column grid with alternating full/empty slots, it's impossible to find 2 consecutive vertical empty slots because:
    // row 0 col c is empty -> index is odd -> col c is odd
    // row 1 col c is empty -> index is odd -> index = 5*1 + col = 5+col is odd -> col c is even
    // Contradiction, so no size 2 item can fit.
    let item_size_2 = ShopCartItem {
        vnum: 10,
        price: 10,
        count: 1,
        item_size: 2,
        coin_type: ShopCoinType::Gold,
    };
    
    assert_eq!(
        ShopTransactionProcessor::can_buy_item(100, 0, &fragmented, &item_size_2),
        Err(ShopTransactionError::InventoryFull)
    );
}
