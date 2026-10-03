#[path = "../src/send_shop_complete_packets.rs"]
mod send_shop_complete_packets;

use send_shop_complete_packets::*;
use std::mem::size_of;
use std::sync::OnceLock;
use std::sync::Mutex;

static TEST_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

fn get_test_lock() -> std::sync::MutexGuard<'static, ()> {
    TEST_MUTEX.get_or_init(|| Mutex::new(())).lock().unwrap()
}

#[test]
fn test_packet_layout() {
    let _lock = get_test_lock();
    assert_eq!(size_of::<TPacketCGShop>(), 5);
}

#[test]
fn test_send_shop_end() {
    let _lock = get_test_lock();
    // Use m2_net_clear_outgoing() from m2_client_bridge or just test the buffer.
    m2_client_bridge::net::send_queue::clear_outgoing_packets();
    
    let mut buf = [0u8; 10];
    let len = m2_send_shop_end(buf.as_mut_ptr());
    assert_eq!(len, 5);
    
    // Check struct memory
    assert_eq!(buf[0], 0x05); // Header 0x0505
    assert_eq!(buf[1], 0x05);
    assert_eq!(buf[2], 0x05); // Length 5
    assert_eq!(buf[3], 0x00);
    assert_eq!(buf[4], SHOP_SUB_END);
}

#[test]
fn test_send_shop_buy() {
    let _lock = get_test_lock();
    m2_client_bridge::net::send_queue::clear_outgoing_packets();
    
    let mut buf = [0u8; 10];
    let pos = 12;
    let count = 5;
    let len = m2_send_shop_buy(pos, count, buf.as_mut_ptr());
    
    assert_eq!(len, 7); // 5 (struct) + 1 (count) + 1 (pos)
    assert_eq!(buf[0], 0x05); // Header 0x0505
    assert_eq!(buf[1], 0x05);
    assert_eq!(buf[2], 0x07); // Length 7
    assert_eq!(buf[3], 0x00);
    assert_eq!(buf[4], SHOP_SUB_BUY);
    assert_eq!(buf[5], count);
    assert_eq!(buf[6], pos);
}

#[test]
fn test_send_shop_sell() {
    let _lock = get_test_lock();
    m2_client_bridge::net::send_queue::clear_outgoing_packets();
    
    let mut buf = [0u8; 10];
    let pos = 42;
    let len = m2_send_shop_sell(pos, buf.as_mut_ptr());
    
    assert_eq!(len, 6); // 5 (struct) + 1 (pos)
    assert_eq!(buf[0], 0x05); // Header 0x0505
    assert_eq!(buf[1], 0x05);
    assert_eq!(buf[2], 0x06); // Length 6
    assert_eq!(buf[3], 0x00);
    assert_eq!(buf[4], SHOP_SUB_SELL);
    assert_eq!(buf[5], pos);
}

#[test]
fn test_send_shop_sell_new() {
    let _lock = get_test_lock();
    m2_client_bridge::net::send_queue::clear_outgoing_packets();
    
    let mut buf = [0u8; 10];
    let pos = 99;
    let count = 3;
    let len = m2_send_shop_sell_new(pos, count, buf.as_mut_ptr());
    
    assert_eq!(len, 7); // 5 (struct) + 1 (pos) + 1 (count)
    assert_eq!(buf[0], 0x05); // Header 0x0505
    assert_eq!(buf[1], 0x05);
    assert_eq!(buf[2], 0x07); // Length 7
    assert_eq!(buf[3], 0x00);
    assert_eq!(buf[4], SHOP_SUB_SELL2);
    assert_eq!(buf[5], pos);
    assert_eq!(buf[6], count);
}

#[test]
fn test_null_out_buf() {
    let _lock = get_test_lock();
    // Safety check - shouldn't panic when out_buf is null
    let len = m2_send_shop_end(std::ptr::null_mut());
    assert_eq!(len, 5);
}
