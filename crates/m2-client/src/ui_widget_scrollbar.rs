// ui_widget_scrollbar.rs - ScrollBar and ListBox UI controls implementation

// C++ equivalents for reference:
// CUIScrollBar
// CUIListBox

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollBarOrientation {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone)]
pub struct ScrollBarState {
    pub orientation: ScrollBarOrientation,
    pub current_pos: f32, // 0.0 to 1.0
    pub page_size: f32,
    pub scroll_step: f32,
    pub is_dragging_thumb: bool,
    
    // Geometry for interaction calculations
    pub track_length: f32,
    pub thumb_length: f32,
    pub drag_start_mouse_pos: f32,
    pub drag_start_scroll_pos: f32,
}

impl ScrollBarState {
    pub fn new() -> Self {
        Self {
            orientation: ScrollBarOrientation::Vertical,
            current_pos: 0.0,
            page_size: 0.1,
            scroll_step: 0.05,
            is_dragging_thumb: false,
            track_length: 100.0,
            thumb_length: 20.0,
            drag_start_mouse_pos: 0.0,
            drag_start_scroll_pos: 0.0,
        }
    }

    pub fn set_pos(&mut self, pos: f32) {
        self.current_pos = pos.clamp(0.0, 1.0);
    }

    pub fn get_pos(&self) -> f32 {
        self.current_pos
    }

    pub fn scroll_up(&mut self) {
        self.set_pos(self.current_pos - self.scroll_step);
    }

    pub fn scroll_down(&mut self) {
        self.set_pos(self.current_pos + self.scroll_step);
    }

    pub fn handle_mouse_wheel(&mut self, delta: i32) {
        if delta > 0 {
            self.scroll_up();
        } else if delta < 0 {
            self.scroll_down();
        }
    }

    pub fn handle_mouse_down_track(&mut self, mouse_pos: f32) {
        // Oblicz czy kliknieto nad czy pod suwakiem
        let thumb_pos = self.current_pos * (self.track_length - self.thumb_length);
        if mouse_pos < thumb_pos {
            self.set_pos(self.current_pos - self.page_size);
        } else if mouse_pos > thumb_pos + self.thumb_length {
            self.set_pos(self.current_pos + self.page_size);
        }
    }

    pub fn begin_drag_thumb(&mut self, mouse_pos: f32) {
        self.is_dragging_thumb = true;
        self.drag_start_mouse_pos = mouse_pos;
        self.drag_start_scroll_pos = self.current_pos;
    }

    pub fn handle_drag_thumb(&mut self, mouse_pos: f32) {
        if !self.is_dragging_thumb {
            return;
        }

        let delta_pixels = mouse_pos - self.drag_start_mouse_pos;
        let movable_track = self.track_length - self.thumb_length;
        
        if movable_track > 0.0 {
            let delta_pos = delta_pixels / movable_track;
            self.set_pos(self.drag_start_scroll_pos + delta_pos);
        }
    }

    pub fn end_drag_thumb(&mut self) {
        self.is_dragging_thumb = false;
    }
}

#[derive(Debug, Clone)]
pub struct ListBoxState {
    pub lines: Vec<String>,
    pub visible_lines: usize,
    pub scroll_offset: usize,
    pub selected_line: Option<usize>,
    pub line_height: f32,
    pub width: f32,
}

impl ListBoxState {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            visible_lines: 5,
            scroll_offset: 0,
            selected_line: None,
            line_height: 15.0,
            width: 100.0,
        }
    }

    pub fn append_item(&mut self, text: String) {
        self.lines.push(text);
    }

    pub fn insert_item(&mut self, index: usize, text: String) {
        if index <= self.lines.len() {
            self.lines.insert(index, text);
        }
    }

    pub fn remove_item(&mut self, index: usize) {
        if index < self.lines.len() {
            self.lines.remove(index);
            if let Some(sel) = self.selected_line {
                if sel == index {
                    self.selected_line = None;
                } else if sel > index {
                    self.selected_line = Some(sel - 1);
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.lines.clear();
        self.scroll_offset = 0;
        self.selected_line = None;
    }

    pub fn get_item_count(&self) -> usize {
        self.lines.len()
    }

    pub fn get_item_text(&self, index: usize) -> Option<&String> {
        self.lines.get(index)
    }

    pub fn select_item(&mut self, index: usize) {
        if index < self.lines.len() {
            self.selected_line = Some(index);
        }
    }

    pub fn set_scroll_offset(&mut self, offset: usize) {
        let max_offset = if self.lines.len() > self.visible_lines {
            self.lines.len() - self.visible_lines
        } else {
            0
        };
        self.scroll_offset = offset.clamp(0, max_offset);
    }

    pub fn sync_with_scrollbar(&mut self, scroll_pos: f32) {
        let max_offset = if self.lines.len() > self.visible_lines {
            self.lines.len() - self.visible_lines
        } else {
            0
        };
        
        let new_offset = (scroll_pos * max_offset as f32).round() as usize;
        self.set_scroll_offset(new_offset);
    }

    pub fn handle_mouse_wheel(&mut self, delta: i32) {
        if delta > 0 {
            if self.scroll_offset > 0 {
                self.set_scroll_offset(self.scroll_offset - 1);
            }
        } else if delta < 0 {
            self.set_scroll_offset(self.scroll_offset + 1);
        }
    }
}
