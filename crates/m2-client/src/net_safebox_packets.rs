#![allow(non_snake_case)]

use std::mem::size_of;

pub const SAFEBOX_MONEY_STATE_SAVE: u8 = 0;
pub const SAFEBOX_MONEY_STATE_WITHDRAW: u8 = 1;

pub const HEADER_CG_SAFEBOX_CHECKIN: u16 = 0x0820;
pub const HEADER_CG_SAFEBOX_CHECKOUT: u16 = 0x0821;
pub const HEADER_CG_SAFEBOX_ITEM_MOVE: u16 = 0x0822;
pub const HEADER_CG_SAFEBOX_MONEY_CHANGE: u16 = 0x0834;
pub const HEADER_CG_MALL_CHECKOUT: u16 = 0x0840;

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct TItemPos {
    pub window_type: u8,
    pub cell: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct TPacketCGSafeboxCheckin {
    pub header: u16,
    pub length: u16,
    pub bSafePos: u8,
    pub ItemPos: TItemPos,
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct TPacketCGSafeboxCheckout {
    pub header: u16,
    pub length: u16,
    pub bSafePos: u8,
    pub ItemPos: TItemPos,
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct TPacketCGMallCheckout {
    pub header: u16,
    pub length: u16,
    pub bMallPos: u8,
    pub ItemPos: TItemPos,
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct TPacketCGSafeboxItemMove {
    pub header: u16,
    pub length: u16,
    pub pos: u8,
    pub change_pos: u8,
    pub count: u8,
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct TPacketCGSafeboxMoney {
    pub header: u16,
    pub length: u16,
    pub bState: u8,
    pub lMoney: i32,
}

pub fn build_safebox_checkin(safebox_pos: u8, window_type: u8, cell: u16) -> Vec<u8> {
    let packet = TPacketCGSafeboxCheckin {
        header: HEADER_CG_SAFEBOX_CHECKIN,
        length: size_of::<TPacketCGSafeboxCheckin>() as u16,
        bSafePos: safebox_pos,
        ItemPos: TItemPos { window_type, cell },
    };
    let bytes: [u8; size_of::<TPacketCGSafeboxCheckin>()] = unsafe { std::mem::transmute(packet) };
    bytes.to_vec()
}

pub fn build_safebox_checkout(safebox_pos: u8, window_type: u8, cell: u16) -> Vec<u8> {
    let packet = TPacketCGSafeboxCheckout {
        header: HEADER_CG_SAFEBOX_CHECKOUT,
        length: size_of::<TPacketCGSafeboxCheckout>() as u16,
        bSafePos: safebox_pos,
        ItemPos: TItemPos { window_type, cell },
    };
    let bytes: [u8; size_of::<TPacketCGSafeboxCheckout>()] = unsafe { std::mem::transmute(packet) };
    bytes.to_vec()
}

pub fn build_safebox_item_move(src_pos: u8, dst_pos: u8, count: u8) -> Vec<u8> {
    let packet = TPacketCGSafeboxItemMove {
        header: HEADER_CG_SAFEBOX_ITEM_MOVE,
        length: size_of::<TPacketCGSafeboxItemMove>() as u16,
        pos: src_pos,
        change_pos: dst_pos,
        count,
    };
    let bytes: [u8; size_of::<TPacketCGSafeboxItemMove>()] = unsafe { std::mem::transmute(packet) };
    bytes.to_vec()
}

pub fn build_mall_checkout(mall_pos: u8, window_type: u8, cell: u16) -> Vec<u8> {
    let packet = TPacketCGMallCheckout {
        header: HEADER_CG_MALL_CHECKOUT,
        length: size_of::<TPacketCGMallCheckout>() as u16,
        bMallPos: mall_pos,
        ItemPos: TItemPos { window_type, cell },
    };
    let bytes: [u8; size_of::<TPacketCGMallCheckout>()] = unsafe { std::mem::transmute(packet) };
    bytes.to_vec()
}

pub fn build_safebox_money(state: u8, money: i32) -> Vec<u8> {
    let packet = TPacketCGSafeboxMoney {
        header: HEADER_CG_SAFEBOX_MONEY_CHANGE, // Or what C++ uses for CG money change
        length: size_of::<TPacketCGSafeboxMoney>() as u16,
        bState: state,
        lMoney: money,
    };
    let bytes: [u8; size_of::<TPacketCGSafeboxMoney>()] = unsafe { std::mem::transmute(packet) };
    bytes.to_vec()
}
