#[derive(Debug, Clone, PartialEq)]
pub struct TerrainTextureEntry {
    pub file_name: String,
    pub u_scale: f32,
    pub v_scale: f32,
    pub u_offset: f32,
    pub v_offset: f32,
    pub b_splat: bool,
    pub begin: u16,
    pub end: u16,
}

impl Default for TerrainTextureEntry {
    fn default() -> Self {
        Self {
            file_name: String::new(),
            u_scale: 4.0,
            v_scale: 4.0,
            u_offset: 0.0,
            v_offset: 0.0,
            b_splat: true,
            begin: 0,
            end: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TextureSet {
    textures: Vec<TerrainTextureEntry>,
}

impl TextureSet {
    pub fn new() -> Self {
        // C++ implementation initializes with an empty texture (eraser) at index 0.
        Self {
            textures: vec![TerrainTextureEntry::default()],
        }
    }

    pub fn get_texture(&self, index: usize) -> Option<&TerrainTextureEntry> {
        self.textures.get(index)
    }

    pub fn transform_uv(&self, index: usize, u: f32, v: f32) -> (f32, f32) {
        if let Some(tex) = self.get_texture(index) {
            let u_out = u * tex.u_scale + tex.u_offset;
            let v_out = v * tex.v_scale + tex.v_offset;
            (u_out, v_out)
        } else {
            // Default UV transformation if index is out of bounds
            (u * 4.0, v * 4.0)
        }
    }

    pub fn parse(text: &str) -> Result<Self, &'static str> {
        let mut tokens = Vec::new();
        let mut chars = text.chars().peekable();

        // Tokenize into words and quoted strings
        while let Some(&c) = chars.peek() {
            if c.is_whitespace() {
                chars.next();
                continue;
            }
            if c == '"' {
                chars.next(); // consume opening quote
                let mut s = String::new();
                while let Some(&n) = chars.peek() {
                    if n == '"' {
                        chars.next(); // consume closing quote
                        break;
                    }
                    s.push(n);
                    chars.next();
                }
                tokens.push(s);
            } else {
                let mut s = String::new();
                while let Some(&n) = chars.peek() {
                    if n.is_whitespace() || n == '"' {
                        break;
                    }
                    s.push(n);
                    chars.next();
                }
                tokens.push(s);
            }
        }

        if tokens.is_empty() || tokens[0].to_lowercase() != "textureset" {
            return Err("Invalid header, expected TextureSet");
        }

        let mut index = 1;
        let mut texture_count = 0;

        // Parse TextureCount
        while index < tokens.len() {
            if tokens[index].to_lowercase() == "texturecount" {
                if index + 1 < tokens.len() {
                    texture_count = tokens[index + 1].parse().map_err(|_| "Invalid TextureCount value")?;
                    index += 2;
                    break;
                } else {
                    return Err("Missing count value after TextureCount");
                }
            }
            index += 1;
        }

        if texture_count == 0 && !tokens.iter().any(|t| t.to_lowercase() == "texturecount") {
            return Err("Expected TextureCount");
        }

        let mut texture_set = TextureSet::new();

        // Parse textures
        while index < tokens.len() {
            if tokens[index].to_lowercase() == "start" {
                index += 2; // skip "Start" and "TextureXXX"
                
                if index + 7 >= tokens.len() {
                    return Err("Incomplete texture block");
                }

                let file_name = tokens[index].clone();
                let u_scale: f32 = tokens[index + 1].parse().map_err(|_| "Invalid u_scale")?;
                let v_scale: f32 = tokens[index + 2].parse().map_err(|_| "Invalid v_scale")?;
                let u_offset: f32 = tokens[index + 3].parse().map_err(|_| "Invalid u_offset")?;
                let v_offset: f32 = tokens[index + 4].parse().map_err(|_| "Invalid v_offset")?;
                let b_splat: bool = tokens[index + 5].parse::<u8>().map_err(|_| "Invalid b_splat")? != 0;
                let begin: u16 = tokens[index + 6].parse().map_err(|_| "Invalid begin")?;
                let end: u16 = tokens[index + 7].parse().map_err(|_| "Invalid end")?;

                index += 8;

                if index + 1 < tokens.len() && tokens[index].to_lowercase() == "end" {
                    index += 2; // skip "End" and "TextureXXX"
                } else {
                    return Err("Expected End TextureXXX");
                }

                texture_set.textures.push(TerrainTextureEntry {
                    file_name,
                    u_scale,
                    v_scale,
                    u_offset,
                    v_offset,
                    b_splat,
                    begin,
                    end,
                });
            } else {
                index += 1;
            }
        }

        Ok(texture_set)
    }
}
