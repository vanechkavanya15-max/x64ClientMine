//! Modul `ui_window_mgr` — Menedzer drzewa okien i zdarzen interfejsu UI::CWindowManager
//! Wzorzec referencyjny C++: `source/EterPythonLib/PythonWindowManager.cpp`
//! Opis: Odwzorowanie UI::CWindowManager: zarzadzanie warstwami okien (GAME, UI, TOP_MOST), okno w fokusie, przeciaganie okien mysza, przechwytywanie klikniec.

use std::collections::{HashMap, };

pub type WindowId = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowType {
    Normal = 0,
    Slot,
    GridSlot,
    TextLine,
    MarkBox,
    ImageBox,
    ExpImageBox,
    AniImageBox,
    Button,
    RadioButton,
    ToggleButton,
    DragButton,
    Box,
    Bar,
    Line,
    Bar3d,
    NumLine,
    Layer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowFlag {
    Movable = 1 << 0,
    Limit = 1 << 1,
    Dragable = 1 << 2,
    Attach = 1 << 3,
    RestrictX = 1 << 4,
    RestrictY = 1 << 5,
    NotCapture = 1 << 6,
}

#[derive(Debug, Clone)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[derive(Debug, Clone)]
pub struct WindowNode {
    pub id: WindowId,
    pub name: String,
    pub wnd_type: WindowType,
    pub flags: u32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub limit_bias: Rect,
    pub is_show: bool,
    pub is_rendering: bool,
    pub parent_id: Option<WindowId>,
    pub children: Vec<WindowId>,
}

impl WindowNode {
    pub fn new(id: WindowId, wnd_type: WindowType, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            wnd_type,
            flags: 0,
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            limit_bias: Rect { left: 0, top: 0, right: 0, bottom: 0 },
            is_show: false,
            is_rendering: false,
            parent_id: None,
            children: Vec::new(),
        }
    }

    pub fn has_flag(&self, flag: WindowFlag) -> bool {
        (self.flags & (flag as u32)) != 0
    }

    pub fn set_flag(&mut self, flag: WindowFlag, value: bool) {
        if value {
            self.flags |= flag as u32;
        } else {
            self.flags &= !(flag as u32);
        }
    }

    pub fn is_in(&self, mx: i32, my: i32) -> bool {
        mx >= self.x && mx <= self.x + self.width && my >= self.y && my <= self.y + self.height
    }

    pub fn global_x(&self, mgr: &UiWindowMgr) -> i32 {
        let mut x = self.x;
        let mut curr_parent = self.parent_id;
        while let Some(pid) = curr_parent {
            if let Some(parent) = mgr.windows.get(&pid) {
                x += parent.x;
                curr_parent = parent.parent_id;
            } else {
                break;
            }
        }
        x
    }

    pub fn global_y(&self, mgr: &UiWindowMgr) -> i32 {
        let mut y = self.y;
        let mut curr_parent = self.parent_id;
        while let Some(pid) = curr_parent {
            if let Some(parent) = mgr.windows.get(&pid) {
                y += parent.y;
                curr_parent = parent.parent_id;
            } else {
                break;
            }
        }
        y
    }
}

#[derive(Debug, Clone, Default)]
pub struct UiWindowMgr {
    pub initialized: bool,
    pub windows: HashMap<WindowId, WindowNode>,
    pub next_id: WindowId,

    pub root_window: Option<WindowId>,
    pub layer_window_map: HashMap<String, WindowId>,
    pub layer_window_list: Vec<WindowId>,

    pub screen_width: i32,
    pub screen_height: i32,
    pub vres: i32,
    pub hres: i32,

    pub mouse_x: i32,
    pub mouse_y: i32,
    pub drag_x: i32,
    pub drag_y: i32,
    pub picked_x: i32,
    pub picked_y: i32,

    pub once_ignore_mouse_left_button_up_event_flag: bool,
    pub ignore_end_time: u32,

    pub attaching_flag: bool,
    pub attaching_type: u32,
    pub attaching_index: u32,
    pub attaching_slot_number: u32,
    pub attaching_icon_width: u8,
    pub attaching_icon_height: u8,

    pub active_window: Option<WindowId>,
    pub active_window_list: Vec<WindowId>,
    pub lock_window: Option<WindowId>,
    pub lock_window_list: Vec<WindowId>,
    pub point_window: Option<WindowId>,
    pub left_capture_window: Option<WindowId>,
    pub right_capture_window: Option<WindowId>,
    pub middle_capture_window: Option<WindowId>,
    pub key_capture_window_map: HashMap<i32, WindowId>,
    pub reserve_delete_window_list: Vec<WindowId>,
    pub pick_always_window_list: Vec<WindowId>,

    pub aspect: f32,
}

impl UiWindowMgr {
    pub fn new() -> Self {
        let mut mgr = Self {
            initialized: true,
            windows: HashMap::new(),
            next_id: 1,
            root_window: None,
            layer_window_map: HashMap::new(),
            layer_window_list: Vec::new(),
            screen_width: 0,
            screen_height: 0,
            vres: 0,
            hres: 0,
            mouse_x: 0,
            mouse_y: 0,
            drag_x: 0,
            drag_y: 0,
            picked_x: 0,
            picked_y: 0,
            once_ignore_mouse_left_button_up_event_flag: false,
            ignore_end_time: 0,
            attaching_flag: false,
            attaching_type: 0,
            attaching_index: 0,
            attaching_slot_number: 0,
            attaching_icon_width: 0,
            attaching_icon_height: 0,
            active_window: None,
            active_window_list: Vec::new(),
            lock_window: None,
            lock_window_list: Vec::new(),
            point_window: None,
            left_capture_window: None,
            right_capture_window: None,
            middle_capture_window: None,
            key_capture_window_map: HashMap::new(),
            reserve_delete_window_list: Vec::new(),
            pick_always_window_list: Vec::new(),
            aspect: 0.0,
        };

        // Create root window
        let mut root = WindowNode::new(mgr.next_id, WindowType::Normal, "root");
        root.is_show = true;
        let root_id = mgr.next_id;
        mgr.windows.insert(root_id, root);
        mgr.root_window = Some(root_id);
        mgr.next_id += 1;

        // Create layers
        let layers = vec!["GAME", "UI_BOTTOM", "UI", "TOP_MOST", "CURTAIN"];
        for layer_name in layers {
            let mut layer = WindowNode::new(mgr.next_id, WindowType::Layer, layer_name);
            layer.is_show = true;
            layer.parent_id = Some(root_id);
            let layer_id = mgr.next_id;
            mgr.windows.insert(layer_id, layer.clone());
            mgr.layer_window_map.insert(layer_name.to_string(), layer_id);
            mgr.layer_window_list.push(layer_id);
            
            if let Some(root_node) = mgr.windows.get_mut(&root_id) {
                root_node.children.push(layer_id);
            }
            
            mgr.next_id += 1;
        }

        mgr
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn get_aspect(&self) -> f32 {
        self.aspect
    }

    pub fn set_screen_size(&mut self, width: i32, height: i32) {
        self.screen_width = width;
        self.screen_height = height;
    }

    pub fn set_resolution(&mut self, hres: i32, vres: i32) {
        self.hres = hres;
        self.vres = vres;
        if vres != 0 {
            self.aspect = hres as f32 / vres as f32;
        }
    }

    pub fn get_resolution(&self) -> (i32, i32) {
        (self.hres, self.vres)
    }

    pub fn set_mouse_position(&mut self, x: i32, y: i32) {
        if self.hres == 0 {
            return;
        }
        
        let mut nx = x;
        let mut ny = y;
        
        if nx < 0 { nx = 0; }
        if nx >= self.screen_width { nx = self.screen_width; }
        if ny < 0 { ny = 0; }
        if ny >= self.screen_height { ny = self.screen_height; }
        
        self.mouse_x = nx;
        self.mouse_y = ny;
    }

    pub fn get_mouse_position(&self) -> (i32, i32) {
        (self.mouse_x, self.mouse_y)
    }

    pub fn is_dragging(&self) -> bool {
        self.left_capture_window.is_some()
    }

    pub fn get_lock_window(&self) -> Option<WindowId> {
        self.lock_window
    }

    pub fn get_point_window(&self) -> Option<WindowId> {
        self.point_window
    }

    pub fn is_focus(&self) -> bool {
        self.active_window.is_some() || self.lock_window.is_some()
    }

    pub fn is_focus_window(&self, window_id: WindowId) -> bool {
        self.active_window == Some(window_id)
    }

    pub fn register_window(&mut self, layer: &str) -> Option<WindowId> {
        let layer_id = *self.layer_window_map.get(layer)?;
        
        let wnd_id = self.next_id;
        self.next_id += 1;
        
        let mut wnd = WindowNode::new(wnd_id, WindowType::Normal, "");
        wnd.parent_id = Some(layer_id);
        
        if let Some(layer_wnd) = self.windows.get_mut(&layer_id) {
            layer_wnd.children.push(wnd_id);
        }
        
        self.windows.insert(wnd_id, wnd);
        Some(wnd_id)
    }

    pub fn destroy_window(&mut self, wnd_id: WindowId) {
        self.reserve_delete_window_list.push(wnd_id);
    }

    fn pick_window(&self, x: i32, y: i32) -> Option<WindowId> {
        if let Some(lock_id) = self.lock_window {
            return self.pick_window_recursive(lock_id, x, y);
        }

        for &pick_always_id in &self.pick_always_window_list {
            if let Some(wnd) = self.windows.get(&pick_always_id) {
                if wnd.is_rendering && wnd.is_in(x, y) {
                    return Some(pick_always_id);
                }
            }
        }

        for &layer_id in self.layer_window_list.iter().rev() {
            if let Some(picked) = self.pick_window_recursive(layer_id, x, y) {
                if picked != layer_id {
                    return Some(picked);
                }
            }
        }
        None
    }

    fn pick_window_recursive(&self, node_id: WindowId, x: i32, y: i32) -> Option<WindowId> {
        let node = self.windows.get(&node_id)?;
        if !node.is_show {
            return None;
        }

        for &child_id in node.children.iter().rev() {
            if let Some(picked) = self.pick_window_recursive(child_id, x, y) {
                return Some(picked);
            }
        }

        if node.is_in(x, y) {
            Some(node_id)
        } else {
            None
        }
    }

    pub fn run_mouse_move(&mut self, x: i32, y: i32) {


        self.set_mouse_position(x, y);
        let point_window = self.pick_window(self.mouse_x, self.mouse_y);

        if let Some(left_cap) = self.left_capture_window {
            if let Some(mut win) = self.windows.get(&left_cap).cloned() {
                if win.has_flag(WindowFlag::Movable) {
                    let mut dx = self.mouse_x - self.drag_x;
                    let mut dy = self.mouse_y - self.drag_y;
                    
                    if let Some(parent_id) = win.parent_id {
                        if let Some(parent) = self.windows.get(&parent_id) {
                            let (px, py) = (parent.global_x(self), parent.global_y(self));
                            dx -= px;
                            dy -= py;
                        }
                    }

                    if win.has_flag(WindowFlag::RestrictX) {
                        dx = win.x;
                    }
                    if win.has_flag(WindowFlag::RestrictY) {
                        dy = win.y;
                    }

                    if win.has_flag(WindowFlag::Limit) {
                        let limit = &win.limit_bias;
                        let limit_right = self.screen_width - limit.right;
                        let limit_bottom = self.screen_height - limit.bottom;

                        if dx < limit.left { dx = limit.left; }
                        else if dx + win.width >= limit_right { dx = limit_right - win.width; }
                        
                        if dy < limit.top { dy = limit.top; }
                        else if dy + win.height >= limit_bottom { dy = limit_bottom - win.height; }
                    }

                    win.x = dx;
                    win.y = dy;
                    self.windows.insert(left_cap, win);
                } else if win.has_flag(WindowFlag::Dragable) {
                    let _dx = self.mouse_x - self.drag_x;
                    let _dy = self.mouse_y - self.drag_y;
                }
            }
        }

        if self.point_window != point_window {
            // handle mouse over out / in
            self.point_window = point_window;
        }
    }

    pub fn run_mouse_left_button_down(&mut self, x: i32, y: i32) {
        self.set_top_ui_window();
        self.set_mouse_position(x, y);
        
        let p_win_id_opt = self.get_point_window();
        if let Some(p_win_id) = p_win_id_opt {
            if let Some(p_win) = self.windows.get(&p_win_id) {
                let mut target_win_id = p_win_id;
                
                if p_win.has_flag(WindowFlag::Attach) {
                    if let Some(root) = self.get_root(p_win_id) {
                        target_win_id = root;
                    }
                }
                
                if let Some(t_win) = self.windows.get(&target_win_id) {
                    if !t_win.has_flag(WindowFlag::NotCapture) {
                        self.left_capture_window = Some(target_win_id);
                    }
                    let gx = t_win.global_x(self);
                    let gy = t_win.global_y(self);
                    self.drag_x = self.mouse_x - gx;
                    self.drag_y = self.mouse_y - gy;
                    self.picked_x = self.mouse_x;
                    self.picked_y = self.mouse_y;
                }
            }
        }
    }
    
    pub fn run_mouse_left_button_up(&mut self, x: i32, y: i32) {
        if self.once_ignore_mouse_left_button_up_event_flag {
            self.once_ignore_mouse_left_button_up_event_flag = false;
            
            // Dummy time check, since timeGetTime() logic is external
            // We just let it pass or return for now to reflect the C++ structure
            // if time_get_time() < self.ignore_end_time { return; }
        }

        self.set_mouse_position(x, y);

        if self.left_capture_window.is_some() {
            // trigger mouse left button up
            self.left_capture_window = None;
            return;
        }

        self.left_capture_window = None;
    }

    pub fn set_top_ui_window(&mut self) {
        if let Some(ui_layer_id) = self.layer_window_map.get("UI") {
            let layer_id = *ui_layer_id;
            if let Some(pos) = self.layer_window_list.iter().position(|&id| id == layer_id) {
                // If it's not already at the top (end of the list)
                if pos != self.layer_window_list.len() - 1 {
                    let removed = self.layer_window_list.remove(pos);
                    self.layer_window_list.push(removed);
                }
            }
        }
    }

    pub fn get_root(&self, mut id: WindowId) -> Option<WindowId> {
        while let Some(w) = self.windows.get(&id) {
            if let Some(pid) = w.parent_id {
                if pid == self.root_window.unwrap() {
                    return Some(id);
                }
                id = pid;
            } else {
                return Some(id);
            }
        }
        None
    }
    
    // Add missing parts
    pub fn lock_window(&mut self, id: WindowId) {
        self.lock_window = Some(id);
    }
    
    pub fn unlock_window(&mut self) {
        self.lock_window = None;
    }
    
    pub fn activate_window(&mut self, id: WindowId) {
        self.active_window = Some(id);
    }
    
    pub fn deactivate_window(&mut self) {
        self.active_window = None;
    }
    
    pub fn get_activate_window(&self) -> Option<WindowId> {
        self.active_window
    }
}
