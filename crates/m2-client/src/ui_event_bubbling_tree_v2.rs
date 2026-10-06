use crate::ui_wndmgr_hierarchy::{WindowNode, FLAG_ATTACH, FLAG_NOT_CAPTURE, FLAG_NOT_PICK};
use std::collections::HashMap;

/// Modul obslugujacy propagacje zdarzen klikniec w drzewie UI z zachowaniem pelnego
/// parytetu dla Metin2 C++ (PythonWindowManager::RunMouseLeftButtonDown) ale w 
/// poszerzonym modelu z automatycznym bubblingiem, gdzie brak skonsumowania eventu
/// powoduje jego propagacje do rodzica.
pub struct UiEventBubblingTreeV2 {
    pub left_capture_window: Option<u32>,
    pub drag_x: i32,
    pub drag_y: i32,
    pub picked_x: i32,
    pub picked_y: i32,
}

impl Default for UiEventBubblingTreeV2 {
    fn default() -> Self {
        Self::new()
    }
}

impl UiEventBubblingTreeV2 {
    pub fn new() -> Self {
        Self {
            left_capture_window: None,
            drag_x: 0,
            drag_y: 0,
            picked_x: 0,
            picked_y: 0,
        }
    }

    /// Implementuje propagacje zdarzen klikniec myszy (bubbling) do rodzica
    /// jesli kontrolka nie konsumuje zdarzenia lub ma flage ignorowania (np. FLAG_NOT_PICK).
    pub fn run_mouse_left_button_down<F, R>(
        &mut self,
        mouse_x: i32,
        mouse_y: i32,
        point_window: Option<u32>,
        windows: &HashMap<u32, WindowNode>,
        get_rect_fn: R,
        mut on_mouse_left_button_down: F,
    ) -> Option<u32>
    where
        F: FnMut(u32) -> bool,
        R: Fn(u32) -> [i32; 4], // [left, top, right, bottom]
    {
        let mut current_win_id = point_window;

        while let Some(win_id) = current_win_id {
            if let Some(mut win) = windows.get(&win_id) {
                // Jesli ma flage ignorowania (FLAG_NOT_PICK), ignorujemy i wedrujemy do rodzica
                if (win.flags & FLAG_NOT_PICK) != 0 {
                    current_win_id = win.parent_id;
                    continue;
                }

                let mut target_win_id = win_id;

                // FLAG_ATTACH przeskakuje bezposrednio do korzenia (Root Window)
                if (win.flags & FLAG_ATTACH) != 0 {
                    let mut root_id = target_win_id;
                    while let Some(parent_id) = windows.get(&root_id).and_then(|w| w.parent_id) {
                        root_id = parent_id;
                    }
                    target_win_id = root_id;
                    if let Some(root_win) = windows.get(&target_win_id) {
                        win = root_win;
                    } else {
                        break;
                    }
                }

                // Sprawdzamy czy okno konsumuje zdarzenie myszy
                let consumed = on_mouse_left_button_down(target_win_id);

                if consumed {
                    // Update Drag/Capture stanu tylko jesli nie ma FLAG_NOT_CAPTURE
                    if (win.flags & FLAG_NOT_CAPTURE) == 0 {
                        self.left_capture_window = Some(target_win_id);
                    }

                    let rect = get_rect_fn(target_win_id);
                    self.drag_x = mouse_x - rect[0];
                    self.drag_y = mouse_y - rect[1];
                    self.picked_x = mouse_x;
                    self.picked_y = mouse_y;

                    return Some(target_win_id);
                } else {
                    // Bubbling (Propagacja do rodzica)
                    current_win_id = win.parent_id;
                }
            } else {
                break;
            }
        }

        None
    }
}
