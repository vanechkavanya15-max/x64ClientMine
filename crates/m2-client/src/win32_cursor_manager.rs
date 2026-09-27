use std::collections::HashMap;
use std::ffi::c_void;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CursorType {
    Normal = 0,
    Attack,
    Target,
    Talk,
    CantGo,
    Pick,
    Door,
    Chair,
    Magic,
    Buy,
    Sell,
    CameraRotate,
}

#[repr(C)]
pub struct CursorResource {
    pub cursor_type: CursorType,
    pub handle: *mut c_void,
    pub hotspot_x: i32,
    pub hotspot_y: i32,
}

unsafe impl Send for CursorResource {}
unsafe impl Sync for CursorResource {}

pub struct CursorController {
    resources: HashMap<CursorType, CursorResource>,
    current_cursor: CursorType,
    is_visible: bool,
    
    // For camera rotation logic
    saved_cursor: Option<CursorType>,
    saved_visibility: Option<bool>,
}

#[cfg(windows)]
extern "system" {
    fn SetCursor(hCursor: *mut c_void) -> *mut c_void;
    fn ShowCursor(bShow: i32) -> i32;
}

impl CursorController {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
            current_cursor: CursorType::Normal,
            is_visible: true,
            saved_cursor: None,
            saved_visibility: None,
        }
    }

    pub fn register_cursor(&mut self, resource: CursorResource) {
        self.resources.insert(resource.cursor_type, resource);
    }

    pub fn set_cursor(&mut self, cursor_type: CursorType) {
        self.current_cursor = cursor_type;
        if let Some(res) = self.resources.get(&cursor_type) {
            #[cfg(windows)]
            unsafe {
                SetCursor(res.handle);
            }
            #[cfg(not(windows))]
            let _ = res.handle; // Ensure it's used to avoid warnings
        }
    }

    pub fn hide_cursor(&mut self) {
        if self.is_visible {
            self.is_visible = false;
            #[cfg(windows)]
            unsafe {
                ShowCursor(0);
            }
        }
    }

    pub fn show_cursor(&mut self) {
        if !self.is_visible {
            self.is_visible = true;
            #[cfg(windows)]
            unsafe {
                ShowCursor(1);
            }
        }
    }

    pub fn get_current_cursor(&self) -> CursorType {
        self.current_cursor
    }

    pub fn is_cursor_visible(&self) -> bool {
        self.is_visible
    }

    /// Handles automatic cursor hiding when starting to rotate the camera with Right Mouse Button
    pub fn begin_camera_rotate(&mut self) {
        // Save previous state
        self.saved_cursor = Some(self.current_cursor);
        self.saved_visibility = Some(self.is_visible);

        // Set to CameraRotate cursor and hide it
        self.set_cursor(CursorType::CameraRotate);
        self.hide_cursor();
    }

    /// Restores the cursor state after finishing camera rotation
    pub fn end_camera_rotate(&mut self) {
        if let Some(cursor) = self.saved_cursor.take() {
            self.set_cursor(cursor);
        }

        if let Some(visibility) = self.saved_visibility.take() {
            if visibility {
                self.show_cursor();
            } else {
                self.hide_cursor();
            }
        }
    }
}

impl Default for CursorController {
    fn default() -> Self {
        Self::new()
    }
}
