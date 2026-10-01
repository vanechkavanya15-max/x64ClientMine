use m2_client::minimap_compass_mapper_v2::PinType;
use m2_client::ui_window_minimap::UiWindowMinimap;
use m2_client::minimap_navigation::MinimapNavigation;

#[test]
fn test_ui_window_minimap_initialization() {
    let window = UiWindowMinimap::new(100);

    assert_eq!(window.root_handle, 100);
    assert_eq!(window.base_window.name, "MinimapBoard");
    assert_eq!(window.bg_image_box.name, "MinimapBackground");
    assert_eq!(window.pos_text_line.name, "PositionText");
    assert_eq!(window.map_name_text_line.name, "MapNameText");
    assert_eq!(window.zoom_in_btn.name, "ZoomInButton");
    assert_eq!(window.zoom_out_btn.name, "ZoomOutButton");
    assert_eq!(window.player_arrow_img.name, "PlayerArrow");
    assert_eq!(window.compass_img.name, "Compass");

    assert_eq!(window.base_window.children.len(), 7);
}

#[test]
fn test_ui_window_minimap_update_state() {
    let mut window = UiWindowMinimap::new(100);

    window.state.player_x = 123400.0;
    window.state.player_y = 567800.0;
    window.state.current_map_name = "metin2_map_a1".to_string();
    window.state.current_channel = "CH1".to_string();

    assert!(window.update_state().is_ok());

    assert_eq!(window.pos_text_line.text, "X: 1234, Y: 5678");
    assert_eq!(window.map_name_text_line.text, "metin2_map_a1 - CH1");
}

#[test]
fn test_ui_window_minimap_zoom_logic() {
    let mut window = UiWindowMinimap::new(100);

    assert_eq!(window.state.zoom_scale, 1.0);

    assert!(window.handle_zoom_in().is_ok());
    assert_eq!(window.state.zoom_scale, 2.0);

    assert!(window.handle_zoom_in().is_err()); // shouldn't go beyond 2.0
    assert_eq!(window.state.zoom_scale, 2.0);

    assert!(window.handle_zoom_out().is_ok());
    assert_eq!(window.state.zoom_scale, 1.0);

    assert!(window.handle_zoom_out().is_ok());
    assert_eq!(window.state.zoom_scale, 0.5);

    assert!(window.handle_zoom_out().is_err()); // shouldn't go below 0.5
    assert_eq!(window.state.zoom_scale, 0.5);

    assert!(window.handle_zoom_in().is_ok());
    assert_eq!(window.state.zoom_scale, 1.0);
}

#[test]
fn test_ui_window_minimap_pins_management() {
    let mut window = UiWindowMinimap::new(100);

    assert_eq!(window.state.pins.len(), 0);

    assert!(window.add_pin(1, 1000.0, 2000.0, PinType::Monster).is_ok());
    assert!(window.add_pin(2, 3000.0, 4000.0, PinType::PartyMember).is_ok());

    assert_eq!(window.state.pins.len(), 2);
    assert_eq!(window.state.pins[0].id, 1);
    assert_eq!(window.state.pins[1].id, 2);

    assert!(window.clear_pins().is_ok());
    assert_eq!(window.state.pins.len(), 0);
}

#[test]
fn test_ui_window_minimap_calculate_pin_screen_positions() {
    let mut window = UiWindowMinimap::new(100);

    window.state.player_x = 1000.0;
    window.state.player_y = 1000.0;
    window.state.zoom_scale = 1.0;

    let _ = window.add_pin(1, 1000.0, 1000.0, PinType::Player); // center
    let _ = window.add_pin(2, 5000.0, 1000.0, PinType::Monster); // to the right

    let radius = 100.0;
    let positions = window.calculate_pin_screen_positions(radius).unwrap();

    assert_eq!(positions.len(), 2);

    // Player (center)
    assert_eq!(positions[0].0, 1);
    assert_eq!(positions[0].1, 0.0);
    assert_eq!(positions[0].2, 0.0);
    assert_eq!(positions[0].3, PinType::Player.get_color());

    // Monster
    assert_eq!(positions[1].0, 2);
    // distance is (5000 - 1000) * (1/200) = 4000 / 200 = 20.0
    assert_eq!(positions[1].1, 20.0);
    assert_eq!(positions[1].2, 0.0);
    assert_eq!(positions[1].3, PinType::Monster.get_color());
}

#[test]
fn test_minimap_compass_rotation() {
    let rot_zero = MinimapNavigation::calculate_arrow_rotation(0.0);
    assert_eq!(rot_zero, 180.0); // 540.0 - 0.0 = 540.0, 540.0 % 360.0 = 180.0

    let rot_90 = MinimapNavigation::calculate_arrow_rotation(90.0);
    assert_eq!(rot_90, 90.0); // 540.0 - 90.0 = 450.0, 450.0 % 360.0 = 90.0

    let rot_minus_90 = MinimapNavigation::calculate_arrow_rotation(-90.0);
    assert_eq!(rot_minus_90, 270.0); // 540.0 - (-90.0) = 630.0, 630.0 % 360.0 = 270.0
}
