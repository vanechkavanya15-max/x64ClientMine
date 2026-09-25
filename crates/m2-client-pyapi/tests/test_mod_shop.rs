use m2_client_pyapi::mod_shop::{ModShop, ShopItem};

#[test]
fn test_integration_mod_shop() {
    let mut instance = ModShop::new();
    assert!(instance.is_ready());
    assert!(!instance.is_open());

    instance.open();
    assert!(instance.is_open());

    instance.set_item(
        0,
        ShopItem {
            vnum: 100,
            count: 1,
            price: 500,
        },
    );

    instance.set_item(
        1,
        ShopItem {
            vnum: 200,
            count: 5,
            price: 1500,
        },
    );

    assert_eq!(instance.get_item_vnum(0), 100);
    assert_eq!(instance.get_item_count(0), 1);
    assert_eq!(instance.get_item_price(0), 500);

    assert_eq!(instance.get_item_vnum(1), 200);
    assert_eq!(instance.get_item_count(1), 5);
    assert_eq!(instance.get_item_price(1), 1500);

    // Test default for unpopulated slot
    assert_eq!(instance.get_item_vnum(2), 0);
    assert_eq!(instance.get_item_count(2), 0);
    assert_eq!(instance.get_item_price(2), 0);

    instance.close();
    assert!(!instance.is_open());

    // Test clear on close
    assert_eq!(instance.get_item_vnum(0), 0);
    assert_eq!(instance.get_item_count(0), 0);
    assert_eq!(instance.get_item_price(0), 0);

    instance.open();
    instance.set_item(
        3,
        ShopItem {
            vnum: 300,
            count: 10,
            price: 5000,
        },
    );

    assert_eq!(instance.get_item_vnum(3), 300);

    instance.clear();
    assert_eq!(instance.get_item_vnum(3), 0);
}
