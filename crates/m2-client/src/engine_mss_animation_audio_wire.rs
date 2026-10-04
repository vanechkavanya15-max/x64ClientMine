use std::collections::HashMap;
use std::ffi::CString;

use crate::sound_ffi::m2_audio_play_sound_3d;

#[derive(Debug, Clone, PartialEq)]
pub struct MssSoundInstance {
    pub frame: u32,
    pub sound_file_name: String,
}

pub fn parse_mss_file(content: &str, fps: u32) -> Result<Vec<MssSoundInstance>, String> {
    let mut instances = Vec::new();
    let frame_time = 1.0 / (fps as f32);

    let mut count: Option<usize> = None;
    
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        
        let lower = trimmed.to_lowercase();
        if lower.starts_with("sounddatacount") {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Ok(c) = parts[1].parse::<usize>() {
                    count = Some(c);
                    break;
                }
            }
        }
    }

    let _expected_count = match count {
        Some(c) => c,
        None => return Err("File format error, SoundDataCount Unable to find.".to_string()),
    };

    // Now look for SoundDataXX
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        
        let lower = trimmed.to_lowercase();
        if lower.starts_with("sounddata") && !lower.starts_with("sounddatacount") {
            // Format: SoundData00       0.533333 "sound/pc/warrior/skill/samyeon_v.wav"
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 3 {
                if let Ok(time) = parts[1].parse::<f32>() {
                    // Extract the string which is usually in quotes
                    let mut file_name = parts[2..].join(" ");
                    if file_name.starts_with('"') && file_name.ends_with('"') {
                        file_name = file_name[1..file_name.len() - 1].to_string();
                    }
                    
                    let frame = (time / frame_time) as u32;
                    instances.push(MssSoundInstance {
                        frame,
                        sound_file_name: file_name,
                    });
                }
            }
        }
    }

    Ok(instances)
}

pub struct MssAudioWire {
    play_sound_history_map: HashMap<String, f32>,
}

impl Default for MssAudioWire {
    fn default() -> Self {
        Self::new()
    }
}

impl MssAudioWire {
    pub fn new() -> Self {
        Self {
            play_sound_history_map: HashMap::new(),
        }
    }

    pub fn process_sound_events(
        &mut self,
        x: f32,
        y: f32,
        z: f32,
        current_frame: u32,
        instances: &[MssSoundInstance],
        check_frequency: bool,
        current_time_seconds: f32,
    ) {
        for instance in instances {
            if instance.frame == current_frame {
                if check_frequency {
                    let last_play = self
                        .play_sound_history_map
                        .get(&instance.sound_file_name)
                        .copied()
                        .unwrap_or(0.0);

                    // 0.3s cooldown
                    if current_time_seconds - last_play > 0.0 && current_time_seconds - last_play < 0.3 {
                        continue;
                    }

                    self.play_sound_history_map
                        .insert(instance.sound_file_name.clone(), current_time_seconds);
                }

                if let Ok(c_str) = CString::new(instance.sound_file_name.clone()) {
                    m2_audio_play_sound_3d(c_str.as_ptr(), x, y, z, 1.0);
                }
            }
        }
    }
}
