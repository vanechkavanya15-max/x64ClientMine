#[path = "../src/send_shop_trade_commands.rs"]
mod send_shop_trade_commands;

use send_shop_trade_commands::*;
use std::mem::size_of;

#[test]
fn test_packet_cg_shop_struct_size() {
    assert_eq!(size_of::<TPacketCGShop>(), 5, "TPacketCGShop should be exactly 5 bytes (u16 + u16 + u8)");
}

#[test]
fn test_build_shop_buy_packet() {
    let result = build_shop_buy_packet(10, 5).expect("Valid buy packet");
    
    assert_eq!(result.len(), 7, "Length should be 5 + 1 + 1");
    assert_eq!(u16::from_le_bytes([result[0], result[1]]), HEADER_CG_SHOP);
    assert_eq!(u16::from_le_bytes([result[2], result[3]]), 7);
    assert_eq!(result[4], SHOP_SUBHEADER_CG_BUY);
    assert_eq!(result[5], 5, "Count should be 5");
    assert_eq!(result[6], 10, "Pos should be 10");
}

#[test]
fn test_build_shop_buy_packet_negative_count() {
    let result = build_shop_buy_packet(10, -5);
    assert!(result.is_err(), "Should return error for negative count");
}

#[test]
fn test_build_shop_buy_packet_zero_count() {
    let result = build_shop_buy_packet(10, 0);
    assert!(result.is_err(), "Should return error for zero count");
}

#[test]
fn test_build_shop_sell_packet() {
    let result = build_shop_sell_packet(15).expect("Valid sell packet");
    
    assert_eq!(result.len(), 6, "Length should be 5 + 1");
    assert_eq!(u16::from_le_bytes([result[0], result[1]]), HEADER_CG_SHOP);
    assert_eq!(u16::from_le_bytes([result[2], result[3]]), 6);
    assert_eq!(result[4], SHOP_SUBHEADER_CG_SELL);
    assert_eq!(result[5], 15, "Pos should be 15");
}

#[test]
fn test_build_shop_sell_packet_new() {
    let result = build_shop_sell_packet_new(20, 3).expect("Valid sell new packet");
    
    assert_eq!(result.len(), 7, "Length should be 5 + 1 + 1");
    assert_eq!(u16::from_le_bytes([result[0], result[1]]), HEADER_CG_SHOP);
    assert_eq!(u16::from_le_bytes([result[2], result[3]]), 7);
    assert_eq!(result[4], SHOP_SUBHEADER_CG_SELL2);
    assert_eq!(result[5], 20, "Pos should be 20");
    assert_eq!(result[6], 3, "Count should be 3");
}

#[test]
fn test_build_shop_sell_packet_new_negative_count() {
    let result = build_shop_sell_packet_new(20, -1);
    assert!(result.is_err(), "Should return error for negative count");
}

#[test]
fn test_build_shop_end_packet() {
    let result = build_shop_end_packet().expect("Valid end packet");
    
    assert_eq!(result.len(), 5, "Length should be 5");
    assert_eq!(u16::from_le_bytes([result[0], result[1]]), HEADER_CG_SHOP);
    assert_eq!(u16::from_le_bytes([result[2], result[3]]), 5);
    assert_eq!(result[4], SHOP_SUBHEADER_CG_END);
}
