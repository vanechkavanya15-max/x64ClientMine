use m2_client_bridge::net::gc::char_add::{TPacketGCCharacterAdd, PacketError, m2_net_parse_char_add};

#[test]
fn test_char_add_packet_deserialization() {
    let mut buf = [0u8; 35];
    buf[0] = 1; // header
    
    // vid: 123456789
    let vid_bytes = 123456789u32.to_le_bytes();
    buf[1..5].copy_from_slice(&vid_bytes);
    
    // angle: 45.5
    let angle_bytes = 45.5f32.to_le_bytes();
    buf[5..9].copy_from_slice(&angle_bytes);
    
    // x: 1000
    let x_bytes = 1000i32.to_le_bytes();
    buf[9..13].copy_from_slice(&x_bytes);
    
    // y: 2000
    let y_bytes = 2000i32.to_le_bytes();
    buf[13..17].copy_from_slice(&y_bytes);
    
    // z: 3000
    let z_bytes = 3000i32.to_le_bytes();
    buf[17..21].copy_from_slice(&z_bytes);
    
    // b_type: 2
    buf[21] = 2;
    
    // race_num: 3
    let race_num_bytes = 3u16.to_le_bytes();
    buf[22..24].copy_from_slice(&race_num_bytes);
    
    // moving_speed: 100
    buf[24] = 100;
    
    // attack_speed: 120
    buf[25] = 120;
    
    // state_flag: 4
    buf[26] = 4;
    
    // affect_flag: [10, 20]
    let affect_0_bytes = 10u32.to_le_bytes();
    buf[27..31].copy_from_slice(&affect_0_bytes);
    let affect_1_bytes = 20u32.to_le_bytes();
    buf[31..35].copy_from_slice(&affect_1_bytes);

    let packet = TPacketGCCharacterAdd::from_bytes(&buf).expect("Deserializacja powinna sie udac dla poprawnego bufora");

    unsafe {
        assert_eq!(std::ptr::addr_of!(packet.header).read_unaligned(), 1);
        assert_eq!(std::ptr::addr_of!(packet.dw_vid).read_unaligned(), 123456789);
        assert_eq!(std::ptr::addr_of!(packet.angle).read_unaligned(), 45.5);
        assert_eq!(std::ptr::addr_of!(packet.x).read_unaligned(), 1000);
        assert_eq!(std::ptr::addr_of!(packet.y).read_unaligned(), 2000);
        assert_eq!(std::ptr::addr_of!(packet.z).read_unaligned(), 3000);
        assert_eq!(std::ptr::addr_of!(packet.b_type).read_unaligned(), 2);
        assert_eq!(std::ptr::addr_of!(packet.w_race_num).read_unaligned(), 3);
        assert_eq!(std::ptr::addr_of!(packet.b_moving_speed).read_unaligned(), 100);
        assert_eq!(std::ptr::addr_of!(packet.b_attack_speed).read_unaligned(), 120);
        assert_eq!(std::ptr::addr_of!(packet.b_state_flag).read_unaligned(), 4);
        assert_eq!(std::ptr::addr_of!(packet.dw_affect_flag).read_unaligned(), [10, 20]);
    }
}

#[test]
fn test_char_add_packet_buffer_too_short() {
    let buf = [0u8; 34]; // o jeden bajt za malo
    
    let result = TPacketGCCharacterAdd::from_bytes(&buf);
    assert!(result.is_err(), "Powinno zwrocic blad dla zbyt krotkiego bufora");
    assert_eq!(result.unwrap_err(), PacketError::Eof);
}

#[test]
fn test_ffi_m2_net_parse_char_add() {
    let mut buf = [0u8; 35];
    buf[0] = 1; // header
    
    let vid_bytes = 123456789u32.to_le_bytes();
    buf[1..5].copy_from_slice(&vid_bytes);
    
    let x_bytes = 1000i32.to_le_bytes();
    buf[9..13].copy_from_slice(&x_bytes);
    
    let y_bytes = 2000i32.to_le_bytes();
    buf[13..17].copy_from_slice(&y_bytes);
    
    let race_num_bytes = 3u16.to_le_bytes();
    buf[22..24].copy_from_slice(&race_num_bytes);

    let mut out_vid = 0u32;
    let mut out_x = 0i32;
    let mut out_y = 0i32;
    let mut out_race = 0u16;

    let result = unsafe {
        m2_net_parse_char_add(
            buf.as_ptr(),
            35,
            &mut out_vid,
            &mut out_x,
            &mut out_y,
            &mut out_race,
        )
    };

    assert!(result, "FFI call should succeed");
    assert_eq!(out_vid, 123456789);
    assert_eq!(out_x, 1000);
    assert_eq!(out_y, 2000);
    assert_eq!(out_race, 3);
}

#[test]
fn test_ffi_m2_net_parse_char_add_null_ptr() {
    let buf = [0u8; 35];

    let mut out_vid = 0u32;
    let mut out_x = 0i32;
    let mut out_y = 0i32;
    let mut out_race = 0u16;

    let result_null_data = unsafe {
        m2_net_parse_char_add(
            std::ptr::null(),
            35,
            &mut out_vid,
            &mut out_x,
            &mut out_y,
            &mut out_race,
        )
    };
    assert!(!result_null_data, "Should fail on null data pointer");

    let result_null_out = unsafe {
        m2_net_parse_char_add(
            buf.as_ptr(),
            35,
            std::ptr::null_mut(),
            &mut out_x,
            &mut out_y,
            &mut out_race,
        )
    };
    assert!(!result_null_out, "Should fail on null output pointer");
}

#[test]
fn test_ffi_m2_net_parse_char_add_too_short() {
    let buf = [0u8; 34];

    let mut out_vid = 0u32;
    let mut out_x = 0i32;
    let mut out_y = 0i32;
    let mut out_race = 0u16;

    let result = unsafe {
        m2_net_parse_char_add(
            buf.as_ptr(),
            34,
            &mut out_vid,
            &mut out_x,
            &mut out_y,
            &mut out_race,
        )
    };
    assert!(!result, "Should fail when buffer is too short");
}
