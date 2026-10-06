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
    pub point_window: Option<u32>,
    pub capture_window: Option<u32>,
    pub active_window: Option<u32>,
    pub mouse_handler: usize,
    pub mouse_x: i32,
    pub mouse_y: i32,
    pub is_attaching: bool,
    pub attaching_type: u32,
    pub attaching_index: u32,
    pub attaching_slot_number: u32,
    pub attaching_icon_width: u8,
    pub attaching_icon_height: u8,
    pub white_dummy_tex: Option<Arc<GpuTexture>>,
    pub render_dirty: bool,
    pub cached_batches: Vec<(Arc<GpuTexture>, Vec<Quad2D>)>,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
            next_handle: 100,
            root_windows: Vec::new(),
            top_window: None,
            point_window: None,
            capture_window: None,
            active_window: None,
            mouse_handler: 0,
            mouse_x: 0,
            mouse_y: 0,
            is_attaching: false,
            attaching_type: 0,
            attaching_index: 0,
            attaching_slot_number: 0,
            attaching_icon_width: 0,
            attaching_icon_height: 0,
            white_dummy_tex: None,
            render_dirty: true,
            cached_batches: Vec::new(),
        }
    }

    pub fn register(&mut self, wtype: WindowType) -> u32 {
        self.register_with_handler(wtype, 0)
    }

    pub fn register_with_handler(&mut self, wtype: WindowType, py_handler: usize) -> u32 {
        let handle = self.next_handle;
        self.next_handle += 1;
        let mut win = Window::new(handle, wtype);
        win.py_handler = py_handler;
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
            self.render_dirty = true;
            true
        } else {
            false
        }
    }

    pub fn set_size(&mut self, handle: u32, w: i32, h: i32) -> bool {
        if let Some(win) = self.windows.get_mut(&handle) {
            win.width = w;
            win.height = h;
            self.render_dirty = true;
            true
        } else {
            false
        }
    }

    pub fn show(&mut self, handle: u32) -> bool {
        if let Some(win) = self.windows.get_mut(&handle) {
            win.is_show = true;
            self.render_dirty = true;
            true
        } else {
            false
        }
    }

    pub fn hide(&mut self, handle: u32) -> bool {
        if let Some(win) = self.windows.get_mut(&handle) {
            win.is_show = false;
            self.render_dirty = true;
            true
        } else {
            false
        }
    }

    pub fn is_show(&self, handle: u32) -> bool {
        self.windows.get(&handle).map(|w| w.is_show).unwrap_or(false)
    }

    pub fn load_image(&mut self, handle: u32, path: &str) -> bool {
        let (img_w, img_h) = {
            let mut tm = get_ui_texture_manager();
            if let Some(res) = tm.get_image_resource(path) {
                (res.width as i32, res.height as i32)
            } else {
                (0, 0)
            }
        };

        if let Some(win) = self.windows.get_mut(&handle) {
            win.image_path = path.to_string();
            if img_w > 0 && img_h > 0 {
                win.width = img_w;
                win.height = img_h;
            }
            self.render_dirty = true;
            true
        } else {
            false
        }
    }

    pub fn set_rendering_rect(&mut self, handle: u32, l: f32, t: f32, r: f32, b: f32) -> bool {
        if let Some(win) = self.windows.get_mut(&handle) {
            win.rendering_rect = [l, t, r, b];
            self.render_dirty = true;
            true
        } else {
            false
        }
    }

    pub fn set_clip_rect(&mut self, handle: u32, l: f32, t: f32, r: f32, b: f32) -> bool {
        if let Some(win) = self.windows.get_mut(&handle) {
            win.clip_rect = Some([l, t, r, b]);
            self.render_dirty = true;
            true
        } else {
            false
        }
    }

    pub fn set_slot_cool_time(&mut self, handle: u32, slot_id: usize, cool_time: f32, elapsed_time: f32) -> bool {
        if let Some(win) = self.windows.get_mut(&handle) {
            if let Some(ref mut sw) = win.slot_window {
                let res = sw.set_slot_cool_time(slot_id, cool_time, elapsed_time).is_ok();
                if res {
                    self.render_dirty = true;
                }
                return res;
            }
        }
        false
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
            self.render_dirty = true;
            true
        } else {
            false
        }
    }

    pub fn set_color(&mut self, handle: u32, color: [f32; 4]) -> bool {
        if let Some(win) = self.windows.get_mut(&handle) {
            win.color = color;
            self.render_dirty = true;
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
            self.render_dirty = true;
            true
        } else {
            false
        }
    }

    pub fn set_window_horizontal_align(&mut self, handle: u32, h_align: u8) -> bool {
        if let Some(win) = self.windows.get_mut(&handle) {
            win.horizontal_align = h_align;
            self.render_dirty = true;
            true
        } else {
            false
        }
    }

    pub fn set_window_vertical_align(&mut self, handle: u32, v_align: u8) -> bool {
        if let Some(win) = self.windows.get_mut(&handle) {
            win.vertical_align = v_align;
            self.render_dirty = true;
            true
        } else {
            false
        }
    }

    pub fn set_text_horizontal_align(&mut self, handle: u32, align: u8) -> bool {
        if let Some(win) = self.windows.get_mut(&handle) {
            win.text_align = align;
            self.render_dirty = true;
            true
        } else {
            false
        }
    }

    pub fn set_text_vertical_align(&mut self, handle: u32, align: u8) -> bool {
        if let Some(win) = self.windows.get_mut(&handle) {
            win.text_valign = align;
            self.render_dirty = true;
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
        self.mouse_x = mx;
        self.mouse_y = my;
        let hovered = self.pick_window(mx, my, screen_w, screen_h);
        for win in self.windows.values_mut() {
            let is_h = hovered == Some(win.handle);
            if win.is_hover != is_h {
                win.is_hover = is_h;
            }
        }

        // Dwukierunkowe wywolanie callbackow Pythona (OnMouseOverIn / OnMouseOverOut)
        if self.point_window != hovered {
            if let Some(old) = self.point_window {
                if let Some(win) = self.windows.get(&old) {
                    if win.py_handler != 0 {
                        crate::py_event_dispatcher::call_python_object_method0(win.py_handler, "OnMouseOverOut");
                    }
                }
            }
            if let Some(new_h) = hovered {
                if let Some(win) = self.windows.get(&new_h) {
                    if win.py_handler != 0 {
                        crate::py_event_dispatcher::call_python_object_method0(win.py_handler, "OnMouseOverIn");
                    }
                }
            }
            self.point_window = hovered;
        }

        if let Some(cap) = self.capture_window {
            if let Some(win) = self.windows.get(&cap) {
                if win.py_handler != 0 {
                    crate::py_event_dispatcher::call_python_object_method2_i32(win.py_handler, "OnMouseDrag", mx, my);
                }
            }
        }
    }

    pub fn handle_mouse_down(&mut self, mx: i32, my: i32, screen_w: i32, screen_h: i32) -> Option<u32> {
        let hit = self.pick_window(mx, my, screen_w, screen_h)?;
        self.capture_window = Some(hit);
        self.active_window = Some(hit);
        if let Some(win) = self.windows.get_mut(&hit) {
            win.is_pressed = true;
            if win.py_handler != 0 {
                crate::py_event_dispatcher::call_python_object_method0(win.py_handler, "OnMouseLeftButtonDown");
                match win.window_type {
                    WindowType::Button => {
                        crate::py_event_dispatcher::call_python_object_method0(win.py_handler, "DownEvent");
                    }
                    WindowType::RadioButton => {
                        crate::py_event_dispatcher::call_python_object_method0(win.py_handler, "CallEvent");
                    }
                    WindowType::ToggleButton => {
                        crate::py_event_dispatcher::call_python_object_method0(win.py_handler, "OnToggleDown");
                    }
                    _ => {}
                }
            }
        }
        Some(hit)
    }

    pub fn handle_mouse_up(&mut self, mx: i32, my: i32, screen_w: i32, screen_h: i32) -> Option<u32> {
        let hit = self.pick_window(mx, my, screen_w, screen_h);
        for win in self.windows.values_mut() {
            win.is_pressed = false;
        }
        let target = self.capture_window.or(hit);
        if let Some(t) = target {
            if let Some(win) = self.windows.get(&t) {
                if win.py_handler != 0 {
                    crate::py_event_dispatcher::call_python_object_method0(win.py_handler, "OnMouseLeftButtonUp");
                    if win.window_type == WindowType::Button {
                        crate::py_event_dispatcher::call_python_object_method0(win.py_handler, "CallEvent");
                    }
                }
            }
        }
        self.capture_window = None;
        target
    }

    pub fn handle_mouse_right_down(&mut self, mx: i32, my: i32, screen_w: i32, screen_h: i32) -> Option<u32> {
        let hit = self.pick_window(mx, my, screen_w, screen_h)?;
        if let Some(win) = self.windows.get(&hit) {
            if win.py_handler != 0 {
                crate::py_event_dispatcher::call_python_object_method0(win.py_handler, "OnMouseRightButtonDown");
            }
        }
        Some(hit)
    }

    pub fn handle_mouse_right_up(&mut self, mx: i32, my: i32, screen_w: i32, screen_h: i32) -> Option<u32> {
        let hit = self.pick_window(mx, my, screen_w, screen_h)?;
        if let Some(win) = self.windows.get(&hit) {
            if win.py_handler != 0 {
                crate::py_event_dispatcher::call_python_object_method0(win.py_handler, "OnMouseRightButtonUp");
            }
        }
        Some(hit)
    }

    pub fn handle_key_down(&mut self, vkey: u32) {
        if let Some(act) = self.active_window {
            if let Some(win) = self.windows.get(&act) {
                if win.py_handler != 0 {
                    crate::py_event_dispatcher::call_python_object_method1_i32(win.py_handler, "OnKeyDown", vkey as i32);
                }
            }
        }
    }

    pub fn handle_key_up(&mut self, vkey: u32) {
        if let Some(act) = self.active_window {
            if let Some(win) = self.windows.get(&act) {
                if win.py_handler != 0 {
                    crate::py_event_dispatcher::call_python_object_method1_i32(win.py_handler, "OnKeyUp", vkey as i32);
                }
            }
        }
    }

    pub fn handle_char(&mut self, c: char) {
        if let Some(act) = self.active_window {
            if let Some(win) = self.windows.get_mut(&act) {
                if c == '\x08' {
                    // Backspace
                    win.text.pop();
                    crate::py_event_dispatcher::dispatch_ime_backspace();
                    if win.py_handler != 0 {
                        crate::py_event_dispatcher::call_python_object_method0(win.py_handler, "OnIMEUpdate");
                    }
                } else if c == '\t' {
                    if win.py_handler != 0 {
                        crate::py_event_dispatcher::call_python_object_method0(win.py_handler, "OnIMETab");
                    }
                } else if c == '\r' || c == '\n' {
                    if win.py_handler != 0 {
                        crate::py_event_dispatcher::call_python_object_method0(win.py_handler, "OnIMEReturn");
                    }
                } else if c == '\x1b' {
                    if win.py_handler != 0 {
                        crate::py_event_dispatcher::call_python_object_method0(win.py_handler, "OnPressEscapeKey");
                    }
                } else if c >= ' ' {
                    win.text.push(c);
                    crate::py_event_dispatcher::dispatch_ime_char(c);
                    if win.py_handler != 0 {
                        crate::py_event_dispatcher::call_python_object_method0(win.py_handler, "OnIMEUpdate");
                    }
                }
            }
        }
    }

    pub fn handle_tab(&mut self) {
        if let Some(act) = self.active_window {
            if let Some(win) = self.windows.get(&act) {
                if win.py_handler != 0 {
                    crate::py_event_dispatcher::call_python_object_method0(win.py_handler, "OnIMETab");
                }
            }
        }
    }

    pub fn handle_return(&mut self) {
        if let Some(act) = self.active_window {
            if let Some(win) = self.windows.get(&act) {
                if win.py_handler != 0 {
                    crate::py_event_dispatcher::call_python_object_method0(win.py_handler, "OnIMEReturn");
                }
            }
        }
    }

    pub fn handle_escape(&mut self) {
        if let Some(act) = self.active_window {
            if let Some(win) = self.windows.get(&act) {
                if win.py_handler != 0 {
                    crate::py_event_dispatcher::call_python_object_method0(win.py_handler, "OnPressEscapeKey");
                }
            }
        }
    }

    pub fn collect_update_handlers(&self) -> Vec<usize> {
        self.windows.values()
            .filter(|w| w.is_show && w.py_handler != 0 && w.parent.is_none())
            .map(|w| w.py_handler)
            .collect()
    }

    pub fn update_all_windows(&mut self) {
        let handlers = self.collect_update_handlers();
        for h in handlers {
            crate::py_event_dispatcher::call_python_object_method0(h, "OnUpdate");
        }
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
    pub fn get_or_create_white_texture_with_ctx(&mut self, ctx: &crate::wgpu_engine_context::WgpuEngineContext) -> Option<Arc<GpuTexture>> {
        if let Some(ref t) = self.white_dummy_tex {
            return Some(t.clone());
        }

        let white_pixel = [255u8, 255u8, 255u8, 255u8];
        if let Ok(tex) = ctx.create_texture_from_rgba(&white_pixel, 1, 1, Some("UI White Tex")) {
            let arc_tex = Arc::new(tex);
            self.white_dummy_tex = Some(arc_tex.clone());
            Some(arc_tex)
        } else {
            None
        }
    }

    pub fn get_or_create_white_texture(&mut self) -> Option<Arc<GpuTexture>> {
        if let Some(ref t) = self.white_dummy_tex {
            return Some(t.clone());
        }

        let wgpu_guard = crate::wgpu_engine_context::GLOBAL_WGPU_CONTEXT.get()?;
        let ctx_mutex = wgpu_guard.try_lock().ok()?;
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
    
    // Return cached batches if nothing changed since last collect
    if !wm.render_dirty {
        return wm.cached_batches.clone();
    }
    wm.render_dirty = false;
    
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
                if win.color[3] > 0.001 {
                    if let Some(tex) = white_tex {
                        let mut q = Quad2D::new(gx as f32, gy as f32, w as f32, h as f32);
                        q.color = win.color;
                        add_quad(batches, tex.clone(), q);
                    }
                }
            }
            WindowType::Line => {
                if win.color[3] > 0.001 {
                    if let Some(tex) = white_tex {
                        let line_w = if w > 0 { w as f32 } else { 1.0 };
                        let line_h = if h > 0 { h as f32 } else { 1.0 };
                        let mut q = Quad2D::new(gx as f32, gy as f32, line_w, line_h);
                        q.color = win.color;
                        add_quad(batches, tex.clone(), q);
                    }
                }
            }
            WindowType::ImageBox => {
                if !win.image_path.is_empty() {
                    let mut tm = get_ui_texture_manager();
                    if let Some(img_res) = tm.get_image_resource(&win.image_path) {
                        let qw = if win.width > 0 { win.width as f32 } else { img_res.width as f32 };
                        let qh = if win.height > 0 { win.height as f32 } else { img_res.height as f32 };
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
            WindowType::ExpandedImageBox => {
                if !win.image_path.is_empty() {
                    let mut tm = get_ui_texture_manager();
                    if let Some(img_res) = tm.get_image_resource(&win.image_path) {
                        let sx = if win.scale[0] > 0.0 { win.scale[0] } else { 1.0 };
                        let sy = if win.scale[1] > 0.0 { win.scale[1] } else { 1.0 };
                        let base_w = if win.width > 0 { win.width as f32 } else { img_res.width as f32 };
                        let base_h = if win.height > 0 { win.height as f32 } else { img_res.height as f32 };

                        let r_left = base_w * win.rendering_rect[0];
                        let r_top = base_h * win.rendering_rect[1];
                        let r_right = base_w * win.rendering_rect[2];
                        let r_bottom = base_h * win.rendering_rect[3];

                        let qw = (base_w * sx + r_right - r_left).max(0.0);
                        let qh = (base_h * sy + r_bottom - r_top).max(0.0);
                        let qx = gx as f32 - r_left;
                        let qy = gy as f32 - r_top;

                        let u_span = img_res.uv[2] - img_res.uv[0];
                        let v_span = img_res.uv[3] - img_res.uv[1];
                        let u0 = img_res.uv[0] + u_span * (win.rendering_rect[0].max(0.0));
                        let u1 = img_res.uv[2] + u_span * (win.rendering_rect[2].min(0.0));
                        let v0 = img_res.uv[1] + v_span * (win.rendering_rect[1].max(0.0));
                        let v1 = img_res.uv[3] + v_span * (win.rendering_rect[3].min(0.0));

                        let q = Quad2D::with_uv(
                            qx,
                            qy,
                            qw,
                            qh,
                            u0,
                            v0,
                            u1,
                            v1,
                        );
                        add_quad(batches, img_res.texture, q);
                    }
                }
            }
            WindowType::Button | WindowType::RadioButton | WindowType::ToggleButton => {
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
                        // Obsluga wyrownania horyzontalnego:
                        // 2 = TEXT_HORIZONTAL_ALIGN_CENTER, 1 = TEXT_HORIZONTAL_ALIGN_RIGHT, 0 = LEFT
                        let tx = if win.text_align == 2 || win.text_align == crate::ui_window_core::HORIZONTAL_ALIGN_CENTER || win.horizontal_align == crate::ui_window_core::HORIZONTAL_ALIGN_CENTER {
                            if w > 0 {
                                gx as f32 + (w as f32 - text_w) / 2.0
                            } else {
                                gx as f32 - text_w / 2.0
                            }
                        } else if win.text_align == 1 || win.text_align == crate::ui_window_core::HORIZONTAL_ALIGN_RIGHT || win.horizontal_align == crate::ui_window_core::HORIZONTAL_ALIGN_RIGHT {
                            if w > 0 {
                                gx as f32 + w as f32 - text_w
                            } else {
                                gx as f32 - text_w
                            }
                        } else {
                            gx as f32
                        };

                        // Obsluga wyrownania wertykalnego:
                        // 1 = TEXT_VERTICAL_ALIGN_CENTER, 2 = TEXT_VERTICAL_ALIGN_BOTTOM, 0 = TOP
                        let ty = if win.text_valign == 1 || win.vertical_align == crate::ui_window_core::VERTICAL_ALIGN_CENTER {
                            if h > 0 {
                                gy as f32 + (h as f32 - font.line_height) / 2.0
                            } else {
                                gy as f32 - font.line_height / 2.0
                            }
                        } else if win.text_valign == 2 || win.vertical_align == crate::ui_window_core::VERTICAL_ALIGN_BOTTOM {
                            if h > 0 {
                                gy as f32 + h as f32 - font.line_height
                            } else {
                                gy as f32 - font.line_height
                            }
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
            WindowType::Bar3D => {
                if let Some(tex) = white_tex {
                    let fw = w as f32;
                    let fh = h as f32;
                    let fgx = gx as f32;
                    let fgy = gy as f32;

                    // Tlo srodkowe
                    let bg_color = if win.color == [1.0, 1.0, 1.0, 1.0] {
                        [0.18, 0.18, 0.18, 0.9]
                    } else {
                        win.color
                    };
                    let mut bg_quad = Quad2D::new(fgx + 1.0, fgy + 1.0, (fw - 2.0).max(1.0), (fh - 2.0).max(1.0));
                    bg_quad.color = bg_color;
                    add_quad(batches, tex.clone(), bg_quad);

                    // Jasne krawedzie (gora i lewo)
                    let light_col = [0.85, 0.85, 0.85, 1.0];
                    let mut top_q = Quad2D::new(fgx, fgy, fw, 1.0);
                    top_q.color = light_col;
                    add_quad(batches, tex.clone(), top_q);

                    let mut left_q = Quad2D::new(fgx, fgy, 1.0, fh);
                    left_q.color = light_col;
                    add_quad(batches, tex.clone(), left_q);

                    // Ciemne krawedzie (dol i prawo)
                    let dark_col = [0.08, 0.08, 0.08, 1.0];
                    let mut bot_q = Quad2D::new(fgx, fgy + fh - 1.0, fw, 1.0);
                    bot_q.color = dark_col;
                    add_quad(batches, tex.clone(), bot_q);

                    let mut right_q = Quad2D::new(fgx + fw - 1.0, fgy, 1.0, fh);
                    right_q.color = dark_col;
                    add_quad(batches, tex.clone(), right_q);
                }
            }
            WindowType::Board | WindowType::ThinBoard => {
                if let Some(tex) = white_tex {
                    let fw = w as f32;
                    let fh = h as f32;
                    let fgx = gx as f32;
                    let fgy = gy as f32;

                    let is_thin = win.window_type == WindowType::ThinBoard;

                    // Tlo panelu Metin2
                    let bg_color = if win.color == [1.0, 1.0, 1.0, 1.0] {
                        if is_thin { [0.03, 0.03, 0.04, 0.8] } else { [0.05, 0.05, 0.07, 0.9] }
                    } else {
                        win.color
                    };
                    let mut bg = Quad2D::new(fgx, fgy, fw, fh);
                    bg.color = bg_color;
                    add_quad(batches, tex.clone(), bg);

                    // Ramka panelu
                    let border_light = if is_thin {
                        [0.45, 0.45, 0.48, 0.95]
                    } else {
                        [0.55, 0.50, 0.40, 0.95] // Metin2 Board - odcien zlocisto-stalowy
                    };
                    let border_dark = [0.12, 0.12, 0.15, 1.0];

                    // Gora i lewo (jasne / podswietlenie)
                    let mut t_q = Quad2D::new(fgx, fgy, fw, 1.0);
                    t_q.color = border_light;
                    add_quad(batches, tex.clone(), t_q);

                    let mut l_q = Quad2D::new(fgx, fgy, 1.0, fh);
                    l_q.color = border_light;
                    add_quad(batches, tex.clone(), l_q);

                    // Dol i prawo (ciemne / cien)
                    let mut b_q = Quad2D::new(fgx, fgy + fh - 1.0, fw, 1.0);
                    b_q.color = border_dark;
                    add_quad(batches, tex.clone(), b_q);

                    let mut r_q = Quad2D::new(fgx + fw - 1.0, fgy, 1.0, fh);
                    r_q.color = border_dark;
                    add_quad(batches, tex.clone(), r_q);

                    if !is_thin && fw > 4.0 && fh > 4.0 {
                        // Wewnetrzna ramka ozdobna dla Board
                        let inner_border = [0.25, 0.22, 0.18, 0.85];
                        let mut in_t = Quad2D::new(fgx + 2.0, fgy + 2.0, fw - 4.0, 1.0);
                        in_t.color = inner_border;
                        add_quad(batches, tex.clone(), in_t);

                        let mut in_l = Quad2D::new(fgx + 2.0, fgy + 2.0, 1.0, fh - 4.0);
                        in_l.color = inner_border;
                        add_quad(batches, tex.clone(), in_l);

                        let mut in_b = Quad2D::new(fgx + 2.0, fgy + fh - 3.0, fw - 4.0, 1.0);
                        in_b.color = inner_border;
                        add_quad(batches, tex.clone(), in_b);

                        let mut in_r = Quad2D::new(fgx + fw - 3.0, fgy + 2.0, 1.0, fh - 4.0);
                        in_r.color = inner_border;
                        add_quad(batches, tex.clone(), in_r);
                    }
                }
            }
            WindowType::EditLine => {
                let fw = w as f32;
                let fh = h as f32;
                let fgx = gx as f32;
                let fgy = gy as f32;

                // Tlo pola tekstowego i obramowanie
                if let Some(tex) = white_tex {
                    let bg_color = if win.color == [1.0, 1.0, 1.0, 1.0] {
                        [0.05, 0.05, 0.05, 0.8]
                    } else {
                        win.color
                    };
                    let mut bg = Quad2D::new(fgx, fgy, fw, fh);
                    bg.color = bg_color;
                    add_quad(batches, tex.clone(), bg);

                    // Wklesla ramka editline (ciemna u gory/po lewej, jasniejsza na dole/po prawej)
                    let edge_dark = [0.1, 0.1, 0.12, 1.0];
                    let edge_light = [0.35, 0.35, 0.38, 1.0];

                    let mut t_q = Quad2D::new(fgx, fgy, fw, 1.0);
                    t_q.color = edge_dark;
                    add_quad(batches, tex.clone(), t_q);

                    let mut l_q = Quad2D::new(fgx, fgy, 1.0, fh);
                    l_q.color = edge_dark;
                    add_quad(batches, tex.clone(), l_q);

                    let mut b_q = Quad2D::new(fgx, fgy + fh - 1.0, fw, 1.0);
                    b_q.color = edge_light;
                    add_quad(batches, tex.clone(), b_q);

                    let mut r_q = Quad2D::new(fgx + fw - 1.0, fgy, 1.0, fh);
                    r_q.color = edge_light;
                    add_quad(batches, tex.clone(), r_q);
                }

                // Renderowanie tekstu wpisanego w pole
                let font = get_global_font_atlas();
                let text_w = font.measure_text_width(&win.text);
                let tx = fgx + 4.0;
                let ty = fgy + (fh - font.line_height).max(0.0) / 2.0;

                if !win.text.is_empty() {
                    if let Some(ref font_tex) = font.texture {
                        let t_quads = font.build_text_quads(&win.text, tx, ty, win.text_color, 1.0);
                        for q in t_quads {
                            add_quad(batches, font_tex.clone(), q);
                        }
                    }
                }

                // Migajacy kursor
                let now_ms = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_millis())
                    .unwrap_or(0);
                let cursor_visible = (now_ms / 500) % 2 == 0;

                if cursor_visible {
                    if let Some(tex) = white_tex {
                        let cur_x = tx + text_w;
                        let cur_y = ty;
                        let cur_h = font.line_height.min(fh.max(12.0));
                        let mut cursor_q = Quad2D::new(cur_x, cur_y, 1.5, cur_h);
                        cursor_q.color = win.text_color;
                        add_quad(batches, tex.clone(), cursor_q);
                    }
                }
            }
            WindowType::SlotWindow => {
                if let Some(ref sw) = win.slot_window {
                    let fgx = gx as f32;
                    let fgy = gy as f32;

                    for cell in &sw.cells {
                        let cx = fgx + cell.x as f32;
                        let cy = fgy + cell.y as f32;
                        let cw = cell.w as f32;
                        let ch = cell.h as f32;

                        // 1. Tlo slotu
                        if let Some(tex) = white_tex {
                            let mut cell_bg = Quad2D::new(cx, cy, cw, ch);
                            cell_bg.color = [0.08, 0.08, 0.1, 0.75];
                            add_quad(batches, tex.clone(), cell_bg);

                            // Ramka slotu
                            let border_col = if sw.selected_slot == Some(cell.id) {
                                [0.85, 0.75, 0.3, 0.9] // Highlight zloty przy hoverze
                            } else {
                                [0.25, 0.25, 0.3, 0.8] // Domyslna ramka
                            };
                            let mut t_q = Quad2D::new(cx, cy, cw, 1.0);
                            t_q.color = border_col;
                            add_quad(batches, tex.clone(), t_q);

                            let mut l_q = Quad2D::new(cx, cy, 1.0, ch);
                            l_q.color = border_col;
                            add_quad(batches, tex.clone(), l_q);

                            let mut b_q = Quad2D::new(cx, cy + ch - 1.0, cw, 1.0);
                            b_q.color = border_col;
                            add_quad(batches, tex.clone(), b_q);

                            let mut r_q = Quad2D::new(cx + cw - 1.0, cy, 1.0, ch);
                            r_q.color = border_col;
                            add_quad(batches, tex.clone(), r_q);
                        }

                        // 2. Przedmiot w slocie
                        if let Some(ref item) = cell.item {
                            if !item.icon_path.is_empty() {
                                let mut tm = get_ui_texture_manager();
                                if let Some(img_res) = tm.get_image_resource(&item.icon_path) {
                                    let mut q = Quad2D::with_uv(
                                        cx + 1.0, cy + 1.0, cw - 2.0, ch - 2.0,
                                        img_res.uv[0], img_res.uv[1], img_res.uv[2], img_res.uv[3],
                                    );
                                    q.color = item.color;
                                    add_quad(batches, img_res.texture, q);
                                }
                            }

                            // 3. Efekt chlodzenia (CoolTime fan / pie-slice overlay)
                            if item.cool_time > 0.0 && item.elapsed_time < item.cool_time {
                                let ratio = (1.0 - item.elapsed_time / item.cool_time).clamp(0.0, 1.0);
                                if let Some(tex) = white_tex {
                                    let cool_h = (ch - 2.0) * ratio;
                                    let mut cool_q = Quad2D::new(cx + 1.0, cy + ch - 1.0 - cool_h, cw - 2.0, cool_h);
                                    cool_q.color = [0.0, 0.0, 0.0, 0.65]; // Ciemna polprzezroczysta nakladka
                                    add_quad(batches, tex.clone(), cool_q);
                                }
                            }

                            // 4. Licznik ilosci (stack count)
                            if item.count > 1 {
                                let count_str = item.count.to_string();
                                let font = get_global_font_atlas();
                                if let Some(ref font_tex) = font.texture {
                                    let cw_text = font.measure_text_width(&count_str);
                                    let tx = cx + cw - cw_text - 2.0;
                                    let ty = cy + ch - font.line_height - 2.0;
                                    let t_quads = font.build_text_quads(&count_str, tx, ty, [1.0, 1.0, 0.7, 1.0], 0.9);
                                    for q in t_quads {
                                        add_quad(batches, font_tex.clone(), q);
                                    }
                                }
                            }
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

    wm.cached_batches = texture_batches.clone();
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

    #[test]
    fn test_ui_window_mgr_new_window_types() {
        let mut wm = WindowManager::new();
        let bar3d = wm.register(WindowType::Bar3D);
        let board = wm.register(WindowType::Board);
        let thin_board = wm.register(WindowType::ThinBoard);
        let edit_line = wm.register(WindowType::EditLine);

        assert_eq!(wm.windows.get(&bar3d).unwrap().window_type, WindowType::Bar3D);
        assert_eq!(wm.windows.get(&board).unwrap().window_type, WindowType::Board);
        assert_eq!(wm.windows.get(&thin_board).unwrap().window_type, WindowType::ThinBoard);
        assert_eq!(wm.windows.get(&edit_line).unwrap().window_type, WindowType::EditLine);
    }

    #[test]
    fn test_ui_window_mgr_slot_cool_time_and_clip() {
        let mut wm = WindowManager::new();
        let slot_win = wm.register(WindowType::SlotWindow);
        wm.set_clip_rect(slot_win, 50.0, 50.0, 200.0, 200.0);

        let win = wm.windows.get(&slot_win).unwrap();
        assert_eq!(win.clip_rect, Some([50.0, 50.0, 200.0, 200.0]));

        // Dodanie slotu i ustawienie cooldownu
        let mut sw = crate::ui_widget_slotwindow::SlotWindow::new(1, 1, (32, 32));
        let _ = sw.set_slot(0, 10, 1, [1.0, 1.0, 1.0, 1.0]);
        let res = sw.set_slot_cool_time(0, 15.0, 5.0);
        assert!(res.is_ok());

        let cell = &sw.cells[0];
        let item = cell.item.as_ref().unwrap();
        assert_eq!(item.cool_time, 15.0);
        assert_eq!(item.elapsed_time, 5.0);
    }
}

// ============================================================================
// FFI C-Bridge dla WindowManagera
// ============================================================================

#[no_mangle]
pub extern "C" fn m2_wndmgr_set_clip_rect(handle: u32, l: f32, t: f32, r: f32, b: f32) -> bool {
    get_window_manager().set_clip_rect(handle, l, t, r, b)
}

#[no_mangle]
pub extern "C" fn m2_wndmgr_set_slot_cool_time(handle: u32, slot_id: usize, cool_time: f32, elapsed_time: f32) -> bool {
    get_window_manager().set_slot_cool_time(handle, slot_id, cool_time, elapsed_time)
}

#[no_mangle]
pub extern "C" fn m2_wndmgr_enable_scissor_rect(handle: u32) -> bool {
    let mut wm = get_window_manager();
    if let Some(win) = wm.windows.get_mut(&handle) {
        win.scissor_enabled = true;
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn m2_wndmgr_disable_scissor_rect(handle: u32) -> bool {
    let mut wm = get_window_manager();
    if let Some(win) = wm.windows.get_mut(&handle) {
        win.scissor_enabled = false;
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn m2_wndmgr_is_scissor_rect_enabled(handle: u32) -> bool {
    let wm = get_window_manager();
    if let Some(win) = wm.windows.get(&handle) {
        win.scissor_enabled
    } else {
        false
    }
}


