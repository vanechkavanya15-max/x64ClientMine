use std::time::{SystemTime, UNIX_EPOCH};

use crate::ui_loading_progress_bar::{LoadingProgressBar, LoadingPhase};

pub struct LoadingScreen {
    pub progress_bar: LoadingProgressBar,
    pub map_name: String,
    pub background_image: String,
    pub hints: Vec<String>,
    pub current_hint: String,
}

impl Default for LoadingScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl LoadingScreen {
    pub fn new() -> Self {
        Self {
            progress_bar: LoadingProgressBar::new(),
            map_name: String::new(),
            background_image: "default_loading.jpg".to_string(),
            hints: Vec::new(),
            current_hint: String::new(),
        }
    }

    pub fn set_map(&mut self, map_name: &str) {
        self.map_name = map_name.to_string();
        self.background_image = format!("{}_loading.jpg", map_name);
        self.update_hint();
    }

    pub fn load_hints(&mut self, hints: Vec<String>) {
        self.hints = hints;
        self.update_hint();
    }

    pub fn update_hint(&mut self) {
        if self.hints.is_empty() {
            self.current_hint = String::new();
            return;
        }

        // Prosty LCG jako zastepstwo dla zaleznosci od `rand`
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let index = (now % self.hints.len() as u128) as usize;
        self.current_hint = self.hints[index].clone();
    }

    pub fn update(&mut self, delta_time: f32) {
        self.progress_bar.update(delta_time);
    }
    
    pub fn set_phase(&mut self, phase: LoadingPhase) {
        self.progress_bar.set_phase(phase);
    }
    
    pub fn force_complete(&mut self) {
        self.progress_bar.force_complete();
    }

    pub fn get_progress(&self) -> f32 {
        self.progress_bar.get_current_progress()
    }

    pub fn is_completed(&self) -> bool {
        self.progress_bar.is_fully_loaded()
    }
}
