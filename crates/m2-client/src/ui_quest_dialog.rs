//! ui_quest_dialog.rs
//! Component holding the logical state of the Quest Dialog UI.

use std::sync::{Mutex, OnceLock};

#[derive(Default, Debug, Clone)]
pub struct UiQuestDialog {
    pub skin_type: u8,
    pub title: String,
    pub clock_title: String,
    pub clock_value: u32,
    pub counter_name: String,
    pub counter_value: u32,
}

impl UiQuestDialog {
    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

static UI_QUEST_DIALOG: OnceLock<Mutex<UiQuestDialog>> = OnceLock::new();

pub fn get_ui_quest_dialog() -> &'static Mutex<UiQuestDialog> {
    UI_QUEST_DIALOG.get_or_init(|| Mutex::new(UiQuestDialog::default()))
}
