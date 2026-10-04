use crate::actor_combo_window_timing::{ActorComboState, RaceMotionData};

pub struct EngineComboInputController {
    pub combo_state: ActorComboState,
    pub is_atk_key_pressed: bool,
    pub next_combo_triggered: bool,
}

impl EngineComboInputController {
    pub fn new() -> Self {
        Self {
            combo_state: ActorComboState::new(),
            is_atk_key_pressed: false,
            next_combo_triggered: false,
        }
    }

    pub fn set_attack_key_state(&mut self, is_press: bool) {
        self.is_atk_key_pressed = is_press;
    }

    pub fn trigger_single_attack(&mut self) -> bool {
        self.combo_state.input_combo_attack_command()
    }

    pub fn set_motion_data(&mut self, motion_data: RaceMotionData) {
        self.combo_state.cur_race_motion_data = Some(motion_data);
        self.combo_state.attacking_elapsed_time = 0.0;
        self.next_combo_triggered = false;
    }

    pub fn update_frame(&mut self, delta_time: f32) -> bool {
        if self.combo_state.cur_combo_index == 0 {
            if self.is_atk_key_pressed {
                if self.combo_state.input_combo_attack_command() {
                    self.combo_state.attacking_elapsed_time = 0.0;
                    self.next_combo_triggered = true;
                    return true;
                }
            }
            return false;
        }

        self.combo_state.attacking_elapsed_time += delta_time;

        if self.is_atk_key_pressed {
            self.combo_state.input_combo_attack_command();
        }

        let old_index = self.combo_state.cur_combo_index;
        self.combo_state.combo_process();
        
        if self.combo_state.cur_combo_index > old_index {
            self.combo_state.attacking_elapsed_time = 0.0;
            self.next_combo_triggered = true;
            return true;
        }
        
        self.next_combo_triggered = false;
        false
    }
    
    pub fn clear_combo(&mut self) {
        self.combo_state.clear_combo();
        self.is_atk_key_pressed = false;
        self.next_combo_triggered = false;
    }
}
