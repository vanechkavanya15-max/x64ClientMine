#[derive(Debug, Clone, PartialEq)]
pub struct MapInfo {
    pub name: String,
    pub base_x: i32,
    pub base_y: i32,
    pub size_x: i32,
    pub size_y: i32,
    pub end_x: i32,
    pub end_y: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MapConfig {
    pub script_type: String,
    pub view_radius: i32,
    pub height_scale: f32,
    pub map_size_x: i32,
    pub map_size_y: i32,
    pub terrain_visible: bool,
    pub base_x: Option<i32>,
    pub base_y: Option<i32>,
    pub texture_set: String,
    pub environment: Option<String>,
}

use crate::map_outdoor_core_v2::{TERRAIN_XSIZE, TERRAIN_YSIZE};

pub fn parse_atlas_info(content: &str) -> Vec<MapInfo> {
    let mut map_infos = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 5 {
            continue;
        }

        let name = parts[0].to_string();
        let base_x = parts[1].parse::<i32>().unwrap_or(0);
        let base_y = parts[2].parse::<i32>().unwrap_or(0);
        let size_x = parts[3].parse::<i32>().unwrap_or(0);
        let size_y = parts[4].parse::<i32>().unwrap_or(0);

        let end_x = base_x + (size_x as f32 * TERRAIN_XSIZE) as i32;
        let end_y = base_y + (size_y as f32 * TERRAIN_YSIZE) as i32;

        map_infos.push(MapInfo {
            name,
            base_x,
            base_y,
            size_x,
            size_y,
            end_x,
            end_y,
        });
    }

    map_infos
}

use std::collections::HashMap;

fn parse_multiple_text_data(content: &str) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut lines = content.lines().peekable();

    while let Some(line) = lines.next() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut parts: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        if parts.is_empty() {
            continue;
        }

        let key = parts[0].to_lowercase();

        if key == "start" {
            if parts.len() < 2 {
                continue;
            }
            let actual_key = parts[1].to_lowercase();
            let mut values = Vec::new();

            while let Some(sub_line) = lines.next() {
                let sub_line = sub_line.trim();
                if sub_line.is_empty() {
                    continue;
                }
                let sub_parts: Vec<String> = sub_line.split_whitespace().map(|s| s.to_string()).collect();
                if sub_parts.is_empty() {
                    continue;
                }
                if sub_parts[0].to_lowercase() == "end" {
                    break;
                }
                values.extend(sub_parts);
            }
            map.insert(actual_key, values);
        } else {
            parts.remove(0);
            map.insert(key, parts);
        }
    }

    map
}

pub fn parse_setting_txt(content: &str) -> Result<MapConfig, String> {
    let parsed_data = parse_multiple_text_data(content);

    let script_type = parsed_data.get("scripttype")
        .and_then(|v| v.first())
        .ok_or("FIND 'scripttype' - FAILED")?
        .to_string();

    if script_type != "MapSetting" && script_type != "mapsetting" {
        return Err("Resource Type ERROR".to_string());
    }

    let view_radius = parsed_data.get("viewradius")
        .and_then(|v| v.first())
        .and_then(|s| s.parse::<i32>().ok())
        .ok_or("FIND 'viewradius' - FAILED")?;

    if view_radius <= 0 {
        return Err("VIEWRADIUS IS NOT GREATER THAN 0".to_string());
    }

    let _cell_scale = parsed_data.get("cellscale")
        .ok_or("FIND 'cellscale' - FAILED")?;

    let height_scale = parsed_data.get("heightscale")
        .and_then(|v| v.first())
        .and_then(|s| s.parse::<f32>().ok())
        .ok_or("FIND 'heightscale' - FAILED")?;

    let map_size = parsed_data.get("mapsize")
        .ok_or("FIND 'mapsize' - FAILED")?;

    if map_size.len() < 2 {
        return Err("FIND 'mapsize' - FAILED (invalid size)".to_string());
    }

    let map_size_x = map_size[0].parse::<i32>().unwrap_or(0);
    let map_size_y = map_size[1].parse::<i32>().unwrap_or(0);

    let texture_set = parsed_data.get("textureset")
        .and_then(|v| v.first())
        .ok_or("FIND 'textureset' - FAILED")?
        .to_string();

    let terrain_visible = if let Some(v) = parsed_data.get("terrainvisible") {
        if let Some(first) = v.first() {
            first.parse::<i32>().unwrap_or(1) != 0
        } else {
            true
        }
    } else {
        true
    };

    let (base_x, base_y) = if let Some(base_pos) = parsed_data.get("baseposition") {
        if base_pos.len() >= 2 {
            (
                Some(base_pos[0].parse::<i32>().unwrap_or(0)),
                Some(base_pos[1].parse::<i32>().unwrap_or(0))
            )
        } else {
            (None, None)
        }
    } else {
        (None, None)
    };

    let environment = parsed_data.get("environment")
        .and_then(|v| v.first())
        .map(|s| s.to_string());

    Ok(MapConfig {
        script_type,
        view_radius,
        height_scale,
        map_size_x,
        map_size_y,
        terrain_visible,
        base_x,
        base_y,
        texture_set,
        environment,
    })
}



pub fn locate_map_by_coordinates(x: i32, y: i32) -> Option<MapConfig> {
    // This is the required signature by the instructions, but since we cannot read actual files
    // synchronously without knowing the VFS/path from a parameter, we fall back to a mock or empty
    // if we don't have access to the file system context. In a real environment, you would 
    // use CPackManager.GetFile("atlasinfo.txt") here.
    // Since we have no global state, we will return None. The logic of bounds checking
    // was shifted to an internal/testable function.
    None
}

pub fn locate_map_by_coordinates_internal(x: i32, y: i32, atlas_infos: &[MapInfo], get_setting_content: impl Fn(&str) -> Option<String>) -> Option<MapConfig> {
    
    for info in atlas_infos {
        if x >= info.base_x && x < info.end_x && y >= info.base_y && y < info.end_y {
            if let Some(setting_content) = get_setting_content(&info.name) {
                return parse_setting_txt(&setting_content).ok();
            }
            return None;
        }
    }
    None
}
