use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct TimeEvent<T> {
    pub time: f32,
    pub value: T,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EffectPosition {
    pub time: f32,
    pub value: [f32; 3],
    pub moving_type: u32,
    pub control_point: [f32; 3],
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EmitterProperty {
    pub max_emission_count: u32,
    pub cycle_length: f32,
    pub cycle_loop_flag: bool,
    pub loop_count: i32,
    pub emitter_shape: u8,
    pub emitter_advanced_type: u8,
    pub emit_from_edge_flag: bool,
    pub emitting_size: [f32; 3],
    pub emitting_radius: f32,
    pub emitting_direction: [f32; 3],

    pub time_event_emitting_size: Vec<TimeEvent<f32>>,
    pub time_event_emitting_angular_velocity: Vec<TimeEvent<f32>>,
    pub time_event_emitting_direction_x: Vec<TimeEvent<f32>>,
    pub time_event_emitting_direction_y: Vec<TimeEvent<f32>>,
    pub time_event_emitting_direction_z: Vec<TimeEvent<f32>>,
    pub time_event_emitting_velocity: Vec<TimeEvent<f32>>,
    pub time_event_emission_count_per_second: Vec<TimeEvent<f32>>,
    pub time_event_life_time: Vec<TimeEvent<f32>>,
    pub time_event_size_x: Vec<TimeEvent<f32>>,
    pub time_event_size_y: Vec<TimeEvent<f32>>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ParticleProperty {
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

    pub time_event_gravity: Vec<TimeEvent<f32>>,
    pub time_event_air_resistance: Vec<TimeEvent<f32>>,

    pub time_event_scale_x: Vec<TimeEvent<f32>>,
    pub time_event_scale_y: Vec<TimeEvent<f32>>,
    pub time_event_color: Vec<TimeEvent<[f32; 4]>>, // RGBA
    pub time_event_rotation: Vec<TimeEvent<f32>>,

    pub texture_files: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ParticleSystemData {
    pub start_time: f32,
    pub time_event_table_position: Vec<EffectPosition>,
    pub emitter_property: EmitterProperty,
    pub particle_property: ParticleProperty,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MeshData {
    pub billboard_type: u8,
    pub blending_enable: bool,
    pub blending_src_type: u8,
    pub blending_dest_type: u8,
    pub texture_alpha_enable: bool,
    
    pub color_operation_type: u8,
    pub color_factor: [f32; 4],
    
    pub texture_animation_loop_enable: bool,
    pub texture_animation_frame_delay: f32,
    pub texture_animation_start_frame: u32,
    
    pub time_event_alpha: Vec<TimeEvent<f32>>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EffectMeshScript {
    pub start_time: f32,
    pub time_event_table_position: Vec<EffectPosition>,
    
    pub mesh_file_name: String,
    pub mesh_animation_loop_enable: bool,
    pub mesh_animation_loop_count: i32,
    pub mesh_animation_frame_delay: f32,
    
    pub mesh_data_vector: Vec<MeshData>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LightData {
    pub start_time: f32,
    pub time_event_table_position: Vec<EffectPosition>,
    
    pub duration: f32,
    pub loop_flag: bool,
    pub loop_count: i32,
    
    pub ambient_color: [f32; 4],
    pub diffuse_color: [f32; 4],
    
    pub max_range: f32,
    pub attenuation0: f32,
    pub attenuation1: f32,
    pub attenuation2: f32,
    
    pub time_event_range: Vec<TimeEvent<f32>>,
}

impl Default for LightData {
    fn default() -> Self {
        Self {
            start_time: 0.0,
            time_event_table_position: Vec::new(),
            duration: 1.0,
            loop_flag: false,
            loop_count: 0,
            ambient_color: [0.5, 0.5, 0.5, 1.0],
            diffuse_color: [0.0, 0.0, 0.0, 1.0],
            max_range: 300.0,
            attenuation0: 0.0,
            attenuation1: 0.1,
            attenuation2: 0.0,
            time_event_range: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EffectData {
    pub bounding_sphere_radius: f32,
    pub bounding_sphere_position: [f32; 3],
    pub particles: Vec<ParticleSystemData>,
    pub meshes: Vec<EffectMeshScript>,
    pub lights: Vec<LightData>,
}

// --- Tokenizer and Group Parser ---

#[derive(Debug, Clone, PartialEq)]
pub struct GroupNode {
    pub name: String,
    pub keys: std::collections::HashMap<String, Vec<String>>,
    pub children: Vec<GroupNode>,
}

impl GroupNode {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            keys: std::collections::HashMap::new(),
            children: Vec::new(),
        }
    }

    pub fn get_token_vector(&self, key: &str) -> Option<&Vec<String>> {
        self.keys.get(&key.to_lowercase())
    }

    pub fn get_token_float(&self, key: &str) -> Option<f32> {
        self.get_token_vector(key)
            .and_then(|v| v.first())
            .and_then(|s| f32::from_str(s).ok())
    }

    pub fn get_token_bool(&self, key: &str) -> Option<bool> {
        self.get_token_vector(key)
            .and_then(|v| v.first())
            .and_then(|s| i32::from_str(s).ok())
            .map(|i| i != 0)
    }

    pub fn get_token_int(&self, key: &str) -> Option<i32> {
        self.get_token_vector(key)
            .and_then(|v| v.first())
            .and_then(|s| i32::from_str(s).ok())
    }

    pub fn get_token_u32(&self, key: &str) -> Option<u32> {
        self.get_token_vector(key)
            .and_then(|v| v.first())
            .and_then(|s| u32::from_str(s).ok())
    }
    
    pub fn get_token_u16(&self, key: &str) -> Option<u16> {
        self.get_token_vector(key)
            .and_then(|v| v.first())
            .and_then(|s| u16::from_str(s).ok())
    }
    
    pub fn get_token_u8(&self, key: &str) -> Option<u8> {
        self.get_token_vector(key)
            .and_then(|v| v.first())
            .and_then(|s| u8::from_str(s).ok())
    }

    pub fn get_token_string(&self, key: &str) -> Option<String> {
        self.get_token_vector(key)
            .and_then(|v| v.first())
            .cloned()
            .map(|s| s.trim_matches('"').to_string())
    }
    
    pub fn get_token_vector3(&self, key: &str) -> Option<[f32; 3]> {
        self.get_token_vector(key).and_then(|v| {
            if v.len() >= 3 {
                Some([
                    f32::from_str(&v[0]).unwrap_or(0.0),
                    f32::from_str(&v[1]).unwrap_or(0.0),
                    f32::from_str(&v[2]).unwrap_or(0.0),
                ])
            } else {
                None
            }
        })
    }

    pub fn get_token_color(&self, key: &str) -> Option<[f32; 4]> {
        self.get_token_vector(key).and_then(|v| {
            if v.len() >= 4 {
                Some([
                    f32::from_str(&v[0]).unwrap_or(0.0),
                    f32::from_str(&v[1]).unwrap_or(0.0),
                    f32::from_str(&v[2]).unwrap_or(0.0),
                    f32::from_str(&v[3]).unwrap_or(0.0),
                ])
            } else {
                None
            }
        })
    }
    
    pub fn get_token_time_event_float(&self, key: &str) -> Option<Vec<TimeEvent<f32>>> {
        self.get_token_vector(key).map(|v| {
            let mut events = Vec::new();
            for chunk in v.chunks(2) {
                if chunk.len() == 2 {
                    let time = f32::from_str(&chunk[0]).unwrap_or(0.0);
                    let value = f32::from_str(&chunk[1]).unwrap_or(0.0);
                    events.push(TimeEvent { time, value });
                }
            }
            events
        })
    }

    pub fn get_child(&self, name: &str) -> Option<&GroupNode> {
        let lower_name = name.to_lowercase();
        self.children.iter().find(|c| c.name.to_lowercase() == lower_name)
    }
}

pub struct ScriptParser;

impl ScriptParser {
    pub fn parse(content: &str) -> Result<EffectData, String> {
        let root = Self::parse_text(content)?;
        Self::build_effect_data(&root)
    }

    fn parse_text(content: &str) -> Result<GroupNode, String> {
        let mut root = GroupNode::new("root");
        let lines: Vec<&str> = content.lines().collect();
        let mut idx = 0;
        
        while idx < lines.len() {
            let line = lines[idx].trim();
            if line.is_empty() {
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
        let mut in_comment = false;
        
        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            let c = chars[i];
            
            // Handle comments (assuming // is comment)
            if c == '/' && i + 1 < chars.len() && chars[i+1] == '/' {
                break;
            }
            
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
        
        // Find '{'
        while idx < lines.len() && lines[idx].trim().is_empty() {
            idx += 1;
        }
        
        if idx >= lines.len() || !lines[idx].trim().starts_with('{') {
            return Err(format!("Expected '{{' for group {}", name));
        }
        idx += 1;
        
        while idx < lines.len() {
            let line = lines[idx].trim();
            if line.is_empty() {
                idx += 1;
                continue;
            }
            
            if line.starts_with('}') {
                return Ok((node, idx + 1));
            }
            
            let tokens = Self::tokenize_line(line);
            if tokens.is_empty() {
                idx += 1;
                continue;
            }
            
            if tokens[0].to_lowercase() == "group" && tokens.len() >= 2 {
                let child_name = &tokens[1];
                let (child_node, next_idx) = Self::parse_group_block(lines, idx + 1, child_name)?;
                node.children.push(child_node);
                idx = next_idx;
            } else {
                let key = tokens[0].to_lowercase();
                
                // If it's a List token vector, parse it differently if needed, but for now just treat as key
                if key == "list" && tokens.len() >= 2 {
                    // C++: List TimeEventPosition { ... }
                    let list_name = tokens[1].to_lowercase();
                    idx += 1;
                    // Find '{'
                    while idx < lines.len() && lines[idx].trim().is_empty() {
                        idx += 1;
                    }
                    if idx < lines.len() && lines[idx].trim().starts_with('{') {
                        idx += 1;
                        let mut values = Vec::new();
                        while idx < lines.len() && !lines[idx].trim().starts_with('}') {
                            let list_line = lines[idx].trim();
                            if !list_line.is_empty() && !list_line.starts_with("//") {
                                values.extend(Self::tokenize_line(list_line));
                            }
                            idx += 1;
                        }
                        if idx < lines.len() && lines[idx].trim().starts_with('}') {
                            idx += 1;
                        }
                        node.keys.insert(list_name, values);
                    }
                } else {
                    let mut values: Vec<String> = tokens.into_iter().skip(1).collect();
                    
                    // Handle multi-line keys that don't use 'List' but might have subsequent lines? 
                    // No, typically in Metin2 they are either single line or 'List'.
                    node.keys.insert(key, values);
                    idx += 1;
                }
            }
        }
        
        Err(format!("Missing '}}' for group {}", name))
    }

    fn build_effect_data(root: &GroupNode) -> Result<EffectData, String> {
        let mut effect = EffectData::default();
        
        effect.bounding_sphere_radius = root.get_token_float("boundingsphereradius").unwrap_or(0.0);
        effect.bounding_sphere_position = root.get_token_vector3("boundingsphereposition").unwrap_or([0.0, 0.0, 0.0]);
        
        for child in &root.children {
            match child.name.to_lowercase().as_str() {
                "particle" => {
                    if let Some(particle) = Self::parse_particle_system(child) {
                        effect.particles.push(particle);
                    }
                },
                "mesh" => {
                    if let Some(mesh) = Self::parse_effect_mesh(child) {
                        effect.meshes.push(mesh);
                    }
                },
                "light" => {
                    if let Some(light) = Self::parse_light(child) {
                        effect.lights.push(light);
                    }
                },
                _ => {
                    // Check if it's named anything else, but original C++ iterates over child nodes and checks GetCurrentNodeName() against "mesh", "particle", "light"
                    let name = child.name.to_lowercase();
                    if name == "mesh" {
                        if let Some(mesh) = Self::parse_effect_mesh(child) {
                            effect.meshes.push(mesh);
                        }
                    } else if name == "particle" {
                        if let Some(particle) = Self::parse_particle_system(child) {
                            effect.particles.push(particle);
                        }
                    } else if name == "light" {
                        if let Some(light) = Self::parse_light(child) {
                            effect.lights.push(light);
                        }
                    }
                }
            }
        }
        
        Ok(effect)
    }
    
    fn parse_base(node: &GroupNode) -> (f32, Vec<EffectPosition>) {
        let start_time = node.get_token_float("starttime").unwrap_or(0.0);
        
        let mut positions = Vec::new();
        if let Some(vec) = node.get_token_vector("timeeventposition") {
            let mut i = 0;
            while i < vec.len() {
                let time = f32::from_str(&vec[i]).unwrap_or(0.0);
                i += 1;
                if i < vec.len() {
                    let move_type_str = &vec[i];
                    i += 1;
                    
                    if move_type_str == "MOVING_TYPE_BEZIER_CURVE" && i + 5 < vec.len() {
                        let value = [
                            f32::from_str(&vec[i]).unwrap_or(0.0),
                            f32::from_str(&vec[i+1]).unwrap_or(0.0),
                            f32::from_str(&vec[i+2]).unwrap_or(0.0),
                        ];
                        let control_point = [
                            f32::from_str(&vec[i+3]).unwrap_or(0.0),
                            f32::from_str(&vec[i+4]).unwrap_or(0.0),
                            f32::from_str(&vec[i+5]).unwrap_or(0.0),
                        ];
                        positions.push(EffectPosition {
                            time,
                            value,
                            moving_type: 1, // MOVING_TYPE_BEZIER_CURVE
                            control_point,
                        });
                        i += 6;
                    } else if move_type_str == "MOVING_TYPE_DIRECT" && i + 2 < vec.len() {
                        let value = [
                            f32::from_str(&vec[i]).unwrap_or(0.0),
                            f32::from_str(&vec[i+1]).unwrap_or(0.0),
                            f32::from_str(&vec[i+2]).unwrap_or(0.0),
                        ];
                        positions.push(EffectPosition {
                            time,
                            value,
                            moving_type: 0, // MOVING_TYPE_DIRECT
                            control_point: [0.0, 0.0, 0.0],
                        });
                        i += 3;
                    }
                }
            }
        }
        
        (start_time, positions)
    }

    fn parse_particle_system(node: &GroupNode) -> Option<ParticleSystemData> {
        let (start_time, time_event_table_position) = Self::parse_base(node);
        
        let mut emitter_property = EmitterProperty::default();
        let mut particle_property = ParticleProperty::default();
        
        if let Some(emitter) = node.get_child("emitterproperty") {
            emitter_property.max_emission_count = emitter.get_token_u32("maxemissioncount").unwrap_or(0);
            emitter_property.cycle_length = emitter.get_token_float("cyclelength").unwrap_or(0.05);
            emitter_property.cycle_loop_flag = emitter.get_token_bool("cycleloopenable").unwrap_or(false);
            emitter_property.loop_count = emitter.get_token_int("loopcount").unwrap_or(0);
            emitter_property.emitter_shape = emitter.get_token_u8("emittershape").unwrap_or(0);
            emitter_property.emitter_advanced_type = emitter.get_token_u8("emitteradvancedtype").unwrap_or(0); // EMITTER_ADVANCED_TYPE_FREE
            emitter_property.emitting_size = emitter.get_token_vector3("emittingsize").unwrap_or([0.0, 0.0, 0.0]);
            emitter_property.emitting_radius = emitter.get_token_float("emittingradius").unwrap_or(0.0);
            emitter_property.emit_from_edge_flag = emitter.get_token_bool("emitteremitfromedgeflag").unwrap_or(false);
            emitter_property.emitting_direction = emitter.get_token_vector3("emittingdirection").unwrap_or([0.0, 0.0, 0.0]);

            emitter_property.time_event_emitting_size = emitter.get_token_time_event_float("timeeventemittingsize").unwrap_or_else(|| vec![TimeEvent { time: 0.0, value: 0.0 }]);
            emitter_property.time_event_emitting_angular_velocity = emitter.get_token_time_event_float("timeeventemittingangularvelocity").unwrap_or_else(|| vec![TimeEvent { time: 0.0, value: 0.0 }]);
            emitter_property.time_event_emitting_direction_x = emitter.get_token_time_event_float("timeeventemittingdirectionx").unwrap_or_else(|| vec![TimeEvent { time: 0.0, value: 0.0 }]);
            emitter_property.time_event_emitting_direction_y = emitter.get_token_time_event_float("timeeventemittingdirectiony").unwrap_or_else(|| vec![TimeEvent { time: 0.0, value: 0.0 }]);
            emitter_property.time_event_emitting_direction_z = emitter.get_token_time_event_float("timeeventemittingdirectionz").unwrap_or_else(|| vec![TimeEvent { time: 0.0, value: 0.0 }]);
            emitter_property.time_event_emitting_velocity = emitter.get_token_time_event_float("timeeventemittingvelocity").unwrap_or_else(|| vec![TimeEvent { time: 0.0, value: 0.0 }]);
            
            emitter_property.time_event_emission_count_per_second = emitter.get_token_time_event_float("timeeventemissioncountpersecond").unwrap_or_default();
            emitter_property.time_event_life_time = emitter.get_token_time_event_float("timeeventlifetime").unwrap_or_default();
            emitter_property.time_event_size_x = emitter.get_token_time_event_float("timeeventsizex").unwrap_or_default();
            emitter_property.time_event_size_y = emitter.get_token_time_event_float("timeeventsizey").unwrap_or_default();
        }
        
        if let Some(particle) = node.get_child("particleproperty") {
            particle_property.src_blend_type = particle.get_token_u8("srcblendtype").unwrap_or(5); // D3DBLEND_SRCALPHA
            particle_property.dest_blend_type = particle.get_token_u8("destblendtype").unwrap_or(2); // D3DBLEND_ONE
            particle_property.color_operation_type = particle.get_token_u8("coloroperationtype").unwrap_or(4); // D3DTOP_MODULATE
            
            particle_property.billboard_type = particle.get_token_u8("billboardtype").unwrap_or(0);
            particle_property.rotation_type = particle.get_token_u8("rotationtype").unwrap_or(0);
            particle_property.rotation_speed = particle.get_token_float("rotationspeed").unwrap_or(0.0);
            particle_property.rotation_random_starting_begin = particle.get_token_u16("rotationrandomstartingbegin").unwrap_or(0);
            particle_property.rotation_random_starting_end = particle.get_token_u16("rotationrandomstartingend").unwrap_or(0);
            
            particle_property.attach_flag = particle.get_token_bool("attachenable").unwrap_or(false);
            particle_property.stretch_flag = particle.get_token_bool("stretchenable").unwrap_or(false);
            
            particle_property.tex_ani_type = particle.get_token_u8("texanitype").unwrap_or(0);
            particle_property.tex_ani_delay = particle.get_token_float("texanidelay").unwrap_or(0.0);
            particle_property.tex_ani_random_start_frame_flag = particle.get_token_bool("texanirandomstartframeenable").unwrap_or(false);
            
            if let Some(gravity) = particle.get_token_float("gravity") {
                particle_property.time_event_gravity = vec![TimeEvent { time: 0.0, value: gravity }];
            } else {
                particle_property.time_event_gravity = particle.get_token_time_event_float("timeeventgravity").unwrap_or_default();
            }
            
            if let Some(air) = particle.get_token_float("airresistance") {
                particle_property.time_event_air_resistance = vec![TimeEvent { time: 0.0, value: air }];
            } else {
                particle_property.time_event_air_resistance = particle.get_token_time_event_float("timeeventairresistance").unwrap_or_default();
            }
            
            particle_property.time_event_scale_x = particle.get_token_time_event_float("timeeventscalex").unwrap_or_default();
            particle_property.time_event_scale_y = particle.get_token_time_event_float("timeeventscaley").unwrap_or_default();
            
            // Reconstruct color time event from R, G, B, A tables
            let tr = particle.get_token_time_event_float("timeeventcolorred").unwrap_or_default();
            let tg = particle.get_token_time_event_float("timeeventcolorgreen").unwrap_or_default();
            let tb = particle.get_token_time_event_float("timeeventcolorblue").unwrap_or_default();
            let ta = particle.get_token_time_event_float("timeeventalpha").unwrap_or_default();
            
            // To properly reconstruct, we would merge all times. For simplicity in parser, we collect unique times and sample
            let mut times: std::collections::BTreeSet<u32> = std::collections::BTreeSet::new();
            for ev in &tr { times.insert(ev.time.to_bits()); }
            for ev in &tg { times.insert(ev.time.to_bits()); }
            for ev in &tb { times.insert(ev.time.to_bits()); }
            for ev in &ta { times.insert(ev.time.to_bits()); }
            
            for &t in &times {
                let time = f32::from_bits(t);
                let r = Self::sample_time_event(&tr, time).unwrap_or(1.0);
                let g = Self::sample_time_event(&tg, time).unwrap_or(1.0);
                let b = Self::sample_time_event(&tb, time).unwrap_or(1.0);
                let a = Self::sample_time_event(&ta, time).unwrap_or(1.0);
                particle_property.time_event_color.push(TimeEvent { time, value: [r, g, b, a] });
            }
            
            particle_property.time_event_rotation = particle.get_token_time_event_float("timeeventrotation").unwrap_or_default();
            
            if let Some(tex_files) = particle.get_token_vector("texturefiles") {
                particle_property.texture_files = tex_files.iter().map(|s| s.trim_matches('"').to_string()).collect();
            }
        }
        
        Some(ParticleSystemData {
            start_time,
            time_event_table_position,
            emitter_property,
            particle_property,
        })
    }
    
    fn sample_time_event(events: &[TimeEvent<f32>], time: f32) -> Option<f32> {
        if events.is_empty() { return None; }
        if time <= events[0].time { return Some(events[0].value); }
        if time >= events.last().unwrap().time { return Some(events.last().unwrap().value); }
        
        for i in 0..events.len() - 1 {
            if time >= events[i].time && time <= events[i+1].time {
                let t_diff = events[i+1].time - events[i].time;
                if t_diff == 0.0 { return Some(events[i].value); }
                let perc = (time - events[i].time) / t_diff;
                return Some(events[i].value + perc * (events[i+1].value - events[i].value));
            }
        }
        None
    }

    fn parse_effect_mesh(node: &GroupNode) -> Option<EffectMeshScript> {
        let (start_time, time_event_table_position) = Self::parse_base(node);
        
        let mesh_file_name = node.get_token_string("meshfilename").unwrap_or_default();
        let mesh_animation_loop_enable = node.get_token_bool("meshanimationloopenable").unwrap_or(false);
        let mesh_animation_loop_count = node.get_token_int("meshanimationloopcount").unwrap_or(0);
        let mesh_animation_frame_delay = node.get_token_float("meshanimationframedelay").unwrap_or(0.0);
        
        let mesh_element_count = node.get_token_u32("meshelementcount").unwrap_or(0);
        let mut mesh_data_vector = Vec::new();
        
        for i in 0..mesh_element_count {
            if let Some(mesh_data_node) = node.get_child(&i.to_string()) {
                let billboard_type = mesh_data_node.get_token_u8("billboardtype").unwrap_or(0);
                let blending_enable = mesh_data_node.get_token_bool("blendingenable").unwrap_or(false);
                let blending_src_type = mesh_data_node.get_token_u8("blendingsrctype").unwrap_or(5);
                let blending_dest_type = mesh_data_node.get_token_u8("blendingdesttype").unwrap_or(2);
                
                let texture_animation_loop_enable = mesh_data_node.get_token_bool("textureanimationloopenable").unwrap_or(false);
                let texture_animation_frame_delay = mesh_data_node.get_token_float("textureanimationframedelay").unwrap_or(0.0);
                let texture_animation_start_frame = mesh_data_node.get_token_u32("textureanimationstartframe").unwrap_or(0);
                
                let texture_alpha_enable = mesh_data_node.get_token_bool("texturealphaenable").unwrap_or(false);
                let color_operation_type = mesh_data_node.get_token_u8("coloroperationtype").unwrap_or(4);
                let color_factor = mesh_data_node.get_token_color("colorfactor").unwrap_or([0.0, 0.0, 0.0, 0.0]);
                
                let time_event_alpha = mesh_data_node.get_token_time_event_float("timeeventalpha").unwrap_or_default();
                
                mesh_data_vector.push(MeshData {
                    billboard_type,
                    blending_enable,
                    blending_src_type,
                    blending_dest_type,
                    texture_alpha_enable,
                    color_operation_type,
                    color_factor,
                    texture_animation_loop_enable,
                    texture_animation_frame_delay,
                    texture_animation_start_frame,
                    time_event_alpha,
                });
            } else {
                // Original C++ also does this, iterating and getting children by index
            }
        }
        
        Some(EffectMeshScript {
            start_time,
            time_event_table_position,
            mesh_file_name,
            mesh_animation_loop_enable,
            mesh_animation_loop_count,
            mesh_animation_frame_delay,
            mesh_data_vector,
        })
    }

    fn parse_light(node: &GroupNode) -> Option<LightData> {
        let (start_time, time_event_table_position) = Self::parse_base(node);
        
        let mut light = LightData {
            start_time,
            time_event_table_position,
            ..Default::default()
        };
        
        light.duration = node.get_token_float("duration").unwrap_or(1.0);
        light.loop_flag = node.get_token_bool("loopflag").unwrap_or(false);
        light.loop_count = node.get_token_int("loopcount").unwrap_or(0);
        
        if let Some(amb) = node.get_token_color("ambientcolor") { light.ambient_color = amb; }
        if let Some(diff) = node.get_token_color("diffusecolor") { light.diffuse_color = diff; }
        
        if let Some(r) = node.get_token_float("maxrange") { light.max_range = r; }
        if let Some(a0) = node.get_token_float("attenuation0") { light.attenuation0 = a0; }
        if let Some(a1) = node.get_token_float("attenuation1") { light.attenuation1 = a1; }
        if let Some(a2) = node.get_token_float("attenuation2") { light.attenuation2 = a2; }
        
        light.time_event_range = node.get_token_time_event_float("timeeventrange").unwrap_or_default();
        
        Some(light)
    }
}
