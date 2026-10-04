use m2_client::target_board_info_controller::{ElementType, TargetBoardInfoController, TargetRank};

#[test]
fn test_set_and_clear_target() {
    let mut controller = TargetBoardInfoController::new();

    // Set target
    controller.set_target(
        1234,
        "Dziki Pies".to_string(),
        5,
        TargetRank::Normal,
        None,
    );

    assert_eq!(controller.get_current_vid(), Some(1234));
    assert_eq!(controller.get_formatted_name(), Some("Lv.5 Dziki Pies [Zwykly]".to_string()));
    assert_eq!(controller.get_hp_percent(), Some(100.0));
    assert_eq!(controller.get_element_icon_name(), None);

    // Clear target
    controller.clear_target();
    assert_eq!(controller.get_current_vid(), None);
    assert_eq!(controller.get_formatted_name(), None);
    assert_eq!(controller.get_hp_percent(), None);
}

#[test]
fn test_update_hp_from_packet() {
    let mut controller = TargetBoardInfoController::new();

    controller.set_target(
        5555,
        "Wodz Orkow".to_string(),
        50,
        TargetRank::Boss,
        Some(ElementType::Earth),
    );

    assert_eq!(controller.get_hp_percent(), Some(100.0));
    assert_eq!(controller.get_element_icon_name(), Some("Ziemia"));
    assert_eq!(controller.get_formatted_name(), Some("Lv.50 Wodz Orkow [Boss]".to_string()));

    // Update with valid vid
    controller.update_from_packet(5555, 75);
    assert_eq!(controller.get_hp_percent(), Some(75.0));

    // Update with invalid vid (should be ignored)
    controller.update_from_packet(1111, 20);
    assert_eq!(controller.get_hp_percent(), Some(75.0));
    
    // Update exceeding 100
    controller.update_from_packet(5555, 150);
    assert_eq!(controller.get_hp_percent(), Some(100.0));
}

#[test]
fn test_element_types() {
    let elements = vec![
        (ElementType::Fire, "Ogien"),
        (ElementType::Ice, "Lod"),
        (ElementType::Lightning, "Blyskawica"),
        (ElementType::Wind, "Wiatr"),
        (ElementType::Dark, "Mrok"),
        (ElementType::Earth, "Ziemia"),
    ];

    for (element, expected_name) in elements {
        assert_eq!(element.as_str(), expected_name);
    }
}

#[test]
fn test_target_ranks() {
    let ranks = vec![
        (TargetRank::Normal, "Zwykly"),
        (TargetRank::Cursed, "Przeklety"),
        (TargetRank::Commander, "Dowodca"),
        (TargetRank::Boss, "Boss"),
    ];

    for (rank, expected_name) in ranks {
        assert_eq!(rank.as_str(), expected_name);
    }
}
