
#[derive(Debug, Clone, PartialEq)]
pub struct MseTimeEventFloat {
    pub time: f32,
    pub value: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MseTimeEventColor {
    pub time: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MseEmitterProperty {
    pub max_emission_count: u32,
    pub cycle_length: f32,
    pub cycle_loop_enable: bool,
    pub loop_count: i32,
    pub emitter_shape: u8,
    pub emitter_advanced_type: u8,
    pub emitting_size: [f32; 3],
    pub emitting_radius: f32,
    pub emitting_direction: [f32; 3],
    pub emit_from_edge_flag: bool,
    
    pub time_event_emitting_size: Vec<MseTimeEventFloat>,
    pub time_event_emitting_angular_velocity: Vec<MseTimeEventFloat>,
    pub time_event_emitting_direction_x: Vec<MseTimeEventFloat>,
    pub time_event_emitting_direction_y: Vec<MseTimeEventFloat>,
    pub time_event_emitting_direction_z: Vec<MseTimeEventFloat>,
    pub time_event_emitting_velocity: Vec<MseTimeEventFloat>,
    pub time_event_emission_count_per_second: Vec<MseTimeEventFloat>,
    pub time_event_life_time: Vec<MseTimeEventFloat>,
    pub time_event_size_x: Vec<MseTimeEventFloat>,
    pub time_event_size_y: Vec<MseTimeEventFloat>,
}

impl Default for MseEmitterProperty {
    fn default() -> Self {
        Self {
            max_emission_count: 0,
            cycle_length: 0.05,
            cycle_loop_enable: false,
            loop_count: 0,
            emitter_shape: 0,
            emitter_advanced_type: 0, // EMITTER_ADVANCED_TYPE_FREE = 0 in C++?
            emitting_size: [0.0, 0.0, 0.0],
            emitting_radius: 0.0,
            emitting_direction: [0.0, 0.0, 0.0],
            emit_from_edge_flag: false,
            time_event_emitting_size: vec![MseTimeEventFloat { time: 0.0, value: 0.0 }],
            time_event_emitting_angular_velocity: vec![MseTimeEventFloat { time: 0.0, value: 0.0 }],
            time_event_emitting_direction_x: vec![MseTimeEventFloat { time: 0.0, value: 0.0 }],
            time_event_emitting_direction_y: vec![MseTimeEventFloat { time: 0.0, value: 0.0 }],
            time_event_emitting_direction_z: vec![MseTimeEventFloat { time: 0.0, value: 0.0 }],
            time_event_emitting_velocity: vec![MseTimeEventFloat { time: 0.0, value: 0.0 }],
            time_event_emission_count_per_second: Vec::new(),
            time_event_life_time: Vec::new(),
            time_event_size_x: Vec::new(),
            time_event_size_y: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MseParticleProperty {
    pub src_blend_type: u8,
    pub dest_blend_type: u8,
    pub color_operation_type: u8,
    pub billboard_type: u8,
    pub rotation_type: u8,
    pub rotation_speed: f32,
    pub rotation_random_starting_begin: u16,
    pub rotation_random_starting_end: u16,
    pub attach_flag: bool,
    pub stretch_flag: bool,
    pub tex_ani_type: u8,
    pub tex_ani_delay: f32,
    pub tex_ani_random_start_frame_flag: bool,

    pub time_event_gravity: Vec<MseTimeEventFloat>,
    pub time_event_air_resistance: Vec<MseTimeEventFloat>,
    pub time_event_scale_x: Vec<MseTimeEventFloat>,
    pub time_event_scale_y: Vec<MseTimeEventFloat>,
    pub time_event_color: Vec<MseTimeEventColor>,
    pub time_event_rotation: Vec<MseTimeEventFloat>,
    
    pub texture_files: Vec<String>,
}

impl Default for MseParticleProperty {
    fn default() -> Self {
        Self {
            src_blend_type: 5, // D3DBLEND_SRCALPHA = 5
            dest_blend_type: 2, // D3DBLEND_ONE = 2
            color_operation_type: 4, // D3DTOP_MODULATE = 4
            billboard_type: 0,
            rotation_type: 0,
            rotation_speed: 0.0,
            rotation_random_starting_begin: 0,
            rotation_random_starting_end: 0,
            attach_flag: false,
            stretch_flag: false,
            tex_ani_type: 0,
            tex_ani_delay: 0.0,
            tex_ani_random_start_frame_flag: false,
            time_event_gravity: Vec::new(),
            time_event_air_resistance: Vec::new(),
            time_event_scale_x: Vec::new(),
            time_event_scale_y: Vec::new(),
            time_event_color: Vec::new(),
            time_event_rotation: Vec::new(),
            texture_files: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MseEffectData {
    pub emitter_property: MseEmitterProperty,
    pub particle_property: MseParticleProperty,
}

impl MseEffectData {
    pub fn parse(text: &str) -> Result<MseEffectData, &'static str> {
        let mut effect_data = MseEffectData {
            emitter_property: MseEmitterProperty::default(),
            particle_property: MseParticleProperty::default(),
        };

        let tokens = tokenize(text);
        if tokens.is_empty() {
            return Err("Empty script");
        }
        
        let mut i = 0;
        let mut in_emitter = false;
        let mut in_particle = false;

        let mut time_event_r = Vec::new();
        let mut time_event_g = Vec::new();
        let mut time_event_b = Vec::new();
        let mut time_event_a = Vec::new();

        while i < tokens.len() {
            let token = tokens[i].to_lowercase();
            
            if token == "group" {
                if i + 1 < tokens.len() {
                    let group_name = tokens[i+1].to_lowercase();
                    if group_name == "emitterproperty" {
                        in_emitter = true;
                        in_particle = false;
                    } else if group_name == "particleproperty" {
                        in_emitter = false;
                        in_particle = true;
                    }
                }
                i += 2;
                continue;
            }

            if in_emitter {
                if token == "maxemissioncount" {
                    effect_data.emitter_property.max_emission_count = parse_u32(&tokens, &mut i)?;
                    continue;
                } else if token == "cyclelength" {
                    effect_data.emitter_property.cycle_length = parse_f32(&tokens, &mut i)?;
                    continue;
                } else if token == "cycleloopenable" {
                    effect_data.emitter_property.cycle_loop_enable = parse_bool(&tokens, &mut i)?;
                    continue;
                } else if token == "loopcount" {
                    effect_data.emitter_property.loop_count = parse_i32(&tokens, &mut i)?;
                    continue;
                } else if token == "emittershape" {
                    effect_data.emitter_property.emitter_shape = parse_u8(&tokens, &mut i)?;
                    continue;
                } else if token == "emitteradvancedtype" {
                    effect_data.emitter_property.emitter_advanced_type = parse_u8(&tokens, &mut i)?;
                    continue;
                } else if token == "emittingsize" {
                    effect_data.emitter_property.emitting_size = parse_vec3(&tokens, &mut i)?;
                    continue;
                } else if token == "emittingradius" {
                    effect_data.emitter_property.emitting_radius = parse_f32(&tokens, &mut i)?;
                    continue;
                } else if token == "emittingdirection" {
                    effect_data.emitter_property.emitting_direction = parse_vec3(&tokens, &mut i)?;
                    continue;
                } else if token == "emitteremitfromedgeflag" {
                    effect_data.emitter_property.emit_from_edge_flag = parse_bool(&tokens, &mut i)?;
                    continue;
                } else if token == "timeeventemittingsize" {
                    effect_data.emitter_property.time_event_emitting_size = parse_time_event_float(&tokens, &mut i)?;
                    continue;
                } else if token == "timeeventemittingangularvelocity" {
                    effect_data.emitter_property.time_event_emitting_angular_velocity = parse_time_event_float(&tokens, &mut i)?;
                    continue;
                } else if token == "timeeventemittingdirectionx" {
                    effect_data.emitter_property.time_event_emitting_direction_x = parse_time_event_float(&tokens, &mut i)?;
                    continue;
                } else if token == "timeeventemittingdirectiony" {
                    effect_data.emitter_property.time_event_emitting_direction_y = parse_time_event_float(&tokens, &mut i)?;
                    continue;
                } else if token == "timeeventemittingdirectionz" {
                    effect_data.emitter_property.time_event_emitting_direction_z = parse_time_event_float(&tokens, &mut i)?;
                    continue;
                } else if token == "timeeventemittingvelocity" {
                    effect_data.emitter_property.time_event_emitting_velocity = parse_time_event_float(&tokens, &mut i)?;
                    continue;
                } else if token == "timeeventemissioncountpersecond" {
                    effect_data.emitter_property.time_event_emission_count_per_second = parse_time_event_float(&tokens, &mut i)?;
                    continue;
                } else if token == "timeeventlifetime" {
                    effect_data.emitter_property.time_event_life_time = parse_time_event_float(&tokens, &mut i)?;
                    continue;
                } else if token == "timeeventsizex" {
                    effect_data.emitter_property.time_event_size_x = parse_time_event_float(&tokens, &mut i)?;
                    continue;
                } else if token == "timeeventsizey" {
                    effect_data.emitter_property.time_event_size_y = parse_time_event_float(&tokens, &mut i)?;
                    continue;
                }
            } else if in_particle {
                if token == "srcblendtype" {
                    effect_data.particle_property.src_blend_type = parse_u8(&tokens, &mut i)?;
                    continue;
                } else if token == "destblendtype" {
                    effect_data.particle_property.dest_blend_type = parse_u8(&tokens, &mut i)?;
                    continue;
                } else if token == "coloroperationtype" {
                    effect_data.particle_property.color_operation_type = parse_u8(&tokens, &mut i)?;
                    continue;
                } else if token == "billboardtype" {
                    effect_data.particle_property.billboard_type = parse_u8(&tokens, &mut i)?;
                    continue;
                } else if token == "rotationtype" {
                    effect_data.particle_property.rotation_type = parse_u8(&tokens, &mut i)?;
                    continue;
                } else if token == "rotationspeed" {
                    effect_data.particle_property.rotation_speed = parse_f32(&tokens, &mut i)?;
                    continue;
                } else if token == "rotationrandomstartingbegin" {
                    effect_data.particle_property.rotation_random_starting_begin = parse_u16(&tokens, &mut i)?;
                    continue;
                } else if token == "rotationrandomstartingend" {
                    effect_data.particle_property.rotation_random_starting_end = parse_u16(&tokens, &mut i)?;
                    continue;
                } else if token == "attachenable" {
                    effect_data.particle_property.attach_flag = parse_bool(&tokens, &mut i)?;
                    continue;
                } else if token == "stretchenable" {
                    effect_data.particle_property.stretch_flag = parse_bool(&tokens, &mut i)?;
                    continue;
                } else if token == "texanitype" {
                    effect_data.particle_property.tex_ani_type = parse_u8(&tokens, &mut i)?;
                    continue;
                } else if token == "texanidelay" {
                    effect_data.particle_property.tex_ani_delay = parse_f32(&tokens, &mut i)?;
                    continue;
                } else if token == "texanirandomstartframeenable" {
                    effect_data.particle_property.tex_ani_random_start_frame_flag = parse_bool(&tokens, &mut i)?;
                    continue;
                } else if token == "gravity" {
                    let g = parse_f32(&tokens, &mut i)?;
                    effect_data.particle_property.time_event_gravity.push(MseTimeEventFloat { time: 0.0, value: g });
                    continue;
                } else if token == "timeeventgravity" {
                    effect_data.particle_property.time_event_gravity = parse_time_event_float(&tokens, &mut i)?;
                    continue;
                } else if token == "airresistance" {
                    let a = parse_f32(&tokens, &mut i)?;
                    effect_data.particle_property.time_event_air_resistance.push(MseTimeEventFloat { time: 0.0, value: a });
                    continue;
                } else if token == "timeeventairresistance" {
                    effect_data.particle_property.time_event_air_resistance = parse_time_event_float(&tokens, &mut i)?;
                    continue;
                } else if token == "timeeventscalex" {
                    effect_data.particle_property.time_event_scale_x = parse_time_event_float(&tokens, &mut i)?;
                    continue;
                } else if token == "timeeventscaley" {
                    effect_data.particle_property.time_event_scale_y = parse_time_event_float(&tokens, &mut i)?;
                    continue;
                } else if token == "timeeventrotation" {
                    effect_data.particle_property.time_event_rotation = parse_time_event_float(&tokens, &mut i)?;
                    continue;
                } else if token == "timeeventcolorred" {
                    time_event_r = parse_time_event_float(&tokens, &mut i)?;
                    continue;
                } else if token == "timeeventcolorgreen" {
                    time_event_g = parse_time_event_float(&tokens, &mut i)?;
                    continue;
                } else if token == "timeeventcolorblue" {
                    time_event_b = parse_time_event_float(&tokens, &mut i)?;
                    continue;
                } else if token == "timeeventalpha" {
                    time_event_a = parse_time_event_float(&tokens, &mut i)?;
                    continue;
                } else if token == "texturefiles" {
                    effect_data.particle_property.texture_files = parse_string_list(&tokens, &mut i)?;
                    continue;
                }
            }
            i += 1;
        }

        if !time_event_r.is_empty() || !time_event_g.is_empty() || !time_event_b.is_empty() || !time_event_a.is_empty() {
            let mut times = std::collections::BTreeSet::new();
            for te in &time_event_r { times.insert(OrderedFloat(te.time)); }
            for te in &time_event_g { times.insert(OrderedFloat(te.time)); }
            for te in &time_event_b { times.insert(OrderedFloat(te.time)); }
            for te in &time_event_a { times.insert(OrderedFloat(te.time)); }

            for f_time in times {
                let time = f_time.0;
                let r = get_time_event_blend_value(time, &time_event_r);
                let g = get_time_event_blend_value(time, &time_event_g);
                let b = get_time_event_blend_value(time, &time_event_b);
                let a = get_time_event_blend_value(time, &time_event_a);
                effect_data.particle_property.time_event_color.push(MseTimeEventColor { time, r, g, b, a });
            }
        }

        // Validate mandatory fields? C++ returns FALSE if some fields aren't found. 
        // For simplicity and avoiding erroring on half-valid scripts we could just return it. 
        // We will just let defaults handle it.

        Ok(effect_data)
    }
}

// Wrapper to use f32 in BTreeSet
#[derive(PartialEq, PartialOrd, Clone, Copy)]
struct OrderedFloat(f32);
impl Eq for OrderedFloat {}
impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

fn get_time_event_blend_value(time: f32, events: &[MseTimeEventFloat]) -> f32 {
    if events.is_empty() {
        return 0.0;
    }
    if events.len() == 1 {
        return events[0].value;
    }
    if time <= events[0].time {
        return events[0].value;
    }
    if time >= events.last().unwrap().time {
        return events.last().unwrap().value;
    }

    for i in 0..events.len() - 1 {
        let cur = &events[i];
        let next = &events[i + 1];
        if time >= cur.time && time < next.time {
            let t = (time - cur.time) / (next.time - cur.time);
            return cur.value + (next.value - cur.value) * t;
        }
    }
    0.0
}

fn tokenize(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut in_quotes = false;

    for c in text.chars() {
        if c == '"' {
            in_quotes = !in_quotes;
            current_token.push(c); // keep quotes for string parsing
        } else if c.is_whitespace() && !in_quotes {
            if !current_token.is_empty() {
                tokens.push(current_token.clone());
                current_token.clear();
            }
        } else {
            current_token.push(c);
        }
    }
    if !current_token.is_empty() {
        tokens.push(current_token);
    }
    tokens
}

fn parse_u8(tokens: &[String], i: &mut usize) -> Result<u8, &'static str> {
    *i += 1;
    if *i < tokens.len() {
        if let Ok(val) = tokens[*i].parse::<u8>() {
            return Ok(val);
        }
    }
    Err("Failed to parse u8")
}

fn parse_u16(tokens: &[String], i: &mut usize) -> Result<u16, &'static str> {
    *i += 1;
    if *i < tokens.len() {
        if let Ok(val) = tokens[*i].parse::<u16>() {
            return Ok(val);
        }
    }
    Err("Failed to parse u16")
}

fn parse_u32(tokens: &[String], i: &mut usize) -> Result<u32, &'static str> {
    *i += 1;
    if *i < tokens.len() {
        if let Ok(val) = tokens[*i].parse::<u32>() {
            return Ok(val);
        }
    }
    Err("Failed to parse u32")
}

fn parse_i32(tokens: &[String], i: &mut usize) -> Result<i32, &'static str> {
    *i += 1;
    if *i < tokens.len() {
        if let Ok(val) = tokens[*i].parse::<i32>() {
            return Ok(val);
        }
    }
    Err("Failed to parse i32")
}

fn parse_f32(tokens: &[String], i: &mut usize) -> Result<f32, &'static str> {
    *i += 1;
    if *i < tokens.len() {
        if let Ok(val) = tokens[*i].parse::<f32>() {
            return Ok(val);
        }
    }
    Err("Failed to parse f32")
}

fn parse_bool(tokens: &[String], i: &mut usize) -> Result<bool, &'static str> {
    *i += 1;
    if *i < tokens.len() {
        if tokens[*i] == "1" || tokens[*i].to_lowercase() == "true" {
            return Ok(true);
        }
        if tokens[*i] == "0" || tokens[*i].to_lowercase() == "false" {
            return Ok(false);
        }
    }
    Err("Failed to parse bool")
}

fn parse_vec3(tokens: &[String], i: &mut usize) -> Result<[f32; 3], &'static str> {
    let x = parse_f32(tokens, i)?;
    let y = parse_f32(tokens, i)?;
    let z = parse_f32(tokens, i)?;
    Ok([x, y, z])
}

fn parse_time_event_float(tokens: &[String], i: &mut usize) -> Result<Vec<MseTimeEventFloat>, &'static str> {
    *i += 1;
    let mut events = Vec::new();
    
    if *i >= tokens.len() {
        return Ok(events);
    }

    // Obsluga formatu w klamrach { t1 v1 t2 v2 ... }
    if tokens[*i] == "{" {
        *i += 1;
        while *i < tokens.len() && tokens[*i] != "}" {
            let time = tokens[*i].parse::<f32>().map_err(|_| "Invalid time in time event float")?;
            *i += 1;
            if *i >= tokens.len() || tokens[*i] == "}" {
                return Err("Missing value in time event pair");
            }
            let value = tokens[*i].parse::<f32>().map_err(|_| "Invalid value in time event float")?;
            events.push(MseTimeEventFloat { time, value });
            *i += 1;
        }
        return Ok(events);
    }

    // Sprawdzmy czy pierwszy token to liczba calkowita 'count' poprzedzajaca pary
    if let Ok(count) = tokens[*i].parse::<usize>() {
        let mut is_count_header = true;
        if *i + count * 2 < tokens.len() {
            for offset in 1..=count * 2 {
                if tokens[*i + offset].parse::<f32>().is_err() {
                    is_count_header = false;
                    break;
                }
            }
        } else {
            is_count_header = false;
        }

        if is_count_header && count > 0 {
            for _ in 0..count {
                let time = parse_f32(tokens, i)?;
                let value = parse_f32(tokens, i)?;
                events.push(MseTimeEventFloat { time, value });
            }
            return Ok(events);
        }
    }

    // Standardowy format Metin2 Type.cpp: bezposredni ciag par (time, value)
    while *i + 1 < tokens.len() {
        if let (Ok(time), Ok(value)) = (tokens[*i].parse::<f32>(), tokens[*i + 1].parse::<f32>()) {
            events.push(MseTimeEventFloat { time, value });
            *i += 2;
        } else {
            break;
        }
    }

    if !events.is_empty() {
        *i -= 1;
    }
    Ok(events)
}

fn parse_string_list(tokens: &[String], i: &mut usize) -> Result<Vec<String>, &'static str> {
    *i += 1;
    let mut list = Vec::new();
    if *i >= tokens.len() {
        return Ok(list);
    }

    // Obsluga formatu w klamrach { "file1" "file2" }
    if tokens[*i] == "{" {
        *i += 1;
        while *i < tokens.len() && tokens[*i] != "}" {
            let mut s = tokens[*i].clone();
            if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
                s = s[1..s.len() - 1].to_string();
            }
            list.push(s);
            *i += 1;
        }
        return Ok(list);
    }

    // Obsluga z naglowkiem count
    if let Ok(count) = tokens[*i].parse::<usize>() {
        for _ in 0..count {
            *i += 1;
            if *i < tokens.len() {
                let mut s = tokens[*i].clone();
                if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
                    s = s[1..s.len() - 1].to_string();
                }
                list.push(s);
            } else {
                return Err("Failed to parse string list");
            }
        }
        return Ok(list);
    }

    // Bezposrednie stringi / sciezki
    while *i < tokens.len() {
        let s = &tokens[*i];
        if s.starts_with('"') || s.ends_with(".dds") || s.ends_with(".tga") || s.ends_with(".bmp") {
            let mut clean = s.clone();
            if clean.starts_with('"') && clean.ends_with('"') && clean.len() >= 2 {
                clean = clean[1..clean.len() - 1].to_string();
            }
            list.push(clean);
            *i += 1;
        } else {
            break;
        }
    }
    if !list.is_empty() {
        *i -= 1;
    }
    Ok(list)
}
