use std::collections::{HashMap, VecDeque};
use std::sync::{Mutex, OnceLock};

pub const WT_NORMAL: u32 = 0;
pub const WT_SLOT: u32 = 1;
pub const WT_GRIDSLOT: u32 = 2;
pub const WT_TEXTLINE: u32 = 3;
pub const WT_MARKBOX: u32 = 4;
pub const WT_IMAGEBOX: u32 = 5;
pub const WT_EXP_IMAGEBOX: u32 = 6;
pub const WT_ANI_IMAGEBOX: u32 = 7;
pub const WT_BUTTON: u32 = 8;
pub const WT_RATIOBUTTON: u32 = 9;
pub const WT_TOGGLEBUTTON: u32 = 10;
pub const WT_DRAGBUTTON: u32 = 11;
pub const WT_BOX: u32 = 12;
pub const WT_BAR: u32 = 13;
pub const WT_LINE: u32 = 14;
pub const WT_BAR3D: u32 = 15;
pub const WT_NUMLINE: u32 = 16;

#[allow(dead_code)]
pub const FLAG_MOVABLE: u32 = 1 << 0;
#[allow(dead_code)]
pub const FLAG_LIMIT: u32 = 1 << 1;
#[allow(dead_code)]
pub const FLAG_SNAP: u32 = 1 << 2;
#[allow(dead_code)]
pub const FLAG_DRAGABLE: u32 = 1 << 3;
#[allow(dead_code)]
pub const FLAG_ATTACH: u32 = 1 << 4;
#[allow(dead_code)]
pub const FLAG_RESTRICT_X: u32 = 1 << 5;
#[allow(dead_code)]
pub const FLAG_RESTRICT_Y: u32 = 1 << 6;
#[allow(dead_code)]
pub const FLAG_NOT_CAPTURE: u32 = 1 << 7;
pub const FLAG_FLOAT: u32 = 1 << 8;
#[allow(dead_code)]
pub const FLAG_NOT_PICK: u32 = 1 << 9;
#[allow(dead_code)]
pub const FLAG_IGNORE_SIZE: u32 = 1 << 10;
#[allow(dead_code)]
pub const FLAG_RTL: u32 = 1 << 11;

#[derive(Debug, Clone, Default)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[derive(Debug, Clone)]
pub struct CWindow {
    pub id: u64,
    pub parent_id: Option<u64>,
    pub child_ids: Vec<u64>,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub rect: Rect,
    pub is_show: bool,
    pub flags: u32,
    pub window_type: u32,
    pub layer: String,
}

impl CWindow {
    pub fn new(id: u64, window_type: u32, layer: String) -> Self {
        Self {
            id,
            parent_id: None,
            child_ids: Vec::new(),
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            rect: Rect::default(),
            is_show: false,
            flags: 0,
            window_type,
            layer,
        }
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
        self.update_rect();
    }

    pub fn set_size(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
        self.update_rect();
    }

    pub fn update_rect(&mut self) {
        // Basic update. The original has alignment logic which requires parent dimensions,
        // but since we only manage local coords here, we will just do standard calculation for now.
        self.rect.left = self.x;
        self.rect.top = self.y;
        self.rect.right = self.x + self.width;
        self.rect.bottom = self.y + self.height;
    }

    pub fn show(&mut self) {
        self.is_show = true;
    }

    pub fn hide(&mut self) {
        self.is_show = false;
    }
}

pub struct CWindowManager {
    pub windows: HashMap<u64, CWindow>,
    pub active_window_list: VecDeque<u64>,
    pub active_window: Option<u64>,
    pub next_id: u64,
}

impl CWindowManager {
    fn new() -> Self {
        Self {
            windows: HashMap::new(),
            active_window_list: VecDeque::new(),
            active_window: None,
            next_id: 1, // Start IDs from 1 so 0 can be considered invalid/null if needed
        }
    }

    pub fn register_window(&mut self, window_type: u32, layer: &str) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let mut window = CWindow::new(id, window_type, layer.to_string());
        // Default CWindow::Show is false, but CWindow constructor doesn't set it to false explicitly in C++ ?
        // Usually, the Python layer calls Show.
        window.update_rect();
        self.windows.insert(id, window);
        id
    }

    pub fn destroy_window(&mut self, id: u64) {
        self.windows.remove(&id);
        self.active_window_list.retain(|&x| x != id);
        if self.active_window == Some(id) {
            self.deactivate_window();
        }
    }

    pub fn set_top(&mut self, id: u64) {
        let parent_id = {
            if let Some(window) = self.windows.get(&id) {
                if window.flags & FLAG_FLOAT == 0 {
                    return; // original checks FLAG_FLOAT, though original PythonWindowManager::SetTop delegates to pParentWindow->SetTop(pWin). 
                    // Let's implement CWindow::SetTop logic here
                }
                window.parent_id
            } else {
                return;
            }
        };

        if let Some(p_id) = parent_id {
            if let Some(parent) = self.windows.get_mut(&p_id) {
                if let Some(pos) = parent.child_ids.iter().position(|&x| x == id) {
                    parent.child_ids.remove(pos);
                    parent.child_ids.push(id);
                }
            }
        }
    }

    pub fn activate_window(&mut self, id: u64) {
        if self.active_window == Some(id) {
            return;
        }

        self.active_window_list.retain(|&x| x != id);

        if let Some(active_id) = self.active_window {
            self.active_window_list.push_back(active_id);
            // OnKillFocus
        }

        self.active_window = Some(id);
        // OnSetFocus
    }

    pub fn deactivate_window_specific(&mut self, id: u64) {
        if self.active_window == Some(id) {
            self.deactivate_window();
        }
    }

    pub fn deactivate_window(&mut self) {
        if let Some(_active_id) = self.active_window {
            // OnKillFocus
            if self.active_window_list.is_empty() {
                self.active_window = None;
            } else {
                if let Some(back_id) = self.active_window_list.pop_back() {
                    self.active_window = Some(back_id);
                    // OnSetFocus
                } else {
                    self.active_window = None;
                }
            }
        }
    }
}

static WND_MGR: OnceLock<Mutex<CWindowManager>> = OnceLock::new();

pub fn get_wnd_mgr() -> &'static Mutex<CWindowManager> {
    WND_MGR.get_or_init(|| Mutex::new(CWindowManager::new()))
}

// C-FFI API
#[no_mangle]
pub extern "C" fn m2_py_wndmgr_register(
    window_type: u32,
    layer: *const std::ffi::c_char,
) -> u64 {
    if layer.is_null() {
        return 0;
    }
    let layer_str = unsafe { std::ffi::CStr::from_ptr(layer).to_string_lossy() };
    
    let mut mgr = get_wnd_mgr().lock().unwrap();
    mgr.register_window(window_type, &layer_str)
}

#[no_mangle]
pub extern "C" fn m2_py_wndmgr_destroy(window_id: u64) {
    let mut mgr = get_wnd_mgr().lock().unwrap();
    mgr.destroy_window(window_id);
}

#[no_mangle]
pub extern "C" fn m2_py_wndmgr_set_position(window_id: u64, x: i32, y: i32) {
    let mut mgr = get_wnd_mgr().lock().unwrap();
    if let Some(win) = mgr.windows.get_mut(&window_id) {
        win.set_position(x, y);
    }
}

#[no_mangle]
pub extern "C" fn m2_py_wndmgr_set_size(window_id: u64, width: i32, height: i32) {
    let mut mgr = get_wnd_mgr().lock().unwrap();
    if let Some(win) = mgr.windows.get_mut(&window_id) {
        win.set_size(width, height);
    }
}

#[no_mangle]
pub extern "C" fn m2_py_wndmgr_set_top(window_id: u64) {
    let mut mgr = get_wnd_mgr().lock().unwrap();
    // Replicate CWindowManager::SetTop(pWin)
    mgr.set_top(window_id);
}

#[no_mangle]
pub extern "C" fn m2_py_wndmgr_is_show(window_id: u64) -> bool {
    let mgr = get_wnd_mgr().lock().unwrap();
    if let Some(win) = mgr.windows.get(&window_id) {
        win.is_show
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn m2_py_wndmgr_show(window_id: u64) {
    let mut mgr = get_wnd_mgr().lock().unwrap();
    if let Some(win) = mgr.windows.get_mut(&window_id) {
        win.show();
    }
}

#[no_mangle]
pub extern "C" fn m2_py_wndmgr_hide(window_id: u64) {
    let mut mgr = get_wnd_mgr().lock().unwrap();
    if let Some(win) = mgr.windows.get_mut(&window_id) {
        win.hide();
    }
}

#[no_mangle]
pub extern "C" fn m2_py_wndmgr_is_focus(window_id: u64) -> bool {
    let mgr = get_wnd_mgr().lock().unwrap();
    mgr.active_window == Some(window_id)
}

#[no_mangle]
pub extern "C" fn m2_py_wndmgr_set_focus(window_id: u64) {
    let mut mgr = get_wnd_mgr().lock().unwrap();
    if mgr.windows.contains_key(&window_id) {
        mgr.activate_window(window_id);
    }
}

#[no_mangle]
pub extern "C" fn m2_py_wndmgr_kill_focus(window_id: u64) {
    let mut mgr = get_wnd_mgr().lock().unwrap();
    if window_id == 0 {
        // Like calling with no args or invalid args
        mgr.deactivate_window();
    } else {
        mgr.deactivate_window_specific(window_id);
    }
}

#[no_mangle]
pub extern "C" fn m2_py_wndmgr_add_flag(window_id: u64, flag: u32) {
    let mut mgr = get_wnd_mgr().lock().unwrap();
    if let Some(win) = mgr.windows.get_mut(&window_id) {
        win.flags |= flag;
    }
}

#[no_mangle]
pub extern "C" fn m2_py_wndmgr_remove_flag(window_id: u64, flag: u32) {
    let mut mgr = get_wnd_mgr().lock().unwrap();
    if let Some(win) = mgr.windows.get_mut(&window_id) {
        win.flags &= !flag;
    }
}

#[no_mangle]
pub extern "C" fn m2_py_wndmgr_set_parent(window_id: u64, parent_id: u64) {
    let mut mgr = get_wnd_mgr().lock().unwrap();
    
    // Check if both exist
    if !mgr.windows.contains_key(&window_id) || !mgr.windows.contains_key(&parent_id) {
        return;
    }

    // Remove from old parent
    let old_parent_id = mgr.windows.get(&window_id).unwrap().parent_id;
    if let Some(old_p_id) = old_parent_id {
        if let Some(old_parent) = mgr.windows.get_mut(&old_p_id) {
            old_parent.child_ids.retain(|&x| x != window_id);
        }
    }

    // Add to new parent
    if let Some(new_parent) = mgr.windows.get_mut(&parent_id) {
        new_parent.child_ids.push(window_id);
    }
    
    // Update child's parent reference
    if let Some(win) = mgr.windows.get_mut(&window_id) {
        win.parent_id = Some(parent_id);
    }
}
