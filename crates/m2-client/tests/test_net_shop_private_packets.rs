use m2_client::net_shop_private_packets::*;

#[test]
fn test_build_private_shop_packet_serialization() {
    let mut sign = [0u8; 33];
    sign[0] = b'S';
    sign[1] = b'h';
    sign[2] = b'o';
    sign[3] = b'p';
    
    let shop = TPacketCGMyShop {
        header: HEADER_CG_MYSHOP,
        length: 60,
        szSign: sign,
        bCount: 2,
    };
    
    assert_eq!({ shop.header }, 0x0802);
    assert_eq!({ shop.bCount }, 2);
    assert_eq!({ shop.szSign[0] }, b'S');
}

#[test]
fn test_shop_item_table_size() {
    assert_eq!(std::mem::size_of::<TItemPos>(), 3);
    // TShopItemTable = vnum(4) + count(1) + pos(3) + price(4) + display_pos(1) = 13
    assert_eq!(std::mem::size_of::<TShopItemTable>(), 13);
}

#[test]
fn test_shop_packet_size() {
    // TPacketCGShop = header(2) + length(2) + subheader(1) = 5
    assert_eq!(std::mem::size_of::<TPacketCGShop>(), 5);
}

#[test]
fn test_packet_creation_shop_buy() {
    let packet = TPacketCGShop {
        header: HEADER_CG_SHOP,
        length: 7, // 5 + 1 (count) + 1 (pos)
        subheader: SHOP_SUB_BUY,
    };
    assert_eq!({ packet.header }, 0x0801);
    assert_eq!({ packet.subheader }, 1);
    assert_eq!({ packet.length }, 7);
}

#[test]
fn test_packet_creation_shop_sell() {
    let packet = TPacketCGShop {
        header: HEADER_CG_SHOP,
        length: 6, // 5 + 1 (pos)
        subheader: SHOP_SUB_SELL,
    };
    assert_eq!({ packet.header }, 0x0801);
    assert_eq!({ packet.subheader }, 2);
    assert_eq!({ packet.length }, 6);
}
