// test_ui_font_utf8_renderer.rs
use m2_client::ui_font_utf8_renderer::{FontAtlasUtf8, FontGlyph, get_global_font_atlas_utf8};
use serial_test::serial;

#[test]
#[serial]
fn test_measure_text_width_ascii() {
    let mut atlas = FontAtlasUtf8::new();
    // In headless test environment, init_atlas should fall back to basic rasterize_fallback_glyphs without failing
    // But since it tries to create WGPU texture which might fail if context is not setup,
    // let's just manually insert some glyphs for testing.
    atlas.glyphs.insert('A', FontGlyph { uv: [0.0; 4], width: 10.0, height: 16.0, advance: 10.0 });
    atlas.glyphs.insert('B', FontGlyph { uv: [0.0; 4], width: 10.0, height: 16.0, advance: 10.0 });
    atlas.glyphs.insert(' ', FontGlyph { uv: [0.0; 4], width: 5.0, height: 16.0, advance: 5.0 });
    
    let width = atlas.measure_text_width("AB A B");
    // 'A'(10) + 'B'(10) + ' '(5) + 'A'(10) + ' '(5) + 'B'(10) = 50
    assert_eq!(width, 50.0);
}

#[test]
#[serial]
fn test_measure_text_width_polish_chars() {
    let mut atlas = FontAtlasUtf8::new();
    
    atlas.glyphs.insert('Z', FontGlyph { uv: [0.0; 4], width: 10.0, height: 16.0, advance: 10.0 });
    atlas.glyphs.insert('a', FontGlyph { uv: [0.0; 4], width: 8.0, height: 16.0, advance: 8.0 });
    atlas.glyphs.insert('ż', FontGlyph { uv: [0.0; 4], width: 8.0, height: 16.0, advance: 8.0 });
    atlas.glyphs.insert('ó', FontGlyph { uv: [0.0; 4], width: 8.0, height: 16.0, advance: 8.0 });
    atlas.glyphs.insert('ł', FontGlyph { uv: [0.0; 4], width: 8.0, height: 16.0, advance: 8.0 });
    atlas.glyphs.insert('ć', FontGlyph { uv: [0.0; 4], width: 8.0, height: 16.0, advance: 8.0 });
    atlas.glyphs.insert(' ', FontGlyph { uv: [0.0; 4], width: 5.0, height: 16.0, advance: 5.0 });
    
    let text = "Zażółć";
    let width = atlas.measure_text_width(text);
    // 'Z'(10) + 'a'(8) + 'ż'(8) + 'ó'(8) + 'ł'(8) + 'ć'(8) = 50
    assert_eq!(width, 50.0);
}

#[test]
#[serial]
fn test_measure_text_width_fallback_unknown_chars() {
    let mut atlas = FontAtlasUtf8::new();
    atlas.glyphs.insert('a', FontGlyph { uv: [0.0; 4], width: 8.0, height: 16.0, advance: 8.0 });
    
    // 'a' = 8.0, unknown char '🚀' should use fallback of 7.0
    let text = "a🚀a";
    let width = atlas.measure_text_width(text);
    // 'a'(8) + '🚀'(7) + 'a'(8) = 23
    assert_eq!(width, 23.0);
}

#[test]
#[serial]
fn test_render_text_utf8_quads_count() {
    let mut atlas = FontAtlasUtf8::new();
    atlas.glyphs.insert('H', FontGlyph { uv: [0.0; 4], width: 10.0, height: 16.0, advance: 10.0 });
    atlas.glyphs.insert('e', FontGlyph { uv: [0.0; 4], width: 8.0, height: 16.0, advance: 8.0 });
    atlas.glyphs.insert('l', FontGlyph { uv: [0.0; 4], width: 6.0, height: 16.0, advance: 6.0 });
    atlas.glyphs.insert('o', FontGlyph { uv: [0.0; 4], width: 8.0, height: 16.0, advance: 8.0 });
    atlas.glyphs.insert(' ', FontGlyph { uv: [0.0; 4], width: 5.0, height: 16.0, advance: 5.0 });
    
    let text = "Hello Hello";
    let quads = atlas.render_text_utf8(text, 0, 0, [1.0, 1.0, 1.0, 1.0], 1.0);
    // 5 chars per "Hello" -> 10 quads. Space doesn't produce quad. Total 10 quads.
    assert_eq!(quads.len(), 10);
}

#[test]
#[serial]
fn test_render_text_utf8_positions() {
    let mut atlas = FontAtlasUtf8::new();
    atlas.glyphs.insert('A', FontGlyph { uv: [0.0, 0.0, 0.1, 0.1], width: 10.0, height: 16.0, advance: 10.0 });
    atlas.glyphs.insert('B', FontGlyph { uv: [0.1, 0.0, 0.2, 0.1], width: 10.0, height: 16.0, advance: 10.0 });
    
    let quads = atlas.render_text_utf8("AB", 10, 20, [1.0, 1.0, 1.0, 1.0], 1.0);
    assert_eq!(quads.len(), 2);
    
    // Quad A
    assert_eq!(quads[0].rect[0], 10.0); // x
    assert_eq!(quads[0].rect[1], 20.0); // y
    
    // Quad B
    assert_eq!(quads[1].rect[0], 20.0); // x (10 + A advance 10)
    assert_eq!(quads[1].rect[1], 20.0); // y
}

#[test]
#[serial]
fn test_render_text_utf8_fallback() {
    let mut atlas = FontAtlasUtf8::new();
    atlas.glyphs.insert('?', FontGlyph { uv: [0.0; 4], width: 7.0, height: 16.0, advance: 7.0 });
    
    // Test with character not in atlas, but with '?' fallback present
    let quads = atlas.render_text_utf8("XYZ", 0, 0, [1.0, 1.0, 1.0, 1.0], 1.0);
    assert_eq!(quads.len(), 3);
    assert_eq!(quads[0].rect[0], 0.0);
    assert_eq!(quads[1].rect[0], 7.0);
    assert_eq!(quads[2].rect[0], 14.0);
}

#[test]
#[serial]
fn test_global_font_atlas_utf8_creation() {
    // Should not panic on accessing global singleton
    let atlas_guard = get_global_font_atlas_utf8();
    assert_eq!(atlas_guard.line_height, 16.0);
}
