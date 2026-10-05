#[path = "../src/property_script_prb_parser.rs"]
mod property_script_prb_parser;

use property_script_prb_parser::{
    parse_property_script, ParsedProperty
};

#[test]
fn test_parse_valid_property_building() {
    let content = r#"YPRT
123456789
PropertyType		"Building"
PropertyName		"test_building"
BuildingFile		"d:/ymir work/zone/b/building.gr2"
ShadowFlag		"1"
"#;

    let parsed = parse_property_script(content).expect("Failed to parse building");
    assert_eq!(parsed.0, 123456789);
    
    if let ParsedProperty::Building(building) = parsed.1 {
        assert_eq!(building.name, "test_building");
        assert_eq!(building.building_file, "d:/ymir work/zone/b/building.gr2");
        assert_eq!(building.attribute_data_file, "d:/ymir work/zone/b/building.mdatr");
        assert_eq!(building.is_shadow_flag, true);
    } else {
        panic!("Parsed wrong property type");
    }
}

#[test]
fn test_parse_valid_property_tree() {
    let content = r#"YPRT
987654321
PropertyType		"Tree"
PropertyName		"test_tree"
TreeFile		"d:/ymir work/tree/tree1.spt"
TreeSize		"1000.5"
TreeVariance		"15.2"
"#;

    let parsed = parse_property_script(content).expect("Failed to parse tree");
    assert_eq!(parsed.0, 987654321);
    
    if let ParsedProperty::Tree(tree) = parsed.1 {
        assert_eq!(tree.name, "test_tree");
        assert_eq!(tree.tree_file, "d:/ymir work/tree/tree1.spt");
        assert_eq!(tree.tree_size, 1000.5);
        assert_eq!(tree.tree_variance, 15.2);
    } else {
        panic!("Parsed wrong property type");
    }
}

#[test]
fn test_parse_valid_property_effect() {
    let content = r#"YPRT
55555
PropertyType		"Effect"
PropertyName		"test_effect"
EffectFile		"d:/ymir work/effect/bg/fire.mse"
"#;

    let parsed = parse_property_script(content).expect("Failed to parse effect");
    assert_eq!(parsed.0, 55555);
    
    if let ParsedProperty::Effect(effect) = parsed.1 {
        assert_eq!(effect.name, "test_effect");
        assert_eq!(effect.effect_file, "d:/ymir work/effect/bg/fire.mse");
    } else {
        panic!("Parsed wrong property type");
    }
}

#[test]
fn test_parse_valid_property_dungeon_block() {
    let content = r#"YPRT
11111
PropertyType		"DungeonBlock"
PropertyName		"test_block"
DungeonBlockFile		"d:/ymir work/zone/dungeon/block1.gr2"
"#;

    let parsed = parse_property_script(content).expect("Failed to parse dungeon block");
    assert_eq!(parsed.0, 11111);
    
    if let ParsedProperty::DungeonBlock(block) = parsed.1 {
        assert_eq!(block.name, "test_block");
        assert_eq!(block.dungeon_block_file, "d:/ymir work/zone/dungeon/block1.gr2");
        assert_eq!(block.attribute_data_file, "d:/ymir work/zone/dungeon/block1.mdatr");
    } else {
        panic!("Parsed wrong property type");
    }
}

#[test]
fn test_parse_valid_property_ambience() {
    let content = r#"YPRT
33333
PropertyType		"Ambience"
PropertyName		"test_ambience"
PlayType		"LOOP"
PlayInterval		"2.5"
PlayIntervalVariation		"0.5"
MaxVolumeAreaPercentage		"0.8"
AmbienceSoundVector		"sound1.wav" "sound2.wav"
"#;

    let parsed = parse_property_script(content).expect("Failed to parse ambience");
    assert_eq!(parsed.0, 33333);
    
    if let ParsedProperty::Ambience(ambience) = parsed.1 {
        assert_eq!(ambience.name, "test_ambience");
        assert_eq!(ambience.play_type, "LOOP");
        assert_eq!(ambience.play_interval, 2.5);
        assert_eq!(ambience.play_interval_variation, 0.5);
        assert_eq!(ambience.max_volume_area_percentage, 0.8);
        assert_eq!(ambience.ambience_sound_vector, vec!["sound1.wav".to_string(), "sound2.wav".to_string()]);
    } else {
        panic!("Parsed wrong property type");
    }
}

#[test]
fn test_parse_invalid_missing_yprt() {
    let content = r#"123456789
PropertyType		"Building"
PropertyName		"test_building"
"#;

    let parsed = parse_property_script(content);
    assert!(parsed.is_err());
}

#[test]
fn test_parse_building_no_shadowflag() {
    let content = r#"YPRT
123456789
PropertyType		"Building"
PropertyName		"test_building"
BuildingFile		"d:/ymir work/zone/b/building.gr2"
"#;

    let parsed = parse_property_script(content).expect("Failed to parse building");
    assert_eq!(parsed.0, 123456789);
    
    if let ParsedProperty::Building(building) = parsed.1 {
        assert_eq!(building.name, "test_building");
        assert_eq!(building.is_shadow_flag, false);
    } else {
        panic!("Parsed wrong property type");
    }
}

#[test]
fn test_parse_crc_same_line_as_yprt() {
    let content = r#"YPRT 123456789
PropertyType		"Building"
PropertyName		"test_building"
BuildingFile		"d:/ymir work/zone/b/building.gr2"
"#;

    let parsed = parse_property_script(content).expect("Failed to parse building");
    assert_eq!(parsed.0, 123456789);
}
// test_property_script_prb implementation complete
