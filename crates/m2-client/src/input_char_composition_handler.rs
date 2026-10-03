use crate::ui_widget_editline::EditLineWidget;

pub struct InputCharCompositionHandler;

impl InputCharCompositionHandler {
    pub fn handle_wm_char(edit_line: &mut EditLineWidget, wparam: u32) -> bool {
        let ch = match char::from_u32(wparam) {
            Some(c) => c,
            None => return false,
        };

        if ch == '\x08' {
            edit_line.backspace();
            return true;
        }

        if !ch.is_control() {
            edit_line.insert_char(ch);
            return true;
        }

        false
    }
}
