// ui_wndmgr_widget_ani_image.rs - AniImageBox UI widget implementation for Metin2 Pure Rust

#[derive(Clone)]
pub struct AniImageBoxWidget {
    pub images: Vec<String>,
    pub current_frame: usize,
    pub delay: u8,
    pub current_delay: u8,
    pub loop_enabled: bool,
    pub is_playing: bool,
}

impl Default for AniImageBoxWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl AniImageBoxWidget {
    pub fn new() -> Self {
        Self {
            images: Vec::new(),
            current_frame: 0,
            delay: 4, // Default matching C++
            current_delay: 0,
            loop_enabled: true,
            is_playing: true,
        }
    }

    pub fn set_delay(&mut self, delay: u8) {
        self.delay = delay;
    }
    
    pub fn set_loop(&mut self, loop_enabled: bool) {
        self.loop_enabled = loop_enabled;
    }
    
    pub fn play(&mut self) {
        self.is_playing = true;
    }
    
    pub fn pause(&mut self) {
        self.is_playing = false;
    }

    pub fn append_image(&mut self, image_path: &str) {
        self.images.push(image_path.to_string());
    }

    pub fn reset_frame(&mut self) {
        self.current_frame = 0;
        self.is_playing = true;
    }

    pub fn update(&mut self) -> bool {
        if !self.is_playing || self.images.is_empty() {
            return false;
        }

        self.current_delay += 1;
        if self.current_delay < self.delay {
            return false;
        }

        self.current_delay = 0;
        self.current_frame += 1;
        
        if self.current_frame >= self.images.len() {
            if self.loop_enabled {
                self.current_frame = 0;
            } else {
                self.current_frame = self.images.len() - 1;
                self.is_playing = false;
            }
            return true; // Indicates OnEndFrame should be called
        }
        
        false
    }

    pub fn get_current_image(&self) -> Option<&String> {
        if self.current_frame < self.images.len() {
            Some(&self.images[self.current_frame])
        } else {
            None
        }
    }

    pub fn clear_images(&mut self) {
        self.images.clear();
        self.current_frame = 0;
        self.current_delay = 0;
        self.is_playing = true;
    }
}
