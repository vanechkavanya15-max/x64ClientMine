use crate::ui_window_core::Window;

pub const FLAG_MOVABLE: u32 = 1 << 0;
pub const FLAG_LIMIT: u32 = 1 << 1;
pub const FLAG_SNAP: u32 = 1 << 2;
pub const FLAG_DRAGABLE: u32 = 1 << 3;
pub const FLAG_ATTACH: u32 = 1 << 4;
pub const FLAG_RESTRICT_X: u32 = 1 << 5;
pub const FLAG_RESTRICT_Y: u32 = 1 << 6;
pub const FLAG_NOT_CAPTURE: u32 = 1 << 7;
pub const FLAG_FLOAT: u32 = 1 << 8;
pub const FLAG_NOT_PICK: u32 = 1 << 9;
pub const FLAG_IGNORE_SIZE: u32 = 1 << 10;
pub const FLAG_RTL: u32 = 1 << 11;

pub fn is_flag(win: &Window, flag: u32) -> bool {
    (win.flags & flag) != 0
}

pub fn add_flag(win: &mut Window, flag: u32) {
    win.flags |= flag;
}

pub fn remove_flag(win: &mut Window, flag: u32) {
    win.flags &= !flag;
}

pub fn parse_and_add_flag_string(win: &mut Window, flag_str: &str) {
    match flag_str.to_lowercase().as_str() {
        "movable" => add_flag(win, FLAG_MOVABLE),
        "limit" => add_flag(win, FLAG_LIMIT),
        "dragable" => add_flag(win, FLAG_DRAGABLE),
        "attach" => add_flag(win, FLAG_ATTACH),
        "restrict_x" => add_flag(win, FLAG_RESTRICT_X),
        "restrict_y" => add_flag(win, FLAG_RESTRICT_Y),
        "not_capture" => add_flag(win, FLAG_NOT_CAPTURE),
        "float" => add_flag(win, FLAG_FLOAT),
        "not_pick" => add_flag(win, FLAG_NOT_PICK),
        "ignore_size" => add_flag(win, FLAG_IGNORE_SIZE),
        "rtl" => add_flag(win, FLAG_RTL),
        "ltr" => remove_flag(win, FLAG_RTL),
        _ => {} // Unknown flag
    }
}
