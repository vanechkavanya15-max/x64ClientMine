//! Modul `system_config` — Zarzadca konfiguracji systemowej metin2.cfg
//! Wzorzec referencyjny C++: `source/UserInterface/PythonSystem.cpp`
//! Opis: Odwzorowanie CPythonSystem: odczyt i zapis pliku metin2.cfg (WIDTH, HEIGHT, BPP, FREQUENCY, WINDOWED, MUSIC_VOLUME, VOICE_VOLUME, SHADOW_LEVEL, GAMMA, VIEW_CHAT).

use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

pub const DEFAULT_VALUE_ALWAYS_SHOW_NAME: bool = true;

#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub width: u32,
    pub height: u32,
    pub bpp: u32,
    pub frequency: u32,
    pub is_software_cursor: bool,
    pub is_object_culling: bool,
    pub i_distance: i32,
    pub i_shadow_level: i32,
    pub music_volume: f32,
    pub voice_volume: i8,
    pub gamma: i32,
    pub is_save_id: i32,
    pub save_id: String,
    pub b_windowed: bool,
    pub b_decompress_dds: bool,
    pub b_no_sound_card: bool,
    pub b_use_default_ime: bool,
    pub b_software_tiling: u8,
    pub b_view_chat: bool,
    pub b_always_show_name: bool,
    pub b_show_damage: bool,
    pub b_show_sales_text: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            width: 1024,
            height: 768,
            bpp: 32,
            frequency: 0,
            is_software_cursor: false,
            is_object_culling: true,
            i_distance: 3,
            i_shadow_level: 3,
            music_volume: 1.0,
            voice_volume: 5,
            gamma: 3,
            is_save_id: 0,
            save_id: String::new(),
            b_windowed: false,
            b_decompress_dds: false,
            b_no_sound_card: false,
            b_use_default_ime: false,
            b_software_tiling: 0,
            b_view_chat: true,
            b_always_show_name: DEFAULT_VALUE_ALWAYS_SHOW_NAME,
            b_show_damage: true,
            b_show_sales_text: true,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SystemConfig {
    pub initialized: bool,
    pub config: Config,
    pub old_config: Config,
}

impl SystemConfig {
    pub fn new() -> Self {
        let mut instance = Self {
            initialized: true,
            config: Config::default(),
            old_config: Config::default(),
        };
        instance.set_default_config();
        instance
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn set_default_config(&mut self) {
        self.config = Config::default();
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }

    pub fn set_config(&mut self, new_config: &Config) {
        self.config = new_config.clone();
    }

    pub fn is_windowed(&self) -> bool {
        self.config.b_windowed
    }

    pub fn is_view_chat(&self) -> bool {
        self.config.b_view_chat
    }

    pub fn set_view_chat_flag(&mut self, flag: i32) {
        self.config.b_view_chat = flag == 1;
    }

    pub fn is_always_show_name(&self) -> bool {
        self.config.b_always_show_name
    }

    pub fn set_always_show_name_flag(&mut self, flag: i32) {
        self.config.b_always_show_name = flag == 1;
    }

    pub fn is_show_damage(&self) -> bool {
        self.config.b_show_damage
    }

    pub fn set_show_damage_flag(&mut self, flag: i32) {
        self.config.b_show_damage = flag == 1;
    }

    pub fn is_show_sales_text(&self) -> bool {
        self.config.b_show_sales_text
    }

    pub fn set_show_sales_text_flag(&mut self, flag: i32) {
        self.config.b_show_sales_text = flag == 1;
    }

    pub fn is_auto_tiling(&self) -> bool {
        self.config.b_software_tiling == 0
    }

    pub fn set_software_tiling(&mut self, is_enable: bool) {
        if is_enable {
            self.config.b_software_tiling = 1;
        } else {
            self.config.b_software_tiling = 2;
        }
    }

    pub fn is_software_tiling(&self) -> bool {
        self.config.b_software_tiling == 1
    }

    pub fn is_use_default_ime(&self) -> bool {
        self.config.b_use_default_ime
    }

    pub fn load_config(&mut self) -> bool {
        self.load_config_from_path("metin2.cfg")
    }

    pub fn load_config_from_path<P: AsRef<Path>>(&mut self, path: P) -> bool {
        let file = match File::open(path) {
            Ok(f) => f,
            Err(_) => return false,
        };

        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = match line {
                Ok(l) => l,
                Err(_) => break,
            };

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 {
                continue;
            }

            let command = parts[0].to_uppercase();
            let value = parts[1];

            match command.as_str() {
                "WIDTH" => {
                    if let Ok(v) = value.parse() {
                        self.config.width = v;
                    }
                }
                "HEIGHT" => {
                    if let Ok(v) = value.parse() {
                        self.config.height = v;
                    }
                }
                "BPP" => {
                    if let Ok(v) = value.parse() {
                        self.config.bpp = v;
                    }
                }
                "FREQUENCY" => {
                    if let Ok(v) = value.parse() {
                        self.config.frequency = v;
                    }
                }
                "SOFTWARE_CURSOR" => {
                    if let Ok(v) = value.parse::<i32>() {
                        self.config.is_software_cursor = v != 0;
                    }
                }
                "OBJECT_CULLING" => {
                    if let Ok(v) = value.parse::<i32>() {
                        self.config.is_object_culling = v != 0;
                    }
                }
                "VISIBILITY" => {
                    if let Ok(v) = value.parse() {
                        self.config.i_distance = v;
                    }
                }
                "MUSIC_VOLUME" => {
                    if !value.contains('.') {
                        if let Ok(v) = value.parse::<i32>() {
                            if v == 0 {
                                self.config.music_volume = 0.0;
                            } else {
                                self.config.music_volume = 10.0_f32.powf(-1.0 + (v as f32) / 5.0);
                            }
                        }
                    } else if let Ok(v) = value.parse() {
                        self.config.music_volume = v;
                    }
                }
                "VOICE_VOLUME" => {
                    if let Ok(v) = value.parse() {
                        self.config.voice_volume = v;
                    }
                }
                "GAMMA" => {
                    if let Ok(v) = value.parse() {
                        self.config.gamma = v;
                    }
                }
                "IS_SAVE_ID" => {
                    if let Ok(v) = value.parse() {
                        self.config.is_save_id = v;
                    }
                }
                "SAVE_ID" => {
                    let id = value.chars().take(20).collect();
                    self.config.save_id = id;
                }
                "WINDOWED" => {
                    if let Ok(v) = value.parse::<i32>() {
                        self.config.b_windowed = v == 1;
                    }
                }
                "USE_DEFAULT_IME" => {
                    if let Ok(v) = value.parse::<i32>() {
                        self.config.b_use_default_ime = v == 1;
                    }
                }
                "SOFTWARE_TILING" => {
                    if let Ok(v) = value.parse() {
                        self.config.b_software_tiling = v;
                    }
                }
                "SHADOW_LEVEL" => {
                    if let Ok(v) = value.parse() {
                        self.config.i_shadow_level = v;
                    }
                }
                "DECOMPRESSED_TEXTURE" => {
                    if let Ok(v) = value.parse::<i32>() {
                        self.config.b_decompress_dds = v == 1;
                    }
                }
                "NO_SOUND_CARD" => {
                    if let Ok(v) = value.parse::<i32>() {
                        self.config.b_no_sound_card = v == 1;
                    }
                }
                "VIEW_CHAT" => {
                    if let Ok(v) = value.parse::<i32>() {
                        self.config.b_view_chat = v == 1;
                    }
                }
                "ALWAYS_VIEW_NAME" => {
                    if let Ok(v) = value.parse::<i32>() {
                        self.config.b_always_show_name = v == 1;
                    }
                }
                "SHOW_DAMAGE" => {
                    if let Ok(v) = value.parse::<i32>() {
                        self.config.b_show_damage = v == 1;
                    }
                }
                "SHOW_SALESTEXT" => {
                    if let Ok(v) = value.parse::<i32>() {
                        self.config.b_show_sales_text = v == 1;
                    }
                }
                _ => {}
            }
        }

        self.old_config = self.config.clone();

        true
    }

    pub fn save_config(&mut self) -> bool {
        self.save_config_to_path("metin2.cfg")
    }

    pub fn save_config_to_path<P: AsRef<Path>>(&mut self, path: P) -> bool {
        let mut file = match File::create(path) {
            Ok(f) => f,
            Err(_) => return false,
        };

        // Zapis do pliku formatowany wedlug C++ z drobnymi uproszczeniami w spacjach, aby zachowac logike zapisu.
        let mut out = String::new();
        out.push_str(&format!(
            "WIDTH						{}
",
            self.config.width
        ));
        out.push_str(&format!(
            "HEIGHT						{}
",
            self.config.height
        ));
        out.push_str(&format!(
            "BPP						{}
",
            self.config.bpp
        ));
        out.push_str(&format!(
            "FREQUENCY					{}
",
            self.config.frequency
        ));
        out.push_str(&format!(
            "SOFTWARE_CURSOR			{}
",
            if self.config.is_software_cursor { 1 } else { 0 }
        ));
        out.push_str(&format!(
            "OBJECT_CULLING				{}
",
            if self.config.is_object_culling { 1 } else { 0 }
        ));
        out.push_str(&format!(
            "VISIBILITY					{}
",
            self.config.i_distance
        ));
        out.push_str(&format!(
            "MUSIC_VOLUME				{:.3}
",
            self.config.music_volume
        ));
        out.push_str(&format!(
            "VOICE_VOLUME				{}
",
            self.config.voice_volume
        ));
        out.push_str(&format!(
            "GAMMA						{}
",
            self.config.gamma
        ));
        out.push_str(&format!(
            "IS_SAVE_ID					{}
",
            self.config.is_save_id
        ));
        out.push_str(&format!(
            "SAVE_ID					{}
",
            self.config.save_id
        ));
        out.push_str(&format!(
            "PRE_LOADING_DELAY_TIME		{}
",
            0
        )); // hardcoded to 0 just to match C++ file structure if necessary, or omitted.
        out.push_str(&format!(
            "DECOMPRESSED_TEXTURE		{}
",
            if self.config.b_decompress_dds { 1 } else { 0 }
        ));

        if self.config.b_windowed {
            out.push_str(&format!(
                "WINDOWED				{}
",
                1
            ));
        }
        if !self.config.b_view_chat {
            out.push_str(&format!(
                "VIEW_CHAT				{}
",
                0
            ));
        }
        if self.config.b_always_show_name != DEFAULT_VALUE_ALWAYS_SHOW_NAME {
            out.push_str(&format!(
                "ALWAYS_VIEW_NAME		{}
",
                if self.config.b_always_show_name { 1 } else { 0 }
            ));
        }
        if !self.config.b_show_damage {
            out.push_str(&format!(
                "SHOW_DAMAGE		{}
",
                0
            ));
        }
        if !self.config.b_show_sales_text {
            out.push_str(&format!(
                "SHOW_SALESTEXT		{}
",
                0
            ));
        }

        out.push_str(&format!(
            "USE_DEFAULT_IME		{}
",
            if self.config.b_use_default_ime { 1 } else { 0 }
        ));
        out.push_str(&format!(
            "SOFTWARE_TILING		{}
",
            self.config.b_software_tiling
        ));
        out.push_str(&format!(
            "SHADOW_LEVEL			{}
",
            self.config.i_shadow_level
        ));
        out.push_str(
            "
",
        );

        if file.write_all(out.as_bytes()).is_err() {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_system_config() {
        let instance = SystemConfig::new();
        assert!(instance.is_ready());
    }
}
