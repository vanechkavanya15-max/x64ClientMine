use crate::sound_bgm_manager::{SoundBgmManager, BgmState};
use crate::sound_spatial_3d::{calculate_spatial_sound, SpatialSoundOutput};
use crate::sound_mixer_core::{SoundMixerCore, SoundCategory as MixerSoundCategory};
use crate::sound_types::{SoundCategory, SoundInstance};
use crate::sound_buffer_cache::{SoundBufferCache, WavAudio};
use crate::sound_wav_parser;
use crate::sound_debouncer::SoundDebouncer;

pub struct AudioEngine {
    bgm_manager: SoundBgmManager,
    mixer: SoundMixerCore,
    buffer_cache: SoundBufferCache,
    debouncer: SoundDebouncer,
    listener_pos: (f32, f32, f32),
    listener_yaw: f32,
}

impl AudioEngine {
    pub fn new() -> Self {
        Self {
            bgm_manager: SoundBgmManager::new(),
            mixer: SoundMixerCore::new(),
            buffer_cache: SoundBufferCache::default(),
            debouncer: SoundDebouncer::new(),
            listener_pos: (0.0, 0.0, 0.0),
            listener_yaw: 0.0,
        }
    }

    pub fn set_listener_position(&mut self, x: f32, y: f32, z: f32) {
        self.listener_pos = (x, y, z);
    }

    pub fn set_listener_orientation(&mut self, yaw: f32) {
        self.listener_yaw = yaw;
    }

    pub fn play_music(&mut self, file_name: &str) {
        self.bgm_manager.play_music(file_name, true);
    }

    pub fn fade_in_music(&mut self, file_name: &str, target_vol: f32, speed: f32) {
        self.bgm_manager.fade_in(file_name, target_vol, speed);
    }

    pub fn fade_out_music(&mut self, speed: f32) {
        self.bgm_manager.fade_out(speed);
    }

    pub fn set_master_volume(&mut self, volume: f32) {
        self.mixer.set_master_volume(volume);
    }

    pub fn set_sfx_volume(&mut self, volume: f32) {
        self.mixer.set_sfx_volume(volume);
    }

    pub fn set_music_volume(&mut self, volume: f32) {
        self.mixer.set_music_volume(volume);
    }
    
    pub fn get_music_volume(&self) -> f32 {
        self.mixer.get_music_volume()
    }
    
    pub fn get_sfx_volume(&self) -> f32 {
        self.mixer.get_sfx_volume()
    }
    
    pub fn play_sound_2d(&mut self, file_name: &str) -> Option<SoundInstance> {
        let vol = self.mixer.compute_final_volume(MixerSoundCategory::SFX, 1.0);
        if vol <= 0.0 {
            return None;
        }

        Some(SoundInstance::new(
            0,
            file_name.to_string(),
            SoundCategory::Effect,
            vol,
            0.0,
            1.0,
            false,
            1
        ))
    }

    pub fn play_sound_3d(&mut self, file_name: &str, x: f32, y: f32, z: f32) -> Option<SoundInstance> {
        let spatial = calculate_spatial_sound(
            x, y, z, 
            self.listener_pos.0, self.listener_pos.1, self.listener_pos.2, 
            self.listener_yaw
        );

        if !spatial.is_audible {
            return None;
        }

        let vol = self.mixer.compute_final_volume(MixerSoundCategory::SFX, spatial.effective_volume);
        
        if vol <= 0.0 {
            return None;
        }

        Some(SoundInstance::new(
            0,
            file_name.to_string(),
            SoundCategory::Effect,
            vol,
            spatial.pan,
            1.0,
            false,
            1
        ))
    }
    
    pub fn update(&mut self, delta_sec: f32, cur_time_ms: u32) {
        self.bgm_manager.update(delta_sec);
        self.debouncer.cleanup_old_records(cur_time_ms);
    }
}

use crate::event_sound_trigger_v2::{TMotionSoundEventData, process_motion_sound_event};

impl AudioEngine {
    pub fn update_sound_instance(
        &mut self,
        fx: f32,
        fy: f32,
        fz: f32,
        dwcur_frame: u32,
        sound_instances: &[TMotionSoundEventData],
        check_frequency: bool,
        cur_time_ms: u32,
    ) {
        for instance in sound_instances {
            if instance.frame == dwcur_frame {
                if check_frequency {
                    let hash = instance.sound_file_name.len() as u32; // Just a dummy hash for now, since we only need *a* hash
                    if !self.debouncer.can_play(0, hash, cur_time_ms) {
                        continue;
                    }
                }
                
                // C++ originally does PlaySound3D(instance.strSoundFileName, fx, fy, fz);
                self.play_sound_3d(&instance.sound_file_name, fx, fy, fz);
            }
        }
    }
}
