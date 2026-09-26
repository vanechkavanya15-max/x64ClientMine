#[path = "../src/font_glyph_parser.rs"]
mod font_glyph_parser;

use font_glyph_parser::{FontLayoutEngine, GlyphMetric};

fn setup_engine() -> FontLayoutEngine {
    let mut engine = FontLayoutEngine::new();

    // Setup some dummy fonts
    let chars = vec![
        ('A', 10.0),
        ('B', 12.0),
        ('C', 11.0),
        ('a', 8.0),
        ('b', 9.0),
        ('c', 8.0),
        (' ', 5.0),
        ('.', 3.0),
        ('1', 7.0),
        ('2', 7.0),
        ('3', 7.0),
    ];

    for (c, adv) in chars {
        engine.register_glyph(GlyphMetric {
            char_code: c,
            uv_rect: (0.0, 0.0, 1.0, 1.0),
            width: adv,
            height: 15.0,
            bearing_x: 0.0,
            bearing_y: 0.0,
            advance_x: adv,
        });
    }

    engine
}

#[test]
fn test_register_and_get_glyph() {
    let mut engine = FontLayoutEngine::new();
    let metric = GlyphMetric {
        char_code: 'A',
        uv_rect: (0.1, 0.2, 0.3, 0.4),
        width: 10.0,
        height: 12.0,
        bearing_x: 1.0,
        bearing_y: 2.0,
        advance_x: 11.0,
    };

    engine.register_glyph(metric.clone());

    let retrieved = engine.get_glyph('A');
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap(), &metric);

    assert!(engine.get_glyph('B').is_none());
}

#[test]
fn test_measure_text_width_simple() {
    let engine = setup_engine();
    // 'A' = 10, 'B' = 12, 'C' = 11 -> 33
    let width = engine.measure_text_width("ABC");
    assert_eq!(width, 33.0);
}

#[test]
fn test_measure_text_width_with_spaces() {
    let engine = setup_engine();
    // 'A' = 10, ' ' = 5, 'B' = 12 -> 27
    let width = engine.measure_text_width("A B");
    assert_eq!(width, 27.0);
}

#[test]
fn test_measure_text_width_multiline() {
    let engine = setup_engine();
    // "ABC" = 33
    // "A B" = 27
    // "AB" = 22
    let width = engine.measure_text_width("ABC\nA B\nAB");
    assert_eq!(width, 33.0);
}

#[test]
fn test_measure_text_width_with_carriage_return() {
    let engine = setup_engine();
    // '\r' should be ignored
    let width = engine.measure_text_width("A\rB\rC");
    assert_eq!(width, 33.0);
}

#[test]
fn test_measure_text_width_unregistered_char() {
    let engine = setup_engine();
    // 'X' is not registered, advance should be 0.0
    // "AXB" = 10 + 0 + 12 = 22
    let width = engine.measure_text_width("AXB");
    assert_eq!(width, 22.0);
}

#[test]
fn test_wrap_text_empty() {
    let engine = setup_engine();
    let lines = engine.wrap_text("", 100.0);
    assert_eq!(lines.len(), 0);
}

#[test]
fn test_wrap_text_no_wrap_needed() {
    let engine = setup_engine();
    // "A B C" = 10 + 5 + 12 + 5 + 11 = 43
    let lines = engine.wrap_text("A B C", 50.0);
    assert_eq!(lines, vec!["A B C"]);
}

#[test]
fn test_wrap_text_wrap_needed() {
    let engine = setup_engine();
    // "A B C"
    // "A" = 10 -> space(5) + "B" = 12 -> 27 (fits 30)
    // "A B" + space(5) + "C"(11) = 43 (> 30) -> wrap
    // "C"
    let lines = engine.wrap_text("A B C", 30.0);
    assert_eq!(lines, vec!["A B", "C"]);
}

#[test]
fn test_wrap_text_explicit_newline() {
    let engine = setup_engine();
    let lines = engine.wrap_text("A\nB\nC", 100.0);
    assert_eq!(lines, vec!["A", "B", "C"]);
}

#[test]
fn test_wrap_text_explicit_newline_and_wrap() {
    let engine = setup_engine();
    // "A B C\na b c" max 30.0
    // "A B" (27) "C" (11) -> fits "A B", "C"
    // "a b c" = 8 + 5 + 9 + 5 + 8 = 35 (> 30)
    // "a b" = 22 (fits 30), space + "c" (8) = 35 (> 30)
    // so -> "A B", "C", "a b", "c"
    let lines = engine.wrap_text("A B C\na b c", 30.0);
    assert_eq!(lines, vec!["A B", "C", "a b", "c"]);
}

#[test]
fn test_wrap_text_long_single_word() {
    let engine = setup_engine();
    // A single word that exceeds the limit cannot be wrapped.
    // "ABC" = 33, limit 20. It should still just be returned as one line if it's a single word (or at least pushed).
    // Our implementation puts it as a single line since there are no spaces.
    let lines = engine.wrap_text("ABC", 20.0);
    assert_eq!(lines, vec!["ABC"]);
}

#[test]
fn test_wrap_text_trailing_spaces_and_newlines() {
    let engine = setup_engine();
    let lines = engine.wrap_text("A \n B \n\n", 100.0);
    assert_eq!(lines, vec!["A ", " B ", "", ""]);
}

#[test]
fn test_wrap_text_multiple_spaces() {
    let engine = setup_engine();
    // "A  B"
    // "A" = 10, "" = 0 (because of consecutive spaces), "B" = 12
    // A space character width is 5.
    let lines = engine.wrap_text("A  B", 20.0); // "A" + space + "" + space + "B" = 10 + 5 + 0 + 5 + 12 = 32 > 20
    // Actually split by ' ': ["A", "", "B"]
    // "A" (10)
    // + space(5) + ""(0) -> 15 (fits 20) -> "A "
    // + space(5) + "B"(12) -> 32 (> 20) -> wrap
    // -> "A ", "B"
    assert_eq!(lines, vec!["A ", "B"]);
}
