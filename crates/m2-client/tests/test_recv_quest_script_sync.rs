#![allow(non_snake_case)]

#[path = "../src/recv_quest_script_sync.rs"]
mod recv_quest_script_sync;

use recv_quest_script_sync::*;

#[test]
fn test_parse_script_packet_valid() {
    let mut data = vec![0u8; std::mem::size_of::<TPacketGCScript>()];
    let packet_ptr = data.as_mut_ptr() as *mut TPacketGCScript;
    unsafe {
        (*packet_ptr).header = 0;
        (*packet_ptr).length = (std::mem::size_of::<TPacketGCScript>() + 12) as u16;
        (*packet_ptr).skin = 5;
        (*packet_ptr).src_size = 0;
    }
    
    // add string "Hello World\0"
    data.extend_from_slice(b"Hello World\0");

    let result = parse_script_packet(&data);
    assert!(result.is_ok());
    let (packet, text) = result.unwrap();
    assert_eq!(packet.skin, 5);
    assert_eq!(text, "Hello World"); // \0 truncated!
}

#[test]
fn test_parse_script_packet_errors() {
    let data = vec![0u8; std::mem::size_of::<TPacketGCScript>() - 1];
    let result = parse_script_packet(&data);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "RecvScriptPacket_RecvError");

    let mut data = vec![0u8; std::mem::size_of::<TPacketGCScript>()];
    let packet_ptr = data.as_mut_ptr() as *mut TPacketGCScript;
    unsafe {
        (*packet_ptr).header = 0;
        (*packet_ptr).length = 2; // Too small
    }
    let result = parse_script_packet(&data);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "RecvScriptPacket_SizeError");
}

#[test]
fn test_parse_quest_info_packet_begin() {
    let mut data = vec![0u8; std::mem::size_of::<TPacketGCQuestInfo>()];
    let packet_ptr = data.as_mut_ptr() as *mut TPacketGCQuestInfo;
    unsafe {
        (*packet_ptr).header = 0;
        (*packet_ptr).length = 0; // Not strictly validated dynamically by C++ here, size check is manual per field.
        (*packet_ptr).index = 100;
        (*packet_ptr).flag = QUEST_SEND_IS_BEGIN | QUEST_SEND_TITLE | QUEST_SEND_CLOCK_VALUE;
    }
    
    data.push(1); // isBegin = 1 (true)
    
    let mut title = vec![0u8; 30];
    b"My Quest Title\0".iter().enumerate().for_each(|(i, &b)| title[i] = b);
    data.extend_from_slice(&title);
    
    let clock_value = 12345i32;
    data.extend_from_slice(&clock_value.to_ne_bytes());

    let result = parse_quest_info_packet(&data);
    assert!(result.is_ok());
    
    let parsed = result.unwrap();
    assert_eq!(parsed.index, 100);
    assert_eq!(parsed.packet_type, QuestPacketType::Begin);
    assert_eq!(parsed.title.unwrap(), "My Quest Title");
    assert_eq!(parsed.clock_value.unwrap(), 12345);
    assert!(parsed.clock_name.is_none());
}

#[test]
fn test_parse_quest_info_packet_end() {
    let mut data = vec![0u8; std::mem::size_of::<TPacketGCQuestInfo>()];
    let packet_ptr = data.as_mut_ptr() as *mut TPacketGCQuestInfo;
    unsafe {
        (*packet_ptr).header = 0;
        (*packet_ptr).index = 200;
        (*packet_ptr).flag = QUEST_SEND_IS_BEGIN;
    }
    
    data.push(0); // isBegin = 0 (false) -> End

    let result = parse_quest_info_packet(&data);
    assert!(result.is_ok());
    
    let parsed = result.unwrap();
    assert_eq!(parsed.index, 200);
    assert_eq!(parsed.packet_type, QuestPacketType::End);
}

#[test]
fn test_parse_quest_info_packet_update() {
    let mut data = vec![0u8; std::mem::size_of::<TPacketGCQuestInfo>()];
    let packet_ptr = data.as_mut_ptr() as *mut TPacketGCQuestInfo;
    unsafe {
        (*packet_ptr).header = 0;
        (*packet_ptr).index = 300;
        (*packet_ptr).flag = QUEST_SEND_ICON_FILE;
    }
    
    let mut icon = vec![0u8; 24];
    b"icon.png\0".iter().enumerate().for_each(|(i, &b)| icon[i] = b);
    data.extend_from_slice(&icon);

    let result = parse_quest_info_packet(&data);
    assert!(result.is_ok());
    
    let parsed = result.unwrap();
    assert_eq!(parsed.index, 300);
    assert_eq!(parsed.packet_type, QuestPacketType::Update);
    assert_eq!(parsed.icon_file.unwrap(), "icon.png");
}

#[test]
fn test_parse_quest_confirm_packet() {
    let mut data = vec![0u8; std::mem::size_of::<TPacketGCQuestConfirm>()];
    let packet_ptr = data.as_mut_ptr() as *mut TPacketGCQuestConfirm;
    unsafe {
        (*packet_ptr).header = 123;
        (*packet_ptr).timeout = 10;
        (*packet_ptr).requestPID = 555;
    }
    
    let result = parse_quest_confirm_packet(&data);
    assert!(result.is_ok());
    let parsed = result.unwrap();
    let (header, timeout, requestPID) = {
        (parsed.header, parsed.timeout, parsed.requestPID)
    };
    assert_eq!(header, 123);
    assert_eq!(timeout, 10);
    assert_eq!(requestPID, 555);
}

#[test]
fn test_build_outgoing_packets() {
    let answer_packet = build_script_answer_packet(2);
    let (answer, answer_header) = (answer_packet.answer, answer_packet.header);
    assert_eq!(answer, 2);
    assert_eq!(answer_header, 0x0901);
    
    let button_packet = build_script_button_packet(42);
    let (idx, button_header) = (button_packet.idx, button_packet.header);
    assert_eq!(idx, 42);
    assert_eq!(button_header, 0x0902);
    
    let input_string = "HelloWorld";
    let input_packet = build_quest_input_string_packet(input_string);
    let input_header = input_packet.header;
    assert_eq!(input_header, 0x0904);
    
    let mut cstr = vec![0u8; input_string.len()];
    cstr.copy_from_slice(&input_packet.szString[..input_string.len()]);
    assert_eq!(cstr, input_string.as_bytes());
}
