use m2_client::system_config::SystemConfig;
use std::fs;

#[test]
fn test_integration_system_config() {
    let instance = SystemConfig::new();
    assert!(instance.is_ready());
}

#[test]
fn test_default_config() {
    let mut instance = SystemConfig::new();
    instance.set_default_config();

    let config = instance.get_config();
    assert_eq!(config.width, 1024);
    assert_eq!(config.height, 768);
    assert_eq!(config.bpp, 32);
    assert_eq!(config.frequency, 0);
    assert_eq!(config.gamma, 3);
    assert_eq!(config.voice_volume, 5);
}

#[test]
fn test_load_and_save_config() {
    let mut instance = SystemConfig::new();

    // Stworzenie pliku metin2.cfg symulujacego standardowa strukture
    let mock_config_content = "\
WIDTH 1920
HEIGHT 1080
BPP 32
FREQUENCY 60
SOFTWARE_CURSOR 1
OBJECT_CULLING 0
VISIBILITY 4
MUSIC_VOLUME 0.5
VOICE_VOLUME 3
GAMMA 2
IS_SAVE_ID 1
SAVE_ID TestUser
WINDOWED 1
USE_DEFAULT_IME 1
SOFTWARE_TILING 1
SHADOW_LEVEL 4
DECOMPRESSED_TEXTURE 1
NO_SOUND_CARD 0
VIEW_CHAT 0
ALWAYS_VIEW_NAME 1
SHOW_DAMAGE 0
SHOW_SALESTEXT 0
";
    
    let test_file_path = "test_metin2.cfg";
    fs::write(test_file_path, mock_config_content).unwrap();

    let loaded = instance.load_config_from_path(test_file_path);
    assert!(loaded, "Powinno poprawnie zaladowac plik konfiguracji");

    let config = instance.get_config();
    assert_eq!(config.width, 1920);
    assert_eq!(config.height, 1080);
    assert_eq!(config.frequency, 60);
    assert_eq!(config.is_software_cursor, true);
    assert_eq!(config.is_object_culling, false);
    assert_eq!(config.i_distance, 4);
    assert_eq!(config.music_volume, 0.5);
    assert_eq!(config.voice_volume, 3);
    assert_eq!(config.gamma, 2);
    assert_eq!(config.is_save_id, 1);
    assert_eq!(config.save_id, "TestUser");
    assert_eq!(config.b_windowed, true);
    assert_eq!(config.b_view_chat, false);
    assert_eq!(config.b_show_damage, false);
    assert_eq!(config.b_show_sales_text, false);
    assert_eq!(config.b_always_show_name, true);

    // Modyfikacja i zapis
    instance.set_view_chat_flag(1);
    instance.set_show_damage_flag(1);

    let save_file_path = "test_out_metin2.cfg";
    let saved = instance.save_config_to_path(save_file_path);
    assert!(saved, "Powinno poprawnie zapisac plik");

    let mut instance_new = SystemConfig::new();
    instance_new.load_config_from_path(save_file_path);

    let config_new = instance_new.get_config();
    assert_eq!(config_new.width, 1920);
    assert_eq!(config_new.height, 1080);
    assert_eq!(config_new.b_view_chat, true);
    assert_eq!(config_new.b_show_damage, true);
    
    // Sprzatanie po testach
    fs::remove_file(test_file_path).unwrap();
    fs::remove_file(save_file_path).unwrap();
}

#[test]
fn test_old_music_volume_compatibility() {
    let mut instance = SystemConfig::new();

    let mock_config_content = "MUSIC_VOLUME 0\n";
    let test_file_path = "test_metin2_music_0.cfg";
    fs::write(test_file_path, mock_config_content).unwrap();

    instance.load_config_from_path(test_file_path);
    assert_eq!(instance.get_config().music_volume, 0.0);

    let mock_config_content = "MUSIC_VOLUME 5\n"; // Max volume w starej skali
    let test_file_path_2 = "test_metin2_music_5.cfg";
    fs::write(test_file_path_2, mock_config_content).unwrap();

    instance.load_config_from_path(test_file_path_2);
    // formula: 10.0^((-1.0 + 5/5)) = 10.0^0 = 1.0
    assert_eq!(instance.get_config().music_volume, 1.0);

    // Sprzatanie po testach
    fs::remove_file(test_file_path).unwrap();
    fs::remove_file(test_file_path_2).unwrap();
}
