use m2_client_bridge::net::gc::item_del::{
    m2_net_parse_item_del, PacketError, TPacketGCItemDel, HEADER_GC_ITEM_DEL,
};

#[test]
fn test_item_del_from_bytes_success() {
    let mut buf = [0u8; 7];
    buf[0..2].copy_from_slice(&HEADER_GC_ITEM_DEL.to_le_bytes()); // header
    buf[2..4].copy_from_slice(&7u16.to_le_bytes());               // length
    buf[4] = 1;                                                   // window_type
    buf[5..7].copy_from_slice(&25u16.to_le_bytes());              // cell

    let packet = TPacketGCItemDel::from_bytes(&buf).expect("Nie udalo sie sparsowac pakietu");
    assert_eq!({ packet.header }, HEADER_GC_ITEM_DEL);
    assert_eq!({ packet.length }, 7);
    assert_eq!({ packet.pos.window_type }, 1);
    assert_eq!({ packet.pos.cell }, 25);
}

#[test]
fn test_item_del_from_bytes_too_short() {
    let buf = [0u8; 6];
    let result = TPacketGCItemDel::from_bytes(&buf);
    assert_eq!(result, Err(PacketError::TooShort));
}

#[test]
fn test_m2_net_parse_item_del_success() {
    let mut buf = [0u8; 7];
    buf[0..2].copy_from_slice(&HEADER_GC_ITEM_DEL.to_le_bytes());
    buf[2..4].copy_from_slice(&7u16.to_le_bytes());
    buf[4] = 2; // window_type
    buf[5..7].copy_from_slice(&500u16.to_le_bytes()); // cell

    let mut out_win = 0u8;
    let mut out_cell = 0u16;

    let success = unsafe {
        m2_net_parse_item_del(
            buf.as_ptr(),
            buf.len() as u32,
            &mut out_win,
            &mut out_cell,
        )
    };

    assert!(success);
    assert_eq!(out_win, 2);
    assert_eq!(out_cell, 500);
}

#[test]
fn test_m2_net_parse_item_del_failure_short_buf() {
    let buf = [0u8; 6];
    let mut out_win = 0u8;
    let mut out_cell = 0u16;

    let success = unsafe {
        m2_net_parse_item_del(
            buf.as_ptr(),
            buf.len() as u32,
            &mut out_win,
            &mut out_cell,
        )
    };

    assert!(!success);
}
