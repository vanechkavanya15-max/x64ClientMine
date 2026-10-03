use m2_client::ui_item_tooltip_builder::ItemTooltipBuilder;
use m2_client::ui_item_tooltip_renderer::{TooltipElement, COLOR_TITLE_WHITE, COLOR_TITLE_GOLD, COLOR_DAMAGE, COLOR_LEVEL_RED, COLOR_SOCKET_BLUE};

#[test]
fn test_builder_basic_name() {
    let renderer = ItemTooltipBuilder::new()
        .name("Miecz")
        .build();

    assert_eq!(renderer.elements.len(), 1);
    assert_eq!(
        renderer.elements[0],
        TooltipElement::Text {
            text: "Miecz".to_string(),
            color: COLOR_TITLE_WHITE,
        }
    );
}

#[test]
fn test_builder_gold_name() {
    let renderer = ItemTooltipBuilder::new()
        .name("Miecz+9")
        .build();

    assert_eq!(renderer.elements.len(), 1);
    assert_eq!(
        renderer.elements[0],
        TooltipElement::Text {
            text: "Miecz+9".to_string(),
            color: COLOR_TITLE_GOLD,
        }
    );
}

#[test]
fn test_builder_requirements_unmet() {
    let renderer = ItemTooltipBuilder::new()
        .name("Zbroja")
        .requirements(50, 0, 30, 0) // Limit level 50, player 30
        .build();

    let has_unmet_level = renderer.elements.iter().any(|el| {
        if let TooltipElement::Text { text, color } = el {
            text == "Od poziomu: 50" && *color == COLOR_LEVEL_RED
        } else {
            false
        }
    });

    assert!(has_unmet_level);
}

#[test]
fn test_builder_battle_stats() {
    let renderer = ItemTooltipBuilder::new()
        .name("Zatruty Miecz")
        .battle_stats(200, 250, 150, 180, 0, 20)
        .build();

    let has_damage = renderer.elements.iter().any(|el| {
        if let TooltipElement::Text { text, color } = el {
            text == "Wartość ataku 200-250" && *color == COLOR_DAMAGE
        } else {
            false
        }
    });

    assert!(has_damage);
}

#[test]
fn test_builder_sockets() {
    let renderer = ItemTooltipBuilder::new()
        .name("Klinga")
        .add_socket(Some(28430)) // Kamień penetracji
        .build();

    let has_socket_text = renderer.elements.iter().any(|el| {
        if let TooltipElement::Text { text, color } = el {
            text == "Kamień Duszy (Vnum: 28430)" && *color == COLOR_SOCKET_BLUE
        } else {
            false
        }
    });

    let has_socket_image = renderer.elements.iter().any(|el| {
        if let TooltipElement::Image { image_path, width, height } = el {
            image_path == "d:/ymir work/ui/items/28430.tga" && *width == 32 && *height == 32
        } else {
            false
        }
    });

    assert!(has_socket_text);
    assert!(has_socket_image);
}

#[test]
fn test_builder_empty() {
    let renderer = ItemTooltipBuilder::new().build();
    assert_eq!(renderer.elements.len(), 0);
    assert_eq!(renderer.width, 20); // Just padding
    assert_eq!(renderer.height, 20); // Just padding
}
