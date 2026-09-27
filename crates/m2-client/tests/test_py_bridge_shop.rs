use serial_test::serial;
use m2_client::py_bridge_shop::{
    m2_py_shop_is_open, m2_py_shop_get_item_count, m2_py_shop_get_item_id, m2_py_shop_get_item_price, m2_py_shop_build_private_shop,
    HEADER_CG_MYSHOP, TPacketCGMyShop, TShopItemTable
};
use m2_client::recv_shop_logic_v2::SHOP_INSTANCE;
use m2_client::send_shop_exchange_v2::TItemPos;

fn reset_shop_state() {
    if let Ok(mut shop) = SHOP_INSTANCE.write() {
        shop.clear();
    }
}

#[test]
#[serial]
fn test_shop_is_open() {
    reset_shop_state();
    assert_eq!(m2_py_shop_is_open(), false);

    if let Ok(mut shop) = SHOP_INSTANCE.write() {
        shop.is_open = true;
    }
    assert_eq!(m2_py_shop_is_open(), true);
}

#[test]
#[serial]
fn test_shop_get_item_count() {
    reset_shop_state();
    assert_eq!(m2_py_shop_get_item_count(), 0);

    if let Ok(mut shop) = SHOP_INSTANCE.write() {
        shop.items[0].vnum = 100;
        shop.items[1].vnum = 200;
        shop.items[3].vnum = 300;
    }
    assert_eq!(m2_py_shop_get_item_count(), 3);
}

#[test]
#[serial]
fn test_shop_get_item_id() {
    reset_shop_state();
    if let Ok(mut shop) = SHOP_INSTANCE.write() {
        shop.items[0].vnum = 100;
        shop.items[5].vnum = 999;
    }
    assert_eq!(m2_py_shop_get_item_id(0), 100);
    assert_eq!(m2_py_shop_get_item_id(1), 0);
    assert_eq!(m2_py_shop_get_item_id(5), 999);
    assert_eq!(m2_py_shop_get_item_id(999), 0); // Out of bounds
}

#[test]
#[serial]
fn test_shop_get_item_price() {
    reset_shop_state();
    if let Ok(mut shop) = SHOP_INSTANCE.write() {
        shop.items[2].price = 1500;
    }
    assert_eq!(m2_py_shop_get_item_price(2), 1500);
    assert_eq!(m2_py_shop_get_item_price(0), 0);
    assert_eq!(m2_py_shop_get_item_price(999), 0); // Out of bounds
}

#[test]
#[serial]
fn test_shop_build_private_shop_empty() {
    let mut out_buf = [0u8; 1024];
    
    let size = m2_py_shop_build_private_shop(
        std::ptr::null(),
        0,
        std::ptr::null(),
        0,
        out_buf.as_mut_ptr(),
    );
    
    let expected_len = std::mem::size_of::<TPacketCGMyShop>();
    assert_eq!(size, expected_len);
    
    let packet = unsafe { std::ptr::read_unaligned(out_buf.as_ptr() as *const TPacketCGMyShop) };
    let header = packet.header;
    let length = packet.length;
    let b_count = packet.b_count;
    
    assert_eq!(header, HEADER_CG_MYSHOP);
    assert_eq!(length, expected_len as u16);
    assert_eq!(b_count, 0);
}

#[test]
#[serial]
fn test_shop_build_private_shop_with_items() {
    let mut out_buf = [0u8; 1024];
    let title_bytes = b"My Cool Shop";
    
    let items = vec![
        TShopItemTable {
            vnum: 1234,
            count: 1,
            pos: TItemPos { window_type: 1, cell: 5 },
            price: 5000,
            display_pos: 0,
        },
        TShopItemTable {
            vnum: 5678,
            count: 2,
            pos: TItemPos { window_type: 1, cell: 6 },
            price: 10000,
            display_pos: 1,
        },
    ];
    
    let size = m2_py_shop_build_private_shop(
        title_bytes.as_ptr(),
        title_bytes.len(),
        items.as_ptr(),
        items.len(),
        out_buf.as_mut_ptr(),
    );
    
    let expected_len = std::mem::size_of::<TPacketCGMyShop>() + std::mem::size_of::<TShopItemTable>() * 2;
    assert_eq!(size, expected_len);
    
    let packet = unsafe { std::ptr::read_unaligned(out_buf.as_ptr() as *const TPacketCGMyShop) };
    let header = packet.header;
    let length = packet.length;
    let b_count = packet.b_count;
    
    assert_eq!(header, HEADER_CG_MYSHOP);
    assert_eq!(length, expected_len as u16);
    assert_eq!(b_count, 2);
    
    for i in 0..title_bytes.len() {
        assert_eq!(packet.sz_sign[i], title_bytes[i]);
    }
    
    let items_offset = std::mem::size_of::<TPacketCGMyShop>();
    let parsed_items: *const TShopItemTable = unsafe { out_buf.as_ptr().add(items_offset) as *const TShopItemTable };
    let item1 = unsafe { std::ptr::read_unaligned(parsed_items) };
    let item2 = unsafe { std::ptr::read_unaligned(parsed_items.add(1)) };
    
    let vnum1 = item1.vnum;
    let price1 = item1.price;
    let vnum2 = item2.vnum;
    let price2 = item2.price;
    
    assert_eq!(vnum1, 1234);
    assert_eq!(price1, 5000);
    assert_eq!(vnum2, 5678);
    assert_eq!(price2, 10000);
}
