#[path = "../src/recv_chat.rs"]
mod recv_chat;

use recv_chat::{
    parse_chat, parse_damage_info, m2_net_parse_damage_info, sanitize_chat_text,
};

#[test]
fn test_parse_chat_talking() {
    let mut data = vec![0; 5]; // header size
    
    // header = 0xAA (dummy)
    data[0] = 0xAA;
    data[1] = 0x00;
    
    // length = 5 (header) + 11 (text: "Hello World") = 16
    data[2] = 16;
    data[3] = 0;
    
    // type = 0 (Talking)
    data[4] = 0;
    
    // text
    data.extend_from_slice(b"Hello World");
    
    let result = parse_chat(&data).expect("Failed to parse valid chat packet");
    
    let chat_type = result.0;
    let text = result.1;
    
    assert_eq!(chat_type, 0); // Talking
    assert_eq!(text, "Hello World");
}

#[test]
fn test_parse_chat_notice() {
    let mut data = vec![0; 5];
    data[0] = 0xAA;
    data[2] = 20; // 5 + 15
    data[4] = 2; // Notice
    
    data.extend_from_slice(b"Server Restart!");
    
    let result = parse_chat(&data).expect("Failed to parse valid chat packet");
    
    assert_eq!(result.0, 2); // Notice
    assert_eq!(result.1, "Server Restart!");
}

#[test]
fn test_parse_damage_info() {
    let mut data = vec![0; 13]; // header size
    
    data[0] = 0xCC;
    data[2] = 13;
    
    // vid: 1000
    data[4] = 0xE8;
    data[5] = 0x03;
    data[6] = 0x00;
    data[7] = 0x00;
    
    // flag: 1
    data[8] = 1;
    
    // damage: -500
    data[9] = 0x0C;
    data[10] = 0xFE;
    data[11] = 0xFF;
    data[12] = 0xFF;
    
    let result = parse_damage_info(&data).expect("Failed to parse valid damage packet");
    
    let vid = result.0;
    let flag = result.1;
    let damage = result.2;
    
    assert_eq!(vid, 1000);
    assert_eq!(flag, 1);
    assert_eq!(damage, -500);
}

#[test]
fn test_m2_net_parse_damage() {
    let mut data = vec![0; 13];
    
    data[0] = 0xCC;
    data[2] = 13;
    
    // vid: 2000
    data[4] = 0xD0;
    data[5] = 0x07;
    data[6] = 0x00;
    data[7] = 0x00;
    
    // flag: 0
    data[8] = 0;
    
    // damage: 150
    data[9] = 0x96;
    data[10] = 0x00;
    data[11] = 0x00;
    data[12] = 0x00;
    
    let mut out_vid: u32 = 0;
    let mut out_dmg: i32 = 0;
    
    let result = m2_net_parse_damage_info(
        data.as_ptr(),
        data.len(),
        &mut out_vid,
        &mut out_dmg,
    );
    
    assert!(result);
    assert_eq!(out_vid, 2000);
    assert_eq!(out_dmg, 150);
}

#[test]
fn test_m2_net_parse_damage_invalid() {
    let data = vec![0; 5]; // Buffer too small
    
    let mut out_vid: u32 = 0;
    let mut out_dmg: i32 = 0;
    
    let result = m2_net_parse_damage_info(
        data.as_ptr(),
        data.len(),
        &mut out_vid,
        &mut out_dmg,
    );
    
    assert!(!result);
    assert_eq!(out_vid, 0);
    assert_eq!(out_dmg, 0);
}

#[test]
fn test_sanitize_chat_text() {
    let raw = "Hello\x07World\n\x08";
    let clean = sanitize_chat_text(raw);
    assert_eq!(clean, "HelloWorld\n");
}

#[test]
fn test_parse_buffer_too_small() {
    let data = vec![0; 2]; // smaller than TPacketGCChat
    let result = parse_chat(&data);
    assert!(result.is_none());
}

#[test]
fn test_parse_chat_buffer_smaller_than_length() {
    let mut data = vec![0; 5];
    data[2] = 20; 
    data[4] = 0; 
    
    let result = parse_chat(&data);
    assert!(result.is_none());
}

#[test]
fn test_filter_color_tags() {
    use recv_chat::filter_color_tags;
    let raw = "|cFF00FF00Hello|r |cffffffffWorld|r";
    let clean = filter_color_tags(raw);
    assert_eq!(clean, "Hello World");
}
