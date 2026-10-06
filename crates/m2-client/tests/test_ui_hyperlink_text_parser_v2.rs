#[path = "../src/ui_hyperlink_text_parser_v2.rs"]
mod ui_hyperlink_text_parser_v2;

use ui_hyperlink_text_parser_v2::*;

fn to_u16_str(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

#[test]
fn test_get_text_tag_plain() {
    let src = to_u16_str("Hello");
    let mut tag_len = 0;
    let mut extra_info = Vec::new();
    let tag = get_text_tag(&src, src.len() as i32, &mut tag_len, &mut extra_info);
    assert_eq!(tag, TEXT_TAG_PLAIN);
    assert_eq!(tag_len, 1);
}

#[test]
fn test_get_text_tag_color() {
    let src = to_u16_str("|cffffffff");
    let mut tag_len = 0;
    let mut extra_info = Vec::new();
    let tag = get_text_tag(&src, src.len() as i32, &mut tag_len, &mut extra_info);
    assert_eq!(tag, TEXT_TAG_COLOR);
    assert_eq!(tag_len, 10);
    assert_eq!(extra_info, to_u16_str("ffffffff"));
}

#[test]
fn test_get_text_tag_restore_color() {
    let src = to_u16_str("|r");
    let mut tag_len = 0;
    let mut extra_info = Vec::new();
    let tag = get_text_tag(&src, src.len() as i32, &mut tag_len, &mut extra_info);
    assert_eq!(tag, TEXT_TAG_RESTORE_COLOR);
    assert_eq!(tag_len, 2);
}

#[test]
fn test_get_text_tag_tag() {
    let src = to_u16_str("||");
    let mut tag_len = 0;
    let mut extra_info = Vec::new();
    let tag = get_text_tag(&src, src.len() as i32, &mut tag_len, &mut extra_info);
    assert_eq!(tag, TEXT_TAG_TAG);
    assert_eq!(tag_len, 2);
}

#[test]
fn test_get_text_tag_hyperlink_start() {
    let src = to_u16_str("|H");
    let mut tag_len = 0;
    let mut extra_info = Vec::new();
    let tag = get_text_tag(&src, src.len() as i32, &mut tag_len, &mut extra_info);
    assert_eq!(tag, TEXT_TAG_HYPERLINK_START);
    assert_eq!(tag_len, 2);
}

#[test]
fn test_get_text_tag_hyperlink_end() {
    let src = to_u16_str("|h");
    let mut tag_len = 0;
    let mut extra_info = Vec::new();
    let tag = get_text_tag(&src, src.len() as i32, &mut tag_len, &mut extra_info);
    assert_eq!(tag, TEXT_TAG_HYPERLINK_END);
    assert_eq!(tag_len, 2);
}

#[test]
fn test_get_text_tag_output_string_hyperlink() {
    let src = to_u16_str("prefix |Hitem:1234|h[ItemName]|h suffix");
    let output = get_text_tag_output_string(&src, src.len() as i32);
    let output_str = String::from_utf16(&output).unwrap();
    assert_eq!(output_str, "prefix [ItemName] suffix");
}

#[test]
fn test_get_text_tag_output_string_colors() {
    let src = to_u16_str("test |cffff0000color|r test2");
    let output = get_text_tag_output_string(&src, src.len() as i32);
    let output_str = String::from_utf16(&output).unwrap();
    assert_eq!(output_str, "test color test2");
}

#[test]
fn test_get_text_tag_output_string_escaped_pipe() {
    let src = to_u16_str("test || test");
    let output = get_text_tag_output_string(&src, src.len() as i32);
    let output_str = String::from_utf16(&output).unwrap();
    assert_eq!(output_str, "test | test");
}

#[test]
fn test_get_text_tag_output_len() {
    let src = to_u16_str("prefix |Hitem:1234|h[ItemName]|h suffix");
    let len = get_text_tag_output_len(&src, src.len() as i32);
    let expected = 24; // length of "prefix [ItemName] suffix"
    assert_eq!(len, expected);
}

#[test]
fn test_get_text_tag_internal_pos() {
    let src = to_u16_str("ab|cffff0000cd|r");
    // Visible: "abcd"
    // offset 0 => a (pos 0)
    // offset 1 => b (pos 1)
    // offset 2 => c (pos 2 because color_tag is true and it preserves internal_offset = i which is the pos of |c)
    // offset 3 => d (pos 2)
    let pos0 = get_text_tag_internal_pos_from_render_pos(&src, src.len() as i32, 0);
    assert_eq!(pos0, 0);
    
    let pos1 = get_text_tag_internal_pos_from_render_pos(&src, src.len() as i32, 1);
    assert_eq!(pos1, 1);
    
    let pos2 = get_text_tag_internal_pos_from_render_pos(&src, src.len() as i32, 2);
    assert_eq!(pos2, 2);
    
    let pos3 = get_text_tag_internal_pos_from_render_pos(&src, src.len() as i32, 3);
    assert_eq!(pos3, 2);
}

#[test]
fn test_find_color_tag_start_position() {
    let src = to_u16_str("xx|cffff0000xx|r");
    let pos = find_color_tag_start_position(&src, src.len() as i32);
    assert_eq!(pos, 13);
}

#[test]
fn test_find_color_tag_end_position() {
    let src = to_u16_str("|cffff0000xx|ryy");
    let pos = find_color_tag_end_position(&src, src.len() as i32);
    assert_eq!(pos, 13);
}
