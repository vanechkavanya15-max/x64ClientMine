#[path = "../src/py_bridge_shop_private.rs"]
mod py_bridge_shop_private;

use py_bridge_shop_private::{
    m2_py_shop_set_private_shop, m2_py_shop_is_private_shop, m2_py_shop_is_main_player_private_shop,
    m2_py_shop_add_private_shop_item_stock, m2_py_shop_del_private_shop_item_stock,
    m2_py_shop_clear_private_shop_stock, m2_py_shop_get_private_shop_item_price,
    m2_py_shop_build_private_shop, TItemPos, TShopItemTable, TPacketCGMyShop,
    HEADER_CG_MYSHOP,
};

#[test]
fn test_struct_sizes() {
    assert_eq!(std::mem::size_of::<TItemPos>(), 3);
    assert_eq!(std::mem::size_of::<TShopItemTable>(), 13);
    assert_eq!(std::mem::size_of::<TPacketCGMyShop>(), 38);
}

#[test]
fn test_private_shop_state_flags() {
    m2_py_shop_set_private_shop(true, false);
    assert_eq!(m2_py_shop_is_private_shop(), true);
    assert_eq!(m2_py_shop_is_main_player_private_shop(), false);

    m2_py_shop_set_private_shop(false, true);
    assert_eq!(m2_py_shop_is_private_shop(), false);
    assert_eq!(m2_py_shop_is_main_player_private_shop(), true);
}

#[test]
fn test_private_shop_stock_management() {
    m2_py_shop_clear_private_shop_stock();
    assert_eq!(m2_py_shop_get_private_shop_item_price(1, 10), 0);

    m2_py_shop_add_private_shop_item_stock(1, 10, 5, 1500);
    assert_eq!(m2_py_shop_get_private_shop_item_price(1, 10), 1500);

    m2_py_shop_add_private_shop_item_stock(1, 10, 5, 2500); // overwrite
    assert_eq!(m2_py_shop_get_private_shop_item_price(1, 10), 2500);

    m2_py_shop_add_private_shop_item_stock(2, 20, 6, 3000);
    assert_eq!(m2_py_shop_get_private_shop_item_price(2, 20), 3000);

    m2_py_shop_del_private_shop_item_stock(1, 10);
    assert_eq!(m2_py_shop_get_private_shop_item_price(1, 10), 0);
    assert_eq!(m2_py_shop_get_private_shop_item_price(2, 20), 3000);

    m2_py_shop_clear_private_shop_stock();
    assert_eq!(m2_py_shop_get_private_shop_item_price(2, 20), 0);
}

#[test]
fn test_build_private_shop() {
    m2_py_shop_clear_private_shop_stock();
    m2_py_shop_add_private_shop_item_stock(1, 10, 2, 1000);
    m2_py_shop_add_private_shop_item_stock(1, 11, 1, 2000); // Should be ordered first
    m2_py_shop_add_private_shop_item_stock(1, 12, 3, 3000);

    let title = "My Shop!";
    let mut out_buf = vec![0u8; 1024];

    let written_len = m2_py_shop_build_private_shop(
        title.as_ptr(),
        title.len(),
        out_buf.as_mut_ptr(),
    );

    let expected_len = std::mem::size_of::<TPacketCGMyShop>() + 3 * std::mem::size_of::<TShopItemTable>();
    assert_eq!(written_len, expected_len);

    let packet = unsafe { std::ptr::read_unaligned(out_buf.as_ptr() as *const TPacketCGMyShop) };
    assert_eq!({ packet.header }, HEADER_CG_MYSHOP);
    assert_eq!({ packet.length } as usize, expected_len);
    assert_eq!({ packet.b_count }, 3);

    let mut expected_sign = [0u8; 33];
    expected_sign[..title.len()].copy_from_slice(title.as_bytes());
    assert_eq!({ packet.sz_sign }, expected_sign);

    let items_ptr = unsafe { out_buf.as_ptr().add(std::mem::size_of::<TPacketCGMyShop>()) } as *const TShopItemTable;
    let items = unsafe { std::slice::from_raw_parts(items_ptr, 3) };

    // Should be sorted by display_pos: 1, 2, 3
    assert_eq!({ items[0].display_pos }, 1);
    assert_eq!({ items[0].price }, 2000);
    
    assert_eq!({ items[1].display_pos }, 2);
    assert_eq!({ items[1].price }, 1000);
    
    assert_eq!({ items[2].display_pos }, 3);
    assert_eq!({ items[2].price }, 3000);
}
