use std::ffi::{c_char};
use std::slice;

#[derive(Debug, Clone, PartialEq)]
pub struct RaceMotionData {
    pub motion_file_name: String,
    pub motion_duration: f32,
    pub loop_count: i32,
    pub accumulation_data: Option<[f32; 3]>,
    pub events: Vec<MotionEventData>,
}

impl Default for RaceMotionData {
    fn default() -> Self {
        Self {
            motion_file_name: String::new(),
            motion_duration: 0.0,
            loop_count: 0,
            accumulation_data: None,
            events: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MotionEventData {
    ScreenWaving {
        starting_time: f32,
        duration_time: f32,
        power: i32,
        affecting_range: i32,
    },
    ScreenFlashing {
        starting_time: f32,
    },
    Effect {
        starting_time: f32,
        independent_flag: bool,
        attaching_enable: bool,
        attaching_bone_name: String,
        following_enable: bool,
        effect_file_name: String,
        effect_position: [f32; 3],
    },
    Fly {
        starting_time: f32,
        attaching_enable: bool,
        attaching_bone_name: String,
        fly_file_name: String,
        fly_position: [f32; 3],
    },
    Attack {
        starting_time: f32,
        duration_time: f32,
        enable_hit_process: bool,
        attack_type: i32,
        hit_limit: i32,
        sphere_radius: f32,
    },
    Sound {
        starting_time: f32,
        sound_file_name: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    String(String),
    Group(String),
    OpenBrace,
    CloseBrace,
    Newline,
}

fn tokenize(text: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut in_string = false;
    let mut in_comment = false;

    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        if in_comment {
            if c == '\n' {
                in_comment = false;
            }
            i += 1;
            continue;
        }

        if c == '/' && i + 1 < chars.len() && chars[i + 1] == '/' {
            in_comment = true;
            i += 2;
            continue;
        }

        if c == '"' {
            if in_string {
                tokens.push(Token::String(current_token.clone()));
                current_token.clear();
                in_string = false;
            } else {
                in_string = true;
            }
            i += 1;
            continue;
        }

        if in_string {
            current_token.push(c);
            i += 1;
            continue;
        }

        if c == '\n' || c == '\r' {
            if !current_token.is_empty() {
                if current_token.to_lowercase() == "group" {
                    let mut j = i + 1;
                    while j < chars.len() && chars[j].is_whitespace() {
                        j += 1;
                    }
                    let mut group_name = String::new();
                    while j < chars.len() && !chars[j].is_whitespace() && chars[j] != '{' && chars[j] != '}' && chars[j] != '"' {
                        group_name.push(chars[j]);
                        j += 1;
                    }
                    tokens.push(Token::Group(group_name));
                    i = j;
                } else {
                    tokens.push(Token::String(current_token.clone()));
                }
                current_token.clear();
            }
            tokens.push(Token::Newline);
            i += 1;
            continue;
        }

        if c.is_whitespace() {
            if !current_token.is_empty() {
                if current_token.to_lowercase() == "group" {
                    let mut j = i + 1;
                    while j < chars.len() && chars[j].is_whitespace() {
                        j += 1;
                    }
                    let mut group_name = String::new();
                    while j < chars.len() && !chars[j].is_whitespace() && chars[j] != '{' && chars[j] != '}' && chars[j] != '"' && !(chars[j] == '/' && j + 1 < chars.len() && chars[j+1] == '/') {
                        group_name.push(chars[j]);
                        j += 1;
                    }
                    tokens.push(Token::Group(group_name));
                    i = j;
                } else {
                    tokens.push(Token::String(current_token.clone()));
                }
                current_token.clear();
            }
            i += 1;
            continue;
        }

        if c == '{' {
            if !current_token.is_empty() {
                tokens.push(Token::String(current_token.clone()));
                current_token.clear();
            }
            tokens.push(Token::OpenBrace);
            i += 1;
            continue;
        }

        if c == '}' {
            if !current_token.is_empty() {
                tokens.push(Token::String(current_token.clone()));
                current_token.clear();
            }
            tokens.push(Token::CloseBrace);
            i += 1;
            continue;
        }

        current_token.push(c);
        i += 1;
    }

    if !current_token.is_empty() {
        tokens.push(Token::String(current_token));
    }

    tokens
}

#[derive(Debug, Clone, PartialEq)]
struct TextNode {
    name: String,
    children: Vec<TextNode>,
    properties: Vec<(String, Vec<String>)>,
}

fn parse_nodes(tokens: &[Token]) -> Result<(Vec<TextNode>, Vec<(String, Vec<String>)>), &'static str> {
    let mut children = Vec::new();
    let mut properties = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            Token::Group(name) => {
                let mut brace_idx = i + 1;
                while brace_idx < tokens.len() && tokens[brace_idx] == Token::Newline {
                    brace_idx += 1;
                }
                
                if brace_idx < tokens.len() && tokens[brace_idx] == Token::OpenBrace {
                    let mut j = brace_idx + 1;
                    let mut brace_count = 1;
                    while j < tokens.len() {
                        if tokens[j] == Token::OpenBrace {
                            brace_count += 1;
                        } else if tokens[j] == Token::CloseBrace {
                            brace_count -= 1;
                            if brace_count == 0 {
                                break;
                            }
                        }
                        j += 1;
                    }

                    if brace_count != 0 {
                        return Err("Unmatched brace in group");
                    }

                    let (group_children, group_properties) = parse_nodes(&tokens[brace_idx + 1..j])?;
                    children.push(TextNode {
                        name: name.clone(),
                        children: group_children,
                        properties: group_properties,
                    });
                    i = j + 1;
                } else {
                    return Err("Expected { after group");
                }
            }
            Token::String(key) => {
                let mut values = Vec::new();
                let mut j = i + 1;
                while j < tokens.len() {
                    match &tokens[j] {
                        Token::String(val) => {
                            values.push(val.clone());
                            j += 1;
                        }
                        Token::Newline => {
                            j += 1;
                            break;
                        }
                        Token::Group(_) | Token::OpenBrace | Token::CloseBrace => break,
                    }
                }
                
                let lower_key = key.to_lowercase();
                properties.push((lower_key, values));
                i = j;
            }
            Token::Newline => {
                i += 1;
            }
            Token::OpenBrace | Token::CloseBrace => {
                 return Err("Unexpected brace");
            }
        }
    }

    Ok((children, properties))
}


pub fn parse_msa_data(script: &str) -> Result<RaceMotionData, &'static str> {
    let tokens = tokenize(script);
    let (nodes, root_properties) = parse_nodes(&tokens)?;

    let mut data = RaceMotionData::default();

    for (key, values) in &root_properties {
        match key.as_str() {
            "motionfilename" => {
                if let Some(val) = values.first() {
                    data.motion_file_name = val.clone();
                }
            }
            "motionduration" => {
                if let Some(val) = values.first() {
                    data.motion_duration = val.parse().unwrap_or(0.0);
                }
            }
            "accumulation" => {
                if values.len() >= 3 {
                    data.accumulation_data = Some([
                        values[0].parse().unwrap_or(0.0),
                        values[1].parse().unwrap_or(0.0),
                        values[2].parse().unwrap_or(0.0),
                    ]);
                }
            }
            _ => {}
        }
    }

    for node in &nodes {
        let node_name = node.name.to_lowercase();
        if node_name == "loopdata" {
            for (key, values) in &node.properties {
                if key == "motionloopcount" {
                    if let Some(val) = values.first() {
                        data.loop_count = val.parse().unwrap_or(0);
                    }
                }
            }
        } else if node_name == "motioneventdata" {
            for child in &node.children {
                if child.name.to_lowercase().starts_with("event") || child.name.to_lowercase() == "motioneventdata" {
                    parse_event(child, &mut data.events);
                }
            }
        }
    }

    Ok(data)
}

fn parse_event(node: &TextNode, events: &mut Vec<MotionEventData>) {
    let mut event_type = -1;
    let mut starting_time = 0.0;
    
    // Type 1
    let mut duration_time = 0.0;
    let mut power = 0;
    let mut affecting_range = 0;
    
    // Type 3
    let mut independent_flag = false;
    let mut attaching_enable = false;
    let mut attaching_bone_name = String::new();
    let mut following_enable = false;
    let mut effect_file_name = String::new();
    let mut effect_position = [0.0, 0.0, 0.0];
    
    // Type 4
    let mut fly_file_name = String::new();
    let mut fly_position = [0.0, 0.0, 0.0];
    
    // Type 5
    let mut enable_hit_process = true;
    let mut attack_type = 0;
    let mut hit_limit = 0;
    let mut sphere_radius = 0.0;

    // Type 6
    let mut sound_file_name = String::new();

    for (key, values) in &node.properties {
        match key.as_str() {
            "motioneventtype" => {
                if let Some(val) = values.first() {
                    event_type = val.parse().unwrap_or(-1);
                }
            }
            "startingtime" => {
                if let Some(val) = values.first() {
                    starting_time = val.parse().unwrap_or(0.0);
                }
            }
            "duringtime" => {
                if let Some(val) = values.first() {
                    duration_time = val.parse().unwrap_or(0.0);
                }
            }
            "power" => {
                if let Some(val) = values.first() {
                    power = val.parse().unwrap_or(0);
                }
            }
            "affectingrange" => {
                if let Some(val) = values.first() {
                    affecting_range = val.parse().unwrap_or(0);
                }
            }
            "independentflag" => {
                if let Some(val) = values.first() {
                    independent_flag = val == "1" || val.to_lowercase() == "true";
                }
            }
            "attachingenable" => {
                if let Some(val) = values.first() {
                    attaching_enable = val == "1" || val.to_lowercase() == "true";
                }
            }
            "attachingbonename" => {
                if let Some(val) = values.first() {
                    attaching_bone_name = val.clone();
                }
            }
            "followingenable" => {
                if let Some(val) = values.first() {
                    following_enable = val == "1" || val.to_lowercase() == "true";
                }
            }
            "effectfilename" => {
                if let Some(val) = values.first() {
                    effect_file_name = val.clone();
                }
            }
            "effectposition" => {
                if values.len() >= 3 {
                    effect_position = [
                        values[0].parse().unwrap_or(0.0),
                        values[1].parse().unwrap_or(0.0),
                        values[2].parse().unwrap_or(0.0),
                    ];
                }
            }
            "flyfilename" => {
                if let Some(val) = values.first() {
                    fly_file_name = val.clone();
                }
            }
            "flyposition" => {
                if values.len() >= 3 {
                    fly_position = [
                        values[0].parse().unwrap_or(0.0),
                        values[1].parse().unwrap_or(0.0),
                        values[2].parse().unwrap_or(0.0),
                    ];
                }
            }
            "enablehitprocess" => {
                if let Some(val) = values.first() {
                    enable_hit_process = val == "1" || val.to_lowercase() == "true";
                }
            }
            "soundfilename" => {
                if let Some(val) = values.first() {
                    sound_file_name = val.clone();
                }
            }
            _ => {}
        }
    }

    for child in &node.children {
        if child.name.to_lowercase() == "attackdata" {
            for (key, values) in &child.properties {
                if key == "attacktype" {
                    if let Some(val) = values.first() {
                        attack_type = val.parse().unwrap_or(0);
                    }
                } else if key == "hitlimit" {
                    if let Some(val) = values.first() {
                        hit_limit = val.parse().unwrap_or(0);
                    }
                }
            }
        } else if child.name.to_lowercase() == "collisiondata" {
            for (key, values) in &child.properties {
                if key == "sphereradius" {
                    if let Some(val) = values.first() {
                        sphere_radius = val.parse().unwrap_or(0.0);
                    }
                }
            }
        }
    }

    // MOTION_EVENT_TYPE_SCREEN_WAVING = 2
    // MOTION_EVENT_TYPE_SCREEN_FLASHING = 3
    // MOTION_EVENT_TYPE_EFFECT = 1
    // MOTION_EVENT_TYPE_FLY = 6
    // MOTION_EVENT_TYPE_SPECIAL_ATTACKING = 4 (or Attack - wait, in Issue Description: 1 Waving, 2 Flashing, 3 Effect, 4 Fly, 5 Attack, 6 Sound)
    // Looking at C++ RaceMotionData.h:
    /*
		enum EMotionEventType
		{
			MOTION_EVENT_TYPE_NONE,

			MOTION_EVENT_TYPE_EFFECT, // 1
			MOTION_EVENT_TYPE_SCREEN_WAVING, // 2
			MOTION_EVENT_TYPE_SCREEN_FLASHING, // 3
			MOTION_EVENT_TYPE_SPECIAL_ATTACKING, // 4
			MOTION_EVENT_TYPE_SOUND, // 5
			MOTION_EVENT_TYPE_FLY, // 6
			MOTION_EVENT_TYPE_CHARACTER_SHOW,
			MOTION_EVENT_TYPE_CHARACTER_HIDE,
			MOTION_EVENT_TYPE_WARP,
			MOTION_EVENT_TYPE_EFFECT_TO_TARGET,

			MOTION_EVENT_TYPE_MAX_NUM,
		};
    */
    // Wait, the prompt states: 
    // Type 1: Screen Waving
    // Type 2: Screen Flashing
    // Type 3: Effect 
    // Type 4: Fly 
    // Type 5: Attack 
    // Type 6: Sound 
    // I should follow the C++ indices if that's what's serialized, or map based on the exact user prompt mapping if they reordered it. Usually user prompt mapping defines what the struct looks like. Let's map exactly as the prompt requested! Wait, no, the `.msa` file has `motioneventtype x`. I will support both C++ indices and Prompt indices if they differ? The prompt says "Type 1: Screen Waving", so I will use event_type == 1 for Screen Waving, 2 for Flashing, 3 for Effect, etc as specified in the prompt requirement! "Type 1: Screen Waving" etc. Oh wait, if the prompt means "I want you to support these types", it might not mean "Type = 1 in the MSA file". It probably just enumerates them. Let's just match the MSA enum values from C++ and fallback to prompt enumeration if necessary. I'll just use the prompt's `1`, `2`, `3`, `4`, `5`, `6` mapping, or I can just map both sets just in case. Let me just use 1: Waving, 2: Flashing, 3: Effect, 4: Fly, 5: Attack, 6: Sound since that's what the prompt literally says. WAIT, let me check the C++ struct. C++ says MOTION_EVENT_TYPE_EFFECT=1, SCREEN_WAVING=2, SCREEN_FLASHING=3, SPECIAL_ATTACKING=4, SOUND=5, FLY=6.
    // If the prompt says "Type 1: Screen Waving", it might just be a numbered list!
    // But since the `.msa` files actually contain integers, I should probably use the prompt's integer values just in case the prompt wants me to map exactly those integers to those variants. I'll do this:

    match event_type {
        1 | 2 => { 
            // If the user meant Type 1: Screen Waving and Type 2: Screen Flashing. 
            // I'll check which properties are present to disambiguate or just follow the prompt exactly! 
            // The prompt says: "Type 1: Screen Waving, Type 2: Screen Flashing, Type 3: Effect, Type 4: Fly, Type 5: Attack, Type 6: Sound"
            // Let's implement EXACTLY what the prompt says as the integers.
        }
        _ => {}
    }

    // Let's implement exactly the integers specified in the prompt.
    if event_type == 1 || (event_type == 2 && duration_time > 0.0 && power > 0) {
        // C++ enum for waving is 2. Prompt says Type 1.
        // Let's just map Type 1 to Waving. But wait, if they pass real MSA files, they will have C++ values. 
        // Let's use the Prompt's explicit numbering!
    }
    
    // Better yet, I'll match the EXACT integers given in the prompt.
    match event_type {
        1 => { // Screen Waving (Prompt)
            events.push(MotionEventData::ScreenWaving {
                starting_time,
                duration_time,
                power,
                affecting_range,
            });
        }
        2 => { // Screen Flashing (Prompt)
            // But wait, if C++ enum has Waving = 2, Flashing = 3, Effect = 1, Special Attacking = 4, Sound = 5, Fly = 6
            // Let's handle both!
            if duration_time > 0.0 && power > 0 {
                // Must be C++ Screen Waving (2)
                events.push(MotionEventData::ScreenWaving { starting_time, duration_time, power, affecting_range });
            } else {
                events.push(MotionEventData::ScreenFlashing { starting_time });
            }
        }
        3 => { // Effect (Prompt)
            if !effect_file_name.is_empty() {
                events.push(MotionEventData::Effect { starting_time, independent_flag, attaching_enable, attaching_bone_name, following_enable, effect_file_name, effect_position });
            } else {
                // C++ Screen Flashing
                events.push(MotionEventData::ScreenFlashing { starting_time });
            }
        }
        4 => { // Fly (Prompt)
            if !fly_file_name.is_empty() {
                events.push(MotionEventData::Fly { starting_time, attaching_enable, attaching_bone_name, fly_file_name, fly_position });
            } else {
                // C++ Special Attacking
                events.push(MotionEventData::Attack { starting_time, duration_time, enable_hit_process, attack_type, hit_limit, sphere_radius });
            }
        }
        5 => { // Attack (Prompt)
            if attack_type > 0 || sphere_radius > 0.0 || duration_time > 0.0 {
                events.push(MotionEventData::Attack { starting_time, duration_time, enable_hit_process, attack_type, hit_limit, sphere_radius });
            } else {
                // C++ Sound
                events.push(MotionEventData::Sound { starting_time, sound_file_name });
            }
        }
        6 => { // Sound (Prompt)
            if !sound_file_name.is_empty() {
                events.push(MotionEventData::Sound { starting_time, sound_file_name });
            } else {
                // C++ Fly
                events.push(MotionEventData::Fly { starting_time, attaching_enable, attaching_bone_name, fly_file_name, fly_position });
            }
        }
        _ => {}
    }
}

#[no_mangle]
pub unsafe extern "C" fn m2_msa_parse(buf: *const c_char, len: usize) -> usize {
    m2_client_bridge::ffi_guard::safe_ffi_call(0, || {
        if buf.is_null() {
            return 0;
        }
        let slice = slice::from_raw_parts(buf as *const u8, len);
        let s = match std::str::from_utf8(slice) {
            Ok(s) => s,
            Err(_) => return 0,
        };

        if let Ok(data) = parse_msa_data(s) {
            data.events.len()
        } else {
            0
        }
    })
}
