use m2_client::ui_item_tooltip_renderer::*;

#[test]
fn test_tooltip_title_colors() {
    let mut renderer = ItemTooltipRenderer::new();
    
    renderer.add_title("Miecz Pełni Księżyca+9");
    assert_eq!(renderer.elements.len(), 1);
    if let TooltipElement::Text { text, color } = &renderer.elements[0] {
        assert_eq!(text, "Miecz Pełni Księżyca+9");
        assert_eq!(*color, COLOR_TITLE_GOLD);
    } else {
        panic!("Expected text element");
    }

    renderer.clear();
    renderer.add_title("Miecz Pełni Księżyca+0");
    assert_eq!(renderer.elements.len(), 1);
    if let TooltipElement::Text { text, color } = &renderer.elements[0] {
        assert_eq!(text, "Miecz Pełni Księżyca+0");
        assert_eq!(*color, COLOR_TITLE_WHITE);
    } else {
        panic!("Expected text element");
    }
}

#[test]
fn test_tooltip_battle_stats_and_applies() {
    let mut renderer = ItemTooltipRenderer::new();
    
    renderer.add_battle_stats(150, 200, 0, 0, 0, 25);
    assert_eq!(renderer.elements.len(), 2);
    if let TooltipElement::Text { text, .. } = &renderer.elements[0] {
        assert_eq!(text, "Wartość ataku 150-200");
    } else { panic!("Expected text"); }
    if let TooltipElement::Text { text, .. } = &renderer.elements[1] {
        assert_eq!(text, "Szybkość ataku 25");
    } else { panic!("Expected text"); }
    
    renderer.add_applies(&[(5, 10), (1, 2000)]); // 5=STR, 1=Max HP
    assert_eq!(renderer.elements.len(), 4);
    if let TooltipElement::Text { text, .. } = &renderer.elements[2] {
        assert_eq!(text, "Siła: +10");
    } else { panic!("Expected text"); }
    if let TooltipElement::Text { text, .. } = &renderer.elements[3] {
        assert_eq!(text, "Max PŻ: +2000");
    } else { panic!("Expected text"); }
}

#[test]
fn test_tooltip_attributes_and_sockets() {
    let mut renderer = ItemTooltipRenderer::new();
    
    // Test 7 attributes (e.g. max layout)
    let attrs = [
        (1, 2000), // Max HP +2000
        (5, 12),   // STR +12
        (15, 10),  // Crit +10
        (16, 10),  // Pierce +10
        (17, 10),  // Half-humans +10
        (43, 20),  // Exp Bonus +20
        (44, 20),  // Gold Bonus +20
    ];
    renderer.add_attributes(&attrs);
    assert_eq!(renderer.elements.len(), 7);
    
    renderer.clear();
    
    // Test Sockets
    renderer.add_sockets(&[Some(28430), None, None]); // Deathblow Stone +4, Empty, Empty
    assert_eq!(renderer.elements.len(), 4); // 1 image, 1 text, 1 text, 1 text
    
    if let TooltipElement::Image { image_path, .. } = &renderer.elements[0] {
        assert_eq!(image_path, "d:/ymir work/ui/items/28430.tga");
    } else { panic!("Expected image"); }
    
    if let TooltipElement::Text { text, color } = &renderer.elements[1] {
        assert_eq!(text, "Kamień Duszy (Vnum: 28430)");
        assert_eq!(*color, COLOR_SOCKET_BLUE);
    } else { panic!("Expected text"); }
    
    if let TooltipElement::Text { text, color } = &renderer.elements[2] {
        assert_eq!(text, "Puste gniazdo");
        assert_eq!(*color, COLOR_SOCKET_EMPTY);
    } else { panic!("Expected text"); }
}

#[test]
fn test_tooltip_dimension_calculation() {
    let mut renderer = ItemTooltipRenderer::new();
    
    // Build a mock tooltip
    renderer.add_title("Miecz Żalu+9"); // 12 chars approx
    renderer.add_item_type(1, 3); // "Broń dwuręczna" (14 chars)
    renderer.add_battle_stats(250, 290, 0, 0, 0, 0); // "Wartość ataku 250-290" (21 chars)
    
    renderer.finalize_dimensions();
    
    // Elements: 3 texts. Height: 3 * 15 + 20 padding = 65
    assert_eq!(renderer.elements.len(), 3);
    assert_eq!(renderer.height, 65);
    
    // Max width: 21 chars * 6 = 126 + 20 padding = 146
    assert_eq!(renderer.width, 146);
    
    // Add socket image to test image sizing
    renderer.append_image("test.tga".to_string(), 32, 32);
    renderer.finalize_dimensions();
    
    // Height: 3*15 + 32 + 20 padding = 97
    assert_eq!(renderer.height, 97);
}

#[test]
fn test_tooltip_item_type() {
    let mut renderer = ItemTooltipRenderer::new();
    
    renderer.add_item_type(1, 0); // Weapon, Sword
    assert_eq!(renderer.elements.len(), 1);
    if let TooltipElement::Text { text, .. } = &renderer.elements[0] {
        assert_eq!(text, "Miecz");
    } else { panic!("Expected text"); }

    renderer.clear();
    renderer.add_item_type(2, 0); // Armor
    assert_eq!(renderer.elements.len(), 1);
    if let TooltipElement::Text { text, .. } = &renderer.elements[0] {
        assert_eq!(text, "Zbroja");
    } else { panic!("Expected text"); }
}

#[test]
fn test_tooltip_requirements_level_met() {
    let mut renderer = ItemTooltipRenderer::new();
    
    renderer.add_requirements(30, 60, 35, 0);
    assert_eq!(renderer.elements.len(), 1);
    if let TooltipElement::Text { text, color } = &renderer.elements[0] {
        assert_eq!(text, "Od poziomu: 30");
        assert_eq!(*color, COLOR_CLASS_MET);
    } else { panic!("Expected text"); }
}

#[test]
fn test_tooltip_requirements_level_unmet() {
    let mut renderer = ItemTooltipRenderer::new();
    
    renderer.add_requirements(30, 60, 25, 0);
    assert_eq!(renderer.elements.len(), 1);
    if let TooltipElement::Text { text, color } = &renderer.elements[0] {
        assert_eq!(text, "Od poziomu: 30");
        assert_eq!(*color, COLOR_CLASS_UNMET);
    } else { panic!("Expected text"); }
}

#[test]
fn test_tooltip_requirements_class() {
    let mut renderer = ItemTooltipRenderer::new();
    
    let anti_flags = 48; // Ban Shaman and Sura
    
    // Player is Warrior (job 0) -> Allowed
    renderer.add_requirements(0, anti_flags, 1, 0);
    assert_eq!(renderer.elements.len(), 1);
    if let TooltipElement::Text { text, color } = &renderer.elements[0] {
        assert_eq!(text, "Klasy: Wojownik, Ninja");
        assert_eq!(*color, COLOR_CLASS_MET);
    } else { panic!("Expected text"); }
    
    renderer.clear();
    
    // Player is Sura (job 2) -> Banned
    renderer.add_requirements(0, anti_flags, 1, 2);
    assert_eq!(renderer.elements.len(), 1);
    if let TooltipElement::Text { text, color } = &renderer.elements[0] {
        assert_eq!(text, "Klasy: Wojownik, Ninja");
        assert_eq!(*color, COLOR_CLASS_UNMET);
    } else { panic!("Expected text"); }
}
