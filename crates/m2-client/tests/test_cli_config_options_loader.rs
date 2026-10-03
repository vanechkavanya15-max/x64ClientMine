#[path = "../src/cli_config_options_loader.rs"]
mod cli_config_options_loader;

use cli_config_options_loader::{ConfigLoader, ConfigOptions};
use std::fs;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_load_default() {
    let config = ConfigLoader::load_from_file("non_existent_file.cfg");
    assert_eq!(config, ConfigOptions::default());
}

#[test]
fn test_load_from_cfg() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "WIDTH\t1920").unwrap();
    writeln!(file, "HEIGHT 1080").unwrap();
    writeln!(file, "BPP=16").unwrap();
    writeln!(file, "WINDOWED\t0").unwrap();
    writeln!(file, "MUSIC_VOLUME\t0.5").unwrap();
    
    let path = file.path().to_str().unwrap();
    let config = ConfigLoader::load_from_file(path);
    
    assert_eq!(config.width, 1920);
    assert_eq!(config.height, 1080);
    assert_eq!(config.bpp, 16);
    assert_eq!(config.windowed, false);
    assert_eq!(config.music_volume, 0.5);
}

#[test]
fn test_cli_override() {
    let mut config = ConfigOptions::default();
    let args = vec![
        String::from("--width"), String::from("2560"),
        String::from("--height=1440"),
        String::from("--fullscreen"),
    ];
    
    ConfigLoader::merge_with_cli(&mut config, args);
    
    assert_eq!(config.width, 2560);
    assert_eq!(config.height, 1440);
    assert_eq!(config.windowed, false);
}

#[test]
fn test_save_cfg() {
    let config = ConfigOptions {
        width: 1280,
        height: 720,
        bpp: 32,
        windowed: true,
        music_volume: 0.8,
    };
    
    let file = NamedTempFile::new().unwrap();
    let path = file.path().to_str().unwrap();
    
    ConfigLoader::save_to_file(&config, path).unwrap();
    
    let loaded_config = ConfigLoader::load_from_file(path);
    assert_eq!(config, loaded_config);
}
