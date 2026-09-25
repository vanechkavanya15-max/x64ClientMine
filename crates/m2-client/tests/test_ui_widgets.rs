use m2_client::ui_widgets::{ButtonState, CBar, CButton, CEditLine, CImageBox, CTextLine, UiWidgets};

#[test]
fn test_integration_ui_widgets() {
    let instance = UiWidgets::new();
    assert!(instance.is_ready());
}

#[test]
fn test_cbutton_logic() {
    let mut btn = CButton::new();
    
    // Test poczatkowy stan
    assert_eq!(btn.state, ButtonState::Up);
    assert!(btn.is_enable());
    assert!(!btn.is_disable());
    assert!(!btn.is_pressed());
    
    // Test ustawianie visuals
    btn.set_up_visual("up.tga");
    btn.set_over_visual("over.tga");
    btn.set_down_visual("down.tga");
    btn.set_disable_visual("disable.tga");
    
    assert_eq!(btn.get_up_visual_file_name(), "up.tga");
    assert_eq!(btn.get_over_visual_file_name(), "over.tga");
    assert_eq!(btn.get_down_visual_file_name(), "down.tga");
    
    // Test disable
    btn.disable();
    assert!(btn.is_disable());
    assert_eq!(btn.state, ButtonState::Disabled);
    
    // Po wylaczeniu akcje nie powinny dzialac
    btn.on_mouse_over_in();
    assert_eq!(btn.state, ButtonState::Disabled);
    assert!(!btn.on_mouse_left_button_down());
    
    // Test enable
    btn.enable();
    assert!(btn.is_enable());
    assert_eq!(btn.state, ButtonState::Up);
    
    // Test mouse events
    btn.on_mouse_over_in();
    assert_eq!(btn.state, ButtonState::Over);
    
    btn.on_mouse_over_out();
    assert_eq!(btn.state, ButtonState::Up);
    
    assert!(btn.on_mouse_left_button_down());
    assert!(btn.is_pressed());
    assert_eq!(btn.state, ButtonState::Down);
    
    assert!(btn.on_mouse_left_button_up());
    assert!(!btn.is_pressed());
    assert_eq!(btn.state, ButtonState::Over);
}

#[test]
fn test_ctextline_logic() {
    let mut text_line = CTextLine::new();
    
    // Wartosci domyslne
    assert_eq!(text_line.font_color, 0xffcccccc);
    
    text_line.set_max(20);
    assert_eq!(text_line.max_len, 20);
    
    text_line.set_horizontal_align(1);
    assert_eq!(text_line.horizontal_align, 1);
    
    text_line.set_vertical_align(2);
    assert_eq!(text_line.vertical_align, 2);
    
    text_line.set_secret(true);
    assert!(text_line.secret);
    
    text_line.set_outline(true);
    assert!(text_line.outline);
    
    text_line.set_feather(true);
    assert!(text_line.feather);
    
    text_line.set_multi_line(true);
    assert!(text_line.multi_line);
    
    text_line.set_font_name("Arial");
    assert_eq!(text_line.font_name, "Arial.fnt");
    
    text_line.set_font_color(0xff00ff00);
    assert_eq!(text_line.font_color, 0xff00ff00);
    
    text_line.set_limit_width(100.0);
    assert_eq!(text_line.limit_width, 100.0);
    
    text_line.set_text("Hello");
    assert_eq!(text_line.get_text(), "Hello");
}

#[test]
fn test_ceditline_logic() {
    let mut edit_line = CEditLine::new();
    
    // Test braku focusu
    edit_line.type_char('a');
    assert_eq!(edit_line.get_text(), "");
    
    // Test z focusem
    edit_line.set_focus(true);
    edit_line.type_char('a');
    edit_line.type_char('b');
    edit_line.type_char('c');
    assert_eq!(edit_line.get_text(), "abc");
    assert_eq!(edit_line.cursor_pos, 3);
    
    // Test backspace
    edit_line.backspace();
    assert_eq!(edit_line.get_text(), "ab");
    assert_eq!(edit_line.cursor_pos, 2);
    
    // Test limitu dlugosci
    edit_line.set_max(3);
    edit_line.type_char('c');
    assert_eq!(edit_line.get_text(), "abc");
    edit_line.type_char('d'); // Powinno zostac zignorowane
    assert_eq!(edit_line.get_text(), "abc");
    
    // Test ustawiania tekstu
    edit_line.set_text("test");
    assert_eq!(edit_line.get_text(), "test");
    assert_eq!(edit_line.cursor_pos, 4);
    
    // Test secret (haslo)
    edit_line.set_secret(true);
    assert!(edit_line.text_line.secret);
}

#[test]
fn test_cimagebox_logic() {
    let mut image_box = CImageBox::new();
    
    assert!(!image_box.loaded);
    assert_eq!(image_box.get_width(), 0);
    assert_eq!(image_box.get_height(), 0);
    
    // Nieudane ladowanie puste
    assert!(!image_box.load_image(""));
    
    // Ladowanie obrazu
    assert!(image_box.load_image("test.tga"));
    assert!(image_box.loaded);
    assert_eq!(image_box.get_width(), 100);
    assert_eq!(image_box.get_height(), 100);
    
    // Diffuse color
    image_box.set_diffuse_color(0.5, 0.5, 0.5, 1.0);
    assert_eq!(image_box.diffuse_color, (0.5, 0.5, 0.5, 1.0));
}

#[test]
fn test_cbar_logic() {
    let mut bar = CBar::new();
    
    assert_eq!(bar.color, 0xff000000);
    
    bar.set_color(0xffff0000);
    assert_eq!(bar.color, 0xffff0000);
}
