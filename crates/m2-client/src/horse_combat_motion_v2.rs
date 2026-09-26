#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HorseComboState {
    Idle,
    Attacking,
    PreInputWindow,
    ComboWindowOpen,
    ComboExpired,
}

#[derive(Debug, Clone, Default)]
pub struct HorseComboTimeData {
    pub combo_window_start: f32,
    pub next_combo_time: f32,
    pub combo_window_end: f32,
    pub motion_duration: f32,
    pub has_time_data: bool,
}

#[derive(Debug, Clone)]
pub struct HorseCombatMotionV2 {
    pub cur_combo_index: u8,
    pub max_combo_count: u8,
    pub is_pre_input: bool,
    pub elapsed_time: f32,
    pub state: HorseComboState,
    pub time_data: Option<HorseComboTimeData>,
}

impl HorseCombatMotionV2 {
    pub fn new() -> Self {
        Self {
            cur_combo_index: 0,
            max_combo_count: 5,
            is_pre_input: false,
            elapsed_time: 0.0,
            state: HorseComboState::Idle,
            time_data: None,
        }
    }

    pub fn set_time_data(&mut self, time_data: Option<HorseComboTimeData>) {
        self.time_data = time_data;
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.cur_combo_index == 0 {
            self.state = HorseComboState::Idle;
            return;
        }

        self.elapsed_time += delta_time;

        if let Some(ref data) = self.time_data {
            if data.has_time_data {
                if self.elapsed_time > data.combo_window_end {
                    self.state = HorseComboState::ComboExpired;
                    self.clear_combo();
                } else if self.elapsed_time > data.next_combo_time {
                    self.state = HorseComboState::ComboWindowOpen;
                    if self.is_pre_input {
                        self.run_next_combo();
                        self.is_pre_input = false;
                    }
                } else if self.elapsed_time > data.combo_window_start {
                    self.state = HorseComboState::PreInputWindow;
                } else {
                    self.state = HorseComboState::Attacking;
                }
            } else {
                if self.elapsed_time > data.motion_duration * 0.9 {
                    self.state = HorseComboState::ComboWindowOpen;
                } else {
                    self.state = HorseComboState::Attacking;
                }
            }
        }
    }

    pub fn input_attack(&mut self) -> bool {
        if self.is_pre_input {
            return false;
        }

        if self.cur_combo_index == 0 {
            self.run_next_combo();
            return true;
        }

        if let Some(ref data) = self.time_data {
            if data.has_time_data {
                if self.elapsed_time > data.combo_window_end {
                    return false;
                }

                if self.elapsed_time > data.next_combo_time {
                    self.run_next_combo();
                    return true;
                } else if self.elapsed_time > data.combo_window_start {
                    self.is_pre_input = true;
                    return false;
                }
            } else {
                if self.elapsed_time > data.motion_duration * 0.9 {
                    self.run_next_combo();
                    return true;
                }
            }
        }

        false
    }

    pub fn run_next_combo(&mut self) {
        if self.cur_combo_index >= self.max_combo_count {
            self.cur_combo_index = 1;
        } else {
            self.cur_combo_index += 1;
        }
        
        self.elapsed_time = 0.0;
        self.state = HorseComboState::Attacking;
        self.is_pre_input = false;
    }

    pub fn clear_combo(&mut self) {
        self.cur_combo_index = 0;
        self.is_pre_input = false;
        self.elapsed_time = 0.0;
        self.state = HorseComboState::Idle;
        self.time_data = None;
    }
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn m2_horse_combat_get_hit_sphere(combo_idx: u8) -> (f32, f32) {
    match combo_idx {
        1 => (120.0, 150.0),
        2 => (120.0, 150.0),
        3 => (130.0, 160.0),
        4 => (130.0, 160.0),
        5 => (150.0, 200.0),
        _ => (100.0, 100.0),
    }
}
