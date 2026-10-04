#[derive(Debug, Clone, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone)]
pub struct MusicTrack {
    pub identity: String,
    pub volume: f32,
    pub fade_target_volume: f32,
    pub fade_rate_per_sec: f32,
    pub is_playing: bool,
}

impl Default for MusicTrack {
    fn default() -> Self {
        Self {
            identity: String::new(),
            volume: 0.0,
            fade_target_volume: 0.0,
            fade_rate_per_sec: 0.0,
            is_playing: false,
        }
    }
}

impl MusicTrack {
    pub fn fade(&mut self, target_volume: f32, duration_sec: f32) {
        self.fade_target_volume = target_volume.clamp(0.0, 1.0);
        if (self.volume - self.fade_target_volume).abs() > 0.0001 {
            let rate = 1.0 / duration_sec;
            self.fade_rate_per_sec = if self.volume > self.fade_target_volume {
                -rate
            } else {
                rate
            };
        } else {
            self.fade_rate_per_sec = 0.0;
        }
    }

    pub fn stop_fading(&mut self) {
        self.fade_rate_per_sec = 0.0;
        self.fade_target_volume = self.volume;
    }

    pub fn update(&mut self, delta_time_sec: f32) {
        if self.fade_rate_per_sec != 0.0 {
            let delta_vol = self.fade_rate_per_sec * delta_time_sec;
            let mut new_vol = self.volume + delta_vol;
            
            if (self.fade_rate_per_sec > 0.0 && new_vol >= self.fade_target_volume) ||
               (self.fade_rate_per_sec < 0.0 && new_vol <= self.fade_target_volume) {
                new_vol = self.fade_target_volume;
                self.fade_rate_per_sec = 0.0;
                
                if self.fade_target_volume <= 0.0001 {
                    self.is_playing = false;
                }
            }
            
            self.volume = new_vol.clamp(0.0, 1.0);
        }
    }
}

#[derive(Debug, Clone)]
pub struct SoundInstance3D {
    pub identity: String,
    pub position: Vector3,
    pub volume: f32,
    pub is_playing: bool,
}

#[derive(Debug)]
pub struct SpatialSoundEngine {
    pub music_tracks: [MusicTrack; 2],
    pub current_music_index: usize,
    pub sound_volume: f32,
    pub music_volume: f32,
    pub master_volume: f32,
    
    pub listener_position: Vector3,
    pub listener_forward: Vector3,
    pub listener_up: Vector3,
    
    pub sounds_3d: Vec<SoundInstance3D>,
}

impl Default for SpatialSoundEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl SpatialSoundEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            music_tracks: [MusicTrack::default(), MusicTrack::default()],
            current_music_index: 0,
            sound_volume: 1.0,
            music_volume: 1.0,
            master_volume: 1.0,
            listener_position: Vector3::new(0.0, 0.0, 0.0),
            listener_forward: Vector3::new(0.0, 0.0, -1.0),
            listener_up: Vector3::new(0.0, 1.0, 0.0),
            sounds_3d: Vec::new(),
        };
        engine.set_listener_position(0.0, 0.0, 0.0);
        engine.set_listener_orientation(0.0, 0.0, -1.0, 0.0, 1.0, 0.0);
        engine
    }
    
    pub fn set_listener_position(&mut self, x: f32, y: f32, z: f32) {
        self.listener_position = Vector3::new(x, y, z);
    }
    
    pub fn set_listener_orientation(&mut self, forward_x: f32, forward_y: f32, forward_z: f32, up_x: f32, up_y: f32, up_z: f32) {
        // C++: ma_engine_listener_set_direction(&m_Engine, 0, forwardX, forwardY, -forwardZ);
        // ma_engine_listener_set_world_up(&m_Engine, 0, upX, -upY, upZ);
        self.listener_forward = Vector3::new(forward_x, forward_y, -forward_z);
        self.listener_up = Vector3::new(up_x, -up_y, up_z);
    }
    
    pub fn set_sound_volume(&mut self, volume: f32) {
        self.sound_volume = volume.clamp(0.0, 1.0);
    }
    
    pub fn set_music_volume(&mut self, volume: f32) {
        self.music_volume = volume.clamp(0.0, 1.0);
        self.music_tracks[self.current_music_index].stop_fading();
        self.music_tracks[self.current_music_index].volume = self.music_volume;
    }
    
    pub fn get_music_volume(&self) -> f32 {
        self.music_volume
    }
    
    pub fn set_master_volume(&mut self, volume: f32) {
        self.master_volume = volume.clamp(0.0, 1.0);
    }
    
    pub fn play_sound_3d(&mut self, name: &str, fx: f32, fy: f32, fz: f32) -> Option<&SoundInstance3D> {
        let rel_x = fx - self.listener_position.x;
        let rel_y = fy - self.listener_position.y;
        let rel_z = fz - self.listener_position.z;
        
        let instance = SoundInstance3D {
            identity: name.to_string(),
            position: Vector3::new(rel_x, rel_y, rel_z),
            volume: self.sound_volume,
            is_playing: true,
        };
        
        self.sounds_3d.push(instance);
        self.sounds_3d.last()
    }
    
    pub fn stop_all_sound_3d(&mut self) {
        for sound in &mut self.sounds_3d {
            sound.is_playing = false;
        }
    }
    
    pub fn fade_in_music(&mut self, path: &str, target_volume: f32, fade_in_duration_sec: f32) -> bool {
        if path.is_empty() {
            return false;
        }
        
        let current_track = &mut self.music_tracks[self.current_music_index];
        if current_track.is_playing && current_track.identity == path {
            current_track.fade(target_volume, fade_in_duration_sec);
            current_track.is_playing = true; // resume
            return true;
        }
        
        // fade out current music
        let current_id = self.music_tracks[self.current_music_index].identity.clone();
        if !current_id.is_empty() {
            self.fade_out_music(&current_id, 0.0, 2.0); // 2.0s is implied by crossfade request usually, but let's use the standard
        } else {
            self.music_tracks[self.current_music_index].fade(0.0, fade_in_duration_sec);
        }
        
        // Swap index
        self.current_music_index = 1 - self.current_music_index;
        
        let new_track = &mut self.music_tracks[self.current_music_index];
        new_track.identity = path.to_string();
        new_track.volume = 0.0;
        new_track.is_playing = true;
        new_track.fade(target_volume, fade_in_duration_sec);
        
        true
    }
    
    pub fn fade_out_music(&mut self, name: &str, target_volume: f32, fade_out_duration_sec: f32) {
        for track in &mut self.music_tracks {
            if track.identity == name {
                track.fade(target_volume, fade_out_duration_sec);
            }
        }
    }
    
    pub fn fade_out_all_music(&mut self) {
        let identities: Vec<String> = self.music_tracks.iter()
            .map(|m| m.identity.clone())
            .collect();
            
        for name in identities {
            if !name.is_empty() {
                self.fade_out_music(&name, 0.0, 2.0);
            }
        }
    }
    
    pub fn update(&mut self, delta_time_sec: f32) {
        for track in &mut self.music_tracks {
            track.update(delta_time_sec);
        }
    }
}
