pub const FLY_LINE: u8 = 0;
pub const FLY_HOMING: u8 = 1;
pub const FLY_SINE: u8 = 2;

#[derive(Debug, Clone, PartialEq)]
pub struct FlyingData {
    pub fly_type: u8,
    pub speed: f32,
    pub acceleration: f32,
    pub max_range: f32,
    pub has_tail: bool,
    pub pierce_count: u32,
    pub hit_radius: f32,
}

impl Default for FlyingData {
    fn default() -> Self {
        Self {
            fly_type: FLY_LINE,
            speed: 2000.0,
            acceleration: 0.0,
            max_range: 500.0,
            has_tail: false,
            pierce_count: 0,
            hit_radius: 10.0,
        }
    }
}

impl FlyingData {
    pub fn parse(text: &str) -> Result<FlyingData, &'static str> {
        let mut data = FlyingData::default();
        
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 {
                continue;
            }

            let key = parts[0].to_lowercase();
            let val_str = parts[1];

            match key.as_str() {
                "fly_type" => {
                    data.fly_type = val_str.parse::<u8>().unwrap_or(FLY_LINE);
                }
                "speed" => {
                    if let Ok(v) = val_str.parse::<f32>() {
                        data.speed = v;
                    }
                }
                "acceleration" => {
                    data.acceleration = val_str.parse::<f32>().unwrap_or(0.0);
                }
                "max_range" => {
                    data.max_range = val_str.parse::<f32>().unwrap_or(500.0);
                }
                "has_tail" => {
                    if let Ok(v) = val_str.parse::<u8>() {
                        data.has_tail = v != 0;
                    } else if let Ok(v) = val_str.parse::<bool>() {
                        data.has_tail = v;
                    }
                }
                "pierce_count" => {
                    data.pierce_count = val_str.parse::<u32>().unwrap_or(0);
                }
                "hit_radius" => {
                    if let Ok(v) = val_str.parse::<f32>() {
                        data.hit_radius = v;
                    }
                }
                _ => {} // Ignore unknown keys
            }
        }

        if data.speed <= 0.0 {
            return Err("Invalid speed");
        }

        if data.hit_radius <= 0.0 {
            return Err("Invalid hit radius");
        }

        Ok(data)
    }
}
