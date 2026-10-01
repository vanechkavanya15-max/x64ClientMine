// ui_window_core.rs - Window data models, hierarchy and layout for Metin2 Pure Rust

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowType {
    Normal,
    ImageBox,
    ExpandedImageBox,
    TextLine,
    Button,
    Bar,
    Line,
    Bar3D,
    Box,
    ListBox,
    ThinBoard,
    Board,
    SlotWindow,
    EditLine,
    ScrollBar,
}


pub const HORIZONTAL_ALIGN_LEFT: u8 = 0;
pub const HORIZONTAL_ALIGN_CENTER: u8 = 1;
pub const HORIZONTAL_ALIGN_RIGHT: u8 = 2;

pub const VERTICAL_ALIGN_TOP: u8 = 0;
pub const VERTICAL_ALIGN_CENTER: u8 = 1;
pub const VERTICAL_ALIGN_BOTTOM: u8 = 2;

#[derive(Debug, Clone)]
pub struct Window {
    pub handle: u32,
    pub parent: Option<u32>,
    pub children: Vec<u32>,
    pub local_x: i32,
    pub local_y: i32,
    pub width: i32,
    pub height: i32,
    pub horizontal_align: u8,
    pub vertical_align: u8,
    pub is_show: bool,
    pub is_top: bool,
    pub name: String,
    pub window_type: WindowType,

    // Grafika
    pub image_path: String,
    pub rendering_rect: [f32; 4], // [left, top, right, bottom]
    pub scale: [f32; 2],

    // Kolory
    pub color: [f32; 4],

    // Tekst
    pub text: String,
    pub text_color: [f32; 4],
    pub text_align: u8,

    // Przyciski
    pub up_visual: String,
    pub over_visual: String,
    pub down_visual: String,
    pub is_pressed: bool,
    pub is_hover: bool,
}

impl Window {
    pub fn new(handle: u32, window_type: WindowType) -> Self {
        Self {
            handle,
            parent: None,
            children: Vec::new(),
            local_x: 0,
            local_y: 0,
            width: 0,
            height: 0,
            horizontal_align: HORIZONTAL_ALIGN_LEFT,
            vertical_align: VERTICAL_ALIGN_TOP,
            is_show: true,
            is_top: false,
            name: String::new(),
            window_type,
            image_path: String::new(),
            rendering_rect: [0.0, 0.0, 0.0, 0.0],
            scale: [1.0, 1.0],
            color: [1.0, 1.0, 1.0, 1.0],
            text: String::new(),
            text_color: [1.0, 1.0, 1.0, 1.0],
            text_align: HORIZONTAL_ALIGN_LEFT,
            up_visual: String::new(),
            over_visual: String::new(),
            down_visual: String::new(),
            is_pressed: false,
            is_hover: false,
        }
    }

    /// Oblicza prostokat globalny okna na ekranie [x, y, w, h]
    pub fn calc_global_rect(
        &self,
        parent_rect: Option<[i32; 4]>,
        screen_w: i32,
        screen_h: i32,
    ) -> [i32; 4] {
        let (px, py, pw, ph) = if let Some([px, py, pw, ph]) = parent_rect {
            (px, py, pw, ph)
        } else {
            (0, 0, screen_w, screen_h)
        };

        let gx = match self.horizontal_align {
            HORIZONTAL_ALIGN_CENTER => px + (pw - self.width) / 2 + self.local_x,
            HORIZONTAL_ALIGN_RIGHT => px + pw - self.width - self.local_x,
            _ => px + self.local_x,
        };

        let gy = match self.vertical_align {
            VERTICAL_ALIGN_CENTER => py + (ph - self.height) / 2 + self.local_y,
            VERTICAL_ALIGN_BOTTOM => py + ph - self.height - self.local_y,
            _ => py + self.local_y,
        };

        [gx, gy, self.width, self.height]
    }
}
