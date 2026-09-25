use m2_client_bridge::net::gc::item_set::{TPacketGCItemSet, PacketError, m2_net_parse_item_set, m2_net_free_item_set, HEADER_GC_ITEM_SET};

#[test]
fn test_from_bytes_success() {
    let mut buf = vec![0; 42];
    buf[0] = HEADER_GC_ITEM_SET; // header 21
    buf[1] = 1;  // window_type
    buf[2] = 2;  // cell
    buf[3] = 0;  
    buf[4] = 3;  // vnum
    buf[5] = 0;
    buf[6] = 0;
    buf[7] = 0;
    buf[8] = 4;  // count
    
    // sockets
    buf[9] = 5; buf[10] = 0; buf[11] = 0; buf[12] = 0;
    buf[13] = 6; buf[14] = 0; buf[15] = 0; buf[16] = 0;
    buf[17] = 7; buf[18] = 0; buf[19] = 0; buf[20] = 0;

    // attr 0
    buf[21] = 8; buf[22] = 9; buf[23] = 0;
    
    // attr 1
    buf[24] = 10; buf[25] = 11; buf[26] = 0;
    // ... reszta 0

    let packet = TPacketGCItemSet::from_bytes(&buf).unwrap();

    assert_eq!(packet.header, HEADER_GC_ITEM_SET);
    assert_eq!(packet.Cell.window_type, 1);
    assert_eq!({ packet.Cell.cell }, 2);
    assert_eq!({ packet.vnum }, 3);
    assert_eq!(packet.count, 4);
    assert_eq!({ packet.al_sockets }, [5, 6, 7]);
    
    assert_eq!(packet.a_attrs[0].b_type, 8);
    assert_eq!({ packet.a_attrs[0].s_value }, 9);
    
    assert_eq!(packet.a_attrs[1].b_type, 10);
    assert_eq!({ packet.a_attrs[1].s_value }, 11);
}

#[test]
fn test_from_bytes_too_short() {
    let buf = vec![0; 41];
    let result = TPacketGCItemSet::from_bytes(&buf);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), PacketError::BufferTooSmall);
}

#[test]
fn test_from_bytes_invalid_header() {
    let mut buf = vec![0; 42];
    buf[0] = 99; // bledny naglowek
    let result = TPacketGCItemSet::from_bytes(&buf);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), PacketError::InvalidHeader);
}

#[test]
fn test_ffi_parse_and_free() {
    let mut buf = vec![0; 42];
    buf[0] = HEADER_GC_ITEM_SET; // header
    buf[1] = 1;  // window_type
    buf[2] = 2;  // cell
    buf[3] = 0;  
    buf[4] = 3;  // vnum
    buf[5] = 0;
    buf[6] = 0;
    buf[7] = 0;
    buf[8] = 4;  // count

    let mut out_packet: *mut TPacketGCItemSet = std::ptr::null_mut();
    
    let result = unsafe {
        m2_net_parse_item_set(buf.as_ptr(), buf.len(), &mut out_packet)
    };
    
    assert!(result);
    assert!(!out_packet.is_null());
    
    let packet = unsafe { &*out_packet };
    assert_eq!(packet.header, HEADER_GC_ITEM_SET);
    assert_eq!(packet.Cell.window_type, 1);
    assert_eq!({ packet.Cell.cell }, 2);
    assert_eq!({ packet.vnum }, 3);
    assert_eq!(packet.count, 4);
    
    unsafe {
        m2_net_free_item_set(out_packet);
    }
}
