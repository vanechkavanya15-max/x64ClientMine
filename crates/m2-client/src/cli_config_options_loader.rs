use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct ConfigOptions {
    pub width: u32,
    pub height: u32,
    pub bpp: u8,
    pub windowed: bool,
    pub music_volume: f32,
}

impl Default for ConfigOptions {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            bpp: 32,
            windowed: true,
            music_volume: 1.0,
        }
    }
}

pub struct ConfigLoader;

impl ConfigLoader {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> ConfigOptions {
        let mut config = ConfigOptions::default();
        if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);
            for line in reader.lines().flatten() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                
                let delimiter = if line.contains('=') { "=" } else { " \t" };
                
                let (key, value) = if delimiter == "=" {
                    let mut parts = line.splitn(2, '=');
                    (parts.next().unwrap_or("").trim(), parts.next().unwrap_or("").trim())
                } else {
                    let mut parts = line.split_whitespace();
                    (parts.next().unwrap_or(""), parts.next().unwrap_or(""))
                };

                if key.is_empty() || value.is_empty() {
                    continue;
                }
                
                match key.to_uppercase().as_str() {
                    "WIDTH" => {
                        if let Ok(val) = value.parse::<u32>() {
                            config.width = val;
                        }
                    }
                    "HEIGHT" => {
                        if let Ok(val) = value.parse::<u32>() {
                            config.height = val;
                        }
                    }
                    "BPP" => {
                        if let Ok(val) = value.parse::<u8>() {
                            config.bpp = val;
                        }
                    }
                    "WINDOWED" => {
                        if let Ok(val) = value.parse::<u8>() {
                            config.windowed = val != 0;
                        } else if let Ok(val) = value.parse::<bool>() {
                            config.windowed = val;
                        } else {
                            config.windowed = value.eq_ignore_ascii_case("true");
                        }
                    }
                    "MUSIC_VOLUME" => {
                        if let Ok(val) = value.parse::<f32>() {
                            config.music_volume = val;
                        }
                    }
                    _ => {}
                }
            }
        }
        config
    }

    pub fn save_to_file<P: AsRef<Path>>(config: &ConfigOptions, path: P) -> io::Result<()> {
        let mut file = File::create(path)?;
        writeln!(file, "WIDTH\t\t{}", config.width)?;
        writeln!(file, "HEIGHT\t\t{}", config.height)?;
        writeln!(file, "BPP\t\t{}", config.bpp)?;
        writeln!(file, "WINDOWED\t{}", if config.windowed { 1 } else { 0 })?;
        writeln!(file, "MUSIC_VOLUME\t{:.3}", config.music_volume)?;
        Ok(())
    }

    pub fn merge_with_cli<I>(config: &mut ConfigOptions, args: I)
    where
        I: IntoIterator<Item = String>,
    {
        let mut iter = args.into_iter();
        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "--width" => {
                    if let Some(val) = iter.next().and_then(|v| v.parse::<u32>().ok()) {
                        config.width = val;
                    }
                }
                "--height" => {
                    if let Some(val) = iter.next().and_then(|v| v.parse::<u32>().ok()) {
                        config.height = val;
                    }
                }
                "--bpp" => {
                    if let Some(val) = iter.next().and_then(|v| v.parse::<u8>().ok()) {
                        config.bpp = val;
                    }
                }
                "--windowed" => {
                    config.windowed = true;
                }
                "--fullscreen" => {
                    config.windowed = false;
                }
                "--music-volume" => {
                    if let Some(val) = iter.next().and_then(|v| v.parse::<f32>().ok()) {
                        config.music_volume = val;
                    }
                }
                _ => {
                    if let Some(stripped) = arg.strip_prefix("--width=") {
                        if let Ok(val) = stripped.parse::<u32>() {
                            config.width = val;
                        }
                    } else if let Some(stripped) = arg.strip_prefix("--height=") {
                        if let Ok(val) = stripped.parse::<u32>() {
                            config.height = val;
                        }
                    } else if let Some(stripped) = arg.strip_prefix("--bpp=") {
                        if let Ok(val) = stripped.parse::<u8>() {
                            config.bpp = val;
                        }
                    } else if let Some(stripped) = arg.strip_prefix("--music-volume=") {
                        if let Ok(val) = stripped.parse::<f32>() {
                            config.music_volume = val;
                        }
                    }
                }
            }
        }
    }
}
