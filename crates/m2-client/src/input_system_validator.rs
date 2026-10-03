use crate::full_input_win32_15::{Win32InputSystem, InputEvent};

pub struct InputValidator {
    pub total_processed: usize,
    pub last_key_code: Option<u32>,
    pub total_mouse_moves: usize,
    pub pitch_accum: f32,
    pub yaw_accum: f32,
}

impl InputValidator {
    pub fn new() -> Self {
        Self {
            total_processed: 0,
            last_key_code: None,
            total_mouse_moves: 0,
            pitch_accum: 0.0,
            yaw_accum: 0.0,
        }
    }

    pub fn validate_and_consume(&mut self, system: &mut Win32InputSystem) {
        while let Some(event) = system.get_next_event() {
            self.total_processed += 1;
            match event {
                InputEvent::KeyDown { key_code, .. } => {
                    self.last_key_code = Some(key_code);
                }
                InputEvent::KeyUp { key_code, .. } => {
                    if self.last_key_code == Some(key_code) {
                        self.last_key_code = None;
                    }
                }
                InputEvent::MouseMove { dx, dy } => {
                    self.total_mouse_moves += 1;
                    self.pitch_accum += dy as f32;
                    self.yaw_accum += dx as f32;
                }
                InputEvent::MouseWheel { .. } => {}
                InputEvent::MouseButton { .. } => {}
            }
        }
    }
}

impl Default for InputValidator {
    fn default() -> Self {
        Self::new()
    }
}
