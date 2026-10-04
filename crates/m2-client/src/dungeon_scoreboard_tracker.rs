use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::{Mutex, OnceLock};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DungeonState {
    Waiting = 0,
    InProgress = 1,
    FloorCleared = 2,
    Failure = 3,
}

#[derive(Debug, Clone)]
pub struct DungeonScoreboardTracker {
    pub current_floor: u8,
    pub objective_text: String,
    pub objective_count: u32,
    pub objective_total: u32,
    pub time_left: u32,
    pub state: DungeonState,
}

impl Default for DungeonScoreboardTracker {
    fn default() -> Self {
        Self {
            current_floor: 0,
            objective_text: String::new(),
            objective_count: 0,
            objective_total: 0,
            time_left: 0,
            state: DungeonState::Waiting,
        }
    }
}

impl DungeonScoreboardTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_floor(&mut self, floor: u8) {
        self.current_floor = floor;
    }

    pub fn set_objective(&mut self, text: String, count: u32, total: u32) {
        self.objective_text = text;
        self.objective_count = count;
        self.objective_total = total;
    }

    pub fn set_time_left(&mut self, seconds: u32) {
        self.time_left = seconds;
    }

    pub fn set_state(&mut self, state: DungeonState) {
        self.state = state;
    }

    pub fn tick(&mut self, delta_seconds: u32) {
        if self.state != DungeonState::InProgress {
            return;
        }

        if self.time_left > delta_seconds {
            self.time_left -= delta_seconds;
        } else {
            self.time_left = 0;
            self.state = DungeonState::Failure;
        }
    }
}

static TRACKER: OnceLock<Mutex<DungeonScoreboardTracker>> = OnceLock::new();

pub fn get_tracker() -> &'static Mutex<DungeonScoreboardTracker> {
    TRACKER.get_or_init(|| Mutex::new(DungeonScoreboardTracker::new()))
}

#[no_mangle]
pub extern "C" fn m2_dungeon_tracker_set_floor(floor: u8) {
    if let Ok(mut tracker) = get_tracker().lock() {
        tracker.set_floor(floor);
    }
}

#[no_mangle]
pub extern "C" fn m2_dungeon_tracker_set_objective(
    text: *const c_char,
    count: u32,
    total: u32,
) {
    if text.is_null() {
        return;
    }
    let text_str = unsafe { CStr::from_ptr(text) }.to_string_lossy().into_owned();
    if let Ok(mut tracker) = get_tracker().lock() {
        tracker.set_objective(text_str, count, total);
    }
}

#[no_mangle]
pub extern "C" fn m2_dungeon_tracker_set_time_left(seconds: u32) {
    if let Ok(mut tracker) = get_tracker().lock() {
        tracker.set_time_left(seconds);
    }
}

#[no_mangle]
pub extern "C" fn m2_dungeon_tracker_set_state(state: u8) {
    let state = match state {
        0 => DungeonState::Waiting,
        1 => DungeonState::InProgress,
        2 => DungeonState::FloorCleared,
        3 => DungeonState::Failure,
        _ => DungeonState::Waiting, // default fallback
    };
    if let Ok(mut tracker) = get_tracker().lock() {
        tracker.set_state(state);
    }
}

#[no_mangle]
pub extern "C" fn m2_dungeon_tracker_tick(delta_seconds: u32) {
    if let Ok(mut tracker) = get_tracker().lock() {
        tracker.tick(delta_seconds);
    }
}
