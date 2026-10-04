use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MonsterScript {
    pub base_model_file_name: String,
    pub tree_file_name: String,
    pub attribute_file_name: String,
    pub smoke_bone_name: String,
    pub motion_list_file_name: String,
    pub smoke_effects: Vec<(u32, String)>,
    
    pub shape_data: Vec<ShapeData>,
    pub hair_data: Vec<HairData>,
    pub attaching_data: Vec<AttachingData>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SkinData {
    pub part: i32,
    pub src_file_name: String,
    pub dst_file_name: String,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ShapeData {
    pub shape_index: u32,
    pub model_file_name: String,
    pub skins: Vec<SkinData>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct HairData {
    pub hair_index: u32,
    pub model_file_name: String,
    pub skins: Vec<SkinData>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttachingDataType {
    None = 0,
    CollisionData = 1,
    Effect = 2,
    Object = 3,
}

impl Default for AttachingDataType {
    fn default() -> Self {
        Self::None
    }
}

impl From<u32> for AttachingDataType {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::CollisionData,
            2 => Self::Effect,
            3 => Self::Object,
            _ => Self::None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AttachingData {
    pub data_type: AttachingDataType,
    pub is_attaching: bool,
    pub attaching_model_index: u32,
    pub attaching_bone_name: String,

    pub collision_data: Option<CollisionData>,
    pub effect_data: Option<EffectData>,
    pub object_data: Option<ObjectData>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CollisionData {
    pub collision_type: i32,
    pub spheres: Vec<SphereData>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SphereData {
    pub radius: f32,
    pub position: [f32; 3],
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct EffectData {
    pub file_name: String,
    pub position: [f32; 3],
    pub rotation: [f32; 3],
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ObjectData {
    pub file_name: String,
}

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
                let mut values: Vec<String> = tokens.into_iter().skip(1).collect();
                
                // Check if next non-empty line starts with '{'
                let mut peek_idx = idx + 1;
                while peek_idx < lines.len() && lines[peek_idx].trim().is_empty() {
                    peek_idx += 1;
                }
                
                if peek_idx < lines.len() && lines[peek_idx].trim().starts_with('{') {
                    let (block_tokens, next_idx) = Self::parse_vector_block(&lines, peek_idx)?;
                    values.extend(block_tokens);
                    idx = next_idx;
                } else {
                    idx += 1;
                }
                
                root.keys.insert(key, values);
            }
        }
        
        Ok(root)
    }

    fn parse_vector_block(lines: &[&str], start_idx: usize) -> Result<(Vec<String>, usize), String> {
        let mut tokens = Vec::new();
        let mut idx = start_idx;
        
        if idx >= lines.len() || !lines[idx].trim().starts_with('{') {
            return Err("Expected '{' for vector block".to_string());
        }
        
        let first_line = lines[idx].trim();
        if first_line.len() > 1 {
            let mut line_tokens = Self::tokenize_line(&first_line[1..]);
            tokens.append(&mut line_tokens);
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
            
            if line.starts_with('}') {
                return Ok((tokens, idx + 1));
            }
            
            let mut line_tokens = Self::tokenize_line(line);
            tokens.append(&mut line_tokens);
            idx += 1;
        }
        
        Err("Unexpected end of file while parsing vector block".to_string())
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
                let mut values: Vec<String> = tokens.into_iter().skip(1).collect();
                
                let mut peek_idx = idx + 1;
                while peek_idx < lines.len() && lines[peek_idx].trim().is_empty() {
                    peek_idx += 1;
                }
                
                if peek_idx < lines.len() && lines[peek_idx].trim().starts_with('{') {
                    let (block_tokens, next_idx) = Self::parse_vector_block(lines, peek_idx)?;
                    values.extend(block_tokens);
                    idx = next_idx;
                } else {
                    idx += 1;
                }
                
                node.keys.insert(key, values);
            }
        }
        
        Err(format!("Unexpected end of file while parsing group {}", name))
    }
}

impl ScriptParser {
    pub fn parse(content: &str) -> Result<MonsterScript, String> {
        let root = Self::parse_text(content)?;
        let mut script = MonsterScript::default();

        if let Some(val) = root.get_token_string("basemodelfilename") {
            script.base_model_file_name = val;
        }
        if let Some(val) = root.get_token_string("treefilename") {
            script.tree_file_name = val;
        }
        if let Some(val) = root.get_token_string("attributefilename") {
            script.attribute_file_name = val;
        }
        if let Some(val) = root.get_token_string("smokebonename") {
            script.smoke_bone_name = val;
        }
        if let Some(val) = root.get_token_string("motionlistfilename") {
            script.motion_list_file_name = val;
        }

        if let Some(vec) = root.get_token_vector("smokefilename") {
            if vec.len() % 2 == 0 {
                let count = vec.len() / 2;
                for i in 0..count {
                    let smoke_idx = u32::from_str(&vec[i * 2]).unwrap_or(0);
                    let effect_file = vec[i * 2 + 1].trim_matches('"').to_string();
                    script.smoke_effects.push((smoke_idx, effect_file));
                }
            }
        }

        for child in &root.children {
            let lower_name = child.name.to_lowercase();
            if lower_name == "shapedata" {
                Self::parse_shape_data_group(child, &mut script);
            } else if lower_name == "hairdata" {
                Self::parse_hair_data_group(child, &mut script);
            } else if lower_name == "attachingdata" {
                Self::parse_attaching_data_group(child, &mut script);
            }
        }

        Ok(script)
    }

    fn parse_shape_data_group(node: &GroupNode, script: &mut MonsterScript) {
        let path_name = node.get_token_string("pathname").unwrap_or_default();
        let _special_path = node.get_token_string("specialpath").unwrap_or_default(); // C++ ignores it usually, or just temp
        
        let _shape_data_count = node.get_token_u32("shapedatacount").unwrap_or(0);
        
        for child in &node.children {
            if child.name.to_lowercase().starts_with("shapedata") {
                let mut shape = ShapeData::default();
                
                if let Some(idx) = child.get_token_u32("shapeindex") {
                    shape.shape_index = idx;
                } else {
                    continue;
                }

                if let Some(model) = child.get_token_string("model") {
                    shape.model_file_name = format!("{}{}", path_name, model);
                } else if let Some(local_model) = child.get_token_string("local_model") {
                    shape.model_file_name = local_model;
                }

                if let (Some(local_src), Some(local_tgt)) = (
                    child.get_token_string("local_sourceskin"),
                    child.get_token_string("local_targetskin")
                ) {
                    shape.skins.push(SkinData {
                        part: 0,
                        src_file_name: local_src,
                        dst_file_name: local_tgt,
                    });
                }

                if let (Some(src), Some(tgt)) = (
                    child.get_token_string("sourceskin"),
                    child.get_token_string("targetskin")
                ) {
                    shape.skins.push(SkinData {
                        part: 0,
                        src_file_name: format!("{}{}", path_name, src),
                        dst_file_name: format!("{}{}", path_name, tgt),
                    });
                }
                
                if let (Some(src2), Some(tgt2)) = (
                    child.get_token_string("sourceskin2"),
                    child.get_token_string("targetskin2")
                ) {
                    shape.skins.push(SkinData {
                        part: 0,
                        src_file_name: format!("{}{}", path_name, src2),
                        dst_file_name: format!("{}{}", path_name, tgt2),
                    });
                }
                
                script.shape_data.push(shape);
            }
        }
    }
}

impl ScriptParser {
    fn parse_hair_data_group(node: &GroupNode, script: &mut MonsterScript) {
        let path_name = node.get_token_string("pathname").unwrap_or_default();
        
        let _hair_data_count = node.get_token_u32("hairdatacount").unwrap_or(0);
        
        for child in &node.children {
            if child.name.to_lowercase().starts_with("hairdata") {
                let mut hair = HairData::default();
                
                if let Some(idx) = child.get_token_u32("hairindex") {
                    hair.hair_index = idx;
                } else {
                    continue;
                }

                if let (Some(model), Some(src), Some(tgt)) = (
                    child.get_token_string("model"),
                    child.get_token_string("sourceskin"),
                    child.get_token_string("targetskin")
                ) {
                    hair.model_file_name = format!("{}{}", path_name, model);
                    hair.skins.push(SkinData {
                        part: 0,
                        src_file_name: format!("{}{}", path_name, src),
                        dst_file_name: format!("{}{}", path_name, tgt),
                    });
                }
                
                script.hair_data.push(hair);
            }
        }
    }

    fn parse_attaching_data_group(node: &GroupNode, script: &mut MonsterScript) {
        let _attaching_data_count = node.get_token_u32("attachingdatacount").unwrap_or(0);

        for child in &node.children {
            if child.name.to_lowercase().starts_with("attachingdata") {
                let mut attach = AttachingData::default();
                
                if let Some(dtype) = child.get_token_u32("attachingdatatype") {
                    attach.data_type = AttachingDataType::from(dtype);
                } else {
                    continue;
                }

                attach.is_attaching = child.get_token_bool("isattaching").unwrap_or(false);
                attach.attaching_model_index = child.get_token_u32("attachingmodelindex").unwrap_or(0);
                attach.attaching_bone_name = child.get_token_string("attachingbonename").unwrap_or_default();

                match attach.data_type {
                    AttachingDataType::CollisionData => {
                        let mut col = CollisionData::default();
                        col.collision_type = child.get_token_i32("collisiontype").unwrap_or(0);
                        
                        let _sphere_count = child.get_token_u32("spheredatacount").unwrap_or(0);
                        for sub_child in &child.children {
                            if sub_child.name.to_lowercase().starts_with("spheredata") {
                                let mut sphere = SphereData::default();
                                sphere.radius = sub_child.get_token_float("radius").unwrap_or(0.0);
                                sphere.position = sub_child.get_token_position("position").unwrap_or([0.0, 0.0, 0.0]);
                                col.spheres.push(sphere);
                            }
                        }
                        attach.collision_data = Some(col);
                    },
                    AttachingDataType::Effect => {
                        let mut eff = EffectData::default();
                        eff.file_name = child.get_token_string("effectscriptname").unwrap_or_default();
                        eff.position = child.get_token_position("effectposition").unwrap_or([0.0, 0.0, 0.0]);
                        eff.rotation = child.get_token_position("effectrotation").unwrap_or([0.0, 0.0, 0.0]);
                        attach.effect_data = Some(eff);
                    },
                    AttachingDataType::Object => {
                        let mut obj = ObjectData::default();
                        obj.file_name = child.get_token_string("objectscriptname").unwrap_or_default();
                        attach.object_data = Some(obj);
                    },
                    AttachingDataType::None => {}
                }

                script.attaching_data.push(attach);
            }
        }
    }
}
