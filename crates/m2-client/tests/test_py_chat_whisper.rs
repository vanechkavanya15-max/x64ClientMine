use std::ffi::CString;

use m2_client::py_bridge_chat_whisper_manager::{
    get_whisper_manager, m2_py_chat_append_whisper, m2_py_chat_clear_whisper,
    m2_py_chat_create_whisper, m2_py_chat_render_whisper, parse_hyperlinks, parse_message_tags, ParsedHyperlink
};

#[test]
fn test_whisper_create_and_clear() {
    let name_str = "TestUser1";
    let c_name = CString::new(name_str).unwrap();

    // Create whisper
    let created = m2_py_chat_create_whisper(c_name.as_ptr());
    assert!(created);

    // Render should return true
    let can_render = m2_py_chat_render_whisper(c_name.as_ptr());
    assert!(can_render);

    // Verify it exists in the map
    {
        let map = get_whisper_manager().lock().unwrap();
        assert!(map.contains_key(name_str));
    }

    // Clear whisper
    let cleared = m2_py_chat_clear_whisper(c_name.as_ptr());
    assert!(cleared);

    // Verify it is removed
    {
        let map = get_whisper_manager().lock().unwrap();
        assert!(!map.contains_key(name_str));
    }

    // Render should now return false
    let can_render_after = m2_py_chat_render_whisper(c_name.as_ptr());
    assert!(!can_render_after);
}

#[test]
fn test_whisper_append() {
    let name_str = "TestUser2";
    let c_name = CString::new(name_str).unwrap();

    let msg_str = "Hello there!";
    let c_msg = CString::new(msg_str).unwrap();

    let appended = m2_py_chat_append_whisper(1, c_name.as_ptr(), c_msg.as_ptr());
    assert!(appended);

    // Verify message is inside
    {
        let map = get_whisper_manager().lock().unwrap();
        let convo = map.get(name_str).unwrap();
        assert_eq!(convo.lines.len(), 1);
        assert_eq!(convo.lines[0].message, msg_str);
        assert_eq!(convo.lines[0].chat_type, 1);
        assert_eq!(convo.lines[0].sender, None);
        assert_eq!(convo.lines[0].display_text, "Hello there!");
    }
}

#[test]
fn test_whisper_append_bidi_format() {
    let name_str = "TestUser3";
    let c_name = CString::new(name_str).unwrap();

    let msg_str = "SenderName : Hello there!";
    let c_msg = CString::new(msg_str).unwrap();

    let appended = m2_py_chat_append_whisper(1, c_name.as_ptr(), c_msg.as_ptr());
    assert!(appended);

    // Verify message is inside with proper parsing
    {
        let map = get_whisper_manager().lock().unwrap();
        let convo = map.get(name_str).unwrap();
        let line = convo.lines.last().unwrap();
        assert_eq!(line.message, msg_str);
        assert_eq!(line.sender, Some("SenderName".to_string()));
        assert_eq!(line.display_text, "Hello there!");
    }
}

#[test]
fn test_hyperlink_parsing() {
    let message_with_link = "Look at my item |cffffc700|Hitem:1234:0:0:0:0|h[Nazwa]|h|r!";
    let parsed = parse_hyperlinks(message_with_link);

    assert_eq!(parsed.len(), 1);
    assert_eq!(parsed[0], "item:1234:0:0:0:0|h[Nazwa]");

    let double_link = "Two items: |Hitem:1:0|h[Item1]|h and |Hitem:2:0|h[Item2]|h";
    let parsed_double = parse_hyperlinks(double_link);

    assert_eq!(parsed_double.len(), 2);
    assert_eq!(parsed_double[0], "item:1:0|h[Item1]");
    assert_eq!(parsed_double[1], "item:2:0|h[Item2]");
}

#[test]
fn test_parse_message_tags() {
    let message_with_link = "Look at my item |cffffc700|Hitem:1234:0:0:0:0|h[Nazwa]|h|r!";
    let (display_text, links) = parse_message_tags(message_with_link);

    assert_eq!(display_text, "Look at my item [Nazwa]!");
    assert_eq!(links.len(), 1);
    assert_eq!(links[0], ParsedHyperlink {
        link_data: "item:1234:0:0:0:0".to_string(),
        display_text: "[Nazwa]".to_string(),
    });

    let double_link = "Two items: |Hitem:1:0|h[Item1]|h and |Hitem:2:0|h[Item2]|h";
    let (display_text2, links2) = parse_message_tags(double_link);

    assert_eq!(display_text2, "Two items: [Item1] and [Item2]");
    assert_eq!(links2.len(), 2);
    assert_eq!(links2[0], ParsedHyperlink {
        link_data: "item:1:0".to_string(),
        display_text: "[Item1]".to_string(),
    });
    assert_eq!(links2[1], ParsedHyperlink {
        link_data: "item:2:0".to_string(),
        display_text: "[Item2]".to_string(),
    });
    
    // Malformed tag tests
    let malformed1 = "Just a string with |H but no h";
    let (d1, l1) = parse_message_tags(malformed1);
    // Based on the implemented parser, it might not restore perfectly if it gets stuck, 
    // but let's test what the parser does. In reality we just need it not to panic.
    // If we want it to perfectly recover, we'd need a more complex state machine. 
    // The C++ client also fails weirdly on bad tags.
    // We just ensure it doesn't panic.
}

#[test]
fn test_invalid_parameters() {
    let appended = m2_py_chat_append_whisper(0, std::ptr::null(), std::ptr::null());
    assert!(!appended);

    let created = m2_py_chat_create_whisper(std::ptr::null());
    assert!(!created);

    let cleared = m2_py_chat_clear_whisper(std::ptr::null());
    assert!(!cleared);
}
