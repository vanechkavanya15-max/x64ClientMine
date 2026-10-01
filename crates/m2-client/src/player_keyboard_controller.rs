use crate::win32_keyboard_hooks::{ActionType, ActionBinding, KeyStateTable};
use crate::send_movement_packets_v2::m2_send_move;
use crate::send_combat_packets_v2::m2_send_attack;

pub struct PlayerKeyboardController {
    pub is_moving: bool,
    pub was_moving: bool,
    pub target_rotation: f32,
    pub current_rotation: f32,
    pub turn_speed: f32, // degrees per second
    
    // Key states
    pub is_up: bool,
    pub is_down: bool,
    pub is_left: bool,
    pub is_right: bool,
    pub is_attacking: bool,
    
    // Attack interval
    pub attack_interval_ms: f32,
    pub time_since_last_attack_ms: f32,
    pub target_vid: u32,
    
    // Move sync
    pub time_since_last_move_sync_ms: f32,
    pub move_sync_interval_ms: f32,
}

impl PlayerKeyboardController {
    pub fn new() -> Self {
        Self {
            is_moving: false,
            was_moving: false,
            target_rotation: 0.0,
            current_rotation: 0.0,
            turn_speed: 720.0, // default turn speed
            is_up: false,
            is_down: false,
            is_left: false,
            is_right: false,
            is_attacking: false,
            attack_interval_ms: 300.0,
            time_since_last_attack_ms: 0.0,
            target_vid: 0,
            time_since_last_move_sync_ms: 0.0,
            move_sync_interval_ms: 500.0,
        }
    }

    pub fn get_degree_from_direction(is_up: bool, is_down: bool, is_left: bool, is_right: bool) -> f32 {
        if is_up {
            if is_left {
                return 45.0;
            } else if is_right {
                return -45.0;
            }
            return 0.0;
        } else if is_down {
            if is_left {
                return 135.0;
            } else if is_right {
                return -135.0;
            }
            return 180.0;
        } else {
            if is_left {
                return 90.0;
            } else if is_right {
                return -90.0;
            }
        }
        0.0
    }

    pub fn update_keys(&mut self, key_state: &KeyStateTable, bindings: &[ActionBinding]) {
        self.is_up = false;
        self.is_down = false;
        self.is_left = false;
        self.is_right = false;
        self.is_attacking = false;

        for binding in bindings {
            if key_state.is_pressed(binding.primary_key) {
                match binding.action_type {
                    ActionType::MoveForward => self.is_up = true,
                    ActionType::MoveBackward => self.is_down = true,
                    ActionType::MoveLeft => self.is_left = true,
                    ActionType::MoveRight => self.is_right = true,
                    ActionType::Attack => self.is_attacking = true,
                    _ => {}
                }
            }
        }
    }

    pub fn update_movement(&mut self, camera_yaw: f32) {
        let is_any = self.is_up || self.is_down || self.is_left || self.is_right;
        
        if is_any {
            self.is_moving = true;
            let dir_rot = Self::get_degree_from_direction(self.is_up, self.is_down, self.is_left, self.is_right);
            
            let mut target = camera_yaw + dir_rot;
            target = target % 360.0;
            if target < 0.0 {
                target += 360.0;
            }
            
            self.target_rotation = target;
        } else {
            self.is_moving = false;
        }
    }

    pub fn update_interpolation(&mut self, delta_time_sec: f32) {
        if self.current_rotation == self.target_rotation {
            return;
        }
        
        let mut diff = self.target_rotation - self.current_rotation;
        
        // Shortest path
        if diff > 180.0 {
            diff -= 360.0;
        } else if diff < -180.0 {
            diff += 360.0;
        }
        
        let step = self.turn_speed * delta_time_sec;
        
        if diff.abs() <= step {
            self.current_rotation = self.target_rotation;
        } else {
            self.current_rotation += step * diff.signum();
            self.current_rotation = self.current_rotation % 360.0;
            if self.current_rotation < 0.0 {
                self.current_rotation += 360.0;
            }
        }
    }

    pub fn update_attack(&mut self, delta_time_sec: f32, mut packet_buf: Option<&mut [u8]>) -> usize {
        let mut bytes_written = 0;
        if self.is_attacking {
            self.time_since_last_attack_ms += delta_time_sec * 1000.0;
            if self.time_since_last_attack_ms >= self.attack_interval_ms {
                self.time_since_last_attack_ms = 0.0;
                
                if let Some(buf) = packet_buf.as_deref_mut() {
                    bytes_written = m2_send_attack(0, self.target_vid, buf.as_mut_ptr());
                }
            }
        } else {
            self.time_since_last_attack_ms = self.attack_interval_ms; // trigger immediately next time
        }
        bytes_written
    }

    pub fn update_packet_dispatch(
        &mut self,
        delta_time_sec: f32,
        current_x: i32,
        current_y: i32,
        server_time: u32,
        mut packet_buf: Option<&mut [u8]>,
    ) -> usize {
        let mut bytes_written = 0;
        
        // FUNC_WAIT = 0, FUNC_MOVE = 1
        let func_wait: u8 = 0;
        let func_move: u8 = 1;

        if self.is_moving {
            self.time_since_last_move_sync_ms += delta_time_sec * 1000.0;
            
            if !self.was_moving {
                // Just started moving
                self.time_since_last_move_sync_ms = 0.0;
                if let Some(buf) = packet_buf.as_deref_mut() {
                    bytes_written = m2_send_move(
                        func_move,
                        current_x,
                        current_y,
                        self.current_rotation,
                        server_time,
                        buf.as_mut_ptr(),
                    );
                }
            } else if self.time_since_last_move_sync_ms >= self.move_sync_interval_ms {
                // Continue moving, sync required
                self.time_since_last_move_sync_ms -= self.move_sync_interval_ms;
                if let Some(buf) = packet_buf.as_deref_mut() {
                    bytes_written = m2_send_move(
                        func_move,
                        current_x,
                        current_y,
                        self.current_rotation,
                        server_time,
                        buf.as_mut_ptr(),
                    );
                }
            }
        } else if self.was_moving {
            // Just stopped moving
            if let Some(buf) = packet_buf.as_deref_mut() {
                bytes_written = m2_send_move(
                    func_wait,
                    current_x,
                    current_y,
                    self.current_rotation,
                    server_time,
                    buf.as_mut_ptr(),
                );
            }
        }

        self.was_moving = self.is_moving;
        
        bytes_written
    }
}
