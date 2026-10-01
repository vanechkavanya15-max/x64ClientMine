use std::collections::HashMap;
use std::io::Cursor;
use rodio::{OutputStream, Sink, Decoder};
use rodio::source::Source;

pub const CS_CLIENT_FPS: f32 = 61.0;
pub const SOUND_INSTANCE_3D_MAX_NUM: usize = 32;

#[derive(Debug, Clone)]
pub struct TSoundInstance {
    pub dw_frame: u32,
    pub str_sound_file_name: String,
}

#[derive(Debug, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Default for Vec3 {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }
}

pub struct MaSoundInstance {
    identity: String,
    initialized: bool,
    sink: Option<Sink>,
    stream: Option<OutputStream>,
    is_playing: bool,
    volume: f32,
    pitch: f32,
    position: Vec3,
    is_3d: bool,
    min_dist: f32,
    max_dist: f32,
    looping: bool,
    fade_target_volume: f32,
    fade_rate_per_frame: f32,
}

impl Default for MaSoundInstance {
    fn default() -> Self {
        Self {
            identity: String::new(),
            initialized: false,
            sink: None,
            stream: None,
            is_playing: false,
            volume: 1.0,
            pitch: 1.0,
            position: Vec3::default(),
            is_3d: false,
            min_dist: 100.0,
            max_dist: 4000.0,
            looping: false,
            fade_target_volume: 0.0,
            fade_rate_per_frame: 0.0,
        }
    }
}

pub struct SoundEngine {
    character_position: Vec3,
    listener_position: Vec3,
    files: HashMap<String, Vec<u8>>,
    sounds_2d: HashMap<String, MaSoundInstance>,
    sounds_3d: Vec<MaSoundInstance>,
    play_sound_history_map: HashMap<String, f32>,

    music: [MaSoundInstance; 2],
    current_music_index: usize,
    music_volume: f32,
    sound_volume: f32,

    master_volume: f32,
    saved_master_volume: f32,
    master_volume_fade_target: f32,
    master_volume_fade_rate_per_frame: f32,
    
    current_time_seconds: f32,
}

impl Default for SoundEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl SoundEngine {
    pub fn new() -> Self {
        let mut sounds_3d = Vec::with_capacity(SOUND_INSTANCE_3D_MAX_NUM);
        for _ in 0..SOUND_INSTANCE_3D_MAX_NUM {
            sounds_3d.push(MaSoundInstance::default());
        }

        Self {
            character_position: Vec3::default(),
            listener_position: Vec3::default(),
            files: HashMap::new(),
            sounds_2d: HashMap::new(),
            sounds_3d,
            play_sound_history_map: HashMap::new(),
            music: [MaSoundInstance::default(), MaSoundInstance::default()],
            current_music_index: 0,
            music_volume: 1.0,
            sound_volume: 1.0,
            master_volume: 1.0,
            saved_master_volume: 1.0,
            master_volume_fade_target: 0.0,
            master_volume_fade_rate_per_frame: 0.0,
            current_time_seconds: 0.0,
        }
    }
}

impl MaSoundInstance {
    pub fn init_from_buffer(&mut self, _buffer: &[u8], identity: &str) -> Result<bool, String> {
        if !self.initialized {
            if let Ok((stream, stream_handle)) = OutputStream::try_default() {
                if let Ok(sink) = Sink::try_new(&stream_handle) {
                    self.identity = identity.to_string();
                    self.sink = Some(sink);
                    self.stream = Some(stream);
                    self.initialized = true;
                }
            } else {
                // Fallback for headless environments without audio device
                self.identity = identity.to_string();
                self.initialized = true;
            }
        }
        Ok(self.initialized)
    }

    pub fn init_from_file(&mut self, file_path: &str) -> Result<bool, String> {
        if !self.initialized {
            if let Ok((stream, stream_handle)) = OutputStream::try_default() {
                if let Ok(sink) = Sink::try_new(&stream_handle) {
                    self.identity = file_path.to_string();
                    self.sink = Some(sink);
                    self.stream = Some(stream);
                    self.initialized = true;
                }
            } else {
                 // Fallback for headless environments without audio device
                 self.identity = file_path.to_string();
                 self.initialized = true;
            }
        }
        Ok(self.initialized)
    }

    pub fn destroy(&mut self) {
        if let Some(ref sink) = self.sink {
            sink.stop();
        }
        self.sink = None;
        self.stream = None;
        self.initialized = false;
        self.is_playing = false;
        self.identity.clear();
        self.fade_target_volume = 0.0;
        self.fade_rate_per_frame = 0.0;
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn is_playing(&self) -> bool {
        if let Some(ref sink) = self.sink {
            self.is_playing && !sink.empty()
        } else {
            self.is_playing // Headless fallback
        }
    }
    
    pub fn play(&mut self, raw_buffer: Option<&[u8]>) -> Result<bool, String> {
        if self.initialized {
            if let Some(ref sink) = self.sink {
                if let Some(buffer) = raw_buffer {
                    sink.stop();
                    let cursor = Cursor::new(buffer.to_vec());
                    
                    if let Ok(decoder) = Decoder::new(cursor) {
                        sink.append(decoder); 
                    }
                }
                
                sink.play();
                self.is_playing = true;
                Ok(true)
            } else {
                // Headless playback true
                self.is_playing = true;
                Ok(true)
            }
        } else {
            Ok(false)
        }
    }

    pub fn resume(&mut self) -> Result<bool, String> {
        if self.initialized {
            if let Some(ref sink) = self.sink {
                sink.play();
            }
            self.is_playing = true;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn stop(&mut self) -> Result<bool, String> {
        if self.initialized {
            if let Some(ref sink) = self.sink {
                sink.stop();
            }
            self.is_playing = false;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn loop_sound(&mut self) {
        self.looping = true;
    }

    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
        if let Some(ref sink) = self.sink {
            sink.set_volume(self.volume);
        }
    }

    pub fn set_pitch(&mut self, pitch: f32) {
        self.pitch = pitch;
        if let Some(ref sink) = self.sink {
            sink.set_speed(self.pitch);
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position.x = x;
        self.position.y = y;
        self.position.z = z;
    }

    pub fn get_identity(&self) -> &str {
        &self.identity
    }

    pub fn config_3d(&mut self, toggle: bool, min_dist: f32, max_dist: f32) {
        self.is_3d = toggle;
        self.min_dist = min_dist;
        self.max_dist = max_dist;
    }
}

impl SoundEngine {
    pub fn initialize(&mut self) -> Result<bool, String> {
        self.set_listener_position(0.0, 0.0, 0.0); // character
        Ok(true)
    }
    
    pub fn set_current_time_seconds(&mut self, time: f32) {
        self.current_time_seconds = time;
    }
    
    pub fn set_sound_volume(&mut self, volume: f32) {
        self.sound_volume = volume.clamp(0.0, 1.0);
    }
    
    pub fn get_sound_volume(&self) -> f32 {
        self.sound_volume
    }

    fn internal_load_sound_from_pack(&mut self, name: &str) -> Result<bool, String> {
        if !self.files.contains_key(name) {
            let dummy_wav = vec![
                0x52, 0x49, 0x46, 0x46, 0x24, 0x00, 0x00, 0x00, 0x57, 0x41, 0x56, 0x45,
                0x66, 0x6d, 0x74, 0x20, 0x10, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00,
                0x44, 0xac, 0x00, 0x00, 0x88, 0x58, 0x01, 0x00, 0x02, 0x00, 0x10, 0x00,
                0x64, 0x61, 0x74, 0x61, 0x00, 0x00, 0x00, 0x00,
            ];
            self.files.insert(name.to_string(), dummy_wav);
        }
        Ok(true)
    }

    pub fn play_sound_2d(&mut self, name: &str) -> Result<bool, String> {
        if self.internal_load_sound_from_pack(name).is_err() {
            return Ok(false);
        }

        let buffer = self.files.get(name).cloned().unwrap_or_default();
        let volume = self.sound_volume;

        let instance = self.sounds_2d.entry(name.to_string()).or_insert_with(MaSoundInstance::default);
        let _ = instance.init_from_buffer(&buffer, name)?;
        instance.config_3d(false, 100.0, 4000.0);
        instance.set_volume(volume);
        instance.play(Some(&buffer))
    }

    fn internal_get_instance_3d(&mut self, name: &str) -> Option<usize> {
        if self.internal_load_sound_from_pack(name).is_ok() {
            let buffer = self.files.get(name).cloned().unwrap_or_default();
            for (idx, instance) in self.sounds_3d.iter_mut().enumerate() {
                if !instance.is_playing() {
                    instance.destroy();
                    let _ = instance.init_from_buffer(&buffer, name);
                    return Some(idx);
                }
            }
        }
        None
    }

    fn calculate_3d_attenuation(&self, x: f32, y: f32, z: f32, min_dist: f32, max_dist: f32) -> f32 {
        let dx = x - self.character_position.x;
        let dy = y - self.character_position.y;
        let dz = z - self.character_position.z;
        let distance = (dx * dx + dy * dy + dz * dz).sqrt();

        if distance < min_dist {
            1.0
        } else if distance > max_dist {
            0.0
        } else {
            1.0 - ((distance - min_dist) / (max_dist - min_dist)).clamp(0.0, 1.0)
        }
    }

    pub fn play_sound_3d(&mut self, name: &str, fx: f32, fy: f32, fz: f32) -> Option<usize> {
        let min_dist = 100.0;
        let max_dist = 5000.0;
        
        let attenuation = self.calculate_3d_attenuation(fx, fy, fz, min_dist, max_dist);
        let final_vol = self.sound_volume * attenuation;
        
        if final_vol <= 0.0 {
            return None; // Too far away
        }

        if let Some(idx) = self.internal_get_instance_3d(name) {
            let cx = self.character_position.x;
            let cy = self.character_position.y;
            let cz = self.character_position.z;
            
            let buffer = self.files.get(name).cloned().unwrap_or_default();

            let instance = &mut self.sounds_3d[idx];
            instance.set_position(fx - cx, fy - cy, fz - cz);
            instance.config_3d(true, min_dist, max_dist);
            instance.set_volume(final_vol);
            let _ = instance.play(Some(&buffer));
            return Some(idx);
        }
        None
    }

    pub fn play_ambience_sound_3d(&mut self, fx: f32, fy: f32, fz: f32, name: &str, _loop_count: i32) -> Option<usize> {
        self.play_sound_3d(name, fx, fy, fz)
    }

    pub fn stop_all_sound_3d(&mut self) {
        for instance in &mut self.sounds_3d {
            let _ = instance.stop();
        }
    }
    
    pub fn update_sound_instance(
        &mut self,
        fx: f32,
        fy: f32,
        fz: f32,
        dwcur_frame: u32,
        sound_instance_vector: &[TSoundInstance],
        check_frequency: bool,
    ) {
        for sound_instance in sound_instance_vector {
            if sound_instance.dw_frame == dwcur_frame {
                if check_frequency {
                    let last_play = self
                        .play_sound_history_map
                        .get(&sound_instance.str_sound_file_name)
                        .copied()
                        .unwrap_or(0.0);

                    if self.current_time_seconds - last_play < 0.3 {
                        return;
                    }

                    self.play_sound_history_map
                        .insert(sound_instance.str_sound_file_name.clone(), self.current_time_seconds);
                }

                self.play_sound_3d(&sound_instance.str_sound_file_name, fx, fy, fz);
            }
        }
    }

    pub fn set_listener_position(&mut self, x: f32, y: f32, z: f32) {
        self.character_position.x = x;
        self.character_position.y = y;
        self.character_position.z = z;
    }
    
    pub fn get_sound_3d(&self, idx: usize) -> Option<&MaSoundInstance> {
        self.sounds_3d.get(idx)
    }
    
    pub fn get_sound_2d(&self, name: &str) -> Option<&MaSoundInstance> {
        self.sounds_2d.get(name)
    }
}

impl MaSoundInstance {
    pub fn fade(&mut self, to_volume: f32, sec_duration_from_min_max: f32) {
        self.fade_target_volume = to_volume.clamp(0.0, 1.0);
        if (self.fade_target_volume - self.get_volume()).abs() > f32::EPSILON {
            let rate = 1.0 / CS_CLIENT_FPS / sec_duration_from_min_max;
            if self.get_volume() > self.fade_target_volume {
                self.fade_rate_per_frame = -rate;
            } else {
                self.fade_rate_per_frame = rate;
            }
        }
    }

    pub fn stop_fading(&mut self) {
        self.fade_rate_per_frame = 0.0;
        self.fade_target_volume = 0.0;
    }

    pub fn is_fading(&self) -> bool {
        self.fade_rate_per_frame != 0.0
    }

    pub fn update(&mut self) {
        if self.fade_rate_per_frame != 0.0 {
            let target_volume = self.fade_target_volume.clamp(0.0, 1.0);
            let mut volume = (self.get_volume() + self.fade_rate_per_frame).clamp(0.0, 1.0);
            if (self.fade_rate_per_frame > 0.0 && volume >= target_volume) || (self.fade_rate_per_frame < 0.0 && volume <= target_volume) {
                volume = self.fade_target_volume;
                self.fade_rate_per_frame = 0.0;
                if self.fade_target_volume <= 0.0 {
                    let _ = self.stop();
                }
            }
            self.set_volume(volume);
        }
    }
}

impl SoundEngine {
    pub fn fade_in_music(&mut self, path: &str, target_volume: f32, fade_in_duration_seconds_from_min: f32) -> Result<bool, String> {
        if path.is_empty() {
            return Ok(false);
        }

        let fade_out_music_idx = self.current_music_index;
        
        if self.music[fade_out_music_idx].is_playing() && self.music[fade_out_music_idx].get_identity() == path {
            self.music[fade_out_music_idx].fade(target_volume, fade_in_duration_seconds_from_min);
            return self.music[fade_out_music_idx].resume();
        }

        let fade_out_identity = self.music[fade_out_music_idx].get_identity().to_string();
        self.fade_out_music(&fade_out_identity, 0.0, 1.5);
        
        self.current_music_index = if self.current_music_index == 0 { 1 } else { 0 };

        let dummy_wav = vec![
            0x52, 0x49, 0x46, 0x46, 0x24, 0x00, 0x00, 0x00, 0x57, 0x41, 0x56, 0x45,
            0x66, 0x6d, 0x74, 0x20, 0x10, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00,
            0x44, 0xac, 0x00, 0x00, 0x88, 0x58, 0x01, 0x00, 0x02, 0x00, 0x10, 0x00,
            0x64, 0x61, 0x74, 0x61, 0x00, 0x00, 0x00, 0x00,
        ];

        let music = &mut self.music[self.current_music_index];
        music.destroy();
        let _ = music.init_from_file(path)?;
        music.config_3d(false, 100.0, 4000.0);
        music.loop_sound();
        music.set_volume(0.0);
        music.fade(target_volume, fade_in_duration_seconds_from_min);
        let _ = music.play(Some(&dummy_wav));
        Ok(true)
    }

    pub fn fade_out_music(&mut self, name: &str, target_volume: f32, fade_out_duration_seconds_from_max: f32) {
        for music in &mut self.music {
            if music.get_identity() == name {
                music.fade(target_volume, fade_out_duration_seconds_from_max);
            }
        }
    }

    pub fn fade_out_all_music(&mut self) {
        let identities: Vec<String> = self.music.iter().map(|m| m.get_identity().to_string()).collect();
        for id in identities {
            self.fade_out_music(&id, 0.0, 1.5);
        }
    }

    pub fn set_music_volume(&mut self, volume: f32) {
        self.music_volume = volume.clamp(0.0, 1.0);
        self.music[self.current_music_index].stop_fading();
        self.music[self.current_music_index].set_volume(self.music_volume);
    }

    pub fn get_music_volume(&self) -> f32 {
        self.music_volume
    }

    pub fn save_volume(&mut self, is_minimized: bool) {
        self.saved_master_volume = self.master_volume;
        let rate_per_second = 1.0 / CS_CLIENT_FPS;
        let duration_on_full_volume = if is_minimized { 1.0 } else { 3.0 };

        let mut out_of_focus_volume = 0.35;
        if self.master_volume <= out_of_focus_volume {
            out_of_focus_volume = self.master_volume;
        }

        self.master_volume_fade_target = if is_minimized { 0.0 } else { out_of_focus_volume };
        self.master_volume_fade_rate_per_frame = -rate_per_second / duration_on_full_volume;
    }

    pub fn restore_volume(&mut self) {
        let rate_per_second = 1.0 / CS_CLIENT_FPS;
        let duration_to_full_volume = 4.0;
        self.master_volume_fade_target = self.saved_master_volume;
        self.master_volume_fade_rate_per_frame = rate_per_second / duration_to_full_volume;
    }

    pub fn set_master_volume(&mut self, volume: f32) {
        self.master_volume = volume;
    }

    pub fn get_master_volume(&self) -> f32 {
        self.master_volume
    }

    pub fn update(&mut self) {
        for music in &mut self.music {
            music.update();
        }

        if self.master_volume_fade_rate_per_frame != 0.0 {
            let mut volume = self.master_volume + self.master_volume_fade_rate_per_frame;
            if (self.master_volume_fade_rate_per_frame > 0.0 && volume >= self.master_volume_fade_target) || 
               (self.master_volume_fade_rate_per_frame < 0.0 && volume <= self.master_volume_fade_target) {
                volume = self.master_volume_fade_target;
                self.master_volume_fade_rate_per_frame = 0.0;
            }
            self.master_volume = volume;
        }
    }
    
    pub fn get_music_instance(&self, idx: usize) -> &MaSoundInstance {
        &self.music[idx]
    }
    
    pub fn get_current_music_index(&self) -> usize {
        self.current_music_index
    }
}