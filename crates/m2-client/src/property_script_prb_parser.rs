use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum ParsedProperty {
    Tree(PropertyTree),
    Building(PropertyBuilding),
    Effect(PropertyEffect),
    Ambience(PropertyAmbience),
    DungeonBlock(PropertyDungeonBlock),
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropertyTree {
    pub name: String,
    pub tree_file: String,
    pub tree_size: f32,
    pub tree_variance: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropertyBuilding {
    pub name: String,
    pub building_file: String,
    pub attribute_data_file: String,
    pub is_shadow_flag: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropertyEffect {
    pub name: String,
    pub effect_file: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropertyAmbience {
    pub name: String,
    pub play_type: String,
    pub play_interval: f32,
    pub play_interval_variation: f32,
    pub max_volume_area_percentage: f32,
    pub ambience_sound_vector: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropertyDungeonBlock {
    pub name: String,
    pub dungeon_block_file: String,
    pub attribute_data_file: String,
}

/// Parses a single line into a list of tokens, properly handling quotes and spaces.
fn parse_tokens(line: &str) -> Vec<String> {
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

fn get_no_extension(filename: &str) -> String {
    if let Some(idx) = filename.rfind('.') {
        if let Some(slash_idx) = filename.rfind('/') {
            if idx > slash_idx {
                return filename[..idx].to_string();
            }
        } else if let Some(backslash_idx) = filename.rfind('\\') {
            if idx > backslash_idx {
                return filename[..idx].to_string();
            }
        } else {
            return filename[..idx].to_string();
        }
    }
    filename.to_string()
}

pub fn parse_property_script(content: &str) -> Result<(u32, ParsedProperty), String> {
    let mut lines = content.lines();

    let mut first_line = lines.next().ok_or("Empty file")?.trim();
    if first_line.starts_with("YPRT") {
        first_line = &first_line[4..];
    } else {
        return Err("Missing YPRT header".into());
    }

    first_line = first_line.trim();

    let crc32 = if !first_line.is_empty() {
        first_line.parse::<u32>().unwrap_or(0)
    } else {
        lines.next().ok_or("Missing CRC32")?.trim().parse::<u32>().unwrap_or(0)
    };

    let mut token_map: HashMap<String, Vec<String>> = HashMap::new();

    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut tokens = parse_tokens(line);
        if tokens.is_empty() {
            continue;
        }

        let key = tokens.remove(0).to_lowercase();
        token_map.insert(key, tokens);
    }

    let prop_type_tokens = token_map.get("propertytype").ok_or("Missing PropertyType")?;
    let property_type = prop_type_tokens.first().ok_or("Empty PropertyType")?.as_str();

    let prop_name_tokens = token_map.get("propertyname").ok_or("Missing PropertyName")?;
    let property_name = prop_name_tokens.first().ok_or("Empty PropertyName")?.clone();

    match property_type {
        "Tree" => {
            let tree_file = token_map.get("treefile").and_then(|v| v.first()).cloned().ok_or("Missing TreeFile")?;
            let tree_size = token_map.get("treesize").and_then(|v| v.first()).and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
            let tree_variance = token_map.get("treevariance").and_then(|v| v.first()).and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);

            Ok((crc32, ParsedProperty::Tree(PropertyTree {
                name: property_name,
                tree_file,
                tree_size,
                tree_variance,
            })))
        }
        "Building" => {
            let building_file = token_map.get("buildingfile").and_then(|v| v.first()).cloned().ok_or("Missing BuildingFile")?;
            let attribute_data_file = format!("{}.mdatr", get_no_extension(&building_file));
            
            let is_shadow_flag = token_map.get("shadowflag")
                .and_then(|v| v.first())
                .map(|s| s != "0" && s.to_lowercase() != "false")
                .unwrap_or(false);

            Ok((crc32, ParsedProperty::Building(PropertyBuilding {
                name: property_name,
                building_file,
                attribute_data_file,
                is_shadow_flag,
            })))
        }
        "Effect" => {
            let effect_file = token_map.get("effectfile").and_then(|v| v.first()).cloned().ok_or("Missing EffectFile")?;

            Ok((crc32, ParsedProperty::Effect(PropertyEffect {
                name: property_name,
                effect_file,
            })))
        }
        "Ambience" => {
            let play_type = token_map.get("playtype").and_then(|v| v.first()).cloned().unwrap_or_default();
            let play_interval = token_map.get("playinterval").and_then(|v| v.first()).and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
            let play_interval_variation = token_map.get("playintervalvariation").and_then(|v| v.first()).and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
            let max_volume_area_percentage = token_map.get("maxvolumeareapercentage").and_then(|v| v.first()).and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
            let ambience_sound_vector = token_map.get("ambiencesoundvector").cloned().unwrap_or_default();

            Ok((crc32, ParsedProperty::Ambience(PropertyAmbience {
                name: property_name,
                play_type,
                play_interval,
                play_interval_variation,
                max_volume_area_percentage,
                ambience_sound_vector,
            })))
        }
        "DungeonBlock" => {
            let dungeon_block_file = token_map.get("dungeonblockfile").and_then(|v| v.first()).cloned().ok_or("Missing DungeonBlockFile")?;
            let attribute_data_file = format!("{}.mdatr", get_no_extension(&dungeon_block_file));

            Ok((crc32, ParsedProperty::DungeonBlock(PropertyDungeonBlock {
                name: property_name,
                dungeon_block_file,
                attribute_data_file,
            })))
        }
        _ => Err(format!("Unknown PropertyType: {}", property_type)),
    }
}
// property_script_prb_parser implementation complete
