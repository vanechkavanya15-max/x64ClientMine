use crate::ui_widget_editline_ime::ImeEditLineWidget;
use crate::ui_widget_scrollbar::{ScrollBarState, ScrollBarOrientation};
use crate::ui_widget_slotwindow::{SlotWindow, SlotItem};

pub struct FullUiAdv15Validator;

impl FullUiAdv15Validator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_ime_editline_insert(&self, editline: &mut ImeEditLineWidget, text: &str) -> bool {
        editline.clear();
        editline.insert_string(text);
        editline.text == text && editline.cursor_pos == text.chars().count()
    }

    pub fn validate_ime_editline_selection(&self, editline: &mut ImeEditLineWidget, text: &str) -> bool {
        editline.clear();
        editline.insert_string(text);
        editline.select_all();
        editline.delete_selection();
        editline.text.is_empty() && editline.cursor_pos == 0
    }

    pub fn validate_scrollbar_scroll(&self, scrollbar: &mut ScrollBarState) -> bool {
        scrollbar.set_pos(0.5);
        scrollbar.scroll_up();
        let pos_up = scrollbar.get_pos();
        scrollbar.set_pos(0.5);
        scrollbar.scroll_down();
        let pos_down = scrollbar.get_pos();
        
        pos_up < 0.5 && pos_down > 0.5
    }

    pub fn validate_slotwindow_set_item(&self, slotwindow: &mut SlotWindow, id: usize, vnum: u32, count: u32) -> bool {
        let item = SlotItem::new(vnum, count, "icon.tga".to_string(), false);
        if slotwindow.set_item(id, item).is_ok() {
            if let Some(cell) = slotwindow.cells.iter().find(|c| c.id == id) {
                if let Some(i) = &cell.item {
                    return i.vnum == vnum && i.count == count;
                }
            }
        }
        false
    }
}
