#[derive(Debug, Clone, Copy)]
pub struct ComboInputData {
    pub input_start_time: f32,
    pub next_combo_time: f32,
    pub input_end_time: f32,
}

#[derive(Debug, Clone)]
pub struct RaceMotionData {
    pub combo_input_data: Option<ComboInputData>,
    pub motion_duration: f32,
}

impl RaceMotionData {
    pub fn is_combo_input_time_data(&self) -> bool {
        self.combo_input_data.is_some()
    }

    pub fn get_combo_input_start_time(&self) -> f32 {
        self.combo_input_data.map(|d| d.input_start_time).unwrap_or(0.0)
    }

    pub fn get_next_combo_time(&self) -> f32 {
        self.combo_input_data.map(|d| d.next_combo_time).unwrap_or(0.0)
    }

    pub fn get_combo_input_end_time(&self) -> f32 {
        self.combo_input_data.map(|d| d.input_end_time).unwrap_or(0.0)
    }
    
    pub fn get_motion_duration(&self) -> f32 {
        self.motion_duration
    }
}

pub struct ActorComboState {
    pub cur_combo_index: u32,
    pub is_pre_input: bool,
    pub is_next_pre_input: bool,
    pub is_bow_mode: bool,
    pub is_using_skill: bool,
    pub attacking_elapsed_time: f32,
    pub cur_race_motion_data: Option<RaceMotionData>,
}

impl ActorComboState {
    pub fn new() -> Self {
        Self {
            cur_combo_index: 0,
            is_pre_input: false,
            is_next_pre_input: false,
            is_bow_mode: false,
            is_using_skill: false,
            attacking_elapsed_time: 0.0,
            cur_race_motion_data: None,
        }
    }
    
    pub fn get_attacking_elapsed_time(&self) -> f32 {
        self.attacking_elapsed_time
    }

    pub fn run_next_combo(&mut self) {
        self.cur_combo_index += 1;
    }

    pub fn input_combo_attack_command(&mut self) -> bool {
        if self.is_pre_input {
            return false;
        }

        if self.cur_combo_index == 0 {
            self.run_next_combo();
            return true;
        } else if let Some(ref motion_data) = self.cur_race_motion_data {
            if motion_data.is_combo_input_time_data() {
                let elapsed_time = self.get_attacking_elapsed_time();
                
                if elapsed_time > motion_data.get_combo_input_end_time() {
                    if self.is_bow_mode {
                        self.is_next_pre_input = true;
                    }
                    return false;
                }
                
                if elapsed_time > motion_data.get_next_combo_time() {
                    self.run_next_combo();
                    return true;
                } else if elapsed_time > motion_data.get_combo_input_start_time() {
                    self.is_pre_input = true;
                    return false;
                }
            } else {
                let elapsed_time = self.get_attacking_elapsed_time();
                if elapsed_time > motion_data.get_motion_duration() * 0.9 {
                    self.run_next_combo();
                    return true;
                }
            }
        }
        
        false
    }
    
    pub fn clear_combo(&mut self) {
        self.cur_combo_index = 0;
        self.is_pre_input = false;
        self.cur_race_motion_data = None;
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
                    if elapsed_time > motion_data.get_next_combo_time() {
                        self.run_next_combo();
                        self.is_pre_input = false;
                        return;
                    }
                }
            } else {
                if let Some(ref motion_data) = self.cur_race_motion_data {
                    if motion_data.is_combo_input_time_data() {
                        if elapsed_time > motion_data.get_combo_input_end_time() {
                            self.clear_combo();
                            return;
                        }
                    } else {
                        if elapsed_time > motion_data.get_motion_duration() {
                            self.clear_combo();
                            return;
                        }
                    }
                }
            }
        } else {
            self.is_pre_input = false;
            
            if !self.is_using_skill && self.is_next_pre_input {
                self.run_next_combo();
                self.is_next_pre_input = false;
            }
        }
    }
}
