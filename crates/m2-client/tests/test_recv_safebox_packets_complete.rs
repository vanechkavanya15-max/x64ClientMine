#[path = "../src/recv_safebox_packets_complete.rs"]
mod recv_safebox_packets_complete;

use recv_safebox_packets_complete::*;
use std::cell::RefCell;
use std::mem::{size_of, transmute};

thread_local! {
    static SAFEBOX_OPEN_CALLS: RefCell<Vec<u8>> = RefCell::new(Vec::new());
    static SAFEBOX_SET_CALLS: RefCell<Vec<(u16, u32, u8, u32, u32)>> = RefCell::new(Vec::new());
    static SAFEBOX_DEL_CALLS: RefCell<Vec<u16>> = RefCell::new(Vec::new());
    static SAFEBOX_WRONG_PASSWORD_CALLS: RefCell<usize> = RefCell::new(0);
    static SAFEBOX_MONEY_CHANGE_CALLS: RefCell<Vec<i32>> = RefCell::new(Vec::new());
}

#[no_mangle]
pub unsafe extern "C" fn m2_safebox_open(size: u8) {
    SAFEBOX_OPEN_CALLS.with(|c| c.borrow_mut().push(size));
}

#[no_mangle]
pub unsafe extern "C" fn m2_safebox_set_item(
    pos: u16,
    vnum: u32,
    count: u8,
    flags: u32,
    anti_flags: u32,
    _sockets: *const i32,
    _attrs: *const TPlayerItemAttribute,
) {
    SAFEBOX_SET_CALLS.with(|c| c.borrow_mut().push((pos, vnum, count, flags, anti_flags)));
}

#[no_mangle]
pub unsafe extern "C" fn m2_safebox_del_item(pos: u16) {
    SAFEBOX_DEL_CALLS.with(|c| c.borrow_mut().push(pos));
}

#[no_mangle]
pub unsafe extern "C" fn m2_safebox_wrong_password() {
    SAFEBOX_WRONG_PASSWORD_CALLS.with(|c| *c.borrow_mut() += 1);
}

#[no_mangle]
pub unsafe extern "C" fn m2_safebox_set_money(money: i32) {
    SAFEBOX_MONEY_CHANGE_CALLS.with(|c| c.borrow_mut().push(money));
}

fn clear_mocks() {
    SAFEBOX_OPEN_CALLS.with(|c| c.borrow_mut().clear());
    SAFEBOX_SET_CALLS.with(|c| c.borrow_mut().clear());
    SAFEBOX_DEL_CALLS.with(|c| c.borrow_mut().clear());
    SAFEBOX_WRONG_PASSWORD_CALLS.with(|c| *c.borrow_mut() = 0);
    SAFEBOX_MONEY_CHANGE_CALLS.with(|c| c.borrow_mut().clear());
}

#[test]
fn test_parse_safebox_size() {
    clear_mocks();
    
    let mut packet = TPacketGCSafeboxSize {
        header: GC_SAFEBOX_OPEN,
        length: size_of::<TPacketGCSafeboxSize>() as u16,
        bSize: 42,
    };
    
    let data = unsafe { 
        std::slice::from_raw_parts(&packet as *const _ as *const u8, size_of::<TPacketGCSafeboxSize>()) 
    };
    
    let result = parse_safebox_packet(data);
    assert_eq!(result.unwrap(), size_of::<TPacketGCSafeboxSize>());
    
    SAFEBOX_OPEN_CALLS.with(|c| {
        let calls = c.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0], 42);
    });
}

#[test]
fn test_parse_safebox_set() {
    clear_mocks();
    
    let packet = TPacketGCItemSet {
        header: GC_SAFEBOX_SET,
        length: size_of::<TPacketGCItemSet>() as u16,
        pos: SItemPos { window_type: 2, cell: 15 },
        vnum: 1337,
        count: 5,
        flags: 1,
        anti_flags: 2,
        highlight: 0,
        alSockets: [0, 0, 0],
        aAttr: [TPlayerItemAttribute { bType: 0, sValue: 0 }; 7],
    };
    
    let data = unsafe { 
        std::slice::from_raw_parts(&packet as *const _ as *const u8, size_of::<TPacketGCItemSet>()) 
    };
    
    let result = parse_safebox_packet(data);
    assert_eq!(result.unwrap(), size_of::<TPacketGCItemSet>());
    
    SAFEBOX_SET_CALLS.with(|c| {
        let calls = c.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0], (15, 1337, 5, 1, 2));
    });
}

#[test]
fn test_parse_safebox_del() {
    clear_mocks();
    
    let packet = TPacketGCItemDel {
        header: GC_SAFEBOX_DEL,
        length: size_of::<TPacketGCItemDel>() as u16,
        pos: SItemPos { window_type: 2, cell: 99 },
    };
    
    let data = unsafe { 
        std::slice::from_raw_parts(&packet as *const _ as *const u8, size_of::<TPacketGCItemDel>()) 
    };
    
    let result = parse_safebox_packet(data);
    assert_eq!(result.unwrap(), size_of::<TPacketGCItemDel>());
    
    SAFEBOX_DEL_CALLS.with(|c| {
        let calls = c.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0], 99);
    });
}

#[test]
fn test_parse_safebox_wrong_password() {
    clear_mocks();
    
    let packet = TPacketGCSafeboxWrongPassword {
        header: GC_SAFEBOX_WRONG_PASSWORD,
        length: size_of::<TPacketGCSafeboxWrongPassword>() as u16,
    };
    
    let data = unsafe { 
        std::slice::from_raw_parts(&packet as *const _ as *const u8, size_of::<TPacketGCSafeboxWrongPassword>()) 
    };
    
    let result = parse_safebox_packet(data);
    assert_eq!(result.unwrap(), size_of::<TPacketGCSafeboxWrongPassword>());
    
    SAFEBOX_WRONG_PASSWORD_CALLS.with(|c| {
        assert_eq!(*c.borrow(), 1);
    });
}

#[test]
fn test_parse_safebox_money_change() {
    clear_mocks();
    
    let packet = TPacketGCSafeboxMoneyChange {
        header: GC_SAFEBOX_MONEY_CHANGE,
        length: size_of::<TPacketGCSafeboxMoneyChange>() as u16,
        lMoney: -5000,
    };
    
    let data = unsafe { 
        std::slice::from_raw_parts(&packet as *const _ as *const u8, size_of::<TPacketGCSafeboxMoneyChange>()) 
    };
    
    let result = parse_safebox_packet(data);
    assert_eq!(result.unwrap(), size_of::<TPacketGCSafeboxMoneyChange>());
    
    SAFEBOX_MONEY_CHANGE_CALLS.with(|c| {
        let calls = c.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0], -5000);
    });
}

#[test]
fn test_parse_safebox_packet_too_short_for_header() {
    let data = vec![0x33]; // Only 1 byte
    let result = parse_safebox_packet(&data);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Packet too short for header");
}

#[test]
fn test_parse_safebox_packet_too_short_for_payload() {
    // A valid header but no body
    let mut data = vec![];
    data.extend_from_slice(&GC_SAFEBOX_OPEN.to_le_bytes()); // header
    // Missing body fields (like bSize, length)
    
    let result = parse_safebox_packet(&data);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Packet too short for GC_SAFEBOX_OPEN");
}

#[test]
fn test_parse_safebox_unknown_header() {
    let mut data = vec![];
    let unknown_header: u16 = 0x9999;
    data.extend_from_slice(&unknown_header.to_le_bytes()); 
    
    let result = parse_safebox_packet(&data);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Unknown safebox packet header");
}
