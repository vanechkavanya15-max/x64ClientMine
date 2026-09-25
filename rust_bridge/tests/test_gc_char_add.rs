use m2_client_bridge::net::gc::char_add::CharAddPacket;

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

    let packet = CharAddPacket::from_bytes(&buf).expect("Deserializacja powinna sie udac dla poprawnego bufora");

    assert_eq!(packet.header, 1);
    assert_eq!(packet.vid, 123456789);
    assert_eq!(packet.angle, 45.5);
    assert_eq!(packet.x, 1000);
    assert_eq!(packet.y, 2000);
    assert_eq!(packet.z, 3000);
    assert_eq!(packet.b_type, 2);
    assert_eq!(packet.race_num, 3);
    assert_eq!(packet.moving_speed, 100);
    assert_eq!(packet.attack_speed, 120);
    assert_eq!(packet.state_flag, 4);
    assert_eq!(packet.affect_flag, [10, 20]);
}

#[test]
fn test_char_add_packet_buffer_too_short() {
    let buf = [0u8; 34]; // o jeden bajt za malo
    
    let result = CharAddPacket::from_bytes(&buf);
    assert!(result.is_err(), "Powinno zwrocic blad dla zbyt krotkiego bufora");
    assert_eq!(result.unwrap_err(), "Bufor jest zbyt krotki");
}
