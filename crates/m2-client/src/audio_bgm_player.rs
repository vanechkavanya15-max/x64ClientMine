#[derive(Debug, Clone, PartialEq)]
pub struct BgmTrack {
    pub title: String,
    pub file_path: String,
    pub volume: f32,
    pub is_looping: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BgmPlayerState {
    pub current_track: Option<BgmTrack>,
    pub is_playing: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AudioBgmPlayer {
    pub state: BgmPlayerState,
    fade_duration_ms: u32,
    fade_elapsed_ms: u32,
    fade_start_volume: f32,
    is_fading_out: bool,
}

impl Default for AudioBgmPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioBgmPlayer {
    pub fn new() -> Self {
        Self {
            state: BgmPlayerState {
                current_track: None,
                is_playing: false,
            },
            fade_duration_ms: 0,
            fade_elapsed_ms: 0,
            fade_start_volume: 0.0,
            is_fading_out: false,
        }
    }

    pub fn play_bgm(&mut self, track_path: &str, loop_flag: bool) {
        let title = track_path
            .split('/')
            .last()
            .unwrap_or(track_path)
            .split('\\')
            .last()
            .unwrap_or(track_path)
            .to_string();

        self.state.current_track = Some(BgmTrack {
            title,
            file_path: track_path.to_string(),
            volume: 1.0,
            is_looping: loop_flag,
        });
        self.state.is_playing = true;
        self.is_fading_out = false;
        self.fade_duration_ms = 0;
        self.fade_elapsed_ms = 0;
        self.fade_start_volume = 1.0;
    }

    pub fn stop_bgm(&mut self) {
        self.state.is_playing = false;
        self.is_fading_out = false;
        if let Some(track) = &mut self.state.current_track {
            track.volume = 0.0;
        }
    }

    pub fn set_bgm_volume(&mut self, volume: f32) {
        let clamped = volume.clamp(0.0, 1.0);
        if let Some(track) = &mut self.state.current_track {
            track.volume = clamped;
        }
        if !self.is_fading_out {
            self.fade_start_volume = clamped;
        }
    }

    pub fn fade_out_bgm(&mut self, duration_ms: u32) {
        if !self.state.is_playing || self.state.current_track.is_none() {
            return;
        }
        if duration_ms == 0 {
            self.stop_bgm();
            return;
        }
        self.is_fading_out = true;
        self.fade_duration_ms = duration_ms;
        self.fade_elapsed_ms = 0;
        if let Some(track) = &self.state.current_track {
            self.fade_start_volume = track.volume;
        }
    }

    pub fn update(&mut self, delta_ms: u32) {
        if !self.state.is_playing || !self.is_fading_out {
            return;
        }

        self.fade_elapsed_ms = self.fade_elapsed_ms.saturating_add(delta_ms);

        if self.fade_elapsed_ms >= self.fade_duration_ms {
            self.stop_bgm();
        } else {
            let ratio = 1.0 - (self.fade_elapsed_ms as f32 / self.fade_duration_ms as f32);
            let new_volume = self.fade_start_volume * ratio;
            if let Some(track) = &mut self.state.current_track {
                track.volume = new_volume.clamp(0.0, 1.0);
            }
        }
    }
}
