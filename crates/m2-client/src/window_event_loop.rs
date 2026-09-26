#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowEvent {
    KeyDown(u32),
    KeyUp(u32),
    MouseDown(MouseButton, i32, i32),
    MouseUp(MouseButton, i32, i32),
    MouseMove(i32, i32),
    MouseWheel(i32),
    Resize(u32, u32),
    CloseRequested,
    FocusChanged(bool),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputState {
    pub keys_down: [bool; 256],
    pub mouse_x: i32,
    pub mouse_y: i32,
    pub delta_x: i32,
    pub delta_y: i32,
    pub mouse_left_down: bool,
    pub mouse_right_down: bool,
    pub mouse_middle_down: bool,
    pub scroll_delta: i32,
    pub window_width: u32,
    pub window_height: u32,
    pub is_focused: bool,
    pub close_requested: bool,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            keys_down: [false; 256],
            mouse_x: 0,
            mouse_y: 0,
            delta_x: 0,
            delta_y: 0,
            mouse_left_down: false,
            mouse_right_down: false,
            mouse_middle_down: false,
            scroll_delta: 0,
            window_width: 800,
            window_height: 600,
            is_focused: true,
            close_requested: false,
        }
    }

    pub fn process_event(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::KeyDown(vk) => {
                if vk < 256 {
                    self.keys_down[vk as usize] = true;
                }
            }
            WindowEvent::KeyUp(vk) => {
                if vk < 256 {
                    self.keys_down[vk as usize] = false;
                }
            }
            WindowEvent::MouseDown(btn, x, y) => {
                self.update_mouse_pos(x, y);
                match btn {
                    MouseButton::Left => self.mouse_left_down = true,
                    MouseButton::Right => self.mouse_right_down = true,
                    MouseButton::Middle => self.mouse_middle_down = true,
                }
            }
            WindowEvent::MouseUp(btn, x, y) => {
                self.update_mouse_pos(x, y);
                match btn {
                    MouseButton::Left => self.mouse_left_down = false,
                    MouseButton::Right => self.mouse_right_down = false,
                    MouseButton::Middle => self.mouse_middle_down = false,
                }
            }
            WindowEvent::MouseMove(x, y) => {
                self.update_mouse_pos(x, y);
            }
            WindowEvent::MouseWheel(delta) => {
                self.scroll_delta += delta;
            }
            WindowEvent::Resize(w, h) => {
                self.window_width = w;
                self.window_height = h;
            }
            WindowEvent::FocusChanged(focused) => {
                self.is_focused = focused;
                if !focused {
                    self.keys_down.fill(false);
                    self.mouse_left_down = false;
                    self.mouse_right_down = false;
                    self.mouse_middle_down = false;
                }
            }
            WindowEvent::CloseRequested => {
                self.close_requested = true;
            }
        }
    }

    fn update_mouse_pos(&mut self, x: i32, y: i32) {
        self.delta_x += x - self.mouse_x;
        self.delta_y += y - self.mouse_y;
        self.mouse_x = x;
        self.mouse_y = y;
    }

    pub fn is_key_down(&self, vk: u32) -> bool {
        if vk < 256 {
            self.keys_down[vk as usize]
        } else {
            false
        }
    }

    pub fn consume_mouse_delta(&mut self) -> (i32, i32) {
        let dx = self.delta_x;
        let dy = self.delta_y;
        self.delta_x = 0;
        self.delta_y = 0;
        (dx, dy)
    }

    pub fn consume_scroll_delta(&mut self) -> i32 {
        let d = self.scroll_delta;
        self.scroll_delta = 0;
        d
    }
}

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}
