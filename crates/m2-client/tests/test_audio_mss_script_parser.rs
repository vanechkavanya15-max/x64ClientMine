use m2_client::audio_mss_script_parser::{AudioMssScriptParser, MssSoundTrigger, SoundData};
use std::fs;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_parse_valid_mss_content() {
    let content = r#"
ScriptType        CharacterSoundInformation

SoundDataCount    3
SoundData00       0.100000 "sound/common/swing/w_sword_1.wav"
SoundData01       0.500000 "sound/common/swing/w_sword_2.wav"
SoundData02       0.900000 sound/common/swing/w_sword_3.wav
"#;
    let result = AudioMssScriptParser::parse_mss_content(content, Some("custom/path/"));
    assert!(result.is_ok());
    let script = result.unwrap();

    assert_eq!(script.data.len(), 3);
    assert_eq!(script.data[0].time, 0.1);
    assert_eq!(script.data[0].file_name, "custom/path/sound/common/swing/w_sword_1.wav");

    assert_eq!(script.data[1].time, 0.5);
    assert_eq!(script.data[1].file_name, "custom/path/sound/common/swing/w_sword_2.wav");

    assert_eq!(script.data[2].time, 0.9);
    assert_eq!(script.data[2].file_name, "custom/path/sound/common/swing/w_sword_3.wav");
    
    // FPS is 60 -> frame = time / (1/60) = time * 60
    assert_eq!(script.instances.len(), 3);
    assert_eq!(script.instances[0].frame, 6);
    assert_eq!(script.instances[1].frame, 30);
    assert_eq!(script.instances[2].frame, 54);
}

#[test]
fn test_parse_malformed_mss_content() {
    let missing_count = r#"
SoundData00       0.100000 "sound/1.wav"
"#;
    let result = AudioMssScriptParser::parse_mss_content(missing_count, None);
    assert!(result.is_err());
    
    let bad_time = r#"
SoundDataCount    1
SoundData00       abc "sound/1.wav"
"#;
    let result = AudioMssScriptParser::parse_mss_content(bad_time, None);
    assert!(result.is_err());

    let incomplete_data = r#"
SoundDataCount    2
SoundData00       0.100000 "sound/1.wav"
"#;
    let result = AudioMssScriptParser::parse_mss_content(incomplete_data, None);
    assert!(result.is_err()); // expects 2 items, only found 1
}

#[test]
fn test_mss_sound_trigger_logic() {
    let content = r#"
SoundDataCount    3
SoundData00       0.100000 "sound/1.wav"
SoundData01       0.500000 "sound/2.wav"
SoundData02       0.900000 "sound/3.wav"
"#;
    // frames: 6, 30, 54
    let script = AudioMssScriptParser::parse_mss_content(content, None).unwrap();
    let script_arc = std::sync::Arc::new(script);
    let mut trigger = MssSoundTrigger::new(script_arc);

    // Initial update to set base frame
    let sounds = trigger.update(0);
    assert!(sounds.is_empty());

    // Advance to frame 5 - no trigger yet
    let sounds = trigger.update(5);
    assert!(sounds.is_empty());

    // Advance to frame 10 - triggers frame 6 sound
    let sounds = trigger.update(10);
    assert_eq!(sounds.len(), 1);
    assert_eq!(sounds[0], "sound/1.wav");

    // Jump to frame 60 - triggers frame 30 and 54 sounds
    let sounds = trigger.update(60);
    assert_eq!(sounds.len(), 2);
    assert_eq!(sounds[0], "sound/2.wav");
    assert_eq!(sounds[1], "sound/3.wav");

    // Loop wraparound: jump back to frame 10 (simulate loop from frame > 60 back to 0)
    // Should trigger remaining frames (none) + frame 6
    let sounds = trigger.update(10);
    assert_eq!(sounds.len(), 1);
    assert_eq!(sounds[0], "sound/1.wav");
}

#[test]
fn test_audio_mss_script_parser_caching() {
    let parser = AudioMssScriptParser::new();
    
    let mut temp_file = NamedTempFile::new().unwrap();
    let content = r#"
SoundDataCount    1
SoundData00       0.100000 "test.wav"
"#;
    temp_file.write_all(content.as_bytes()).unwrap();
    let path = temp_file.path().to_str().unwrap().to_string();

    let script1 = parser.load_sound_script(&path, None).expect("Should parse");
    let script2 = parser.load_sound_script(&path, None).expect("Should use cache");

    // Ensure it's the exact same Arc (cached)
    assert!(std::sync::Arc::ptr_eq(&script1, &script2));
}
