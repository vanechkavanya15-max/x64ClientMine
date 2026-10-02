use crate::recv_points::GlobalGameState;
use crate::stamina_system::StaminaSystem;
use crate::ui_window_minimap::UiWindowMinimap;
use crate::ui_window_quickslot::{QuickSlotWindow, TQuickSlot};
use crate::minimap_compass_mapper_v2::{MinimapCompassMapperV2, MinimapScreenPos, m2_minimap_world_to_screen, PinType};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlayerStatus {
    pub hp_percent: f32,
    pub sp_percent: f32,
    pub stamina_percent: f32,
    pub hp: i32,
    pub max_hp: i32,
    pub sp: i32,
    pub max_sp: i32,
    pub stamina: u32,
    pub max_stamina: u32,
}

impl PlayerStatus {
    pub fn new() -> Self {
        Self {
            hp_percent: 0.0,
            sp_percent: 0.0,
            stamina_percent: 0.0,
            hp: 0,
            max_hp: 0,
            sp: 0,
            max_sp: 0,
            stamina: 0,
            max_stamina: 0,
        }
    }

    pub fn update(&mut self, state: &GlobalGameState, stamina_sys: &StaminaSystem) {
        self.hp = state.hp;
        self.max_hp = state.max_hp;
        self.sp = state.sp;
        self.max_sp = state.max_sp;
        
        self.stamina = stamina_sys.current_stamina as u32;
        self.max_stamina = stamina_sys.max_stamina as u32;

        if self.max_hp > 0 {
            self.hp_percent = (self.hp as f32 / self.max_hp as f32).clamp(0.0, 1.0);
        } else {
            self.hp_percent = 0.0;
        }

        if self.max_sp > 0 {
            self.sp_percent = (self.sp as f32 / self.max_sp as f32).clamp(0.0, 1.0);
        } else {
            self.sp_percent = 0.0;
        }

        if self.max_stamina > 0 {
            self.stamina_percent = (self.stamina as f32 / self.max_stamina as f32).clamp(0.0, 1.0);
        } else {
            self.stamina_percent = 0.0;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExpBar {
    pub exp: i32,
    pub next_exp: i32,
    pub globes: [f32; 4],
}

impl ExpBar {
    pub fn new() -> Self {
        Self {
            exp: 0,
            next_exp: 0,
            globes: [0.0; 4],
        }
    }

    pub fn update(&mut self, state: &GlobalGameState) {
        self.exp = state.exp;
        self.next_exp = state.next_exp;

        if self.next_exp <= 0 {
            self.globes = [0.0; 4];
            return;
        }

        let total_percent = (self.exp as f32 / self.next_exp as f32).clamp(0.0, 1.0) * 100.0;

        for i in 0..4 {
            let threshold_min = i as f32 * 25.0;
            let threshold_max = (i + 1) as f32 * 25.0;

            if total_percent >= threshold_max {
                self.globes[i] = 100.0;
            } else if total_percent > threshold_min {
                self.globes[i] = ((total_percent - threshold_min) / 25.0) * 100.0;
            } else {
                self.globes[i] = 0.0;
            }
        }
    }
}

pub struct TaskBar {
    pub quickslot_window: QuickSlotWindow,
    // Store cooldowns by global index: Option<(start_time, duration)>
    pub cooldowns: [Option<(f32, f32)>; crate::ui_window_quickslot::QUICKSLOT_MAX_NUM_SERVER],
}

impl TaskBar {
    pub fn new() -> Self {
        Self {
            quickslot_window: QuickSlotWindow::new(),
            cooldowns: [None; crate::ui_window_quickslot::QUICKSLOT_MAX_NUM_SERVER],
        }
    }

    pub fn set_cooldown(&mut self, local_idx: u8, start_time: f32, duration: f32) {
        let global_idx = self.quickslot_window.get_global_index(local_idx);
        if global_idx < self.cooldowns.len() {
            self.cooldowns[global_idx] = Some((start_time, duration));
        }
    }

    pub fn get_cooldown_percent(&self, local_idx: u8, current_time: f32) -> f32 {
        let global_idx = self.quickslot_window.get_global_index(local_idx);
        if global_idx >= self.cooldowns.len() {
            return 0.0;
        }

        if let Some((start_time, duration)) = self.cooldowns[global_idx] {
            if duration > 0.0 && current_time >= start_time {
                let elapsed = current_time - start_time;
                if elapsed >= duration {
                    return 0.0; // Cooldown finished
                } else {
                    return 1.0 - (elapsed / duration).clamp(0.0, 1.0);
                }
            }
        }

        0.0
    }

    pub fn use_slot(&mut self, local_idx: u8) -> Option<TQuickSlot> {
        self.quickslot_window.use_slot(local_idx)
    }

    pub fn on_key_pressed(&mut self, key_code: u32, is_shift_pressed: bool) -> Option<TQuickSlot> {
        self.quickslot_window.on_key_pressed(key_code, is_shift_pressed)
    }
}

pub struct MinimapHud {
    pub window: UiWindowMinimap,
    pub compass_rotation: f32,
    pub radius: f32,
}

impl MinimapHud {
    pub fn new(root_handle: u32, radius: f32) -> Self {
        Self {
            window: UiWindowMinimap::new(root_handle),
            compass_rotation: 0.0,
            radius,
        }
    }

    pub fn update_camera_rotation(&mut self, camera_rotation: f32) {
        self.compass_rotation = MinimapCompassMapperV2::calculate_compass_rotation(camera_rotation);
        self.window.state.camera_rotation = camera_rotation;
        let _ = self.window.update_state();
    }
    
    pub fn get_pins_with_screen_pos(&self) -> Vec<(u32, MinimapScreenPos, u32)> {
        let effective_radius = self.radius * self.window.state.zoom_scale;
        
        self.window.state.pins.iter().map(|pin| {
            let screen_pos = m2_minimap_world_to_screen(
                self.window.state.player_x,
                self.window.state.player_y,
                pin.x,
                pin.y,
                effective_radius
            );
            (pin.id, screen_pos, pin.pin_type.get_color())
        }).collect()
    }
    
    pub fn add_pin_player(&mut self, id: u32, x: f32, y: f32) {
        let _ = self.window.add_pin(id, x, y, PinType::Player);
    }
    
    pub fn add_pin_monster(&mut self, id: u32, x: f32, y: f32) {
        let _ = self.window.add_pin(id, x, y, PinType::Monster);
    }

    pub fn add_pin_npc(&mut self, id: u32, x: f32, y: f32) {
        let _ = self.window.add_pin(id, x, y, PinType::Metin); // NPC / Yellow
    }
    
    pub fn zoom_in(&mut self) -> Result<(), &'static str> {
        self.window.handle_zoom_in()
    }

    pub fn zoom_out(&mut self) -> Result<(), &'static str> {
        self.window.handle_zoom_out()
    }
}
