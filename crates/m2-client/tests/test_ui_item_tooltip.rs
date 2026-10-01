use m2_client::ui_item_tooltip::*;

#[test]
fn test_name_color_gold() {
    let mut tooltip = ItemTooltip::new();
    let data = ItemTooltipData {
        name: "Miecz Bojowy +9".to_string(),
        ..Default::default()
    };
    tooltip.build_tooltip(&data, 1);
    
    assert_eq!(tooltip.lines[0].text, "Miecz Bojowy +9");
    assert_eq!(tooltip.lines[0].color, COLOR_TITLE_GOLD);
}

#[test]
fn test_name_color_white() {
    let mut tooltip = ItemTooltip::new();
    let data = ItemTooltipData {
        name: "Miecz Bojowy".to_string(),
        ..Default::default()
    };
    tooltip.build_tooltip(&data, 1);
    
    assert_eq!(tooltip.lines[0].text, "Miecz Bojowy");
    assert_eq!(tooltip.lines[0].color, COLOR_TITLE_WHITE);
}

#[test]
fn test_level_limit_red() {
    let mut tooltip = ItemTooltip::new();
    let data = ItemTooltipData {
        name: "Miecz".to_string(),
        level_limit: 30,
        ..Default::default()
    };
    tooltip.build_tooltip(&data, 15); // Player level < requirement
    
    assert_eq!(tooltip.lines[1].text, "Od poziomu: 30");
    assert_eq!(tooltip.lines[1].color, COLOR_LEVEL_RED);
}

#[test]
fn test_level_limit_grey() {
    let mut tooltip = ItemTooltip::new();
    let data = ItemTooltipData {
        name: "Miecz".to_string(),
        level_limit: 30,
        ..Default::default()
    };
    tooltip.build_tooltip(&data, 45); // Player level >= requirement
    
    assert_eq!(tooltip.lines[1].text, "Od poziomu: 30");
    assert_eq!(tooltip.lines[1].color, COLOR_LEVEL_GREY);
}

#[test]
fn test_damage_and_defense() {
    let mut tooltip = ItemTooltip::new();
    let data = ItemTooltipData {
        name: "Zbroja".to_string(),
        defense: 120,
        damage_min: 10,
        damage_max: 15,
        ..Default::default()
    };
    tooltip.build_tooltip(&data, 1);
    
    let damage_line = tooltip.lines.iter().find(|l| l.text.contains("Wartość ataku"));
    assert!(damage_line.is_some());
    assert_eq!(damage_line.unwrap().text, "Wartość ataku 10-15");
    assert_eq!(damage_line.unwrap().color, COLOR_DAMAGE);

    let defense_line = tooltip.lines.iter().find(|l| l.text.contains("Obrona"));
    assert!(defense_line.is_some());
    assert_eq!(defense_line.unwrap().text, "Obrona 120");
    assert_eq!(defense_line.unwrap().color, COLOR_DAMAGE);
}

#[test]
fn test_bonuses_and_attributes() {
    let mut tooltip = ItemTooltip::new();
    let data = ItemTooltipData {
        name: "Miecz".to_string(),
        base_bonuses: vec![("Szybkość ataku".to_string(), 15)],
        attrs: vec![("Silny przeciwko Orkom".to_string(), 20)],
        ..Default::default()
    };
    tooltip.build_tooltip(&data, 1);

    let bonus_line = tooltip.lines.iter().find(|l| l.text.contains("Szybkość ataku"));
    assert!(bonus_line.is_some());
    assert_eq!(bonus_line.unwrap().text, "Szybkość ataku: +15");
    assert_eq!(bonus_line.unwrap().color, COLOR_BASE_BONUS);

    let attr_line = tooltip.lines.iter().find(|l| l.text.contains("Orkom"));
    assert!(attr_line.is_some());
    assert_eq!(attr_line.unwrap().text, "Silny przeciwko Orkom: +20");
    assert_eq!(attr_line.unwrap().color, COLOR_ATTRIBUTE);
}

#[test]
fn test_sockets() {
    let mut tooltip = ItemTooltip::new();
    let data = ItemTooltipData {
        name: "Miecz".to_string(),
        sockets: [Some(28430), None, None], // 28430 = KD Powtórki +4
        ..Default::default()
    };
    tooltip.build_tooltip(&data, 1);

    let stone_line = tooltip.lines.iter().find(|l| l.text.contains("Kamień Duszy"));
    assert!(stone_line.is_some());
    assert_eq!(stone_line.unwrap().color, COLOR_SOCKET_BLUE);

    let empty_line = tooltip.lines.iter().find(|l| l.text.contains("Puste gniazdo"));
    assert!(empty_line.is_some());
    assert_eq!(empty_line.unwrap().color, COLOR_SOCKET_EMPTY);
}

#[test]
fn test_positioning() {
    let mut tooltip = ItemTooltip::new();
    tooltip.width = 200;
    tooltip.height = 100;
    
    // Normal case
    tooltip.update_position(100, 100, 800, 600);
    assert_eq!(tooltip.x, 115);
    assert_eq!(tooltip.y, 115);

    // Right edge
    tooltip.update_position(750, 100, 800, 600);
    assert_eq!(tooltip.x, 600); // 800 - 200

    // Bottom edge
    tooltip.update_position(100, 550, 800, 600);
    assert_eq!(tooltip.y, 500); // 600 - 100

    // Top-left boundary check (negative coordinate safety via offset + max)
    tooltip.update_position(-50, -50, 800, 600);
    assert_eq!(tooltip.x, 0);
    assert_eq!(tooltip.y, 0);
}
