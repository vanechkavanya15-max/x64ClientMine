#[derive(Debug, Clone, PartialEq)]
pub enum MotionEventData {
    Sound {
        frame: usize,
        file: String,
    },
    Effect {
        target_bone: String,
        file: String,
    },
    Fly {
        file: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct MotionScript {
    pub motion_duration: f32,
    pub accumulation: Option<[f32; 3]>,
    pub events: Vec<MotionEventData>,
}

pub fn parse_msa_script(content: &str) -> Result<MotionScript, String> {
    let mut script = MotionScript {
        motion_duration: 0.0,
        accumulation: None,
        events: Vec::new(),
    };

    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();
        
        if line.is_empty() || line.starts_with("//") {
            i += 1;
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        let key = parts[0].to_lowercase();

        if key == "motionduration" && parts.len() > 1 {
            script.motion_duration = parts[1].parse().unwrap_or(0.0);
        } else if key == "accumulation" && parts.len() > 3 {
            script.accumulation = Some([
                parts[1].parse().unwrap_or(0.0),
                parts[2].parse().unwrap_or(0.0),
                parts[3].parse().unwrap_or(0.0),
            ]);
        } else if key == "group" && parts.len() > 1 && parts[1].to_lowercase() == "motioneventdata" {
            // Find opening brace
            i += 1;
            while i < lines.len() && lines[i].trim() != "{" {
                i += 1;
            }
            i += 1; // skip {

            while i < lines.len() && lines[i].trim() != "}" {
                let inner_line = lines[i].trim();
                let inner_parts: Vec<&str> = inner_line.split_whitespace().collect();
                
                if !inner_parts.is_empty() && inner_parts[0].to_lowercase() == "group" {
                    // Start of an event group
                    i += 1;
                    while i < lines.len() && lines[i].trim() != "{" {
                        i += 1;
                    }
                    i += 1; // skip {

                    let mut event_type = -1;
                    let mut starting_time = 0.0;
                    let mut file_name = String::new();
                    let mut attaching_bone_name = String::new();

                    while i < lines.len() && lines[i].trim() != "}" {
                        let ev_line = lines[i].trim();
                        let ev_parts: Vec<&str> = ev_line.split_whitespace().collect();
                        
                        if !ev_parts.is_empty() {
                            let k = ev_parts[0].to_lowercase();
                            if k == "motioneventtype" && ev_parts.len() > 1 {
                                event_type = ev_parts[1].parse().unwrap_or(-1);
                            } else if k == "startingtime" && ev_parts.len() > 1 {
                                starting_time = ev_parts[1].parse().unwrap_or(0.0);
                            } else if k == "soundfilename" {
                                if ev_parts.len() > 1 {
                                    file_name = ev_line["SoundFileName".len()..].trim().trim_matches('"').to_string();
                                }
                            } else if k == "effectfilename" {
                                if ev_parts.len() > 1 {
                                    file_name = ev_line["EffectFileName".len()..].trim().trim_matches('"').to_string();
                                }
                            } else if k == "flyfilename" {
                                if ev_parts.len() > 1 {
                                    file_name = ev_line["FlyFileName".len()..].trim().trim_matches('"').to_string();
                                }
                            } else if k == "attachingbonename" && ev_parts.len() > 1 {
                                attaching_bone_name = ev_line["AttachingBoneName".len()..].trim().trim_matches('"').to_string();
                            }
                        }
                        i += 1;
                    }
                    
                    // Approximate frame rate of 30 fps
                    let frame = (starting_time / (1.0 / 30.0_f32)).round() as usize;

                    // MOTIONSOUND = 5, MOTIONEFFECT = 1, MOTIONFLY = 6
                    if event_type == 5 {
                        script.events.push(MotionEventData::Sound {
                            frame,
                            file: file_name,
                        });
                    } else if event_type == 1 {
                        script.events.push(MotionEventData::Effect {
                            target_bone: attaching_bone_name,
                            file: file_name,
                        });
                    } else if event_type == 6 {
                        script.events.push(MotionEventData::Fly {
                            file: file_name,
                        });
                    }
                }
                i += 1;
            }
        }
        i += 1;
    }

    Ok(script)
}
