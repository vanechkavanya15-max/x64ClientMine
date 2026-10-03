// ui_ani_image_box_widget.rs - AniImageBox UI widget implementation for Metin2 Pure Rust

#[derive(Debug, Clone)]
pub struct AniImageBoxWidget {
    pub images: Vec<String>,
    pub current_frame: usize,
    pub delay: f32, // Delay between frames in seconds
    pub timer: f32, // Accumulator for elapsed time
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
            delay: 0.1, // Default 100ms
            timer: 0.0,
            loop_enabled: true,
            is_playing: true,
        }
    }

    pub fn append_image(&mut self, image_path: &str) {
        self.images.push(image_path.to_string());
    }

    pub fn clear_images(&mut self) {
        self.images.clear();
        self.current_frame = 0;
        self.timer = 0.0;
    }

    pub fn set_delay(&mut self, delay: f32) {
        self.delay = if delay < 0.0 { 0.0 } else { delay };
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

    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.timer = 0.0;
        self.is_playing = true;
    }

    pub fn update(&mut self, delta_time: f32) {
        if !self.is_playing || self.images.is_empty() || self.delay <= 0.0 {
            return;
        }

        self.timer += delta_time;

        if self.timer >= self.delay {
            let frames_to_advance = (self.timer / self.delay) as usize;
            self.timer %= self.delay;

            if self.loop_enabled {
                self.current_frame = (self.current_frame + frames_to_advance) % self.images.len();
            } else {
                self.current_frame += frames_to_advance;
                if self.current_frame >= self.images.len() {
                    self.current_frame = self.images.len() - 1;
                    self.is_playing = false; // Stop when animation finishes if not looping
                }
            }
        }
    }

    pub fn get_current_image(&self) -> Option<&String> {
        if self.images.is_empty() {
            None
        } else {
            self.images.get(self.current_frame)
        }
    }
}
