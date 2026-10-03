#[path = "../src/cli_config_options_loader.rs"]
mod cli_config_options_loader;

use cli_config_options_loader::{ConfigLoader, ConfigOptions};
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_cli_args_parsing_width_height() {
    let mut config = ConfigOptions::default();
    let args = vec![
        String::from("--width"), String::from("1024"),
        String::from("--height"), String::from("768"),
    ];
    
    ConfigLoader::merge_with_cli(&mut config, args);
    assert_eq!(config.width, 1024);
    assert_eq!(config.height, 768);
}

#[test]
fn test_cli_args_parsing_windowed() {
    let mut config = ConfigOptions::default();
    config.windowed = false; // start with fullscreen
    
    let args = vec![String::from("--windowed")];
    ConfigLoader::merge_with_cli(&mut config, args);
    assert!(config.windowed);
    
    let args2 = vec![String::from("--fullscreen")];
    ConfigLoader::merge_with_cli(&mut config, args2);
    assert!(!config.windowed);
}

#[test]
fn test_cfg_file_missing_creates_default() {
    let config = ConfigLoader::load_from_file("a_completely_random_file_name_that_does_not_exist.cfg");
    assert_eq!(config.width, 800);
    assert_eq!(config.height, 600);
    assert_eq!(config.bpp, 32);
    assert!(config.windowed);
    assert_eq!(config.music_volume, 1.0);
}

#[test]
fn test_cfg_file_malformed_values_ignored() {
    let mut file = NamedTempFile::new().unwrap();
    // Valid and invalid values
    writeln!(file, "WIDTH\t1280").unwrap();
    writeln!(file, "HEIGHT\tNOT_A_NUMBER").unwrap();
    writeln!(file, "BPP\t16").unwrap();
    writeln!(file, "UNKNOWN_KEY\t100").unwrap();
    
    let path = file.path().to_str().unwrap();
    let config = ConfigLoader::load_from_file(path);
    
    // WIDTH and BPP should be successfully parsed
    assert_eq!(config.width, 1280);
    assert_eq!(config.bpp, 16);
    
    // HEIGHT should stay default due to parsing failure
    assert_eq!(config.height, 600);
}

#[test]
fn test_save_and_load_cycle_maintains_values() {
    let original = ConfigOptions {
        width: 1440,
        height: 900,
        bpp: 16,
        windowed: false,
        music_volume: 0.25,
    };
    
    let file = NamedTempFile::new().unwrap();
    let path = file.path().to_str().unwrap();
    
    ConfigLoader::save_to_file(&original, path).unwrap();
    
    let loaded = ConfigLoader::load_from_file(path);
    assert_eq!(original, loaded);
}
