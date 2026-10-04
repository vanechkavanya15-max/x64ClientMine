// ui_wndmgr_widget_thin_board.rs

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NineSliceGeometry {
    pub top_left: Rect,
    pub top: Rect,
    pub top_right: Rect,
    pub left: Rect,
    pub center: Rect,
    pub right: Rect,
    pub bottom_left: Rect,
    pub bottom: Rect,
    pub bottom_right: Rect,
}

impl NineSliceGeometry {
    pub fn calculate(width: f32, height: f32, corner_size: f32) -> Self {
        let cw = corner_size;
        let ch = corner_size;

        let inner_width = (width - 2.0 * cw).max(0.0);
        let inner_height = (height - 2.0 * ch).max(0.0);

        let top_y = 0.0;
        let bottom_y = height - ch;
        let left_x = 0.0;
        let right_x = width - cw;

        Self {
            top_left: Rect::new(left_x, top_y, cw, ch),
            top: Rect::new(cw, top_y, inner_width, ch),
            top_right: Rect::new(right_x, top_y, cw, ch),
            left: Rect::new(left_x, ch, cw, inner_height),
            center: Rect::new(cw, ch, inner_width, inner_height),
            right: Rect::new(right_x, ch, cw, inner_height),
            bottom_left: Rect::new(left_x, bottom_y, cw, ch),
            bottom: Rect::new(cw, bottom_y, inner_width, ch),
            bottom_right: Rect::new(right_x, bottom_y, cw, ch),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThinBoard {
    pub geometry: NineSliceGeometry,
    pub width: f32,
    pub height: f32,
    pub corner_size: f32,
}

impl ThinBoard {
    pub fn new(width: f32, height: f32) -> Self {
        let corner_size = 16.0;
        Self {
            geometry: NineSliceGeometry::calculate(width, height, corner_size),
            width,
            height,
            corner_size,
        }
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
        self.geometry = NineSliceGeometry::calculate(width, height, self.corner_size);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    Normal,
    Hover,
    Down,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BoardWithTitleBar {
    pub board: ThinBoard,
    pub title: String,
    pub title_bar_rect: Rect,
    pub close_button_rect: Rect,
    pub close_button_state: ButtonState,
}

impl BoardWithTitleBar {
    const TITLE_BAR_HEIGHT: f32 = 24.0;
    const CLOSE_BTN_WIDTH: f32 = 16.0;
    const CLOSE_BTN_HEIGHT: f32 = 16.0;
    const CLOSE_BTN_MARGIN_RIGHT: f32 = 8.0;

    pub fn new(width: f32, height: f32, title: &str) -> Self {
        let board = ThinBoard::new(width, height);
        let title_bar_rect = Rect::new(0.0, 0.0, width, Self::TITLE_BAR_HEIGHT);
        
        let close_btn_margin_top = (Self::TITLE_BAR_HEIGHT - Self::CLOSE_BTN_HEIGHT) / 2.0;

        let close_button_rect = Rect::new(
            width - Self::CLOSE_BTN_WIDTH - Self::CLOSE_BTN_MARGIN_RIGHT,
            close_btn_margin_top,
            Self::CLOSE_BTN_WIDTH,
            Self::CLOSE_BTN_HEIGHT,
        );

        Self {
            board,
            title: title.to_string(),
            title_bar_rect,
            close_button_rect,
            close_button_state: ButtonState::Normal,
        }
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.board.resize(width, height);
        self.title_bar_rect.width = width;
        
        self.close_button_rect.x = width - self.close_button_rect.width - Self::CLOSE_BTN_MARGIN_RIGHT;
    }

    pub fn handle_mouse_move(&mut self, x: f32, y: f32) {
        if self.close_button_rect.contains(x, y) {
            if self.close_button_state != ButtonState::Down {
                self.close_button_state = ButtonState::Hover;
            }
        } else {
            self.close_button_state = ButtonState::Normal;
        }
    }

    pub fn handle_mouse_down(&mut self, x: f32, y: f32) -> bool {
        if self.close_button_rect.contains(x, y) {
            self.close_button_state = ButtonState::Down;
            return true;
        }
        false
    }

    pub fn handle_mouse_up(&mut self, x: f32, y: f32) -> bool {
        let was_down = self.close_button_state == ButtonState::Down;
        if self.close_button_rect.contains(x, y) {
            self.close_button_state = ButtonState::Hover;
            return was_down; // Zwraca true, jeśli nastąpiło kliknięcie
        } else {
            self.close_button_state = ButtonState::Normal;
            return false;
        }
    }
}
