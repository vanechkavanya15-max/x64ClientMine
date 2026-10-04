use m2_client::net_safebox_packets::*;
use std::mem::size_of;

#[test]
fn test_build_safebox_checkin() {
    let pkt = build_safebox_checkin(5, 1, 10);
    assert_eq!(pkt.len(), size_of::<TPacketCGSafeboxCheckin>());
    
    // Check specific fields.
    assert_eq!(pkt[0], (HEADER_CG_SAFEBOX_CHECKIN & 0xFF) as u8);
    assert_eq!(pkt[1], (HEADER_CG_SAFEBOX_CHECKIN >> 8) as u8);
    assert_eq!(pkt[2], size_of::<TPacketCGSafeboxCheckin>() as u8);
    assert_eq!(pkt[4], 5); // bSafePos
    assert_eq!(pkt[5], 1); // window_type
    assert_eq!(pkt[6], 10); // cell
}

#[test]
fn test_build_safebox_checkout() {
    let pkt = build_safebox_checkout(3, 2, 15);
    assert_eq!(pkt.len(), size_of::<TPacketCGSafeboxCheckout>());
    
    assert_eq!(pkt[0], (HEADER_CG_SAFEBOX_CHECKOUT & 0xFF) as u8);
    assert_eq!(pkt[1], (HEADER_CG_SAFEBOX_CHECKOUT >> 8) as u8);
    assert_eq!(pkt[4], 3); // bSafePos
    assert_eq!(pkt[5], 2); // window_type
    assert_eq!(pkt[6], 15); // cell
}

#[test]
fn test_build_safebox_item_move() {
    let pkt = build_safebox_item_move(1, 10, 1, 12, 1);
    assert_eq!(pkt.len(), size_of::<TPacketCGSafeboxItemMove>());
    
    assert_eq!(pkt[0], (HEADER_CG_SAFEBOX_ITEM_MOVE & 0xFF) as u8);
    assert_eq!(pkt[1], (HEADER_CG_SAFEBOX_ITEM_MOVE >> 8) as u8);
    assert_eq!(pkt[4], 1); // src_window
    assert_eq!(pkt[5], 10); // src_cell
    assert_eq!(pkt[7], 1); // dst_window
    assert_eq!(pkt[8], 12); // dst_cell
    assert_eq!(pkt[10], 1); // count
}

#[test]
fn test_build_safebox_money() {
    let pkt = build_safebox_money(SAFEBOX_MONEY_STATE_SAVE, 1000);
    assert_eq!(pkt.len(), size_of::<TPacketCGSafeboxMoney>());
    
    assert_eq!(pkt[0], (HEADER_CG_SAFEBOX_MONEY_CHANGE & 0xFF) as u8);
    assert_eq!(pkt[1], (HEADER_CG_SAFEBOX_MONEY_CHANGE >> 8) as u8);
    assert_eq!(pkt[4], SAFEBOX_MONEY_STATE_SAVE);
    
    let l_money = i32::from_le_bytes([pkt[5], pkt[6], pkt[7], pkt[8]]);
    assert_eq!(l_money, 1000);
}

#[test]
fn test_build_mall_checkout() {
    let pkt = build_mall_checkout(4, 1, 20);
    assert_eq!(pkt.len(), size_of::<TPacketCGMallCheckout>());
    
    assert_eq!(pkt[0], (HEADER_CG_MALL_CHECKOUT & 0xFF) as u8);
    assert_eq!(pkt[1], (HEADER_CG_MALL_CHECKOUT >> 8) as u8);
    assert_eq!(pkt[4], 4); // bMallPos
    assert_eq!(pkt[5], 1); // window_type
    assert_eq!(pkt[6], 20); // cell
}
