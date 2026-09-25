//! Modul map_setting dla skrzynki m2-client-terrain
//! Parser konfiguracji mapy Setting.txt
//! Odczyt wymiarow siatki, skali wysokosci, sciezki do textureset i otoczenia

/// Glowna struktura implementacyjna
#[derive(Debug, Clone, Default)]
pub struct MapSetting {
    pub initialized: bool,
    pub map_size_x: u32,
    pub map_size_y: u32,
    pub height_scale: f32,
    pub texture_set: String,
    pub environment: String,
}

impl MapSetting {
    pub fn new() -> Self {
        Self {
            initialized: false,
            map_size_x: 0,
            map_size_y: 0,
            height_scale: 0.0,
            texture_set: String::new(),
            environment: String::new(),
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Funkcja parsujaca zawartosc pliku Setting.txt.
    /// Wymaga obecnosci kluczy: MapSize, HeightScale, TextureSet, Environment.
    pub fn parse(content: &str) -> Result<Self, String> {
        let mut setting = Self::new();
        let mut has_map_size = false;
        let mut has_height_scale = false;
        let mut has_texture_set = false;
        let mut has_environment = false;

        for line in content.lines() {
            let line = line.trim();
            // Ignorowanie pustych linii i komentarzy
            if line.is_empty() || line.starts_with('#') || line.starts_with("//") {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            let key = parts[0].to_lowercase();
            match key.as_str() {
                "mapsize" => {
                    if parts.len() >= 3 {
                        setting.map_size_x = parts[1].parse().map_err(|_| "Blad parsowania MapSize X".to_string())?;
                        setting.map_size_y = parts[2].parse().map_err(|_| "Blad parsowania MapSize Y".to_string())?;
                        has_map_size = true;
                    } else {
                        return Err("Nieprawidlowa liczba argumentow dla MapSize".to_string());
                    }
                }
                "heightscale" => {
                    if parts.len() >= 2 {
                        setting.height_scale = parts[1].parse().map_err(|_| "Blad parsowania HeightScale".to_string())?;
                        has_height_scale = true;
                    } else {
                        return Err("Nieprawidlowa liczba argumentow dla HeightScale".to_string());
                    }
                }
                "textureset" => {
                    if parts.len() >= 2 {
                        let val = line[parts[0].len()..].trim();
                        setting.texture_set = val.trim_matches('"').trim().to_string();
                        has_texture_set = true;
                    } else {
                        return Err("Nieprawidlowa liczba argumentow dla TextureSet".to_string());
                    }
                }
                "environment" => {
                    if parts.len() >= 2 {
                        let val = line[parts[0].len()..].trim();
                        setting.environment = val.trim_matches('"').trim().to_string();
                        has_environment = true;
                    } else {
                        return Err("Nieprawidlowa liczba argumentow dla Environment".to_string());
                    }
                }
                _ => {
                    // Inne klucze moga byc ignorowane, np. ViewRadius, CellScale, BasePosition itp.
                }
            }
        }

        if !has_map_size || !has_height_scale || !has_texture_set || !has_environment {
            return Err("Brak wymaganych pol w pliku konfiguracyjnym".to_string());
        }

        setting.initialized = true;
        Ok(setting)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_map_setting() {
        let obj = MapSetting::new();
        assert!(!obj.is_ready());
    }
}
