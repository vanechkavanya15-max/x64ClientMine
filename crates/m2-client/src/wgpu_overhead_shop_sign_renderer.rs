use std::sync::Arc;
use glam::Mat4;
use crate::wgpu_engine_context::{GpuTexture, Quad2D};
use crate::ui_private_shop_overhead_sign::PrivateShopSignManager;
use crate::ui_font_renderer::get_global_font_atlas;

/// Collects render batches for all private shop signs
pub fn collect_shop_sign_render_batches(
    manager: &mut PrivateShopSignManager,
    view_proj: &Mat4,
    screen_w: f32,
    screen_h: f32,
    white_tex: Arc<GpuTexture>,
) -> Vec<(Arc<GpuTexture>, Vec<Quad2D>)> {
    let font_char_width = 6.0;
    let font_char_height = 12.0;
    let padding_x = 2.0;
    let padding_y = 2.0;

    manager.update_render_states(
        view_proj,
        screen_w,
        screen_h,
        font_char_width,
        font_char_height,
        padding_x,
        padding_y,
    );

    let mut rect_quads = Vec::new();
    let mut text_quads = Vec::new();

    let font_tex_opt = {
        let mut font = get_global_font_atlas();
        let _ = font.init_atlas();
        font.texture.clone()
    };

    let states = manager.get_render_states();
    for state in states {
        // Find the corresponding sign for its title
        if let Some(sign) = manager.get_sign(state.vid) {
            let left = state.screen_x - state.width * 0.5;
            let top = state.screen_y - state.height;

            // 1. Background frame/border (ramka)
            // Slightly larger quad behind the background quad to act as a border.
            // C++ Metin2 uses thin borders, typically 1 pixel wide.
            let border_width = 1.0;
            let mut frame = Quad2D::new(
                left - border_width, 
                top - border_width, 
                state.width + border_width * 2.0, 
                state.height + border_width * 2.0
            );
            frame.color = [0.0, 0.0, 0.0, 1.0]; // Solid black border
            rect_quads.push(frame);

            // 2. Background inner
            let mut bg = Quad2D::new(left, top, state.width, state.height);
            // Typically Metin2 shop backgrounds are semi-transparent dark gray/blueish. 
            // We use semi-transparent black for now as it's safe.
            bg.color = [0.0, 0.0, 0.0, 0.3]; 
            rect_quads.push(bg);

            // 3. Title Text
            let font = get_global_font_atlas();
            // Start x for text (centered based on text width, we use the measure from font to be exact)
            let text_width = font.measure_text_width(&sign.title);
            let text_start_x = state.screen_x - text_width * 0.5;
            // The text baseline would typically be bottom or center of the rect
            let text_start_y = state.screen_y - padding_y; 

            // Text color (Yellowish for shop titles, matching typical Metin2 colors)
            let text_color = [1.0, 1.0, 0.0, 1.0];
            
            let quads = font.build_text_quads(&sign.title, text_start_x, text_start_y, text_color, 1.0);
            text_quads.extend(quads);
        }
    }

    let mut batches = Vec::new();
    if !rect_quads.is_empty() {
        batches.push((white_tex, rect_quads));
    }
    if !text_quads.is_empty() {
        if let Some(font_tex) = font_tex_opt {
            batches.push((font_tex, text_quads));
        }
    }

    batches
}
