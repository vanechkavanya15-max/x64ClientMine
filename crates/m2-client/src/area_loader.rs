//! Modul `area_loader` — Zarzadca obiektow statycznych sektora mapy CArea (budynki, drzewa)
//! Wzorzec referencyjny C++: `source/GameLib/Area.cpp`
//! Opis: Odwzorowanie CArea i TObjectData: wczytywanie rozmieszczenia obiektow statycznych (domy, mury, mosty, drzewa, portale) w sektorze mapy (AreaData.txt).

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ObjectData {
    pub position: Position,
    pub crc: u32,
    pub portal_id: [u8; 8],
    
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
    pub height_bias: f32,
    
    pub range: u32,
    pub max_volume_area_percentage: f32,
}

impl ObjectData {
    pub fn initialize_rotation(&mut self) {
        self.yaw = 0.0;
        self.pitch = 0.0;
        self.roll = 0.0;
    }
}

#[derive(Debug, Clone, Default)]
pub struct AreaLoader {
    pub initialized: bool,
    pub object_data_vector: Vec<ObjectData>,
}

impl AreaLoader {
    pub fn new() -> Self {
        Self { 
            initialized: true,
            object_data_vector: Vec::new(),
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn get_object_data_count(&self) -> usize {
        self.object_data_vector.len()
    }

    pub fn get_object_data(&self, index: usize) -> Option<&ObjectData> {
        self.object_data_vector.get(index)
    }

    pub fn clear(&mut self) {
        self.object_data_vector.clear();
    }

    pub fn load_multiple_text_data(content: &str) -> std::collections::HashMap<String, Vec<String>> {
        let mut map = std::collections::HashMap::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i].trim();
            if line.is_empty() {
                i += 1;
                continue;
            }

            let tokens: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
            if tokens.is_empty() {
                i += 1;
                continue;
            }

            let first_token = tokens[0].to_lowercase();
            if first_token == "start" && tokens.len() > 1 {
                let key = tokens[1].to_lowercase();
                let mut st_token_vector = Vec::new();
                i += 1;

                while i < lines.len() {
                    let sub_line = lines[i].trim();
                    if sub_line.is_empty() {
                        i += 1;
                        continue;
                    }

                    let sub_tokens: Vec<String> = sub_line.split_whitespace().map(|s| s.to_string()).collect();
                    if sub_tokens.is_empty() {
                        i += 1;
                        continue;
                    }

                    let sub_first = sub_tokens[0].to_lowercase();
                    if sub_first == "end" {
                        break;
                    }

                    st_token_vector.extend(sub_tokens);
                    i += 1;
                }
                map.insert(key, st_token_vector);
            } else {
                let key = first_token;
                let value = tokens[1..].to_vec();
                map.insert(key, value);
            }
            i += 1;
        }

        map
    }

    pub fn load_object_from_string(&mut self, content: &str) -> bool {
        let token_map = Self::load_multiple_text_data(content);

        if !token_map.contains_key("areadatafile") {
            return false;
        }

        if let Some(count_tokens) = token_map.get("objectcount") {
            if let Some(count) = count_tokens.get(0).and_then(|s| s.parse::<u32>().ok()) {
                for i in 0..count {
                    let obj_name = format!("object{:03}", i);
                    if let Some(vector) = token_map.get(&obj_name) {
                        if vector.len() >= 4 {
                            let mut obj = ObjectData::default();
                            obj.position.x = vector[0].parse().unwrap_or(0.0);
                            obj.position.y = vector[1].parse().unwrap_or(0.0);
                            obj.position.z = vector[2].parse().unwrap_or(0.0);
                            obj.crc = vector[3].parse().unwrap_or(0);
                            
                            obj.initialize_rotation();
                            
                            if vector.len() > 4 {
                                if let Some(s) = vector[4].find('#') {
                                    obj.yaw = vector[4][0..s].parse().unwrap_or(0.0);
                                    if let Some(s2) = vector[4][s+1..].find('#') {
                                        let p = s + 1;
                                        let real_s2 = p + s2;
                                        obj.pitch = vector[4][p..real_s2].parse().unwrap_or(0.0);
                                        obj.roll = vector[4][real_s2+1..].parse().unwrap_or(0.0);
                                    }
                                } else {
                                    obj.roll = vector[4].parse().unwrap_or(0.0);
                                }
                            }
                            
                            if vector.len() > 5 {
                                obj.height_bias = vector[5].parse().unwrap_or(0.0);
                            }
                            
                            if vector.len() > 6 {
                                let max_portals = std::cmp::min(vector.len() - 6, 8);
                                for portal_idx in 0..max_portals {
                                    obj.portal_id[portal_idx] = vector[6 + portal_idx].parse().unwrap_or(0);
                                }
                            }
                            
                            self.object_data_vector.push(obj);
                        }
                    }
                }
            } else {
                return false;
            }
        } else {
            return false;
        }

        true
    }

    pub fn load_ambience_from_string(&mut self, content: &str) -> bool {
        let token_map = Self::load_multiple_text_data(content);

        if !token_map.contains_key("areaambiencedatafile") {
            return false;
        }

        if let Some(count_tokens) = token_map.get("objectcount") {
            if let Some(count) = count_tokens.get(0).and_then(|s| s.parse::<u32>().ok()) {
                for i in 0..count {
                    let obj_name = format!("object{:03}", i);
                    if let Some(vector) = token_map.get(&obj_name) {
                        if vector.len() >= 5 {
                            let mut obj = ObjectData::default();
                            obj.position.x = vector[0].parse().unwrap_or(0.0);
                            obj.position.y = vector[1].parse().unwrap_or(0.0);
                            obj.position.z = vector[2].parse().unwrap_or(0.0);
                            obj.crc = vector[3].parse().unwrap_or(0);
                            obj.range = vector[4].parse().unwrap_or(0);
                            
                            obj.initialize_rotation();
                            obj.height_bias = 0.0;
                            obj.max_volume_area_percentage = 0.0;
                            
                            if vector.len() >= 6 {
                                obj.max_volume_area_percentage = vector[5].parse().unwrap_or(0.0);
                            }
                            
                            self.object_data_vector.push(obj);
                        }
                    }
                }
            } else {
                return false;
            }
        } else {
            return false;
        }

        true
    }

    pub fn build_object_instances(&mut self) {
        self.object_data_vector.sort_by(|a, b| a.crc.cmp(&b.crc));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_area_loader() {
        let instance = AreaLoader::new();
        assert!(instance.is_ready());
    }
}
