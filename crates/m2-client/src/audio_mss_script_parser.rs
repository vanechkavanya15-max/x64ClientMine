use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::fs;

#[derive(Debug, Clone, PartialEq)]
pub struct SoundData {
    pub time: f32,
    pub file_name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SoundScriptInstance {
    pub frame: u32,
    pub file_name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MssScript {
    pub data: Vec<SoundData>,
    pub instances: Vec<SoundScriptInstance>,
}

#[derive(Debug)]
pub enum MssParserError {
    IoError(String),
    FormatError(String),
}

impl std::fmt::Display for MssParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MssParserError::IoError(msg) => write!(f, "IO Error: {}", msg),
            MssParserError::FormatError(msg) => write!(f, "Format Error: {}", msg),
        }
    }
}

impl std::error::Error for MssParserError {}

pub struct AudioMssScriptParser {
    cache: Mutex<HashMap<String, Arc<MssScript>>>,
}

impl AudioMssScriptParser {
    pub fn new() -> Self {
        Self {
            cache: Mutex::new(HashMap::new()),
        }
    }

    pub fn load_sound_script(&self, file_name: &str, path_header: Option<&str>) -> Result<Arc<MssScript>, MssParserError> {
        let cache_key = match path_header {
            Some(header) => format!("{}::{}", header, file_name),
            None => file_name.to_string(),
        };

        {
            let cache = self.cache.lock().unwrap();
            if let Some(script) = cache.get(&cache_key) {
                return Ok(script.clone());
            }
        }

        let content = fs::read_to_string(file_name)
            .map_err(|e| MssParserError::IoError(format!("Failed to open {}: {}", file_name, e)))?;

        let script = Self::parse_mss_content(&content, path_header)?;
        let script_arc = Arc::new(script);

        {
            let mut cache = self.cache.lock().unwrap();
            cache.insert(cache_key, script_arc.clone());
        }

        Ok(script_arc)
    }
    
    pub fn parse_mss_content(content: &str, path_header: Option<&str>) -> Result<MssScript, MssParserError> {
        let mut sound_data_count: Option<usize> = None;
        let mut data_map: HashMap<usize, SoundData> = HashMap::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            let key = parts[0].to_lowercase();

            if key == "sounddatacount" {
                if parts.len() < 2 {
                    return Err(MssParserError::FormatError("SoundDataCount without value".to_string()));
                }
                let count = parts[1].parse::<usize>().map_err(|_| {
                    MssParserError::FormatError(format!("Invalid SoundDataCount value: {}", parts[1]))
                })?;
                sound_data_count = Some(count);
                continue;
            }

            if key.starts_with("sounddata") && key != "sounddatacount" {
                let index_str = &key["sounddata".len()..];
                let index = index_str.parse::<usize>().map_err(|_| {
                    MssParserError::FormatError(format!("Invalid sounddata index: {}", key))
                })?;

                if parts.len() < 3 {
                    return Err(MssParserError::FormatError(format!("Incomplete sounddata line: {}", line)));
                }

                let time = parts[1].parse::<f32>().map_err(|_| {
                    MssParserError::FormatError(format!("Invalid time value: {}", parts[1]))
                })?;

                // Handle quotes for the file name
                let mut name_start = -1;
                let mut name_end = -1;
                
                if let Some(start_quote) = line.find('"') {
                    if let Some(end_quote) = line[start_quote + 1..].find('"') {
                        name_start = start_quote as i32 + 1;
                        name_end = (start_quote + 1 + end_quote) as i32;
                    }
                }
                
                let raw_file_name = if name_start != -1 && name_end != -1 {
                    &line[name_start as usize..name_end as usize]
                } else {
                    parts[2]
                };

                let file_name = if let Some(header) = path_header {
                    format!("{}{}", header, raw_file_name)
                } else {
                    raw_file_name.to_string()
                };

                data_map.insert(index, SoundData { time, file_name });
            }
        }

        let count = sound_data_count.ok_or_else(|| {
            MssParserError::FormatError("SoundDataCount missing".to_string())
        })?;

        let mut data = Vec::with_capacity(count);
        for i in 0..count {
            let sound_data = data_map.remove(&i).ok_or_else(|| {
                MssParserError::FormatError(format!("Missing sounddata{:02}", i))
            })?;
            data.push(sound_data);
        }

        let fps = 60.0;
        let frame_time = 1.0 / fps;
        
        let instances = data.iter().map(|d| {
            SoundScriptInstance {
                frame: (d.time / frame_time).round() as u32,
                file_name: d.file_name.clone(),
            }
        }).collect();

        Ok(MssScript { data, instances })
    }
}

pub struct MssSoundTrigger {
    last_frame: Option<u32>,
    script: Arc<MssScript>,
}

impl MssSoundTrigger {
    pub fn new(script: Arc<MssScript>) -> Self {
        Self {
            last_frame: None,
            script,
        }
    }

    pub fn reset(&mut self) {
        self.last_frame = None;
    }

    pub fn update(&mut self, current_frame: u32) -> Vec<String> {
        let mut triggered_sounds = Vec::new();
        
        if self.script.instances.is_empty() {
            self.last_frame = Some(current_frame);
            return triggered_sounds;
        }

        let last_frame = self.last_frame.unwrap_or(0);

        if current_frame > last_frame {
            // Normal progression
            for instance in &self.script.instances {
                if instance.frame > last_frame && instance.frame <= current_frame {
                    triggered_sounds.push(instance.file_name.clone());
                }
            }
        } else if current_frame < last_frame {
            // Animation looped
            // Trigger remaining sounds from last frame to max (assume no upper limit known easily, 
            // so we just trigger up to max u32, practically the end of animation)
            for instance in &self.script.instances {
                if instance.frame > last_frame {
                    triggered_sounds.push(instance.file_name.clone());
                }
            }
            // Trigger from 0 to current frame
            for instance in &self.script.instances {
                if instance.frame <= current_frame {
                    triggered_sounds.push(instance.file_name.clone());
                }
            }
        } else if self.last_frame.is_none() {
            // First time update, handle all frames up to current_frame
            for instance in &self.script.instances {
                if instance.frame <= current_frame {
                    triggered_sounds.push(instance.file_name.clone());
                }
            }
        }

        self.last_frame = Some(current_frame);
        triggered_sounds
    }
}
