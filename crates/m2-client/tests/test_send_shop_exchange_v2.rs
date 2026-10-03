use m2_client::send_shop_exchange_v2::*;
use std::mem::size_of;

#[test]
fn test_send_shop_buy() {
    let mut out_buf = [0u8; 128];
    let expected_len = size_of::<TPacketCGShop>() + 2;
    
    let written = m2_send_shop_buy_v2(5, 10, out_buf.as_mut_ptr());
    assert_eq!(written, expected_len, "m2_send_shop_buy returned wrong length");
    
    let packet_slice = &out_buf[0..size_of::<TPacketCGShop>()];
    let packet = unsafe { std::ptr::read_unaligned(packet_slice.as_ptr() as *const TPacketCGShop) };
    
    assert_eq!({packet.header}, HEADER_CG_SHOP);
    assert_eq!({packet.length}, expected_len as u16);
    assert_eq!(packet.subheader, SHOP_SUB_BUY);
    
    assert_eq!(out_buf[size_of::<TPacketCGShop>()], 10, "Count mismatch");
    assert_eq!(out_buf[size_of::<TPacketCGShop>() + 1], 5, "Pos mismatch");
}

#[test]
fn test_send_shop_buy_null() {
    let written = m2_send_shop_buy_v2(5, 10, std::ptr::null_mut());
    assert_eq!(written, 0, "m2_send_shop_buy with null out_buf should return 0");
}

#[test]
fn test_send_exchange_start() {
    let mut out_buf = [0u8; 128];
    let expected_len = size_of::<TPacketCGExchange>();
    
    let written = m2_send_exchange_start(12345, out_buf.as_mut_ptr());
    assert_eq!(written, expected_len, "m2_send_exchange_start returned wrong length");
    
    let packet = unsafe { std::ptr::read_unaligned(out_buf.as_ptr() as *const TPacketCGExchange) };
    
    assert_eq!({packet.header}, HEADER_CG_EXCHANGE);
    assert_eq!({packet.length}, expected_len as u16);
    assert_eq!(packet.subheader, EXCHANGE_SUB_START);
    assert_eq!({packet.arg1}, 12345);
}

#[test]
fn test_send_exchange_elk_add() {
    let mut out_buf = [0u8; 128];
    let expected_len = size_of::<TPacketCGExchange>();
    
    let written = m2_send_exchange_elk_add(50000, out_buf.as_mut_ptr());
    assert_eq!(written, expected_len, "m2_send_exchange_elk_add returned wrong length");
    
    let packet = unsafe { std::ptr::read_unaligned(out_buf.as_ptr() as *const TPacketCGExchange) };
    
    assert_eq!({packet.header}, HEADER_CG_EXCHANGE);
    assert_eq!({packet.length}, expected_len as u16);
    assert_eq!(packet.subheader, EXCHANGE_SUB_ELK_ADD);
    assert_eq!({packet.arg1}, 50000);
}

#[test]
fn test_send_exchange_item_add() {
    let mut out_buf = [0u8; 128];
    let expected_len = size_of::<TPacketCGExchange>();
    
    let written = m2_send_exchange_item_add(1, 42, 3, out_buf.as_mut_ptr());
    assert_eq!(written, expected_len, "m2_send_exchange_item_add returned wrong length");
    
    let packet = unsafe { std::ptr::read_unaligned(out_buf.as_ptr() as *const TPacketCGExchange) };
    
    assert_eq!({packet.header}, HEADER_CG_EXCHANGE);
    assert_eq!({packet.length}, expected_len as u16);
    assert_eq!(packet.subheader, EXCHANGE_SUB_ITEM_ADD);
    assert_eq!(packet.pos.window_type, 1);
    assert_eq!({packet.pos.cell}, 42);
    assert_eq!(packet.arg2, 3);
}

#[test]
fn test_send_exchange_item_del() {
    let mut out_buf = [0u8; 128];
    let expected_len = size_of::<TPacketCGExchange>();
    
    let written = m2_send_exchange_item_del(7, out_buf.as_mut_ptr());
    assert_eq!(written, expected_len, "m2_send_exchange_item_del returned wrong length");
    
    let packet = unsafe { std::ptr::read_unaligned(out_buf.as_ptr() as *const TPacketCGExchange) };
    
    assert_eq!({packet.header}, HEADER_CG_EXCHANGE);
    assert_eq!({packet.length}, expected_len as u16);
    assert_eq!(packet.subheader, EXCHANGE_SUB_ITEM_DEL);
    assert_eq!({packet.arg1}, 7);
}

#[test]
fn test_send_exchange_accept() {
    let mut out_buf = [0u8; 128];
    let expected_len = size_of::<TPacketCGExchange>();
    
    let written = m2_send_exchange_accept(out_buf.as_mut_ptr());
    assert_eq!(written, expected_len, "m2_send_exchange_accept returned wrong length");
    
    let packet = unsafe { std::ptr::read_unaligned(out_buf.as_ptr() as *const TPacketCGExchange) };
    
    assert_eq!({packet.header}, HEADER_CG_EXCHANGE);
    assert_eq!({packet.length}, expected_len as u16);
    assert_eq!(packet.subheader, EXCHANGE_SUB_ACCEPT);
}

#[test]
fn test_send_exchange_exit() {
    let mut out_buf = [0u8; 128];
    let expected_len = size_of::<TPacketCGExchange>();
    
    let written = m2_send_exchange_exit(out_buf.as_mut_ptr());
    assert_eq!(written, expected_len, "m2_send_exchange_exit returned wrong length");
    
    let packet = unsafe { std::ptr::read_unaligned(out_buf.as_ptr() as *const TPacketCGExchange) };
    
    assert_eq!({packet.header}, HEADER_CG_EXCHANGE);
    assert_eq!({packet.length}, expected_len as u16);
    assert_eq!(packet.subheader, EXCHANGE_SUB_CANCEL);
}

#[test]
fn test_exchange_null_buffers() {
    assert_eq!(m2_send_exchange_start(12345, std::ptr::null_mut()), 0);
    assert_eq!(m2_send_exchange_elk_add(50000, std::ptr::null_mut()), 0);
    assert_eq!(m2_send_exchange_item_add(1, 42, 3, std::ptr::null_mut()), 0);
    assert_eq!(m2_send_exchange_item_del(7, std::ptr::null_mut()), 0);
    assert_eq!(m2_send_exchange_accept(std::ptr::null_mut()), 0);
    assert_eq!(m2_send_exchange_exit(std::ptr::null_mut()), 0);
}
