#[path = "../src/map_property_parser.rs"]
mod map_property_parser;

use map_property_parser::{parse_tokens, Tokenizer, MapProperty, AreaProperty, AreaAmbience};

#[test]
fn test_parse_tokens() {
    let line1 = "ScriptType MapProperty";
    let tokens1 = parse_tokens(line1);
    assert_eq!(tokens1, vec!["ScriptType", "MapProperty"]);

    let line2 = "MapType \"Outdoor Map\"";
    let tokens2 = parse_tokens(line2);
    assert_eq!(tokens2, vec!["MapType", "Outdoor Map"]);

    let line3 = "  BasePosition \t 1000 2000 \r";
    let tokens3 = parse_tokens(line3);
    assert_eq!(tokens3, vec!["BasePosition", "1000", "2000"]);
}

#[test]
fn test_tokenizer_with_comments() {
    let content = r#"
        // This is a comment
        ScriptType MapProperty // inline comment
        MapSize 2 3
    "#;
    let tk = Tokenizer::new(content);
    assert_eq!(tk.get_string("ScriptType").as_deref(), Some("MapProperty"));
    assert_eq!(tk.get_u32("MapSize", 0), Some(2));
    assert_eq!(tk.get_u32("MapSize", 1), Some(3));
}

#[test]
fn test_map_property_parser() {
    let content = r#"
        ScriptType MapProperty
        MapType "Outdoor"
        MapSize 4 4
        BasePosition 153600 256000
    "#;
    let prop = MapProperty::parse(content).unwrap();
    assert_eq!(prop.script_type, "MapProperty");
    assert_eq!(prop.map_type, "Outdoor");
    assert_eq!(prop.map_size_x, 4);
    assert_eq!(prop.map_size_y, 4);
    assert_eq!(prop.base_position_x, 153600);
    assert_eq!(prop.base_position_y, 256000);
}

#[test]
fn test_map_property_parser_error() {
    let content = r#"
        ScriptType InvalidProperty
        MapSize 1 1
    "#;
    let result = MapProperty::parse(content);
    assert!(result.is_err());
}

#[test]
fn test_area_property_parser() {
    let content = r#"
        ScriptType AreaProperty
        SkyBox "sky/skybox.msenv"
        LightDirection 0.5 0.5 -1.0
        AmbientColor 0.2 0.3 0.4
        DiffuseColor 0.8 0.8 0.8
    "#;
    let prop = AreaProperty::parse(content).unwrap();
    assert_eq!(prop.script_type, "AreaProperty");
    assert_eq!(prop.sky_file, "sky/skybox.msenv");
    assert_eq!(prop.light_direction, [0.5, 0.5, -1.0]);
    assert_eq!(prop.ambient_color, [0.2, 0.3, 0.4]);
    assert_eq!(prop.diffuse_color, [0.8, 0.8, 0.8]);
}

#[test]
fn test_area_ambience_parser() {
    let content = r#"
        ScriptType AreaAmbience
        AmbienceSound "sound/ambience/wind.wav"
        PlayInterval 1.5 3.0
        MaxVolumeAreaPercentage 0.8
    "#;
    let prop = AreaAmbience::parse(content).unwrap();
    assert_eq!(prop.script_type, "AreaAmbience");
    assert_eq!(prop.sound_file, "sound/ambience/wind.wav");
    assert_eq!(prop.play_interval_min, 1.5);
    assert_eq!(prop.play_interval_max, 3.0);
    assert_eq!(prop.volume_percentage, 0.8);
}
