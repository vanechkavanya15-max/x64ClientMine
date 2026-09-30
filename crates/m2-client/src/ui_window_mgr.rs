// ui_window_mgr.rs - Central UI Window Manager, Hierarchical Layout and Quad Dispatcher for Metin2 Pure Rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use crate::ui_window_core::{Window, WindowType};
use crate::ui_texture_mgr::get_ui_texture_manager;
use crate::ui_font_renderer::get_global_font_atlas;
use crate::wgpu_engine_context::{GpuTexture, Quad2D, WgpuEngineContext};

pub struct WindowManager {
    pub windows: HashMap<u32, Window>,
    pub next_handle: u32,
    pub root_windows: Vec<u32>,
    pub top_window: Option<u32>,
    pub white_dummy_tex: Option<Arc<GpuTexture>>,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
            next_handle: 100,
            root_windows: Vec::new(),
            top_window: None,
            white_dummy_tex: None,
        }
    }

    pub fn register(&mut self, wtype: WindowType) -> u32 {
        let handle = self.next_handle;
        self.next_handle += 1;
        let win = Window::new(handle, wtype);
        self.windows.insert(handle, win);
        self.root_windows.push(handle);
        handle
    }

    pub fn set_parent(&mut self, child_handle: u32, parent_handle: u32) -> bool {
        if child_handle == parent_handle || !self.windows.contains_key(&child_handle) || !self.windows.contains_key(&parent_handle) {
            return false;
        }

        // Usun z poprzedniego rodzica lub z root_windows
        if let Some(old_parent) = self.windows.get(&child_handle).and_then(|w| w.parent) {
            if let Some(p_win) = self.windows.get_mut(&old_parent) {
                p_win.children.retain(|&c| c != child_handle);
            }
        } else {
            self.root_windows.retain(|&r| r != child_handle);
        }

        // Dodaj do nowego rodzica
        if let Some(p_win) = self.windows.get_mut(&parent_handle) {
            p_win.children.push(child_handle);
        }
        if let Some(c_win) = self.windows.get_mut(&child_handle) {
            c_win.parent = Some(parent_handle);
        }
        true
    }

    pub fn set_position(&mut self, handle: u32, x: i32, y: i32) -> bool {
        if let Some(win) = self.windows.get_mut(&handle) {
            win.local_x = x;
            win.local_y = y;
            true
        } else {
            false
        }
    }

    pub fn set_size(&mut self, handle: u32, w: i32, h: i32) -> bool {
        if let Some(win) = self.windows.get_mut(&handle) {
            win.width = w;
            win.height = h;
            true
        } else {
            false
        }
    }

    pub fn show(&mut self, handle: u32) -> bool {
        if let Some(win) = self.windows.get_mut(&handle) {
            win.is_show = true;
            true
        } else {
            false
        }
    }

    pub fn hide(&mut self, handle: u32) -> bool {
        if let Some(win) = self.windows.get_mut(&handle) {
            win.is_show = false;
            true
        } else {
            false
        }
    }

    pub fn is_show(&self, handle: u32) -> bool {
        self.windows.get(&handle).map(|w| w.is_show).unwrap_or(false)
    }

    pub fn load_image(&mut self, handle: u32, path: &str) -> bool {
        if let Some(win) = self.windows.get_mut(&handle) {
            win.image_path = path.to_string();
            true
        } else {
            false
        }
    }

    pub fn set_rendering_rect(&mut self, handle: u32, l: f32, t: f32, r: f32, b: f32) -> bool {
        if let Some(win) = self.windows.get_mut(&handle) {
            win.rendering_rect = [l, t, r, b];
            true
        } else {
            false
        }
    }

    pub fn set_scale(&mut self, handle: u32, sx: f32, sy: f32) -> bool {
        if let Some(win) = self.windows.get_mut(&handle) {
            win.scale = [sx, sy];
            true
        } else {
            false
        }
    }

    pub fn set_text(&mut self, handle: u32, text: &str) -> bool {
        if let Some(win) = self.windows.get_mut(&handle) {
            win.text = text.to_string();
            true
        } else {
            false
        }
    }

    pub fn set_color(&mut self, handle: u32, color: [f32; 4]) -> bool {
        if let Some(win) = self.windows.get_mut(&handle) {
            win.color = color;
            true
        } else {
            false
        }
    }

    pub fn set_font_color(&mut self, handle: u32, color: [f32; 4]) -> bool {
        if let Some(win) = self.windows.get_mut(&handle) {
            win.text_color = color;
            true
        } else {
            false
        }
    }

    pub fn set_align(&mut self, handle: u32, h_align: u8, v_align: u8) -> bool {
        if let Some(win) = self.windows.get_mut(&handle) {
            win.horizontal_align = h_align;
            win.vertical_align = v_align;
            true
        } else {
            false
        }
    }

    pub fn set_button_visuals(&mut self, handle: u32, up: &str, over: &str, down: &str) -> bool {
        if let Some(win) = self.windows.get_mut(&handle) {
            win.up_visual = up.to_string();
            win.over_visual = over.to_string();
            win.down_visual = down.to_string();
            true
        } else {
            false
        }
    }

    pub fn set_name(&mut self, handle: u32, name: &str) -> bool {
        if let Some(win) = self.windows.get_mut(&handle) {
            win.name = name.to_string();
            true
        } else {
            false
        }
    }

    /// Zwraca uchwyt okna najglebiej polozonego pod kursorem (hit-test)
    pub fn pick_window(&self, mx: i32, my: i32, screen_w: i32, screen_h: i32) -> Option<u32> {
        fn pick_rec(
            wm: &WindowManager,
            handle: u32,
            parent_rect: Option<[i32; 4]>,
            mx: i32,
            my: i32,
            screen_w: i32,
            screen_h: i32,
        ) -> Option<u32> {
            let win = wm.windows.get(&handle)?;
            if !win.is_show {
                return None;
            }
            let rect = win.calc_global_rect(parent_rect, screen_w, screen_h);
            let [gx, gy, gw, gh] = rect;

            // Sprawdz najpierw dzieci od konca (najwyzszy Z-order)
            for &child in win.children.iter().rev() {
                if let Some(hit) = pick_rec(wm, child, Some(rect), mx, my, screen_w, screen_h) {
                    return Some(hit);
                }
            }

            // Sprawdz samo okno
            if mx >= gx && mx <= gx + gw && my >= gy && my <= gy + gh {
                Some(handle)
            } else {
                None
            }
        }

        for &root in self.root_windows.iter().rev() {
            if let Some(hit) = pick_rec(self, root, None, mx, my, screen_w, screen_h) {
                return Some(hit);
            }
        }
        None
    }

    pub fn handle_mouse_move(&mut self, mx: i32, my: i32, screen_w: i32, screen_h: i32) {
        let hovered = self.pick_window(mx, my, screen_w, screen_h);
        for win in self.windows.values_mut() {
            let is_h = hovered == Some(win.handle);
            if win.is_hover != is_h {
                win.is_hover = is_h;
            }
        }
    }

    pub fn handle_mouse_down(&mut self, mx: i32, my: i32, screen_w: i32, screen_h: i32) -> Option<u32> {
        let hit = self.pick_window(mx, my, screen_w, screen_h)?;
        if let Some(win) = self.windows.get_mut(&hit) {
            win.is_pressed = true;
        }
        Some(hit)
    }

    pub fn handle_mouse_up(&mut self, mx: i32, my: i32, screen_w: i32, screen_h: i32) -> Option<u32> {
        let hit = self.pick_window(mx, my, screen_w, screen_h);
        for win in self.windows.values_mut() {
            win.is_pressed = false;
        }
        hit
    }

    pub fn destroy_window(&mut self, handle: u32) {
        if let Some(win) = self.windows.remove(&handle) {
            if let Some(p) = win.parent {
                if let Some(p_win) = self.windows.get_mut(&p) {
                    p_win.children.retain(|&c| c != handle);
                }
            } else {
                self.root_windows.retain(|&r| r != handle);
            }
            for child in win.children {
                self.destroy_window(child);
            }
        }
    }

    /// Pobiera lub tworzy biala teksture 1x1 do rysowania jednolitych paskow i linii
    pub fn get_or_create_white_texture(&mut self) -> Option<Arc<GpuTexture>> {
        if let Some(ref t) = self.white_dummy_tex {
            return Some(t.clone());
        }

        let wgpu_guard = crate::wgpu_engine_context::GLOBAL_WGPU_CONTEXT.get()?;
        let ctx_mutex = wgpu_guard.lock().ok()?;
        let ctx = ctx_mutex.as_ref()?;

        let white_pixel = [255u8, 255u8, 255u8, 255u8];
        if let Ok(tex) = ctx.create_texture_from_rgba(&white_pixel, 1, 1, Some("UI White Tex")) {
            let arc_tex = Arc::new(tex);
            self.white_dummy_tex = Some(arc_tex.clone());
            Some(arc_tex)
        } else {
            None
        }
    }
}

static WINDOW_MANAGER: OnceLock<Mutex<WindowManager>> = OnceLock::new();

pub fn get_window_manager() -> std::sync::MutexGuard<'static, WindowManager> {
    WINDOW_MANAGER
        .get_or_init(|| Mutex::new(WindowManager::new()))
        .lock()
        .unwrap()
}

/// Zbiera partie quadow do wyrenderowania z calego drzewa widocznych okien UI
pub fn collect_ui_render_batches(screen_w: u32, screen_h: u32) -> Vec<(Arc<GpuTexture>, Vec<Quad2D>)> {
    let mut wm = get_window_manager();
    let white_tex = wm.get_or_create_white_texture();

    // Upewnij sie, ze atlas czcionki jest gotowy
    {
        let mut font = get_global_font_atlas();
        let _ = font.init_atlas();
    }

    let sw = screen_w as i32;
    let sh = screen_h as i32;

    let mut texture_batches: Vec<(Arc<GpuTexture>, Vec<Quad2D>)> = Vec::new();

    fn add_quad(
        batches: &mut Vec<(Arc<GpuTexture>, Vec<Quad2D>)>,
        tex: Arc<GpuTexture>,
        quad: Quad2D,
    ) {
        if let Some((last_tex, quads)) = batches.last_mut() {
            if Arc::ptr_eq(last_tex, &tex) {
                quads.push(quad);
                return;
            }
        }
        batches.push((tex, vec![quad]));
    }

    // Rekurencyjna funkcja trawersowania drzewa okien
    fn visit_window(
        handle: u32,
        parent_rect: Option<[i32; 4]>,
        wm: &mut WindowManager,
        white_tex: &Option<Arc<GpuTexture>>,
        batches: &mut Vec<(Arc<GpuTexture>, Vec<Quad2D>)>,
        screen_w: i32,
        screen_h: i32,
    ) {
        let win = match wm.windows.get(&handle) {
            Some(w) if w.is_show => w.clone(),
            _ => return,
        };

        let rect = win.calc_global_rect(parent_rect, screen_w, screen_h);
        let [gx, gy, w, h] = rect;

        match win.window_type {
            WindowType::Bar | WindowType::Box => {
                if let Some(tex) = white_tex {
                    let mut q = Quad2D::new(gx as f32, gy as f32, w as f32, h as f32);
                    q.color = win.color;
                    add_quad(batches, tex.clone(), q);
                }
            }
            WindowType::Line => {
                if let Some(tex) = white_tex {
                    let line_w = if w > 0 { w as f32 } else { 1.0 };
                    let line_h = if h > 0 { h as f32 } else { 1.0 };
                    let mut q = Quad2D::new(gx as f32, gy as f32, line_w, line_h);
                    q.color = win.color;
                    add_quad(batches, tex.clone(), q);
                }
            }
            WindowType::ImageBox | WindowType::ExpandedImageBox => {
                if !win.image_path.is_empty() {
                    let mut tm = get_ui_texture_manager();
                    if let Some(img_res) = tm.get_image_resource(&win.image_path) {
                        let qw = if w > 0 { w as f32 } else { img_res.width as f32 };
                        let qh = if h > 0 { h as f32 } else { img_res.height as f32 };
                        let q = Quad2D::with_uv(
                            gx as f32,
                            gy as f32,
                            qw,
                            qh,
                            img_res.uv[0],
                            img_res.uv[1],
                            img_res.uv[2],
                            img_res.uv[3],
                        );
                        add_quad(batches, img_res.texture, q);
                    }
                }
            }
            WindowType::Button => {
                let visual_path = if win.is_pressed && !win.down_visual.is_empty() {
                    &win.down_visual
                } else if win.is_hover && !win.over_visual.is_empty() {
                    &win.over_visual
                } else if !win.up_visual.is_empty() {
                    &win.up_visual
                } else {
                    &win.image_path
                };

                if !visual_path.is_empty() {
                    let mut tm = get_ui_texture_manager();
                    if let Some(img_res) = tm.get_image_resource(visual_path) {
                        let qw = if w > 0 { w as f32 } else { img_res.width as f32 };
                        let qh = if h > 0 { h as f32 } else { img_res.height as f32 };
                        let q = Quad2D::with_uv(
                            gx as f32,
                            gy as f32,
                            qw,
                            qh,
                            img_res.uv[0],
                            img_res.uv[1],
                            img_res.uv[2],
                            img_res.uv[3],
                        );
                        add_quad(batches, img_res.texture, q);
                    }
                }

                // Tekst na przycisku
                if !win.text.is_empty() {
                    let font = get_global_font_atlas();
                    if let Some(ref font_tex) = font.texture {
                        let text_w = font.measure_text_width(&win.text);
                        let tx = gx as f32 + (w as f32 - text_w) / 2.0;
                        let ty = gy as f32 + (h as f32 - font.line_height) / 2.0;
                        let t_quads = font.build_text_quads(&win.text, tx, ty, [1.0, 1.0, 1.0, 1.0], 1.0);
                        for q in t_quads {
                            add_quad(batches, font_tex.clone(), q);
                        }
                    }
                }
            }
            WindowType::TextLine => {
                if !win.text.is_empty() {
                    let font = get_global_font_atlas();
                    if let Some(ref font_tex) = font.texture {
                        let text_w = font.measure_text_width(&win.text);
                        let tx = if win.text_align == crate::ui_window_core::HORIZONTAL_ALIGN_CENTER
                            || win.horizontal_align == crate::ui_window_core::HORIZONTAL_ALIGN_CENTER {
                            gx as f32 + (w as f32 - text_w) / 2.0
                        } else if win.text_align == crate::ui_window_core::HORIZONTAL_ALIGN_RIGHT
                            || win.horizontal_align == crate::ui_window_core::HORIZONTAL_ALIGN_RIGHT {
                            gx as f32 + w as f32 - text_w
                        } else {
                            gx as f32
                        };
                        let ty = if win.vertical_align == crate::ui_window_core::VERTICAL_ALIGN_CENTER {
                            gy as f32 + (h as f32 - font.line_height) / 2.0
                        } else {
                            gy as f32
                        };
                        let t_quads = font.build_text_quads(&win.text, tx, ty, win.text_color, 1.0);
                        for q in t_quads {
                            add_quad(batches, font_tex.clone(), q);
                        }
                    }
                }
            }
            _ => {}
        }

        // Trawersuj dzieci
        for child_handle in &win.children {
            visit_window(*child_handle, Some(rect), wm, white_tex, batches, screen_w, screen_h);
        }
    }

    let root_windows = wm.root_windows.clone();
    for root_h in root_windows {
        visit_window(root_h, None, &mut wm, &white_tex, &mut texture_batches, sw, sh);
    }

    texture_batches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_window_mgr_hierarchy_and_layout() {
        let mut wm = WindowManager::new();
        let root = wm.register(WindowType::Normal);
        wm.set_position(root, 100, 100);
        wm.set_size(root, 300, 200);

        let child = wm.register(WindowType::Bar);
        wm.set_parent(child, root);
        wm.set_position(child, 10, 20);
        wm.set_size(child, 50, 50);

        let win = wm.windows.get(&child).unwrap();
        let rect = win.calc_global_rect(Some([100, 100, 300, 200]), 1024, 768);
        assert_eq!(rect, [110, 120, 50, 50]);
    }
}
