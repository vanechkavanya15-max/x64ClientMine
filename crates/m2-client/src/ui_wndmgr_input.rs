// ui_wndmgr_input.rs
use crate::ui_window_mgr::WindowManager;
use crate::ui_window_core::Window;

impl WindowManager {
    pub fn set_focus(&mut self, handle: u32) -> (usize, usize) {
        if self.active_window == Some(handle) {
            return (0, 0);
        }
        let old_h = if let Some(old) = self.active_window {
            self.windows.get(&old).map(|w| w.py_handler).unwrap_or(0)
        } else {
            0
        };
        self.active_window = Some(handle);
        let new_h = self.windows.get(&handle).map(|w| w.py_handler).unwrap_or(0);
        (old_h, new_h)
    }

    pub fn kill_focus(&mut self, handle: u32) -> usize {
        if self.active_window == Some(handle) {
            self.active_window = None;
            self.windows.get(&handle).map(|w| w.py_handler).unwrap_or(0)
        } else {
            0
        }
    }

    pub fn is_focus(&self, handle: u32) -> bool {
        self.active_window == Some(handle)
    }

    pub fn lock_window(&mut self, handle: u32) {
        if self.windows.contains_key(&handle) {
            self.capture_window = Some(handle);
        }
    }

    pub fn unlock_window(&mut self, handle: u32) {
        if let Some(capture) = self.capture_window {
            if capture == handle {
                self.capture_window = None;
            }
        }
    }

    pub fn find_window_at(&self, x: i32, y: i32, screen_width: i32, screen_height: i32) -> Option<u32> {
        let mut hit = None;
        for root in self.root_windows.iter().rev() {
            if let Some(win) = self.find_window_at_recursive(*root, x, y, None, screen_width, screen_height) {
                hit = Some(win);
                break;
            }
        }
        
        // Modal window logic (lock_window)
        // If a window is captured/locked, intercept interactions outside the modal
        if let Some(capture) = self.capture_window {
            if let Some(h) = hit {
                if !self.is_child_of(h, capture) {
                    return Some(capture);
                }
            } else {
                return Some(capture);
            }
        }
        
        hit
    }
    
    fn is_child_of(&self, handle: u32, parent: u32) -> bool {
        if handle == parent {
            return true;
        }
        let mut current = handle;
        while let Some(win) = self.windows.get(&current) {
            if let Some(p) = win.parent {
                if p == parent {
                    return true;
                }
                current = p;
            } else {
                break;
            }
        }
        false
    }

    fn find_window_at_recursive(&self, handle: u32, x: i32, y: i32, parent_rect: Option<[i32; 4]>, screen_width: i32, screen_height: i32) -> Option<u32> {
        let win = self.windows.get(&handle)?;
        if !win.is_show {
            return None;
        }

        let rect = win.calc_global_rect(parent_rect, screen_width, screen_height);
        let in_rect = x >= rect[0] && x <= rect[0] + rect[2] && y >= rect[1] && y <= rect[1] + rect[3];

        // Check children top to bottom (reverse order)
        for child in win.children.iter().rev() {
            if let Some(hit) = self.find_window_at_recursive(*child, x, y, Some(rect), screen_width, screen_height) {
                return Some(hit);
            }
        }

        if in_rect || win.is_flag_pick_always {
            Some(handle)
        } else {
            None
        }
    }

    pub fn attach_icon(&mut self, icon_type: u32, index: u32, slot_number: u32, width: u8, height: u8) {
        self.is_attaching = true;
        self.attaching_type = icon_type;
        self.attaching_index = index;
        self.attaching_slot_number = slot_number;
        self.attaching_icon_width = width;
        self.attaching_icon_height = height;
    }

    pub fn deattach_icon(&mut self) {
        self.is_attaching = false;
        self.attaching_type = 0;
        self.attaching_index = 0;
        self.attaching_slot_number = 0;
    }

    pub fn set_attaching_flag(&mut self, flag: bool) {
        self.is_attaching = flag;
    }

    pub fn get_attaching_type(&self) -> u32 {
        self.attaching_type
    }

    pub fn get_attaching_index(&self) -> u32 {
        self.attaching_index
    }

    pub fn get_attaching_slot_number(&self) -> u32 {
        self.attaching_slot_number
    }

    pub fn get_attaching_icon_size(&self) -> (u8, u8) {
        (self.attaching_icon_width, self.attaching_icon_height)
    }
}
