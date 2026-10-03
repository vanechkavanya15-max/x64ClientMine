use m2_client::msa_script_parser_v2::{parse_msa_data, MotionEventData};
use m2_client::sound_ffi::m2_audio_play_sound_3d;
use std::ffi::CString;

pub struct AnimationSoundSync {
    pub sound_events: Vec<(f32, String)>,
    pub current_time: f32,
}

impl AnimationSoundSync {
    pub fn new() -> Self {
        Self {
            sound_events: Vec::new(),
            current_time: 0.0,
        }
    }

    pub fn load_from_msa(&mut self, msa_content: &str) -> Result<(), &'static str> {
        self.sound_events.clear();
        self.current_time = 0.0;
        
        let motion_data = parse_msa_data(msa_content)?;
        
        for event in motion_data.events {
            if let MotionEventData::Sound { starting_time, sound_file_name } = event {
                self.sound_events.push((starting_time, sound_file_name));
            }
        }
        
        Ok(())
    }

    pub fn reset(&mut self) {
        self.current_time = 0.0;
    }

    pub fn update(&mut self, delta_time: f32, x: f32, y: f32, z: f32, volume: f32) {
        let old_time = self.current_time;
        self.current_time += delta_time;
        
        for (starting_time, sound_file_name) in &self.sound_events {
            // Check if we crossed the starting_time in this frame
            if old_time <= *starting_time && self.current_time > *starting_time {
                if let Ok(c_str) = CString::new(sound_file_name.clone()) {
                    m2_audio_play_sound_3d(c_str.as_ptr(), x, y, z, volume);
                }
            }
        }
    }
}
