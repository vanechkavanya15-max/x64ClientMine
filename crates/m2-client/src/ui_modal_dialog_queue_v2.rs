use std::collections::VecDeque;

#[cfg(windows)]
extern "system" {
    fn SetCapture(hwnd: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn ReleaseCapture() -> i32;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DialogType {
    ConfirmDestroyItem { item_vnum: u32, slot_index: u16 },
    ConfirmExitGame,
    InfoPopup { message: String },
    Question { question: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModalDialog {
    pub id: u32,
    pub dialog_type: DialogType,
}

pub struct UiModalDialogQueueV2 {
    queue: VecDeque<ModalDialog>,
    active_dialog: Option<ModalDialog>,
    mouse_capture_ref: i32,
    next_id: u32,
    hwnd: *mut core::ffi::c_void,
}

// In standard Rust, pointers prevent Send/Sync, but this UI structure is expected
// to be safely manipulated in the main thread. We can manually implement Send/Sync if required,
// but for now we keep it standard.

impl UiModalDialogQueueV2 {
    /// Creates a new `UiModalDialogQueueV2`. `hwnd` can be passed as null or a valid window handle.
    pub fn new(hwnd: *mut core::ffi::c_void) -> Self {
        Self {
            queue: VecDeque::new(),
            active_dialog: None,
            mouse_capture_ref: 0,
            next_id: 1,
            hwnd,
        }
    }

    /// Pushes a new dialog request onto the queue. If there are no active dialogs,
    /// it immediately activates and captures the mouse to block clicks underneath.
    pub fn push_dialog(&mut self, dialog_type: DialogType) -> u32 {
        let dialog = ModalDialog {
            id: self.next_id,
            dialog_type,
        };
        let assigned_id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);

        if self.active_dialog.is_none() {
            self.active_dialog = Some(dialog);
            self.safe_set_capture();
        } else {
            self.queue.push_back(dialog);
        }

        assigned_id
    }

    /// Pops the active dialog, safely releasing mouse capture.
    /// If there are queued dialogs, it activates the next one and captures the mouse again.
    pub fn pop_dialog(&mut self) -> Option<ModalDialog> {
        let current = self.active_dialog.take();
        if current.is_some() {
            self.safe_release_capture();
        }

        // Return to previous state / pull next from queue
        if let Some(next) = self.queue.pop_front() {
            self.active_dialog = Some(next);
            self.safe_set_capture();
        }

        current
    }

    /// Checks if there is an active modal dialog, implying clicks under it should be blocked.
    pub fn has_active_dialog(&self) -> bool {
        self.active_dialog.is_some()
    }

    /// Retrieves a reference to the active dialog, if any.
    pub fn get_active_dialog(&self) -> Option<&ModalDialog> {
        self.active_dialog.as_ref()
    }

    /// Returns the current mouse capture reference count.
    pub fn get_mouse_capture_ref(&self) -> i32 {
        self.mouse_capture_ref
    }

    /// Clears all dialogs and releases any active captures.
    pub fn clear(&mut self) {
        while self.has_active_dialog() {
            self.pop_dialog();
        }
        self.queue.clear();
    }

    fn safe_set_capture(&mut self) {
        if self.mouse_capture_ref == 0 {
            #[cfg(windows)]
            unsafe {
                if !self.hwnd.is_null() {
                    SetCapture(self.hwnd);
                }
            }
        }
        self.mouse_capture_ref += 1;
    }

    fn safe_release_capture(&mut self) {
        if self.mouse_capture_ref > 0 {
            self.mouse_capture_ref -= 1;
            if self.mouse_capture_ref == 0 {
                #[cfg(windows)]
                unsafe {
                    ReleaseCapture();
                }
            }
        }
    }
}

// Needed to allow UiModalDialogQueueV2 to be shared in cross-thread testing scenarios if wrapped in Mutex
unsafe impl Send for UiModalDialogQueueV2 {}
unsafe impl Sync for UiModalDialogQueueV2 {}
