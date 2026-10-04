use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::ui_loading_progress_bar::{LoadingPhase, LoadingProgressBar};

pub struct LoadingScreen {
    pub progress_bar: LoadingProgressBar,
    pub map_name: String,
    pub background_image: String,
    pub hints: Vec<String>,
    pub current_hint: String,
    hint_seed: u32,
}

impl Default for LoadingScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl LoadingScreen {
    pub fn new() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u32)
            .unwrap_or(12345);
            
        Self {
            progress_bar: LoadingProgressBar::new(),
            map_name: String::new(),
            background_image: "default_loading.jpg".to_string(),
            hints: Vec::new(),
            current_hint: String::new(),
            hint_seed: seed,
        }
    }

    pub fn set_map(&mut self, map_name: &str) {
        self.map_name = map_name.to_string();
        
        // Dopasowanie grafiki tla do docelowej mapy (m1, dolina, pustynia, grota)
        let background = match map_name {
            name if name.contains("a1") || name.contains("b1") || name.contains("c1") || name == "m1" => "m1_loading.jpg",
            name if name.contains("a3") || name.contains("b3") || name.contains("c3") || name == "m2" => "m2_loading.jpg",
            name if name.contains("trent") || name.contains("dolina") => "dolina_loading.jpg",
            name if name.contains("desert") || name.contains("pustynia") => "pustynia_loading.jpg",
            name if name.contains("ice") || name.contains("grota") || name.contains("grotto") => "grota_loading.jpg",
            name if name.contains("spider") || name.contains("milgyo") => "loch_loading.jpg",
            _ => "default_loading.jpg",
        };
        
        self.background_image = background.to_string();
        self.update_hint();
    }

    pub fn load_hints(&mut self, hints: Vec<String>) {
        self.hints = hints;
        self.update_hint();
    }
    
    pub fn load_hints_from_file(&mut self, path: &str) -> std::io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        
        let mut loaded_hints = Vec::new();
        
        for line in reader.lines() {
            let line = line?;
            let trimmed = line.trim();
            
            if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("//") {
                continue;
            }
            
            loaded_hints.push(trimmed.to_string());
        }
        
        self.hints = loaded_hints;
        self.update_hint();
        Ok(())
    }

    pub fn update_hint(&mut self) {
        if self.hints.is_empty() {
            self.current_hint = String::new();
            return;
        }

        // Custom deterministic LCG
        self.hint_seed = self.hint_seed.wrapping_mul(1103515245).wrapping_add(12345);
        let index = (self.hint_seed as usize) % self.hints.len();
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
