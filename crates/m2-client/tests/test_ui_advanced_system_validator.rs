
use m2_client::ui_widget_bar3d::{Bar3D, D3dxColor};
use m2_client::texture_atlas_cache::TextureCache;
use m2_client::ui_widget_rendertarget::ActorRenderPreview;

use m2_client::ui_advanced_system_validator::UiAdvancedSystemValidator;

#[test]
fn test_bar3d_colors() {
    let validator = UiAdvancedSystemValidator::new();
    let mut bar = Bar3D::new();
    let left = D3dxColor::new(1.0, 0.0, 0.0, 1.0);
    let right = D3dxColor::new(0.0, 1.0, 0.0, 1.0);
    let center = D3dxColor::new(0.0, 0.0, 1.0, 1.0);
    assert!(validator.validate_bar3d_colors(&mut bar, left, right, center));
}

#[test]
fn test_texture_cache_insert() {
    let validator = UiAdvancedSystemValidator::new();
    let mut cache = TextureCache::new(1024 * 1024);
    assert!(validator.validate_texture_cache_insert(&mut cache, "test.tga", 32, 32, vec![0; 32 * 32 * 4]));
}

#[test]
fn test_actor_render_preview() {
    let validator = UiAdvancedSystemValidator::new();
    let mut preview = ActorRenderPreview::new(0, 0.0);
    assert!(validator.validate_actor_render_preview(&mut preview, 11299, 180.0));
}
