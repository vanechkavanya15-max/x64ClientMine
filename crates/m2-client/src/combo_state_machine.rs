#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComboState {
    Idle,
    Attacking,
    PreInputWindow,
    ComboWindowOpen,
    ComboExpired,
}

#[derive(Debug, Clone, Default)]
pub struct ComboTimeData {
    pub combo_window_start: f32,
    pub next_combo_time: f32,
    pub combo_window_end: f32,
    pub motion_duration: f32,
    pub has_time_data: bool,
}

#[derive(Debug, Clone)]
pub struct ComboStateMachine {
    pub cur_combo_index: u32,
    pub max_combo_count: u32,
    pub is_pre_input: bool,
    pub is_mounting_horse: bool,
    pub elapsed_time: f32,
    pub state: ComboState,
    pub time_data: Option<ComboTimeData>,
}

impl ComboStateMachine {
    pub fn new(max_combo_count: u32) -> Self {
        Self {
            cur_combo_index: 0,
            max_combo_count,
            is_pre_input: false,
            is_mounting_horse: false,
            elapsed_time: 0.0,
            state: ComboState::Idle,
            time_data: None,
        }
    }

    pub fn set_mounting_horse(&mut self, is_mounting: bool) {
        self.is_mounting_horse = is_mounting;
    }

    pub fn set_time_data(&mut self, time_data: Option<ComboTimeData>) {
        self.time_data = time_data;
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.cur_combo_index == 0 {
            self.state = ComboState::Idle;
            return;
        }

        self.elapsed_time += delta_time;

        if let Some(ref data) = self.time_data {
            if data.has_time_data {
                if self.elapsed_time > data.combo_window_end {
                    self.state = ComboState::ComboExpired;
                    self.clear_combo();
                } else if self.elapsed_time > data.next_combo_time {
                    self.state = ComboState::ComboWindowOpen;
                    if self.is_pre_input {
                        self.run_next_combo();
                        self.is_pre_input = false;
                    }
                } else if self.elapsed_time > data.combo_window_start {
                    self.state = ComboState::PreInputWindow;
                } else {
                    self.state = ComboState::Attacking;
                }
            } else {
                if self.elapsed_time > data.motion_duration * 0.9 {
                    self.state = ComboState::ComboWindowOpen;
                } else {
                    self.state = ComboState::Attacking;
                }
            }
        }
    }

    pub fn input_attack(&mut self) -> bool {
        if self.is_pre_input {
            return false;
        }

        // Postac piesza po osiagnieciu max combo nie moze wyprowadzic kolejnego ataku
        // dopoki combo nie wygasnie lub animacja nie powroci do stanu oczekiwania
        if !self.is_mounting_horse && self.cur_combo_index >= self.max_combo_count {
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

    fn run_next_combo(&mut self) {
        if self.is_mounting_horse && self.cur_combo_index >= self.max_combo_count {
            self.cur_combo_index = 1;
        } else {
            self.cur_combo_index += 1;
        }
        
        self.elapsed_time = 0.0;
        self.state = ComboState::Attacking;
        self.is_pre_input = false;

        if self.cur_combo_index >= self.max_combo_count {
            self.on_end_combo();
        }
    }

    pub fn on_end_combo(&mut self) {
        if self.is_mounting_horse {
            // Na koniu combo po osiagnieciu limitu zapetla sie do 1
            // (w C++: if (__IsMountingHorse()) m_dwcurComboIndex = 1;)
        } else {
            // Na pieszo index zostaje na max_combo_count az do zakonczenia animacji
        }
    }

    pub fn clear_combo(&mut self) {
        self.cur_combo_index = 0;
        self.is_pre_input = false;
        self.elapsed_time = 0.0;
        self.state = ComboState::Idle;
        self.time_data = None;
    }
}
