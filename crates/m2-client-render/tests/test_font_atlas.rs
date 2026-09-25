use m2_client_render::font_atlas::FontAtlas;

#[test]
fn test_integration_font_atlas() {
    let instance = FontAtlas::new();
    assert!(instance.is_ready());
}
