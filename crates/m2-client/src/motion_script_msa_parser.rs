use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct GroupNode {
    pub name: String,
    pub keys: HashMap<String, Vec<String>>,
    pub children: Vec<GroupNode>,
}

impl GroupNode {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            keys: HashMap::new(),
            children: Vec::new(),
        }
    }

    pub fn get_token_vector(&self, key: &str) -> Option<&Vec<String>> {
        self.keys.get(&key.to_lowercase())
    }

    pub fn get_token_string(&self, key: &str) -> Option<String> {
        self.get_token_vector(key)
            .and_then(|v| v.first())
            .cloned()
            .map(|s| s.trim_matches('"').to_string())
    }

    pub fn get_token_u32(&self, key: &str) -> Option<u32> {
        self.get_token_vector(key)
            .and_then(|v| v.first())
            .and_then(|s| u32::from_str(s).ok())
    }

    pub fn get_token_i32(&self, key: &str) -> Option<i32> {
        self.get_token_vector(key)
            .and_then(|v| v.first())
            .and_then(|s| i32::from_str(s).ok())
    }

    pub fn get_token_bool(&self, key: &str) -> Option<bool> {
        self.get_token_i32(key).map(|v| v != 0)
    }

    pub fn get_token_float(&self, key: &str) -> Option<f32> {
        self.get_token_vector(key)
            .and_then(|v| v.first())
            .and_then(|s| f32::from_str(s).ok())
    }

    pub fn get_token_position(&self, key: &str) -> Option<[f32; 3]> {
        self.get_token_vector(key).and_then(|v| {
            if v.len() >= 3 {
                let x = f32::from_str(&v[0]).unwrap_or(0.0);
                let y = f32::from_str(&v[1]).unwrap_or(0.0);
                let z = f32::from_str(&v[2]).unwrap_or(0.0);
                Some([x, y, z])
            } else {
                None
            }
        })
    }
}

pub struct ScriptParser;

impl ScriptParser {
    pub fn parse_text(content: &str) -> Result<GroupNode, String> {
        let mut root = GroupNode::new("root");
        let lines: Vec<&str> = content.lines().collect();
        let mut idx = 0;
        
        while idx < lines.len() {
            let line = lines[idx].trim();
            if line.is_empty() || line.starts_with("//") {
                idx += 1;
                continue;
            }
            
            let tokens = Self::tokenize_line(line);
            if tokens.is_empty() {
                idx += 1;
                continue;
            }

            if tokens[0].to_lowercase() == "group" && tokens.len() >= 2 {
                let group_name = &tokens[1];
                let (child_node, next_idx) = Self::parse_group_block(&lines, idx + 1, group_name)?;
                root.children.push(child_node);
                idx = next_idx;
            } else {
                let key = tokens[0].to_lowercase();
                let values: Vec<String> = tokens.into_iter().skip(1).collect();
                root.keys.insert(key, values);
                idx += 1;
            }
        }
        
        Ok(root)
    }
    
    fn tokenize_line(line: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut in_quotes = false;
        let mut current = String::new();
        
        // Naive comment strip
        let line_content = if let Some(idx) = line.find("//") {
            &line[0..idx]
        } else {
            line
        };

        let chars: Vec<char> = line_content.trim().chars().collect();
        let mut i = 0;
        while i < chars.len() {
            let c = chars[i];
            
            if c == '"' {
                in_quotes = !in_quotes;
                current.push(c);
            } else if c.is_whitespace() && !in_quotes {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
            } else {
                current.push(c);
            }
            i += 1;
        }
        if !current.is_empty() {
            tokens.push(current);
        }
        tokens
    }

    fn parse_group_block(lines: &[&str], start_idx: usize, name: &str) -> Result<(GroupNode, usize), String> {
        let mut node = GroupNode::new(name);
        let mut idx = start_idx;
        
        while idx < lines.len() && lines[idx].trim().is_empty() {
            idx += 1;
        }
        
        if idx >= lines.len() || !lines[idx].trim().starts_with('{') {
            return Err(format!("Expected '{{' for group {}", name));
        }
        idx += 1;
        
        while idx < lines.len() {
            let mut line = lines[idx].trim();
            if let Some(comment_idx) = line.find("//") {
                line = line[0..comment_idx].trim();
            }
            
            if line.is_empty() {
                idx += 1;
                continue;
            }

            if line == "}" {
                return Ok((node, idx + 1));
            }

            let tokens = Self::tokenize_line(line);
            if tokens.is_empty() {
                idx += 1;
                continue;
            }

            if tokens[0].to_lowercase() == "group" && tokens.len() >= 2 {
                let group_name = &tokens[1];
                let (child_node, next_idx) = Self::parse_group_block(lines, idx + 1, group_name)?;
                node.children.push(child_node);
                idx = next_idx;
            } else {
                let key = tokens[0].to_lowercase();
                let values: Vec<String> = tokens.into_iter().skip(1).collect();
                node.keys.insert(key, values);
                idx += 1;
            }
        }
        
        Err(format!("Unclosed group {}", name))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MotionEventType {
    None = 0,
    Effect = 1,
    ScreenWaving = 2,
    ScreenFlashing = 3,
    SpecialAttacking = 4,
    Sound = 5,
    Fly = 6,
    CharacterShow = 7,
    CharacterHide = 8,
    Warp = 9,
    EffectToTarget = 10,
}

impl From<i32> for MotionEventType {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Effect,
            2 => Self::ScreenWaving,
            3 => Self::ScreenFlashing,
            4 => Self::SpecialAttacking,
            5 => Self::Sound,
            6 => Self::Fly,
            7 => Self::CharacterShow,
            8 => Self::CharacterHide,
            9 => Self::Warp,
            10 => Self::EffectToTarget,
            _ => Self::None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ComboInputData {
    pub pre_input_time: f32,
    pub next_combo_time: f32,
    pub input_end_time: f32,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct HitTimePosition {
    pub time: f32,
    pub last_position: [f32; 3],
    pub position: [f32; 3],
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SphereData {
    pub radius: f32,
    pub position: [f32; 3],
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CollisionData {
    pub collision_type: i32,
    pub sphere_data: Vec<SphereData>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct HitData {
    pub attack_start_time: f32,
    pub attack_end_time: f32,
    pub weapon_length: f32,
    pub bone_name: String,
    pub hit_positions: Vec<HitTimePosition>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MotionAttackData {
    pub attack_type: i32,
    pub hitting_type: i32,
    pub stiffen_time: f32,
    pub invisible_time: f32,
    pub external_force: f32,
    pub hit_limit_count: i32,
    pub motion_type: i32,
    pub hit_data: Vec<HitData>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct LoopData {
    pub motion_loop_count: i32,
    pub loop_cancel_enable: bool,
    pub loop_start_time: f32,
    pub loop_end_time: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MotionEventData {
    Sound {
        frame: usize,
        starting_time: f32,
        file_name: String,
    },
    Effect {
        frame: usize,
        starting_time: f32,
        is_independent: bool,
        is_attaching: bool,
        is_following: bool,
        attaching_bone_name: String,
        effect_file_name: String,
        effect_position: [f32; 3],
    },
    EffectToTarget {
        frame: usize,
        starting_time: f32,
        effect_file_name: String,
        effect_position: [f32; 3],
        is_following: bool,
        is_fishing_effect: bool,
    },
    Fly {
        frame: usize,
        starting_time: f32,
        is_attaching: bool,
        attaching_bone_name: String,
        fly_file_name: String,
        fly_position: [f32; 3],
    },
    ScreenWaving {
        frame: usize,
        starting_time: f32,
        duration_time: f32,
        power: i32,
        affecting_range: i32,
    },
    ScreenFlashing {
        frame: usize,
        starting_time: f32,
    },
    CharacterShow {
        frame: usize,
        starting_time: f32,
    },
    CharacterHide {
        frame: usize,
        starting_time: f32,
    },
    Warp {
        frame: usize,
        starting_time: f32,
    },
    SpecialAttacking {
        frame: usize,
        starting_time: f32,
        duration_time: f32,
        is_enable_hit_process: bool,
        attack_data: MotionAttackData,
        collision_data: Option<CollisionData>,
    },
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MotionScript {
    pub motion_duration: f32,
    pub accumulation: Option<[f32; 3]>,
    
    pub combo_input_data: Option<ComboInputData>,
    pub attacking_data: Option<MotionAttackData>,
    pub loop_data: Option<LoopData>,
    
    pub events: Vec<MotionEventData>,
}


pub fn parse_msa_script(content: &str) -> Result<MotionScript, String> {
    let root = ScriptParser::parse_text(content)?;
    let mut script = MotionScript::default();
    
    // c_fFrameTime = 1.0f / g_fGameFPS; (Metin2 uses 60 FPS)
    let c_f_frame_time = 1.0 / 60.0_f32;

    if let Some(val) = root.get_token_float("motionduration") {
        script.motion_duration = val;
    }
    
    if let Some(val) = root.get_token_position("accumulation") {
        script.accumulation = Some(val);
    }
    
    for child in &root.children {
        let child_name = child.name.to_lowercase();
        
        if child_name == "comboinputdata" {
            let mut combo = ComboInputData::default();
            if let Some(val) = child.get_token_float("preinputtime") {
                combo.pre_input_time = val;
            }
            if let Some(val) = child.get_token_float("directinputtime") {
                combo.next_combo_time = val;
            }
            if let Some(val) = child.get_token_float("inputlimittime") {
                combo.input_end_time = val;
            }
            script.combo_input_data = Some(combo);
        }
        else if child_name == "attackingdata" {
            let mut attack = MotionAttackData::default();
            attack.attack_type = child.get_token_i32("attacktype").unwrap_or(0); // ATTACK_TYPE_SPLASH
            attack.hitting_type = child.get_token_i32("hittingtype").unwrap_or(0);
            attack.stiffen_time = child.get_token_float("stiffentime").unwrap_or(0.0);
            attack.invisible_time = child.get_token_float("invisibletime").unwrap_or(0.0);
            attack.external_force = child.get_token_float("externalforce").unwrap_or(0.0);
            attack.hit_limit_count = child.get_token_i32("hitlimitcount").unwrap_or(0);
            
            attack.motion_type = child.get_token_i32("motiontype")
                .or_else(|| child.get_token_i32("attackingtype"))
                .unwrap_or(0);

            for hit_child in &child.children {
                if hit_child.name.to_lowercase() == "hitdata" || hit_child.name.to_lowercase().starts_with("hitdata") {
                    let mut hit = HitData::default();
                    hit.attack_start_time = hit_child.get_token_float("attackingstarttime").unwrap_or(0.0);
                    hit.attack_end_time = hit_child.get_token_float("attackingendtime").unwrap_or(0.0);
                    hit.bone_name = hit_child.get_token_string("attackingbone").unwrap_or_default();
                    hit.weapon_length = hit_child.get_token_float("weaponlength").unwrap_or(0.0);

                    if let Some(tv) = hit_child.get_token_vector("hitposition") {
                        let mut i = 0;
                        while i + 6 < tv.len() {
                            let time = f32::from_str(&tv[i]).unwrap_or(0.0);
                            let lx = f32::from_str(&tv[i+1]).unwrap_or(0.0);
                            let ly = f32::from_str(&tv[i+2]).unwrap_or(0.0);
                            let lz = f32::from_str(&tv[i+3]).unwrap_or(0.0);
                            let px = f32::from_str(&tv[i+4]).unwrap_or(0.0);
                            let py = f32::from_str(&tv[i+5]).unwrap_or(0.0);
                            let pz = f32::from_str(&tv[i+6]).unwrap_or(0.0);
                            
                            hit.hit_positions.push(HitTimePosition {
                                time,
                                last_position: [lx, ly, lz],
                                position: [px, py, pz],
                            });
                            i += 7;
                        }
                    }
                    attack.hit_data.push(hit);
                }
            }
            // If no explicit hitdatacount/children, Metin2 falls back to reading flat props in attack node 
            // if it were flat, but we will assume it's structured properly or empty.
            if attack.hit_data.is_empty() && child.get_token_float("attackingstarttime").is_some() {
                let mut hit = HitData::default();
                hit.attack_start_time = child.get_token_float("attackingstarttime").unwrap_or(0.0);
                hit.attack_end_time = child.get_token_float("attackingendtime").unwrap_or(0.0);
                hit.bone_name = child.get_token_string("attackingbone").unwrap_or_default();
                hit.weapon_length = child.get_token_float("weaponlength").unwrap_or(0.0);
                attack.hit_data.push(hit);
            }

            script.attacking_data = Some(attack);
        }
        else if child_name == "loopdata" {
            let mut loop_data = LoopData::default();
            loop_data.motion_loop_count = child.get_token_i32("motionloopcount").unwrap_or(-1);
            loop_data.loop_cancel_enable = child.get_token_bool("loopcancelenable").unwrap_or(false);
            loop_data.loop_start_time = child.get_token_float("loopstarttime").unwrap_or(0.0);
            loop_data.loop_end_time = child.get_token_float("loopendtime").unwrap_or(0.0);
            script.loop_data = Some(loop_data);
        }
        else if child_name == "motioneventdata" {
            for ev_child in &child.children {
                if ev_child.name.to_lowercase() == "event" || ev_child.name.to_lowercase().starts_with("event") {
                    let event_type_raw = ev_child.get_token_i32("motioneventtype").unwrap_or(0);
                    let starting_time = ev_child.get_token_float("startingtime").unwrap_or(0.0);
                    let frame = (starting_time / c_f_frame_time).round() as usize;

                    let ev_type = MotionEventType::from(event_type_raw);
                    match ev_type {
                        MotionEventType::Sound => {
                            let file_name = ev_child.get_token_string("soundfilename").unwrap_or_default();
                            script.events.push(MotionEventData::Sound {
                                frame,
                                starting_time,
                                file_name,
                            });
                        }
                        MotionEventType::Effect => {
                            let is_independent = ev_child.get_token_bool("independentflag").unwrap_or(false);
                            let is_attaching = ev_child.get_token_bool("attachingenable").unwrap_or(false);
                            let attaching_bone_name = ev_child.get_token_string("attachingbonename").unwrap_or_default();
                            let effect_file_name = ev_child.get_token_string("effectfilename").unwrap_or_default();
                            let effect_position = ev_child.get_token_position("effectposition").unwrap_or([0.0, 0.0, 0.0]);
                            let is_following = ev_child.get_token_bool("followingenable").unwrap_or(false);

                            script.events.push(MotionEventData::Effect {
                                frame,
                                starting_time,
                                is_independent,
                                is_attaching,
                                is_following,
                                attaching_bone_name,
                                effect_file_name,
                                effect_position,
                            });
                        }
                        MotionEventType::Fly => {
                            let is_attaching = ev_child.get_token_bool("attachingenable").unwrap_or(false);
                            let attaching_bone_name = ev_child.get_token_string("attachingbonename").unwrap_or_default();
                            let fly_file_name = ev_child.get_token_string("flyfilename").unwrap_or_default();
                            let fly_position = ev_child.get_token_position("flyposition").unwrap_or([0.0, 0.0, 0.0]);

                            script.events.push(MotionEventData::Fly {
                                frame,
                                starting_time,
                                is_attaching,
                                attaching_bone_name,
                                fly_file_name,
                                fly_position,
                            });
                        }
                        MotionEventType::ScreenWaving => {
                            let duration_time = ev_child.get_token_float("duringtime").unwrap_or(0.0);
                            let power = ev_child.get_token_i32("power").unwrap_or(0);
                            let affecting_range = ev_child.get_token_i32("affectingrange").unwrap_or(0);

                            script.events.push(MotionEventData::ScreenWaving {
                                frame,
                                starting_time,
                                duration_time,
                                power,
                                affecting_range,
                            });
                        }
                        MotionEventType::ScreenFlashing => {
                            script.events.push(MotionEventData::ScreenFlashing {
                                frame,
                                starting_time,
                            });
                        }
                        MotionEventType::SpecialAttacking => {
                            let duration_time = ev_child.get_token_float("duringtime").unwrap_or(0.0);
                            let is_enable_hit_process = ev_child.get_token_bool("enablehitprocess").unwrap_or(true);
                            
                            let mut attack_data = MotionAttackData::default();
                            attack_data.attack_type = ev_child.get_token_i32("attacktype").unwrap_or(0);
                            attack_data.hitting_type = ev_child.get_token_i32("hittingtype").unwrap_or(0);
                            attack_data.stiffen_time = ev_child.get_token_float("stiffentime").unwrap_or(0.0);
                            attack_data.invisible_time = ev_child.get_token_float("invisibletime").unwrap_or(0.0);
                            attack_data.external_force = ev_child.get_token_float("externalforce").unwrap_or(0.0);
                            attack_data.hit_limit_count = ev_child.get_token_i32("hitlimitcount").unwrap_or(0);
                            
                            // Load HitData array inside SpecialAttacking if present
                            for hit_child in &ev_child.children {
                                if hit_child.name.to_lowercase() == "hitdata" || hit_child.name.to_lowercase().starts_with("hitdata") {
                                    let mut hit = HitData::default();
                                    hit.attack_start_time = hit_child.get_token_float("attackingstarttime").unwrap_or(0.0);
                                    hit.attack_end_time = hit_child.get_token_float("attackingendtime").unwrap_or(0.0);
                                    hit.bone_name = hit_child.get_token_string("attackingbone").unwrap_or_default();
                                    hit.weapon_length = hit_child.get_token_float("weaponlength").unwrap_or(0.0);
                                    attack_data.hit_data.push(hit);
                                }
                            }

                            if attack_data.hit_data.is_empty() && ev_child.get_token_float("attackingstarttime").is_some() {
                                let mut hit = HitData::default();
                                hit.attack_start_time = ev_child.get_token_float("attackingstarttime").unwrap_or(0.0);
                                hit.attack_end_time = ev_child.get_token_float("attackingendtime").unwrap_or(0.0);
                                hit.bone_name = ev_child.get_token_string("attackingbone").unwrap_or_default();
                                hit.weapon_length = ev_child.get_token_float("weaponlength").unwrap_or(0.0);
                                attack_data.hit_data.push(hit);
                            }
                            
                            let mut collision_data = None;
                            if let Some(col_type) = ev_child.get_token_i32("collisiontype") {
                                let mut col = CollisionData::default();
                                col.collision_type = col_type;
                                
                                for sph_child in &ev_child.children {
                                    if sph_child.name.to_lowercase() == "spheredata" || sph_child.name.to_lowercase().starts_with("spheredata") {
                                        let mut sph = SphereData::default();
                                        sph.radius = sph_child.get_token_float("radius").unwrap_or(0.0);
                                        sph.position = sph_child.get_token_position("position").unwrap_or([0.0, 0.0, 0.0]);
                                        col.sphere_data.push(sph);
                                    }
                                }
                                collision_data = Some(col);
                            }

                            script.events.push(MotionEventData::SpecialAttacking {
                                frame,
                                starting_time,
                                duration_time,
                                is_enable_hit_process,
                                attack_data,
                                collision_data,
                            });
                        }
                        MotionEventType::CharacterShow => {
                            script.events.push(MotionEventData::CharacterShow { frame, starting_time });
                        }
                        MotionEventType::CharacterHide => {
                            script.events.push(MotionEventData::CharacterHide { frame, starting_time });
                        }
                        MotionEventType::Warp => {
                            script.events.push(MotionEventData::Warp { frame, starting_time });
                        }
                        MotionEventType::EffectToTarget => {
                            let effect_file_name = ev_child.get_token_string("effectfilename").unwrap_or_default();
                            let effect_position = ev_child.get_token_position("effectposition").unwrap_or([0.0, 0.0, 0.0]);
                            let is_following = ev_child.get_token_bool("followingenable").unwrap_or(false);
                            let is_fishing_effect = ev_child.get_token_bool("fishingeffectflag").unwrap_or(false);

                            script.events.push(MotionEventData::EffectToTarget {
                                frame,
                                starting_time,
                                effect_file_name,
                                effect_position,
                                is_following,
                                is_fishing_effect,
                            });
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    Ok(script)
}

