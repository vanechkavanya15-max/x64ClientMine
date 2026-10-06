#[derive(Debug, PartialEq)]
pub enum TaskbarAction {
    UseQuickSlot(u8),
    UseFunctionSlot(u8),
    ChangePage(u8),
    OpenInventory,
    OpenCharacter,
    OpenSkills,
    OpenQuests,
    OpenOptions,
    None,
}

pub struct TaskbarController {
    pub active_page: u8,
}

pub const DIK_F1: u32 = 0x3B;
pub const DIK_F2: u32 = 0x3C;
pub const DIK_F3: u32 = 0x3D;
pub const DIK_F4: u32 = 0x3E;

impl TaskbarController {
    pub fn new() -> Self {
        Self {
            active_page: 0,
        }
    }

    pub fn handle_key_down(&mut self, key_code: u32, is_shift_pressed: bool, is_alt_pressed: bool) -> TaskbarAction {
        if is_shift_pressed {
            match key_code {
                crate::py_bridge_app::DIK_1 => {
                    self.active_page = 0;
                    return TaskbarAction::ChangePage(0);
                }
                crate::py_bridge_app::DIK_2 => {
                    self.active_page = 1;
                    return TaskbarAction::ChangePage(1);
                }
                crate::py_bridge_app::DIK_3 => {
                    self.active_page = 2;
                    return TaskbarAction::ChangePage(2);
                }
                crate::py_bridge_app::DIK_4 => {
                    self.active_page = 3;
                    return TaskbarAction::ChangePage(3);
                }
                _ => {}
            }
        }

        if is_alt_pressed {
            return TaskbarAction::None;
        }

        match key_code {
            crate::py_bridge_app::DIK_1 => TaskbarAction::UseQuickSlot(0),
            crate::py_bridge_app::DIK_2 => TaskbarAction::UseQuickSlot(1),
            crate::py_bridge_app::DIK_3 => TaskbarAction::UseQuickSlot(2),
            crate::py_bridge_app::DIK_4 => TaskbarAction::UseQuickSlot(3),
            
            DIK_F1 => TaskbarAction::UseFunctionSlot(0),
            DIK_F2 => TaskbarAction::UseFunctionSlot(1),
            DIK_F3 => TaskbarAction::UseFunctionSlot(2),
            DIK_F4 => TaskbarAction::UseFunctionSlot(3),

            crate::py_bridge_app::DIK_I => TaskbarAction::OpenInventory,
            crate::py_bridge_app::DIK_C => TaskbarAction::OpenCharacter,
            crate::py_bridge_app::DIK_V => TaskbarAction::OpenSkills,
            crate::py_bridge_app::DIK_N => TaskbarAction::OpenQuests,
            crate::py_bridge_app::DIK_ESCAPE => TaskbarAction::OpenOptions,
            _ => TaskbarAction::None,
        }
    }
}
