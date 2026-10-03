use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyType {
    Building,
    Tree,
    Decoration,
    Effect,
    DungeonBlock,
    Unknown(String),
}

impl From<&str> for PropertyType {
    fn from(s: &str) -> Self {
        match s {
            "Building" => PropertyType::Building,
            "Tree" => PropertyType::Tree,
            "Decoration" => PropertyType::Decoration,
            "Effect" => PropertyType::Effect,
            "DungeonBlock" => PropertyType::DungeonBlock,
            _ => PropertyType::Unknown(s.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Property {
    pub id: u32,
    pub name: String,
    pub property_type: PropertyType,
    pub building_file: Option<String>,
    pub is_attribute_data: Option<bool>,
    pub extra_fields: HashMap<String, String>,
}

pub fn parse_property_file(content: &str) -> Option<Property> {
    let mut lines = content.lines().map(str::trim).filter(|l| !l.is_empty());
    
    if lines.next()? != "YPRT" {
        return None;
    }
    
    let id_str = lines.next()?;
    let id = id_str.parse::<u32>().ok()?;
    
    let mut name = String::new();
    let mut property_type = PropertyType::Unknown("".to_string());
    let mut building_file = None;
    let mut is_attribute_data = None;
    let mut extra_fields = HashMap::new();
    
    for line in lines {
        if let Some(idx) = line.find(|c: char| c.is_whitespace()) {
            let key = line[..idx].trim().to_lowercase();
            let val = line[idx..].trim().trim_matches('"').to_string();
            
            match key.as_str() {
                "propertyname" => name = val,
                "propertytype" => property_type = PropertyType::from(val.as_str()),
                "buildingfile" | "dungeondir" => building_file = Some(val),
                "isattributedata" => is_attribute_data = Some(val == "1" || val.to_lowercase() == "true"),
                _ => {
                    extra_fields.insert(key, val);
                }
            }
        }
    }
    
    Some(Property {
        id,
        name,
        property_type,
        building_file,
        is_attribute_data,
        extra_fields,
    })
}
