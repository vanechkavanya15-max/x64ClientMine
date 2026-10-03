use m2_client::ui_number_line_widget::NumberLineAtlasWidget;
use m2_client::ui_window_core::{HORIZONTAL_ALIGN_LEFT, HORIZONTAL_ALIGN_CENTER, HORIZONTAL_ALIGN_RIGHT};

#[test]
fn test_default_initialization() {
    let widget = NumberLineAtlasWidget::default();
    assert_eq!(widget.number_str, "");
    assert_eq!(widget.horizontal_align, HORIZONTAL_ALIGN_LEFT);
    assert_eq!(widget.width_summary, 0);
    assert_eq!(widget.char_width, 0);
    assert_eq!(widget.char_height, 0);
    assert_eq!(widget.spacing, 0);
    assert!(widget.layout.positions.is_empty());
    assert!(widget.layout.uvs.is_empty());
}

#[test]
fn test_set_number_updates_layout() {
    let mut widget = NumberLineAtlasWidget::new();
    widget.set_atlas_info(10, 10, [0.0, 0.0, 1.0, 1.0]);
    
    widget.set_number("123");
    assert_eq!(widget.width_summary, 30);
    assert_eq!(widget.layout.uvs.len(), 3);
    assert_eq!(widget.layout.positions.len(), 3);
    
    // Test U interpolation assuming 10 digits
    let u1_start = 1.0 / 10.0;
    let u1_end = 2.0 / 10.0;
    let uv1 = widget.layout.uvs[0]; // digit '1'
    assert!((uv1[0] - u1_start).abs() < 1e-5);
    assert!((uv1[2] - u1_end).abs() < 1e-5);
}

#[test]
fn test_spacing_and_invalid_chars() {
    let mut widget = NumberLineAtlasWidget::new();
    widget.set_atlas_info(10, 10, [0.0, 0.0, 1.0, 1.0]);
    widget.set_spacing(-2);
    
    widget.set_number("12,3a"); // commas and letters should be ignored
    // Only '1', '2', '3' are digits
    assert_eq!(widget.layout.uvs.len(), 3);
    // width: 3 * 10 + 2 * (-2) = 30 - 4 = 26
    assert_eq!(widget.width_summary, 26);
}

#[test]
fn test_horizontal_align_layout() {
    let mut widget = NumberLineAtlasWidget::new();
    widget.set_atlas_info(10, 10, [0.0, 0.0, 1.0, 1.0]);
    widget.on_change_position(100, 100);
    widget.set_number("123"); // 30 width
    
    widget.set_horizontal_align(HORIZONTAL_ALIGN_LEFT);
    assert_eq!(widget.layout.positions[0].0, 100);
    assert_eq!(widget.layout.positions[1].0, 110);
    
    widget.set_horizontal_align(HORIZONTAL_ALIGN_CENTER);
    // center aligns base_x to center of width: 100 - (30 / 2) = 85
    assert_eq!(widget.layout.positions[0].0, 85);
    
    widget.set_horizontal_align(HORIZONTAL_ALIGN_RIGHT);
    // right aligns base_x to right of width: 100 - 30 = 70
    assert_eq!(widget.layout.positions[0].0, 70);
}

#[test]
fn test_get_quads() {
    let mut widget = NumberLineAtlasWidget::new();
    widget.set_atlas_info(10, 10, [0.0, 0.0, 1.0, 1.0]);
    widget.on_change_position(0, 0);
    widget.set_number("45");
    
    let quads = widget.get_quads();
    assert_eq!(quads.len(), 2);
    // digit '4'
    assert!((quads[0].uv[0] - 0.4).abs() < 1e-5);
    // digit '5'
    assert!((quads[1].uv[0] - 0.5).abs() < 1e-5);
}
