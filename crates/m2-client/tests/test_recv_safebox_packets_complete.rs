use m2_client::recv_safebox_packets_complete::*;
use m2_client::safebox_mall_manager_v2::G_SAFEBOX_MALL_MANAGER;
use std::mem::size_of;

#[test]
fn test_parse_safebox_size() {
    let packet = TPacketGCSafeboxSize {
        header: GC_SAFEBOX_OPEN,
        length: size_of::<TPacketGCSafeboxSize>() as u16,
        bSize: 42,
    };
    
    let data = unsafe { 
        std::slice::from_raw_parts(&packet as *const _ as *const u8, size_of::<TPacketGCSafeboxSize>()) 
    };
    
    let result = parse_safebox_packet(data);
    assert_eq!(result.unwrap(), size_of::<TPacketGCSafeboxSize>());
    
    let mgr = G_SAFEBOX_MALL_MANAGER.read().unwrap();
    assert_eq!(mgr.get_current_safebox_size(), 42 * 5);
}

#[test]
fn test_parse_safebox_set() {
    // najpierw otwieramy magazyn na odpowiedni rozmiar
    {
        let mut mgr = G_SAFEBOX_MALL_MANAGER.write().unwrap();
        mgr.open_safebox(10);
    }

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
    
    let mgr = G_SAFEBOX_MALL_MANAGER.read().unwrap();
    let item = mgr.get_item_data(15).expect("Item should exist at slot 15");
    assert_eq!(item.vnum, 1337);
    assert_eq!(item.count, 5);
}

#[test]
fn test_parse_safebox_del() {
    {
        let mut mgr = G_SAFEBOX_MALL_MANAGER.write().unwrap();
        mgr.open_safebox(10);
    }

    let packet = TPacketGCItemDel {
        header: GC_SAFEBOX_DEL,
        length: size_of::<TPacketGCItemDel>() as u16,
        pos: SItemPos { window_type: 2, cell: 15 },
    };
    
    let data = unsafe { 
        std::slice::from_raw_parts(&packet as *const _ as *const u8, size_of::<TPacketGCItemDel>()) 
    };
    
    let result = parse_safebox_packet(data);
    assert_eq!(result.unwrap(), size_of::<TPacketGCItemDel>());
    
    let mgr = G_SAFEBOX_MALL_MANAGER.read().unwrap();
    let item = mgr.get_item_data(15).expect("Slot should be cleared");
    assert_eq!(item.vnum, 0);
}

#[test]
fn test_parse_safebox_wrong_password() {
    let packet = TPacketGCSafeboxWrongPassword {
        header: GC_SAFEBOX_WRONG_PASSWORD,
        length: size_of::<TPacketGCSafeboxWrongPassword>() as u16,
    };
    
    let data = unsafe { 
        std::slice::from_raw_parts(&packet as *const _ as *const u8, size_of::<TPacketGCSafeboxWrongPassword>()) 
    };
    
    let result = parse_safebox_packet(data);
    assert_eq!(result.unwrap(), size_of::<TPacketGCSafeboxWrongPassword>());
    
    let mgr = G_SAFEBOX_MALL_MANAGER.read().unwrap();
    assert!(mgr.get_wrong_attempts() > 0);
}

#[test]
fn test_parse_safebox_money_change() {
    let packet = TPacketGCSafeboxMoneyChange {
        header: GC_SAFEBOX_MONEY_CHANGE,
        length: size_of::<TPacketGCSafeboxMoneyChange>() as u16,
        lMoney: 50000,
    };
    
    let data = unsafe { 
        std::slice::from_raw_parts(&packet as *const _ as *const u8, size_of::<TPacketGCSafeboxMoneyChange>()) 
    };
    
    let result = parse_safebox_packet(data);
    assert_eq!(result.unwrap(), size_of::<TPacketGCSafeboxMoneyChange>());
    
    let mgr = G_SAFEBOX_MALL_MANAGER.read().unwrap();
    assert_eq!(mgr.get_money(), 50000);
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
    let mut data = vec![];
    data.extend_from_slice(&GC_SAFEBOX_OPEN.to_le_bytes()); // header
    
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
