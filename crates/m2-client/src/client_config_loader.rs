use std::fs;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocaleConfig {
    pub service_code: u32,
    pub codepage: u32,
    pub locale_dir: String,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VideoSystemConfig {
    pub width: u32,
    pub height: u32,
    pub bpp: u32,
    pub windowed: bool,
    pub shadow_level: u32,
}

impl Default for VideoSystemConfig {
    fn default() -> Self {
        Self {
            width: 1024,
            height: 768,
            bpp: 32,
            windowed: false,
            shadow_level: 3,
        }
    }
}

pub fn parse_locale_cfg(content: &str) -> Result<LocaleConfig, String> {
    let parts: Vec<&str> = content.split_whitespace().collect();
    if parts.len() < 3 {
        return Err("Invalid locale.cfg format. Expected at least 3 tokens.".to_string());
    }

    let service_code = parts[0].parse::<u32>().map_err(|_| "Invalid service_code".to_string())?;
    let codepage = parts[1].parse::<u32>().map_err(|_| "Invalid codepage".to_string())?;
    let locale_dir = parts[2].to_string();

    Ok(LocaleConfig {
        service_code,
        codepage,
        locale_dir,
    })
}

pub fn parse_metin2_cfg(content: &str) -> VideoSystemConfig {
    let mut config = VideoSystemConfig::default();

    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let key = parts[0].to_uppercase();
            let value = parts[1];

            match key.as_str() {
                "WIDTH" => if let Ok(v) = value.parse() { config.width = v; },
                "HEIGHT" => if let Ok(v) = value.parse() { config.height = v; },
                "BPP" => if let Ok(v) = value.parse() { config.bpp = v; },
                "WINDOWED" => if let Ok(v) = value.parse::<u32>() { config.windowed = v != 0; },
                "SHADOW_LEVEL" => if let Ok(v) = value.parse() { config.shadow_level = v; },
                _ => {}
            }
        }
    }

    config
}

pub fn save_metin2_cfg(cfg: &VideoSystemConfig, file_path: &str) {
    let windowed_val = if cfg.windowed { 1 } else { 0 };
    let content = format!(
        "WIDTH\t{}\nHEIGHT\t{}\nBPP\t{}\nWINDOWED\t{}\nSHADOW_LEVEL\t{}\n",
        cfg.width, cfg.height, cfg.bpp, windowed_val, cfg.shadow_level
    );

    let _ = fs::write(file_path, content);
}
