use m2_client_bridge::net::gc::damage::{
    m2_net_free_damage, m2_net_parse_damage, TPacketGCDamage, TPacketGCDamageInfo,
    HEADER_GC_DAMAGE_INFO,
};

#[test]
fn test_gc_damage_info_valid() {
    let mut buf = [0u8; 13];
    buf[0..2].copy_from_slice(&HEADER_GC_DAMAGE_INFO.to_le_bytes());
    buf[2..4].copy_from_slice(&13u16.to_le_bytes());
    buf[4..8].copy_from_slice(&1000u32.to_le_bytes());
    buf[8] = 1;
    buf[9..13].copy_from_slice(&5000i32.to_le_bytes());

    let result = TPacketGCDamageInfo::from_bytes(&buf);
    assert!(result.is_ok());

    let packet = result.unwrap();
    assert_eq!({ packet.header }, HEADER_GC_DAMAGE_INFO);
    assert_eq!({ packet.length }, 13);
    assert_eq!(packet.dw_victim_vid(), 1000);
    assert_eq!(packet.b_flag(), 1);
    assert_eq!(packet.l_damage(), 5000);
}

#[test]
fn test_gc_damage_info_too_short() {
    let buf: [u8; 12] = [0; 12];

    let result = TPacketGCDamage::from_bytes(&buf);
    assert!(result.is_err());
    
    let err_msg = result.err().unwrap();
    assert!(err_msg.contains("za krotki"));
}

#[test]
fn test_gc_damage_info_longer_buf() {
    let mut buf = [0u8; 15];
    buf[0..2].copy_from_slice(&HEADER_GC_DAMAGE_INFO.to_le_bytes());
    buf[2..4].copy_from_slice(&13u16.to_le_bytes());
    buf[4..8].copy_from_slice(&1000u32.to_le_bytes());
    buf[8] = 1;
    buf[9..13].copy_from_slice(&5000i32.to_le_bytes());
    buf[13..15].copy_from_slice(&[0xFF, 0xFF]);

    let result = TPacketGCDamage::from_bytes(&buf);
    assert!(result.is_ok());

    let packet = result.unwrap();
    assert_eq!({ packet.header }, HEADER_GC_DAMAGE_INFO);
    assert_eq!(packet.dw_victim_vid(), 1000);
    assert_eq!(packet.b_flag(), 1);
    assert_eq!(packet.l_damage(), 5000);
}

#[test]
fn test_ffi_m2_net_parse_damage() {
    let mut buf = [0u8; 13];
    buf[0..2].copy_from_slice(&HEADER_GC_DAMAGE_INFO.to_le_bytes());
    buf[2..4].copy_from_slice(&13u16.to_le_bytes());
    buf[4..8].copy_from_slice(&2500u32.to_le_bytes());
    buf[8] = 2;
    buf[9..13].copy_from_slice(&(-100i32).to_le_bytes());

    let mut out_packet: *mut TPacketGCDamageInfo = std::ptr::null_mut();

    let success = unsafe {
        m2_net_parse_damage(buf.as_ptr(), buf.len(), &mut out_packet)
    };

    assert!(success);
    assert!(!out_packet.is_null());

    unsafe {
        let packet = &*out_packet;
        assert_eq!({ packet.header }, HEADER_GC_DAMAGE_INFO);
        assert_eq!(packet.dw_victim_vid(), 2500);
        assert_eq!(packet.b_flag(), 2);
        assert_eq!(packet.l_damage(), -100);

        m2_net_free_damage(out_packet);
    }
}
