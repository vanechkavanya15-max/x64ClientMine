use m2_client::client_config_loader::{parse_locale_cfg, parse_metin2_cfg, save_metin2_cfg, LocaleConfig, VideoSystemConfig};
use std::fs;

#[test]
fn test_parse_locale_cfg_valid() {
    let content = "10002 1250 pl";
    let cfg = parse_locale_cfg(content).unwrap();
    assert_eq!(cfg.service_code, 10002);
    assert_eq!(cfg.codepage, 1250);
    assert_eq!(cfg.locale_dir, "pl");
}

#[test]
fn test_parse_locale_cfg_valid_with_extra_spaces_and_lines() {
    let content = "  10002   1250   pl   \n  extra stuff  ";
    let cfg = parse_locale_cfg(content).unwrap();
    assert_eq!(cfg.service_code, 10002);
    assert_eq!(cfg.codepage, 1250);
    assert_eq!(cfg.locale_dir, "pl");
}

#[test]
fn test_parse_locale_cfg_invalid_format() {
    let content = "10002 1250";
    let res = parse_locale_cfg(content);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "Invalid locale.cfg format. Expected at least 3 tokens.");
}

#[test]
fn test_parse_locale_cfg_invalid_service_code() {
    let content = "abc 1250 pl";
    let res = parse_locale_cfg(content);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "Invalid service_code");
}

#[test]
fn test_parse_locale_cfg_invalid_codepage() {
    let content = "10002 xyz pl";
    let res = parse_locale_cfg(content);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "Invalid codepage");
}

#[test]
fn test_parse_metin2_cfg_valid() {
    let content = "\
WIDTH 1920
HEIGHT 1080
BPP 32
WINDOWED 1
SHADOW_LEVEL 5
UNKNOWN_KEY 123
";
    let cfg = parse_metin2_cfg(content);
    assert_eq!(cfg.width, 1920);
    assert_eq!(cfg.height, 1080);
    assert_eq!(cfg.bpp, 32);
    assert_eq!(cfg.windowed, true);
    assert_eq!(cfg.shadow_level, 5);
}

#[test]
fn test_parse_metin2_cfg_empty() {
    let content = "";
    let cfg = parse_metin2_cfg(content);
    assert_eq!(cfg.width, 1024);
    assert_eq!(cfg.height, 768);
    assert_eq!(cfg.bpp, 32);
    assert_eq!(cfg.windowed, false);
    assert_eq!(cfg.shadow_level, 3);
}

#[test]
fn test_parse_metin2_cfg_partial() {
    let content = "\
WIDTH 1280
WINDOWED 0
";
    let cfg = parse_metin2_cfg(content);
    assert_eq!(cfg.width, 1280);
    assert_eq!(cfg.height, 768);
    assert_eq!(cfg.bpp, 32);
    assert_eq!(cfg.windowed, false);
    assert_eq!(cfg.shadow_level, 3);
}

#[test]
fn test_save_metin2_cfg() {
    let cfg = VideoSystemConfig {
        width: 1440,
        height: 900,
        bpp: 16,
        windowed: true,
        shadow_level: 2,
    };
    
    let temp_file = "test_metin2.cfg";
    save_metin2_cfg(&cfg, temp_file);
    
    let content = fs::read_to_string(temp_file).unwrap();
    fs::remove_file(temp_file).unwrap();
    
    assert!(content.contains("WIDTH\t1440"));
    assert!(content.contains("HEIGHT\t900"));
    assert!(content.contains("BPP\t16"));
    assert!(content.contains("WINDOWED\t1"));
    assert!(content.contains("SHADOW_LEVEL\t2"));
    
    // Test parsing it back
    let parsed_cfg = parse_metin2_cfg(&content);
    assert_eq!(parsed_cfg, cfg);
}
