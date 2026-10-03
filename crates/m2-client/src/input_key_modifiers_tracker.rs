#[derive(Debug, Clone, Default)]
pub struct InputKeyModifiersTracker {
    ctrl_pressed: bool,
    shift_pressed: bool,
    alt_pressed: bool,
}

impl InputKeyModifiersTracker {
    pub const VK_SHIFT: u32 = 0x10;
    pub const VK_CONTROL: u32 = 0x11;
    pub const VK_MENU: u32 = 0x12; // ALT

    pub const VK_LSHIFT: u32 = 0xA0;
    pub const VK_RSHIFT: u32 = 0xA1;
    pub const VK_LCONTROL: u32 = 0xA2;
    pub const VK_RCONTROL: u32 = 0xA3;
    pub const VK_LMENU: u32 = 0xA4;
    pub const VK_RMENU: u32 = 0xA5;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn on_key_down(&mut self, key_code: u32) {
        match key_code {
            Self::VK_SHIFT | Self::VK_LSHIFT | Self::VK_RSHIFT => {
                self.shift_pressed = true;
            }
            Self::VK_CONTROL | Self::VK_LCONTROL | Self::VK_RCONTROL => {
                self.ctrl_pressed = true;
            }
            Self::VK_MENU | Self::VK_LMENU | Self::VK_RMENU => {
                self.alt_pressed = true;
            }
            _ => {}
        }
    }

    pub fn on_key_up(&mut self, key_code: u32) {
        match key_code {
            Self::VK_SHIFT | Self::VK_LSHIFT | Self::VK_RSHIFT => {
                self.shift_pressed = false;
            }
            Self::VK_CONTROL | Self::VK_LCONTROL | Self::VK_RCONTROL => {
                self.ctrl_pressed = false;
            }
            Self::VK_MENU | Self::VK_LMENU | Self::VK_RMENU => {
                self.alt_pressed = false;
            }
            _ => {}
        }
    }

    pub fn reset(&mut self) {
        self.ctrl_pressed = false;
        self.shift_pressed = false;
        self.alt_pressed = false;
    }

    pub fn is_ctrl_pressed(&self) -> bool {
        self.ctrl_pressed
    }

    pub fn is_shift_pressed(&self) -> bool {
        self.shift_pressed
    }

    pub fn is_alt_pressed(&self) -> bool {
        self.alt_pressed
    }
}
