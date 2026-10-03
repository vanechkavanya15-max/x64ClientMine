// test_ui_editline_formatter.rs - Testy jednostkowe dla UiEditLineFormatter
#[path = "../src/ui_editline_formatter.rs"]
mod ui_editline_formatter;

use ui_editline_formatter::{EditLineWrapMode, UiEditLineFormatter};

#[test]
fn test_default_formatter() {
    let fmt = UiEditLineFormatter::new();
    assert!(!fmt.is_secret);
    assert_eq!(fmt.mask_char, '*');
    assert_eq!(fmt.wrap_mode, EditLineWrapMode::None);
    assert!(!fmt.outline_style.enabled);

    let res = fmt.format_display_text("Metin2 Pure Rust");
    assert_eq!(res, "Metin2 Pure Rust");
}

#[test]
fn test_secret_masking() {
    let mut fmt = UiEditLineFormatter::new();
    fmt.set_secret_mode(true);

    let res = fmt.format_display_text("tajnehaslo123");
    assert_eq!(res, "*************");
    assert_eq!(res.len(), 13);

    fmt.set_mask_char('#');
    let res2 = fmt.format_display_text("admin");
    assert_eq!(res2, "#####");
}

#[test]
fn test_multiline_word_wrap() {
    let mut fmt = UiEditLineFormatter::new();
    // 80px / 8px = 10 znakow na linie
    fmt.set_wrap_mode(EditLineWrapMode::WordWrap, 80.0, 8.0);

    let text = "To jest test zawijania tekstu";
    let lines = fmt.wrap_lines(text);
    assert!(lines.len() >= 3);
    for line in &lines {
        assert!(line.chars().count() <= 10, "Linia za dluga: '{}' (len={})", line, line.chars().count());
    }
}

#[test]
fn test_multiline_char_wrap() {
    let mut fmt = UiEditLineFormatter::new();
    // 40px / 8px = 5 znakow na linie
    fmt.set_wrap_mode(EditLineWrapMode::CharWrap, 40.0, 8.0);

    let text = "ABCDEFGHIJ";
    let lines = fmt.wrap_lines(text);
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0], "ABCDE");
    assert_eq!(lines[1], "FGHIJ");
}

#[test]
fn test_outline_offsets() {
    let mut fmt = UiEditLineFormatter::new();
    assert!(fmt.get_outline_offsets().is_empty());

    fmt.set_outline(true, [1.0, 0.0, 0.0, 1.0], 1.5);
    let offsets = fmt.get_outline_offsets();
    assert_eq!(offsets.len(), 8);
    assert_eq!(offsets[0], [-1.5, -1.5]);
    assert_eq!(offsets[7], [1.5, 1.5]);
}
