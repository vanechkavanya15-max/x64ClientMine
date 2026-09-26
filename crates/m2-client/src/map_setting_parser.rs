#[derive(Debug, PartialEq)]
pub struct MapSetting {
    pub width_sectors: u32,
    pub height_sectors: u32,
    pub base_x: i32,
    pub base_y: i32,
    pub texture_set_path: String,
    pub environment_path: String,
}

impl MapSetting {
    pub fn parse(text: &str) -> Result<MapSetting, &'static str> {
        let mut width_sectors = None;
        let mut height_sectors = None;
        let mut base_x = None;
        let mut base_y = None;
        let mut texture_set_path = None;
        let mut environment_path = None;

        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") || line.starts_with('#') {
                continue;
            }

            let tokens: Vec<&str> = line.split_whitespace().collect();
            if tokens.is_empty() {
                continue;
            }

            match tokens[0] {
                "MapSize" => {
                    if tokens.len() >= 3 {
                        width_sectors = tokens[1].parse::<u32>().ok();
                        height_sectors = tokens[2].parse::<u32>().ok();
                    }
                }
                "BasePosition" => {
                    if tokens.len() >= 3 {
                        base_x = tokens[1].parse::<i32>().ok();
                        base_y = tokens[2].parse::<i32>().ok();
                    }
                }
                "TextureSet" => {
                    if tokens.len() >= 2 {
                        texture_set_path = Some(tokens[1..].join(" "));
                    }
                }
                "Environment" => {
                    if tokens.len() >= 2 {
                        environment_path = Some(tokens[1..].join(" "));
                    }
                }
                _ => {} // Ignore unknown keys
            }
        }

        let width_sectors = width_sectors.ok_or("Missing or invalid MapSize")?;
        let height_sectors = height_sectors.ok_or("Missing or invalid MapSize")?;
        let base_x = base_x.ok_or("Missing or invalid BasePosition")?;
        let base_y = base_y.ok_or("Missing or invalid BasePosition")?;
        let texture_set_path = texture_set_path.ok_or("Missing TextureSet")?;
        let environment_path = environment_path.ok_or("Missing Environment")?;

        Ok(MapSetting {
            width_sectors,
            height_sectors,
            base_x,
            base_y,
            texture_set_path,
            environment_path,
        })
    }

    pub fn get_world_bounds_cm(&self) -> (i32, i32, i32, i32) {
        let min_x = self.base_x;
        let min_y = self.base_y;
        let max_x = self.base_x + (self.width_sectors as i32 * 25600);
        let max_y = self.base_y + (self.height_sectors as i32 * 25600);
        (min_x, min_y, max_x, max_y)
    }
}
