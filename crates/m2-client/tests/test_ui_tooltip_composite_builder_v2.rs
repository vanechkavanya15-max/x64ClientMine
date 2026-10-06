#[path = "../src/ui_tooltip_composite_builder_v2.rs"]
mod ui_tooltip_composite_builder_v2;

use ui_tooltip_composite_builder_v2::*;

#[test]
fn test_basic_sword_tooltip() {
    let mut builder = TooltipCompositeBuilderV2::new();
    let data = ItemData {
        name: "Miecz Bojowy +9".to_string(),
        rarity: ItemRarity::Rare,
        level_limit: 65,
        class_flags: (1 << 3) | (1 << 5), // banned for assassin and shaman (so allowed for warrior, sura)
        attack_min: 200,
        attack_max: 240,
        ..Default::default()
    };

    // Player level 70, Warrior (0)
    builder.build(&data, 70, 0).unwrap();

    let lines = &builder.lines;
    assert_eq!(lines[0].text, "Miecz Bojowy +9");
    assert_eq!(lines[0].color, [1.0, 0.78, 0.20, 1.0]); // Gold

    // Class should be allowed
    assert_eq!(lines[1].text, "Klasy: Wojownik, Sura");
    assert_eq!(lines[1].color, [0.6, 0.6, 0.6, 1.0]); // Met

    // Level should be allowed
    assert_eq!(lines[2].text, "Od poziomu: 65");
    assert_eq!(lines[2].color, [0.6, 0.6, 0.6, 1.0]); // Met

    assert_eq!(lines[3].text, "Wartość ataku 200-240");
}

#[test]
fn test_level_and_class_unmet() {
    let mut builder = TooltipCompositeBuilderV2::new();
    let data = ItemData {
        name: "Sztylety".to_string(),
        rarity: ItemRarity::Common,
        level_limit: 70,
        class_flags: (1 << 2) | (1 << 4) | (1 << 5), // only assassin allowed
        ..Default::default()
    };

    // Player level 60, Warrior (0)
    builder.build(&data, 60, 0).unwrap();

    let lines = &builder.lines;
    // Class unmet
    assert_eq!(lines[1].text, "Klasy: Ninja");
    assert_eq!(lines[1].color, [1.0, 0.1, 0.1, 1.0]); // Unmet

    // Level unmet
    assert_eq!(lines[2].text, "Od poziomu: 70");
    assert_eq!(lines[2].color, [1.0, 0.1, 0.1, 1.0]); // Unmet
}

#[test]
fn test_bonuses_and_sockets() {
    let mut builder = TooltipCompositeBuilderV2::new();
    let data = ItemData {
        name: "Zbroja z bonami".to_string(),
        defense: 150,
        main_bonuses: vec![
            Bonus { attr_type: 1, value: 2000 }, // Max HP
            Bonus { attr_type: 5, value: 12 },   // STR
        ],
        extra_bonuses: vec![
            Bonus { attr_type: 17, value: 10 },  // Silny na ludzi
        ],
        sockets: [Some(SocketData { vnum: 28430 }), None, None],
        ..Default::default()
    };

    builder.build(&data, 1, 0).unwrap();

    let lines = &builder.lines;
    // Find defense
    assert!(lines.iter().any(|l| l.text == "Obrona 150"));
    
    // Find main bonuses
    assert!(lines.iter().any(|l| l.text == "Max PŻ: +2000" && l.color == [0.8, 0.8, 0.8, 1.0]));
    assert!(lines.iter().any(|l| l.text == "Siła: +12" && l.color == [0.8, 0.8, 0.8, 1.0]));

    // Find extra bonuses
    assert!(lines.iter().any(|l| l.text == "Silny przeciwko ludziom: +10" && l.color == [0.2, 1.0, 0.2, 1.0]));

    // Find sockets
    assert!(lines.iter().any(|l| l.text == "Kamień Duszy (Vnum: 28430)"));
    assert!(lines.iter().any(|l| l.text == "Puste gniazdo"));
}

#[test]
fn test_duration_formatting() {
    let mut builder = TooltipCompositeBuilderV2::new();
    
    // Seconds
    let mut data = ItemData { duration_seconds: Some(45), ..Default::default() };
    builder.build(&data, 1, 0).unwrap();
    assert!(builder.lines.iter().any(|l| l.text == "Pozostały czas: 45 sek."));

    // Minutes
    data.duration_seconds = Some(120);
    builder.build(&data, 1, 0).unwrap();
    assert!(builder.lines.iter().any(|l| l.text == "Pozostały czas: 2 min."));

    // Hours
    data.duration_seconds = Some(7200);
    builder.build(&data, 1, 0).unwrap();
    assert!(builder.lines.iter().any(|l| l.text == "Pozostały czas: 2 godz."));

    // Days
    data.duration_seconds = Some(86400 * 3);
    builder.build(&data, 1, 0).unwrap();
    assert!(builder.lines.iter().any(|l| l.text == "Pozostały czas: 3 dni"));
}

#[test]
fn test_too_many_bonuses_error() {
    let mut builder = TooltipCompositeBuilderV2::new();
    let data = ItemData {
        main_bonuses: vec![
            Bonus { attr_type: 1, value: 1 },
            Bonus { attr_type: 2, value: 1 },
            Bonus { attr_type: 3, value: 1 },
            Bonus { attr_type: 4, value: 1 },
            Bonus { attr_type: 5, value: 1 },
            Bonus { attr_type: 6, value: 1 },
        ],
        ..Default::default()
    };
    
    let res = builder.build(&data, 1, 0);
    assert_eq!(res, Err("Too many main bonuses (max 5)"));
}

#[test]
fn test_rarity_colors() {
    let mut builder = TooltipCompositeBuilderV2::new();
    
    let rarities = [
        (ItemRarity::Common, [1.0, 1.0, 1.0, 1.0]),
        (ItemRarity::Magic, [0.0, 0.5, 1.0, 1.0]),
        (ItemRarity::Rare, [1.0, 0.78, 0.20, 1.0]),
        (ItemRarity::Epic, [0.8, 0.2, 1.0, 1.0]),
        (ItemRarity::Legendary, [1.0, 0.2, 0.2, 1.0]),
    ];

    for (rarity, color) in rarities {
        let data = ItemData {
            name: "Test".to_string(),
            rarity,
            ..Default::default()
        };
        builder.build(&data, 1, 0).unwrap();
        assert_eq!(builder.lines[0].color, color);
    }
}
