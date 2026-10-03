
/// Parses a single line into a list of tokens, properly handling quotes.
pub fn parse_tokens(line: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '"' {
            in_quotes = !in_quotes;
        } else if (c == ' ' || c == '\t' || c == '\r' || c == '\n') && !in_quotes {
            if !current.is_empty() {
                tokens.push(current.clone());
                current.clear();
            }
        } else {
            current.push(c);
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}

#[derive(Debug, Clone)]
pub struct Tokenizer {
    tokens: Vec<Vec<String>>,
}

impl Tokenizer {
    pub fn new(content: &str) -> Self {
        let mut tokens = Vec::new();
        for line in content.lines() {
            let line = line.split("//").next().unwrap_or(line);
            let line_tokens = parse_tokens(line);
            if !line_tokens.is_empty() {
                tokens.push(line_tokens);
            }
        }
        Self { tokens }
    }

    pub fn get_vec(&self, key: &str) -> Option<&Vec<String>> {
        self.tokens.iter().find(|t| t[0].eq_ignore_ascii_case(key))
    }

    pub fn get_string(&self, key: &str) -> Option<String> {
        self.get_vec(key).and_then(|t| t.get(1).cloned())
    }

    pub fn get_u32(&self, key: &str, index: usize) -> Option<u32> {
        self.get_vec(key).and_then(|t| t.get(index + 1).and_then(|s| s.parse().ok()))
    }

    pub fn get_i32(&self, key: &str, index: usize) -> Option<i32> {
        self.get_vec(key).and_then(|t| t.get(index + 1).and_then(|s| s.parse().ok()))
    }

    pub fn get_f32(&self, key: &str, index: usize) -> Option<f32> {
        self.get_vec(key).and_then(|t| t.get(index + 1).and_then(|s| s.parse().ok()))
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MapProperty {
    pub script_type: String,
    pub map_type: String,
    pub map_size_x: u32,
    pub map_size_y: u32,
    pub base_position_x: i32,
    pub base_position_y: i32,
}

impl MapProperty {
    pub fn parse(content: &str) -> Result<Self, String> {
        let tk = Tokenizer::new(content);
        
        let script_type = tk.get_string("ScriptType").unwrap_or_default();
        if !script_type.eq_ignore_ascii_case("MapProperty") && !script_type.is_empty() {
            return Err("Invalid ScriptType for MapProperty".into());
        }

        Ok(Self {
            script_type,
            map_type: tk.get_string("MapType").unwrap_or_default(),
            map_size_x: tk.get_u32("MapSize", 0).unwrap_or(1),
            map_size_y: tk.get_u32("MapSize", 1).unwrap_or(1),
            base_position_x: tk.get_i32("BasePosition", 0).unwrap_or(0),
            base_position_y: tk.get_i32("BasePosition", 1).unwrap_or(0),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AreaProperty {
    pub script_type: String,
    pub sky_file: String,
    pub light_direction: [f32; 3],
    pub ambient_color: [f32; 3],
    pub diffuse_color: [f32; 3],
}

impl AreaProperty {
    pub fn parse(content: &str) -> Result<Self, String> {
        let tk = Tokenizer::new(content);
        
        let script_type = tk.get_string("ScriptType").unwrap_or_default();
        if !script_type.eq_ignore_ascii_case("AreaProperty") && !script_type.is_empty() {
            return Err("Invalid ScriptType for AreaProperty".into());
        }

        Ok(Self {
            script_type,
            sky_file: tk.get_string("SkyFile").or_else(|| tk.get_string("SkyBox")).unwrap_or_default(),
            light_direction: [
                tk.get_f32("LightDirection", 0).unwrap_or(0.0),
                tk.get_f32("LightDirection", 1).unwrap_or(0.0),
                tk.get_f32("LightDirection", 2).unwrap_or(0.0),
            ],
            ambient_color: [
                tk.get_f32("AmbientColor", 0).unwrap_or(0.0),
                tk.get_f32("AmbientColor", 1).unwrap_or(0.0),
                tk.get_f32("AmbientColor", 2).unwrap_or(0.0),
            ],
            diffuse_color: [
                tk.get_f32("DiffuseColor", 0).unwrap_or(0.0),
                tk.get_f32("DiffuseColor", 1).unwrap_or(0.0),
                tk.get_f32("DiffuseColor", 2).unwrap_or(0.0),
            ],
        })
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AreaAmbience {
    pub script_type: String,
    pub sound_file: String,
    pub play_interval_min: f32,
    pub play_interval_max: f32,
    pub volume_percentage: f32,
}

impl AreaAmbience {
    pub fn parse(content: &str) -> Result<Self, String> {
        let tk = Tokenizer::new(content);
        
        let script_type = tk.get_string("ScriptType").unwrap_or_default();
        if !script_type.eq_ignore_ascii_case("AreaAmbience") && !script_type.is_empty() {
            return Err("Invalid ScriptType for AreaAmbience".into());
        }

        Ok(Self {
            script_type,
            sound_file: tk.get_string("SoundFile").or_else(|| tk.get_string("AmbienceSound")).unwrap_or_default(),
            play_interval_min: tk.get_f32("PlayInterval", 0).unwrap_or(0.0),
            play_interval_max: tk.get_f32("PlayInterval", 1).unwrap_or(0.0),
            volume_percentage: tk.get_f32("VolumePercentage", 0).or_else(|| tk.get_f32("MaxVolumeAreaPercentage", 0)).unwrap_or(1.0),
        })
    }
}
