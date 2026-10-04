// ui_wndmgr_widget_scroll_bar.rs - ScrollBar widget implementation

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollBarOrientation {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone)]
pub struct ScrollBarWidget {
    pub orientation: ScrollBarOrientation,
    pub pos: f32, // Ratio from 0.0 to 1.0
    pub step: f32, // Amount to scroll per step (e.g., arrow click)
    pub page_step: f32, // Amount to scroll per page (e.g., track click)
    
    // Geometry for drag calculations
    pub track_length: f32,
    pub thumb_length: f32,
    
    // Drag state
    pub is_dragging: bool,
    pub drag_start_mouse_pos: f32,
    pub drag_start_scroll_pos: f32,
}

impl Default for ScrollBarWidget {
    fn default() -> Self {
        Self::new(ScrollBarOrientation::Vertical)
    }
}

impl ScrollBarWidget {
    pub fn new(orientation: ScrollBarOrientation) -> Self {
        Self {
            orientation,
            pos: 0.0,
            step: 0.05,
            page_step: 0.2,
            track_length: 100.0,
            thumb_length: 20.0,
            is_dragging: false,
            drag_start_mouse_pos: 0.0,
            drag_start_scroll_pos: 0.0,
        }
    }

    pub fn set_pos(&mut self, pos: f32) {
        self.pos = pos.clamp(0.0, 1.0);
    }

    pub fn get_pos(&self) -> f32 {
        self.pos
    }

    pub fn scroll_up(&mut self) {
        self.set_pos(self.pos - self.step);
    }

    pub fn scroll_down(&mut self) {
        self.set_pos(self.pos + self.step);
    }

    pub fn on_mouse_wheel(&mut self, delta: i32) {
        if delta > 0 {
            self.scroll_up();
        } else if delta < 0 {
            self.scroll_down();
        }
    }

    pub fn on_mouse_down_track(&mut self, mouse_coord: f32) {
        let movable_track = self.track_length - self.thumb_length;
        if movable_track <= 0.0 {
            return;
        }

        let thumb_pixel_pos = self.pos * movable_track;

        if mouse_coord < thumb_pixel_pos {
            // Clicked before the thumb (above/left)
            self.set_pos(self.pos - self.page_step);
        } else if mouse_coord > thumb_pixel_pos + self.thumb_length {
            // Clicked after the thumb (below/right)
            self.set_pos(self.pos + self.page_step);
        }
    }

    pub fn begin_drag(&mut self, mouse_coord: f32) {
        self.is_dragging = true;
        self.drag_start_mouse_pos = mouse_coord;
        self.drag_start_scroll_pos = self.pos;
    }

    pub fn update_drag(&mut self, mouse_coord: f32) {
        if !self.is_dragging {
            return;
        }

        let movable_track = self.track_length - self.thumb_length;
        if movable_track > 0.0 {
            let pixel_delta = mouse_coord - self.drag_start_mouse_pos;
            let pos_delta = pixel_delta / movable_track;
            self.set_pos(self.drag_start_scroll_pos + pos_delta);
        }
    }

    pub fn end_drag(&mut self) {
        self.is_dragging = false;
    }
}
