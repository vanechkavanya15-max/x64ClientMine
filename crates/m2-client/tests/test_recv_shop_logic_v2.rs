use m2_client::recv_shop_logic_v2::*;
use std::mem::size_of;

#[test]
fn test_handle_shop_packet_invalid() {
    assert_eq!(m2_net_recv_shop(std::ptr::null(), 0), false);
    
    let buf = vec![0u8; size_of::<TPacketGCShop>() - 1];
    assert_eq!(m2_net_recv_shop(buf.as_ptr(), buf.len()), false);
    
    let mut buf = vec![0u8; size_of::<TPacketGCShop>()];
    let packet = buf.as_mut_ptr() as *mut TPacketGCShop;
    unsafe {
        let length_ptr = std::ptr::addr_of_mut!((*packet).length);
        std::ptr::write_unaligned(length_ptr, size_of::<TPacketGCShop>() as u16 + 1);
    }
    assert_eq!(m2_net_recv_shop(buf.as_ptr(), buf.len()), false);
}

#[test]
fn test_handle_shop_packet_start() {
    let mut buf = vec![0u8; size_of::<TPacketGCShop>() + 4 + size_of::<TPacketGCShopStart>()];
    let packet = buf.as_mut_ptr() as *mut TPacketGCShop;
    unsafe {
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*packet).header), 0);
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*packet).length), buf.len() as u16);
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*packet).subheader), SHOP_SUBHEADER_GC_START);
    }
    
    let vid_ptr = unsafe { buf.as_mut_ptr().add(size_of::<TPacketGCShop>()) as *mut u32 };
    unsafe { std::ptr::write_unaligned(vid_ptr, 1234); }
    
    let start_packet_ptr = unsafe { buf.as_mut_ptr().add(size_of::<TPacketGCShop>() + 4) as *mut TPacketGCShopStart };
    unsafe {
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*start_packet_ptr).items[0].vnum), 500);
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*start_packet_ptr).items[0].price), 100);
    }
    
    assert_eq!(m2_net_recv_shop(buf.as_ptr(), buf.len()), true);
    
    let shop = SHOP_INSTANCE.read().unwrap();
    assert_eq!(shop.is_open, true);
    assert_eq!(shop.vid, 1234);
    let vnum = shop.items[0].vnum; assert_eq!(vnum, 500);
}

#[test]
fn test_handle_shop_packet_update_item() {
    let mut buf = vec![0u8; size_of::<TPacketGCShop>() + size_of::<TPacketGCShopUpdateItem>()];
    let packet = buf.as_mut_ptr() as *mut TPacketGCShop;
    unsafe {
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*packet).header), 0);
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*packet).length), buf.len() as u16);
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*packet).subheader), SHOP_SUBHEADER_GC_UPDATE_ITEM);
    }
    
    let update_packet_ptr = unsafe { buf.as_mut_ptr().add(size_of::<TPacketGCShop>()) as *mut TPacketGCShopUpdateItem };
    unsafe {
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*update_packet_ptr).pos), 5);
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*update_packet_ptr).item.vnum), 999);
    }
    
    assert_eq!(m2_net_recv_shop(buf.as_ptr(), buf.len()), true);
    
    let shop = SHOP_INSTANCE.read().unwrap();
    let vnum = shop.items[5].vnum; assert_eq!(vnum, 999);
}

#[test]
fn test_handle_shop_packet_messages() {
    let mut buf = vec![0u8; size_of::<TPacketGCShop>()];
    let packet = buf.as_mut_ptr() as *mut TPacketGCShop;
    
    unsafe {
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*packet).header), 0);
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*packet).length), buf.len() as u16);
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*packet).subheader), SHOP_SUBHEADER_GC_NOT_ENOUGH_MONEY);
    }
    assert_eq!(m2_net_recv_shop(buf.as_ptr(), buf.len()), true);
    assert_eq!(SHOP_INSTANCE.read().unwrap().error_msg, Some("NOT_ENOUGH_MONEY".to_string()));
    
    unsafe { std::ptr::write_unaligned(std::ptr::addr_of_mut!((*packet).subheader), SHOP_SUBHEADER_GC_SOLDOUT); }
    assert_eq!(m2_net_recv_shop(buf.as_ptr(), buf.len()), true);
    assert_eq!(SHOP_INSTANCE.read().unwrap().error_msg, Some("SOLDOUT".to_string()));
    
    unsafe { std::ptr::write_unaligned(std::ptr::addr_of_mut!((*packet).subheader), SHOP_SUBHEADER_GC_INVENTORY_FULL); }
    assert_eq!(m2_net_recv_shop(buf.as_ptr(), buf.len()), true);
    assert_eq!(SHOP_INSTANCE.read().unwrap().error_msg, Some("INVENTORY_FULL".to_string()));

    unsafe { std::ptr::write_unaligned(std::ptr::addr_of_mut!((*packet).subheader), SHOP_SUBHEADER_GC_INVALID_POS); }
    assert_eq!(m2_net_recv_shop(buf.as_ptr(), buf.len()), true);
    assert_eq!(SHOP_INSTANCE.read().unwrap().error_msg, Some("INVALID_POS".to_string()));
}

#[test]
fn test_handle_shop_packet_end() {
    let mut buf = vec![0u8; size_of::<TPacketGCShop>()];
    let packet = buf.as_mut_ptr() as *mut TPacketGCShop;
    unsafe {
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*packet).header), 0);
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*packet).length), buf.len() as u16);
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*packet).subheader), SHOP_SUBHEADER_GC_END);
    }
    SHOP_INSTANCE.write().unwrap().is_open = true;
    assert_eq!(m2_net_recv_shop(buf.as_ptr(), buf.len()), true);
    assert_eq!(SHOP_INSTANCE.read().unwrap().is_open, false);
}

#[test]
fn test_handle_shop_packet_update_price() {
    let mut buf = vec![0u8; size_of::<TPacketGCShop>() + size_of::<TPacketGCShopUpdatePrice>()];
    let packet = buf.as_mut_ptr() as *mut TPacketGCShop;
    unsafe {
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*packet).header), 0);
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*packet).length), buf.len() as u16);
        std::ptr::write_unaligned(std::ptr::addr_of_mut!((*packet).subheader), SHOP_SUBHEADER_GC_UPDATE_PRICE);
    }
    assert_eq!(m2_net_recv_shop(buf.as_ptr(), buf.len()), true);
}
