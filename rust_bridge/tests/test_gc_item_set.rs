use m2_client_bridge::net::gc::item_set::{
    m2_net_free_item_set, m2_net_parse_item_set, PacketError, TPacketGCItemSet,
    HEADER_GC_ITEM_SET,
};

#[test]
fn test_item_set_valid_deserialization() {
    let mut buf = [0u8; 54];
    buf[0..2].copy_from_slice(&HEADER_GC_ITEM_SET.to_le_bytes()); // header
    buf[2..4].copy_from_slice(&54u16.to_le_bytes());              // length
    buf[4] = 1;                                                  // window_type
    buf[5..7].copy_from_slice(&5u16.to_le_bytes());              // cell
    buf[7..11].copy_from_slice(&10u32.to_le_bytes());            // vnum
    buf[11] = 2;                                                 // count
    buf[12..16].copy_from_slice(&0x100u32.to_le_bytes());        // flags
    buf[16..20].copy_from_slice(&0x200u32.to_le_bytes());        // anti_flags
    buf[20] = 1;                                                 // highlight

    // alSockets
    buf[21..25].copy_from_slice(&11i32.to_le_bytes());
    buf[25..29].copy_from_slice(&22i32.to_le_bytes());
    buf[29..33].copy_from_slice(&33i32.to_le_bytes());

    // aAttr
    for i in 0..7 {
        let offset = 33 + i * 3;
        buf[offset] = (i + 1) as u8;
        buf[offset + 1..offset + 3].copy_from_slice(&((i * 10) as i16).to_le_bytes());
    }

    let packet = TPacketGCItemSet::from_bytes(&buf).unwrap();
    assert_eq!({ packet.header }, HEADER_GC_ITEM_SET);
    assert_eq!({ packet.length }, 54);
    assert_eq!({ packet.pos.window_type }, 1);
    assert_eq!({ packet.pos.cell }, 5);
    assert_eq!({ packet.vnum }, 10);
    assert_eq!(packet.count, 2);
    assert_eq!({ packet.flags }, 0x100);
    assert_eq!({ packet.anti_flags }, 0x200);
    assert_eq!(packet.highlight, 1);

    let sockets = packet.al_sockets();
    assert_eq!(sockets[0], 11);
    assert_eq!(sockets[1], 22);
    assert_eq!(sockets[2], 33);

    let attrs = packet.a_attrs();
    assert_eq!(attrs[0].bType, 1);
    assert_eq!({ attrs[0].sValue }, 0);
    assert_eq!(attrs[1].bType, 2);
    assert_eq!({ attrs[1].sValue }, 10);
}

#[test]
fn test_item_set_buffer_too_small() {
    let buf = [0u8; 53];
    let result = TPacketGCItemSet::from_bytes(&buf);
    assert_eq!(result, Err(PacketError::BufferTooSmall));
}

#[test]
fn test_m2_net_parse_item_set_ffi() {
    let mut buf = [0u8; 54];
    buf[0..2].copy_from_slice(&HEADER_GC_ITEM_SET.to_le_bytes());
    buf[2..4].copy_from_slice(&54u16.to_le_bytes());
    buf[4] = 2; // window_type
    buf[5..7].copy_from_slice(&12u16.to_le_bytes());
    buf[7..11].copy_from_slice(&100u32.to_le_bytes());
    buf[11] = 5; // count

    let mut out_packet: *mut TPacketGCItemSet = std::ptr::null_mut();

    let res = unsafe {
        m2_net_parse_item_set(buf.as_ptr(), buf.len(), &mut out_packet)
    };

    assert!(res);
    assert!(!out_packet.is_null());

    unsafe {
        let packet = &*out_packet;
        assert_eq!({ packet.header }, HEADER_GC_ITEM_SET);
        assert_eq!({ packet.pos.window_type }, 2);
        assert_eq!({ packet.pos.cell }, 12);
        assert_eq!({ packet.vnum }, 100);
        assert_eq!(packet.count, 5);

        m2_net_free_item_set(out_packet);
    }
}
