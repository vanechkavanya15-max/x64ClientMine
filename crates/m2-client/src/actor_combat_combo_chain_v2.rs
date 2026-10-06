#[derive(Debug, Clone, Copy)]
pub struct RaceMotionData {
    pub has_combo_input_time_data: bool,
    pub combo_input_end_time: f32,
    pub next_combo_time: f32,
    pub combo_input_start_time: f32,
    pub motion_duration: f32,
}

pub struct ActorCombatComboChainV2 {
    pub cur_combo_index: u32,
    pub is_pre_input: bool,
    pub is_next_pre_input: bool,
    pub is_bow_mode: bool,
    pub is_using_skill: bool,
    pub is_mounting_horse: bool,
    pub max_combo_count: u32,
    pub cur_race_motion_data: Option<RaceMotionData>,
    pub attacking_elapsed_time: f32,
}

impl ActorCombatComboChainV2 {
    pub fn new() -> Self {
        Self {
            cur_combo_index: 0,
            is_pre_input: false,
            is_next_pre_input: false,
            is_bow_mode: false,
            is_using_skill: false,
            is_mounting_horse: false,
            max_combo_count: 0,
            cur_race_motion_data: None,
            attacking_elapsed_time: 0.0,
        }
    }
}

impl ActorCombatComboChainV2 {
    pub fn get_attacking_elapsed_time(&self) -> f32 {
        self.attacking_elapsed_time
    }

    pub fn input_combo_attack_command(&mut self) -> bool {
        if self.is_pre_input {
            return false;
        }

        if self.cur_combo_index == 0 {
            self.run_next_combo();
            return true;
        } else if let Some(ref motion_data) = self.cur_race_motion_data {
            if motion_data.has_combo_input_time_data {
                let elapsed_time = self.get_attacking_elapsed_time();

                if elapsed_time > motion_data.combo_input_end_time {
                    if self.is_bow_mode {
                        self.is_next_pre_input = true;
                    }
                    return false;
                }

                if elapsed_time > motion_data.next_combo_time {
                    self.run_next_combo();
                    return true;
                } else if elapsed_time > motion_data.combo_input_start_time {
                    self.is_pre_input = true;
                    return false;
                }
            } else {
                let elapsed_time = self.get_attacking_elapsed_time();
                if elapsed_time > motion_data.motion_duration * 0.9 {
                    self.run_next_combo();
                    return true;
                }
            }
        }
        
        false
    }

    pub fn combo_process(&mut self) {
        if self.cur_combo_index != 0 {
            if self.cur_race_motion_data.is_none() {
                self.clear_combo();
                return;
            }

            let elapsed_time = self.get_attacking_elapsed_time();

            if self.is_pre_input {
                if let Some(ref motion_data) = self.cur_race_motion_data {
                    if elapsed_time > motion_data.next_combo_time {
                        self.run_next_combo();
                        self.is_pre_input = false;
                        return;
                    }
                }
            }
        } else {
            self.is_pre_input = false;

            if !self.is_using_skill {
                if self.is_next_pre_input {
                    self.run_next_combo();
                    self.is_next_pre_input = false;
                }
            }
        }
    }

    pub fn run_next_combo(&mut self) {
        self.cur_combo_index += 1;
        
        if self.cur_combo_index >= self.max_combo_count {
            self.on_end_combo();
        }
    }

    pub fn on_end_combo(&mut self) {
        if self.is_mounting_horse {
            self.cur_combo_index = 1;
        }
    }

    pub fn clear_combo(&mut self) {
        self.cur_combo_index = 0;
        self.is_pre_input = false;
        self.cur_race_motion_data = None;
    }
}
