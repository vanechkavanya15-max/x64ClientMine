use m2_client::recv_quest_dialog_v2::{parse_script_packet, parse_quest_script, TPacketGCScript};

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
    assert!(result.is_some());
    let (skin, text) = result.unwrap();
    assert_eq!(skin, 5);
    assert_eq!(text, "Hello World"); // \0 truncated!
}

#[test]
fn test_parse_script_packet_too_small_header() {
    let data = vec![0u8; std::mem::size_of::<TPacketGCScript>() - 1];
    let result = parse_script_packet(&data);
    assert!(result.is_none());
}

#[test]
fn test_parse_script_packet_too_small_length() {
    let mut data = vec![0u8; std::mem::size_of::<TPacketGCScript>()];
    let packet_ptr = data.as_mut_ptr() as *mut TPacketGCScript;
    unsafe {
        (*packet_ptr).header = 0;
        (*packet_ptr).length = (std::mem::size_of::<TPacketGCScript>() + 10) as u16;
        (*packet_ptr).skin = 5;
    }
    
    // string only has 5 bytes instead of 10
    data.extend_from_slice(b"Hello");
    let result = parse_script_packet(&data);
    assert!(result.is_none());
}

#[test]
fn test_parse_quest_script_title_and_text() {
    let script = "[TITLE value;My Great Quest]Welcome to the quest![ENTER]Are you ready?";
    let dialog = parse_quest_script(1, script);
    
    assert_eq!(dialog.skin, 1);
    assert_eq!(dialog.title, "My Great Quest");
    assert_eq!(dialog.content, "Welcome to the quest!\nAre you ready?");
    assert!(dialog.buttons.is_empty());
    assert!(dialog.questions.is_empty());
}

#[test]
fn test_parse_quest_script_buttons_and_questions() {
    let script = "[TITLE value;Choices][QUESTBUTTON name;Take Quest][QUESTION idx;1|name;Yes|idx;2|name;No]What will you do?";
    let dialog = parse_quest_script(3, script);
    
    assert_eq!(dialog.skin, 3);
    assert_eq!(dialog.title, "Choices");
    assert_eq!(dialog.content, "What will you do?");
    assert_eq!(dialog.buttons.len(), 1);
    assert_eq!(dialog.buttons[0], "Take Quest");
    assert_eq!(dialog.questions.len(), 2);
    assert_eq!(dialog.questions[0], "Yes");
    assert_eq!(dialog.questions[1], "No");
}

#[test]
fn test_parse_quest_script_clear() {
    let script = "Some text[CLEAR]Fresh start";
    let dialog = parse_quest_script(0, script);
    assert_eq!(dialog.content, "Fresh start");
}

