#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BgmState {
    Off,
    Play,
    FadeIn,
    FadeOut,
    FadeLimitOut,
}

pub struct SoundBgmManager {
    state: BgmState,
    current_file: String,
    volume: f32,
    target_volume: f32,
    volume_speed: f32,
}

impl SoundBgmManager {
    pub fn new() -> Self {
        Self {
            state: BgmState::Off,
            current_file: String::new(),
            volume: 0.0,
            target_volume: 0.0,
            volume_speed: 0.0,
        }
    }

    pub fn play_music(&mut self, file_name: &str, _loop_play: bool) {
        self.state = BgmState::Play;
        self.current_file = file_name.to_string();
        self.volume = 1.0;
        self.target_volume = 1.0;
        self.volume_speed = 0.0;
    }

    pub fn fade_in(&mut self, file_name: &str, target_vol: f32, speed: f32) {
        self.state = BgmState::FadeIn;
        self.current_file = file_name.to_string();
        self.volume = 0.0;
        self.target_volume = target_vol.clamp(0.0, 1.0);
        self.volume_speed = speed;
    }

    pub fn fade_out(&mut self, speed: f32) {
        self.state = BgmState::FadeOut;
        self.target_volume = 0.0;
        self.volume_speed = speed;
    }

    pub fn update(&mut self, delta_sec: f32) {
        if self.state == BgmState::Off || self.state == BgmState::Play {
            return;
        }

        let speed_per_frame = self.volume_speed * (delta_sec * 60.0);

        match self.state {
            BgmState::FadeIn => {
                self.volume += speed_per_frame;
                if self.volume >= self.target_volume {
                    self.volume = self.target_volume;
                    self.state = BgmState::Play;
                }
            }
            BgmState::FadeOut => {
                self.volume -= speed_per_frame;
                if self.volume <= 0.0 {
                    self.volume = 0.0;
                    self.state = BgmState::Off;
                    self.current_file.clear();
                }
            }
            BgmState::FadeLimitOut => {
                self.volume -= speed_per_frame;
                if self.volume <= self.target_volume {
                    self.volume = self.target_volume;
                    self.state = BgmState::Play;
                }
            }
            _ => {}
        }
        
        self.volume = self.volume.clamp(0.0, 1.0);
    }

    pub fn current_volume(&self) -> f32 {
        self.volume
    }

    // Helper functions for testing to inspect internal state
    pub fn current_file(&self) -> &str {
        &self.current_file
    }

    pub fn state(&self) -> BgmState {
        self.state
    }
}
