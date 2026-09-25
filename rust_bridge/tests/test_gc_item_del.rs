use m2_client_bridge::net::gc::item_del::{TPacketGCItemDel, PacketError, m2_net_parse_item_del};

#[test]
fn test_gc_item_del_from_bytes_success() {
    // HEADER_GC_ITEM_DEL (20), window_type (1), cell (15 00)
    let buf = [20, 1, 15, 0];
    let result = TPacketGCItemDel::from_bytes(&buf);
    
    assert!(result.is_ok());
    let packet = result.unwrap();
    assert_eq!(packet.header, 20);
    assert_eq!(packet.pos.window_type, 1);
    
    // Create a copy of the field to avoid unaligned reference errors from packed struct
    let cell_val = packet.pos.cell;
    assert_eq!(cell_val, 15);
}

#[test]
fn test_gc_item_del_from_bytes_truncated_buffer() {
    let buf = [20, 1, 15]; // Missing 1 byte for cell
    let result = TPacketGCItemDel::from_bytes(&buf);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), PacketError::TooShort);
}

#[test]
fn test_gc_item_del_from_bytes_empty_buffer() {
    let buf: [u8; 0] = [];
    let result = TPacketGCItemDel::from_bytes(&buf);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), PacketError::TooShort);
}

#[test]
fn test_m2_net_parse_item_del_success() {
    let buf = [20, 2, 0xFF, 0x00];
    let mut out_win = 0;
    let mut out_cell = 0;

    let result = unsafe {
        m2_net_parse_item_del(
            buf.as_ptr(),
            buf.len() as u32,
            &mut out_win,
            &mut out_cell,
        )
    };

    assert!(result);
    assert_eq!(out_win, 2);
    assert_eq!(out_cell, 255);
}

#[test]
fn test_m2_net_parse_item_del_failure_short_buf() {
    let buf = [20, 2, 0xFF];
    let mut out_win = 0;
    let mut out_cell = 0;

    let result = unsafe {
        m2_net_parse_item_del(
            buf.as_ptr(),
            buf.len() as u32,
            &mut out_win,
            &mut out_cell,
        )
    };

    assert!(!result);
}

#[test]
fn test_m2_net_parse_item_del_failure_null_ptr() {
    let buf = [20, 2, 0xFF, 0x00];
    let mut out_win = 0;
    let mut out_cell = 0;

    let result = unsafe {
        m2_net_parse_item_del(
            std::ptr::null(),
            buf.len() as u32,
            &mut out_win,
            &mut out_cell,
        )
    };
    assert!(!result);

    let result2 = unsafe {
        m2_net_parse_item_del(
            buf.as_ptr(),
            buf.len() as u32,
            std::ptr::null_mut(),
            &mut out_cell,
        )
    };
    assert!(!result2);
}
