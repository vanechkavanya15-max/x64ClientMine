use std::collections::VecDeque;

use crate::win32_keyboard_hooks;
use crate::win32_mouse_raw;

#[derive(Debug, Clone, PartialEq)]
pub enum InputEvent {
    KeyDown { key_code: u32, timestamp_ms: u64 },
    KeyUp { key_code: u32, timestamp_ms: u64 },
    MouseMove { dx: i32, dy: i32 },
    MouseWheel { delta: i16 },
    MouseButton { button: u16, is_down: bool },
}

pub struct Win32InputSystem {
    pub keyboard_state: win32_keyboard_hooks::KeyStateTable,
    pub mouse_config: win32_mouse_raw::MouseSensitivityConfig,
    pub event_queue: VecDeque<InputEvent>,
    
    // Virtual Camera
    pub pitch: f32,
    pub yaw: f32,
    
    // Focus tracking
    pub is_focused: bool,
}

impl Win32InputSystem {
    pub fn new() -> Self {
        Self {
            keyboard_state: win32_keyboard_hooks::KeyStateTable::new(),
            mouse_config: win32_mouse_raw::MouseSensitivityConfig::default(),
            event_queue: VecDeque::new(),
            pitch: 0.0,
            yaw: 0.0,
            is_focused: true,
        }
    }

    pub fn set_focus(&mut self, focus: bool) {
        self.is_focused = focus;
        if !focus {
            self.keyboard_state.on_kill_focus();
        }
    }

    pub fn dispatch_key_down(&mut self, key_code: u32, timestamp_ms: u64) {
        if !self.is_focused {
            return;
        }

        if let Some(event) = self.keyboard_state.on_key_down(key_code, timestamp_ms) {
            self.event_queue.push_back(InputEvent::KeyDown {
                key_code: event.key_code,
                timestamp_ms: event.timestamp_ms,
            });
        }
    }

    pub fn dispatch_key_up(&mut self, key_code: u32, timestamp_ms: u64) {
        if !self.is_focused {
            return;
        }

        if let Some(event) = self.keyboard_state.on_key_up(key_code, timestamp_ms) {
            self.event_queue.push_back(InputEvent::KeyUp {
                key_code: event.key_code,
                timestamp_ms: event.timestamp_ms,
            });
        }
    }

    pub fn dispatch_raw_mouse(&mut self, raw_data: &[u8]) {
        if !self.is_focused {
            return;
        }

        if let Some(delta) = win32_mouse_raw::parse_raw_input_mouse(raw_data) {
            if delta.dx != 0 || delta.dy != 0 {
                self.event_queue.push_back(InputEvent::MouseMove {
                    dx: delta.dx,
                    dy: delta.dy,
                });
                
                win32_mouse_raw::apply_camera_rotation_delta(
                    delta,
                    &self.mouse_config,
                    &mut self.pitch,
                    &mut self.yaw,
                );
            }

            if delta.wheel_delta != 0 {
                self.event_queue.push_back(InputEvent::MouseWheel {
                    delta: delta.wheel_delta,
                });
            }

            // RI_MOUSE_LEFT_BUTTON_DOWN = 0x0001
            if (delta.button_flags & 0x0001) != 0 {
                self.event_queue.push_back(InputEvent::MouseButton {
                    button: 0,
                    is_down: true,
                });
            }
            // RI_MOUSE_LEFT_BUTTON_UP = 0x0002
            if (delta.button_flags & 0x0002) != 0 {
                self.event_queue.push_back(InputEvent::MouseButton {
                    button: 0,
                    is_down: false,
                });
            }
            
            // RI_MOUSE_RIGHT_BUTTON_DOWN = 0x0004
            if (delta.button_flags & 0x0004) != 0 {
                self.event_queue.push_back(InputEvent::MouseButton {
                    button: 1,
                    is_down: true,
                });
            }
            // RI_MOUSE_RIGHT_BUTTON_UP = 0x0008
            if (delta.button_flags & 0x0008) != 0 {
                self.event_queue.push_back(InputEvent::MouseButton {
                    button: 1,
                    is_down: false,
                });
            }
        }
    }

    pub fn get_next_event(&mut self) -> Option<InputEvent> {
        self.event_queue.pop_front()
    }
    
    pub fn clear_events(&mut self) {
        self.event_queue.clear();
    }
}

impl Default for Win32InputSystem {
    fn default() -> Self {
        Self::new()
    }
}
