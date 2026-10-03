// test_full-ui-adv_09.rs - Jules Swarm Test Suite for full-ui-adv-09
#[path = "../src/ui_editline_formatter.rs"]
mod ui_editline_formatter;

use ui_editline_formatter::*;

#[test]
fn test_ui_adv_09_max_chars_truncation() {
    let mut fmt = UiEditLineFormatter::new();
    fmt.max_chars = 5;
    let res = fmt.format_display_text("123456789");
    assert_eq!(res, "12345");
}

#[test]
fn test_ui_adv_09_empty_string_handling() {
    let fmt = UiEditLineFormatter::new();
    let res = fmt.format_display_text("");
    assert_eq!(res, "");
    let lines = fmt.wrap_lines("");
    assert_eq!(lines, vec![""]);
}

#[test]
fn test_ui_adv_09_outline_clamping() {
    let mut fmt = UiEditLineFormatter::new();
    fmt.set_outline(true, [0.0, 1.0, 0.0, 1.0], 10.0);
    assert_eq!(fmt.outline_style.thickness, 3.0);

    fmt.set_outline(true, [0.0, 1.0, 0.0, 1.0], 0.1);
    assert_eq!(fmt.outline_style.thickness, 0.5);
}

#[test]
fn test_ui_adv_09_long_word_wrap() {
    let mut fmt = UiEditLineFormatter::new();
    fmt.set_wrap_mode(EditLineWrapMode::WordWrap, 32.0, 8.0); // limit 4 chars
    let lines = fmt.wrap_lines("BARDZODLUGIESLOWO");
    assert!(lines.len() >= 4);
}

#[test]
fn test_ui_adv_09_secret_multiline_combination() {
    let mut fmt = UiEditLineFormatter::new();
    fmt.set_secret_mode(true);
    fmt.set_wrap_mode(EditLineWrapMode::CharWrap, 40.0, 8.0); // 5 chars
    let masked = fmt.format_display_text("tajnehaslo");
    let lines = fmt.wrap_lines(&masked);
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0], "*****");
    assert_eq!(lines[1], "*****");
}
