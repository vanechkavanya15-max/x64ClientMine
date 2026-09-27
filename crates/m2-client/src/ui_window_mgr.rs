use crate::ui_window_core::Window;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

pub struct WindowManager {
    windows: HashMap<u32, Window>,
    next_handle: u32,
    top_window: Option<u32>,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
            next_handle: 1,
            top_window: None,
        }
    }

    pub fn register(&mut self, _py_obj: usize) -> u32 {
        let handle = self.next_handle;
        self.next_handle += 1;
        self.windows.insert(handle, Window::new(handle));
        handle
    }

    pub fn set_position(&mut self, handle: u32, x: i32, y: i32) -> bool {
        if let Some(win) = self.windows.get_mut(&handle) {
            win.x = x;
            win.y = y;
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

    pub fn set_top(&mut self, handle: u32) -> bool {
        if let Some(win) = self.windows.get_mut(&handle) {
            win.is_top = true;
            if let Some(old_top) = self.top_window {
                if old_top != handle {
                    if let Some(old_win) = self.windows.get_mut(&old_top) {
                        old_win.is_top = false;
                    }
                }
            }
            self.top_window = Some(handle);
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

    pub fn is_show(&self, handle: u32) -> bool {
        if let Some(win) = self.windows.get(&handle) {
            win.is_show
        } else {
            false
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
