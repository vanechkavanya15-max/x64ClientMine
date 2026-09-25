use m2_client_bridge::net::gc::item_set::PacketGcItemSet;

#[test]
fn test_from_bytes_success() {
    let mut buf = vec![0; 42];
    buf[0] = 20; // header
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

    // attr
    buf[21] = 8; buf[22] = 9; buf[23] = 0;
    // ... reszta 0

    let packet = PacketGcItemSet::from_bytes(&buf).unwrap();

    assert_eq!(packet.header, 20);
    assert_eq!(packet.cell.window_type, 1);
    assert_eq!(packet.cell.cell, 2);
    assert_eq!(packet.vnum, 3);
    assert_eq!(packet.count, 4);
    assert_eq!(packet.al_sockets, [5, 6, 7]);
    assert_eq!(packet.a_attr[0].b_type, 8);
    assert_eq!(packet.a_attr[0].s_value, 9);
}

#[test]
fn test_from_bytes_too_short() {
    let buf = vec![0; 41];
    let result = PacketGcItemSet::from_bytes(&buf);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), "Bufor jest za krotki");
}
