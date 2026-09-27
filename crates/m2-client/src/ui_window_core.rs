#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Window {
    pub handle: u32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub is_show: bool,
    pub is_top: bool,
}

impl Window {
    pub fn new(handle: u32) -> Self {
        Self {
            handle,
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            is_show: false,
            is_top: false,
        }
    }
}
