use crate::ui_widget_bar3d::{Bar3D, D3dxColor};
use crate::texture_atlas_cache::{TextureCache, CachedTexture};
use crate::ui_widget_rendertarget::{ActorRenderPreview, RenderTarget};

pub struct UiAdvancedSystemValidator;

impl UiAdvancedSystemValidator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_bar3d_colors(&self, bar: &mut Bar3D, left: D3dxColor, right: D3dxColor, center: D3dxColor) -> bool {
        bar.set_color(left, right, center);
        bar.left_color == left && bar.right_color == right && bar.center_color == center
    }

    pub fn validate_texture_cache_insert(&self, cache: &mut TextureCache, path: &str, width: u32, height: u32, data: Vec<u8>) -> bool {
        let res = cache.get_or_load(path, 1, || Ok((width, height, data.clone())));
        res.is_ok() && cache.current_memory_usage() >= data.len()
    }

    pub fn validate_actor_render_preview(&self, preview: &mut ActorRenderPreview, vnum: u32, rotation: f32) -> bool {
        preview.set_armor(vnum);
        preview.set_rotation(rotation);
        preview.armor_vnum == vnum && preview.rotation_y == rotation
    }
}
