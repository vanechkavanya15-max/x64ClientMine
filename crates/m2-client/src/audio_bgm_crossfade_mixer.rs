/// Background music (BGM) crossfade mixer.
/// Handles dynamic background music crossfading between tracks
/// using `f32::clamp` and calculated volume transitions.

#[derive(Debug, Clone, PartialEq)]
pub struct BgmTrack {
    pub filepath: String,
    pub volume: f32,
}

impl BgmTrack {
    pub fn new(filepath: String, volume: f32) -> Self {
        Self { filepath, volume }
    }
}

pub struct BgmCrossfadeMixer {
    pub current_track: Option<BgmTrack>,
    pub previous_track: Option<BgmTrack>,
    pub fade_duration_sec: f32,
}

impl BgmCrossfadeMixer {
    pub fn new(fade_duration_sec: f32) -> Self {
        Self {
            current_track: None,
            previous_track: None,
            fade_duration_sec: fade_duration_sec.max(0.001), // avoid division by zero
        }
    }

    pub fn play_music(&mut self, filepath: &str) {
        if let Some(current) = &self.current_track {
            if current.filepath == filepath {
                return;
            }
            self.previous_track = self.current_track.clone();
        }
        
        self.current_track = Some(BgmTrack::new(filepath.to_string(), 0.0));
    }

    pub fn stop_music(&mut self) {
        if self.current_track.is_some() {
            self.previous_track = self.current_track.clone();
            self.current_track = None;
        }
    }

    pub fn update(&mut self, delta_time_sec: f32) {
        let fade_rate_per_sec = 1.0 / self.fade_duration_sec;
        let delta_volume = fade_rate_per_sec * delta_time_sec;

        if let Some(prev) = &mut self.previous_track {
            prev.volume = (prev.volume - delta_volume).clamp(0.0, 1.0);
            if prev.volume == 0.0 {
                self.previous_track = None;
            }
        }

        if let Some(curr) = &mut self.current_track {
            curr.volume = (curr.volume + delta_volume).clamp(0.0, 1.0);
        }
    }
}
