#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FadeState {
    Stopped,
    Playing,
    FadeIn,
    FadeOut,
    Crossfading, // Handling transition between two tracks
}

pub struct BgmFader {
    state: FadeState,
    current_track: Option<String>,
    next_track: Option<String>,
    
    // For single track fading
    volume: f32,
    target_volume: f32,
    fade_speed: f32, // volume change per second
    
    // For crossfading (optional track 2 volume)
    crossfade_volume: f32,
    
    // Limits
    max_volume: f32,
}

impl BgmFader {
    pub fn new() -> Self {
        Self {
            state: FadeState::Stopped,
            current_track: None,
            next_track: None,
            volume: 0.0,
            target_volume: 0.0,
            fade_speed: 0.0,
            crossfade_volume: 0.0,
            max_volume: 1.0,
        }
    }
    
    pub fn set_max_volume(&mut self, max_vol: f32) {
        self.max_volume = max_vol.clamp(0.0, 1.0);
        if self.state == FadeState::Playing {
            self.volume = self.max_volume;
        }
    }

    pub fn max_volume(&self) -> f32 {
        self.max_volume
    }

    pub fn play(&mut self, track: &str) {
        self.state = FadeState::Playing;
        self.current_track = Some(track.to_string());
        self.next_track = None;
        self.volume = self.max_volume;
        self.target_volume = self.max_volume;
        self.fade_speed = 0.0;
        self.crossfade_volume = 0.0;
    }

    pub fn fade_in(&mut self, track: &str, duration_sec: f32) {
        self.state = FadeState::FadeIn;
        self.current_track = Some(track.to_string());
        self.next_track = None;
        self.volume = 0.0;
        self.target_volume = self.max_volume;
        self.fade_speed = if duration_sec > 0.0 { self.max_volume / duration_sec } else { self.max_volume };
        self.crossfade_volume = 0.0;
        
        if duration_sec <= 0.0 {
            self.volume = self.max_volume;
            self.state = FadeState::Playing;
        }
    }

    pub fn fade_out(&mut self, duration_sec: f32) {
        if self.state == FadeState::Stopped {
            return;
        }
        self.state = FadeState::FadeOut;
        self.target_volume = 0.0;
        self.fade_speed = if duration_sec > 0.0 { self.volume / duration_sec } else { self.volume };
        
        if duration_sec <= 0.0 {
            self.volume = 0.0;
            self.stop();
        }
    }
    
    pub fn crossfade(&mut self, next_track: &str, duration_sec: f32) {
        if self.state == FadeState::Stopped {
            self.fade_in(next_track, duration_sec);
            return;
        }
        
        self.state = FadeState::Crossfading;
        self.next_track = Some(next_track.to_string());
        self.target_volume = 0.0; // target for current track
        self.fade_speed = if duration_sec > 0.0 { self.max_volume / duration_sec } else { self.max_volume };
        self.crossfade_volume = 0.0; // starts at 0 for next track
        
        if duration_sec <= 0.0 {
            self.current_track = Some(next_track.to_string());
            self.next_track = None;
            self.volume = self.max_volume;
            self.state = FadeState::Playing;
        }
    }
    
    pub fn enter_battle_zone(&mut self, battle_track: &str, duration_sec: f32) {
        self.crossfade(battle_track, duration_sec);
    }
    
    pub fn exit_battle_zone(&mut self, normal_track: &str, duration_sec: f32) {
        self.crossfade(normal_track, duration_sec);
    }

    pub fn stop(&mut self) {
        self.state = FadeState::Stopped;
        self.current_track = None;
        self.next_track = None;
        self.volume = 0.0;
        self.target_volume = 0.0;
        self.fade_speed = 0.0;
        self.crossfade_volume = 0.0;
    }

    pub fn update(&mut self, delta_sec: f32) {
        if self.state == FadeState::Stopped || self.state == FadeState::Playing {
            return;
        }
        
        let delta_vol = self.fade_speed * delta_sec;

        match self.state {
            FadeState::FadeIn => {
                self.volume += delta_vol;
                if self.volume >= self.target_volume {
                    self.volume = self.target_volume;
                    self.state = FadeState::Playing;
                }
            }
            FadeState::FadeOut => {
                self.volume -= delta_vol;
                if self.volume <= 0.0 {
                    self.volume = 0.0;
                    self.stop();
                }
            }
            FadeState::Crossfading => {
                // Fade out current track
                self.volume -= delta_vol;
                if self.volume <= 0.0 {
                    self.volume = 0.0;
                }
                
                // Fade in next track
                self.crossfade_volume += delta_vol;
                if self.crossfade_volume >= self.max_volume {
                    self.crossfade_volume = self.max_volume;
                }
                
                // When current is fully faded out and next is fully faded in
                if self.volume <= 0.0 && self.crossfade_volume >= self.max_volume {
                    self.current_track = self.next_track.take();
                    self.volume = self.max_volume;
                    self.crossfade_volume = 0.0;
                    self.state = FadeState::Playing;
                }
            }
            _ => {}
        }
        
        // Final clamp
        self.volume = self.volume.clamp(0.0, self.max_volume);
        self.crossfade_volume = self.crossfade_volume.clamp(0.0, self.max_volume);
    }

    pub fn state(&self) -> FadeState {
        self.state
    }

    pub fn current_track(&self) -> Option<&str> {
        self.current_track.as_deref()
    }
    
    pub fn next_track(&self) -> Option<&str> {
        self.next_track.as_deref()
    }

    pub fn current_volume(&self) -> f32 {
        self.volume
    }
    
    pub fn next_volume(&self) -> f32 {
        self.crossfade_volume
    }
}
