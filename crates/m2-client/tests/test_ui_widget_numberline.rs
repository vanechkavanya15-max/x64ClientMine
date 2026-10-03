#[path = "../src/ui_widget_numberline.rs"]
pub mod ui_widget_numberline;

use ui_widget_numberline::NumberLineWidget;
use m2_client::ui_window_core::{HORIZONTAL_ALIGN_LEFT, HORIZONTAL_ALIGN_CENTER, HORIZONTAL_ALIGN_RIGHT};
use m2_client::ui_texture_mgr::UiImageResource;
use std::sync::Arc;
use m2_client::wgpu_engine_context::{GpuTexture, Quad2D};

#[test]
fn test_numberline_default_values() {
    let widget = NumberLineWidget::new();
    assert_eq!(widget.path, "d:/ymir work/ui/game/taskbar/");
    assert_eq!(widget.horizontal_align, HORIZONTAL_ALIGN_LEFT);
    assert!(widget.image_instances.is_empty());
    assert_eq!(widget.width_summary, 0);
}

#[test]
fn test_numberline_set_path() {
    let mut widget = NumberLineWidget::new();
    widget.set_path("d:/new_path/");
    assert_eq!(widget.path, "d:/new_path/");
}

#[test]
fn test_numberline_set_horizontal_align() {
    let mut widget = NumberLineWidget::new();
    widget.set_horizontal_align(HORIZONTAL_ALIGN_CENTER);
    assert_eq!(widget.horizontal_align, HORIZONTAL_ALIGN_CENTER);
    widget.set_horizontal_align(HORIZONTAL_ALIGN_RIGHT);
    assert_eq!(widget.horizontal_align, HORIZONTAL_ALIGN_RIGHT);
}

#[test]
fn test_numberline_render_positions_calc_logic() {
    let mut widget = NumberLineWidget::new();
    widget.width_summary = 100;
    
    // Test for LEFT
    widget.set_horizontal_align(HORIZONTAL_ALIGN_LEFT);
    widget.on_change_position(100, 200, false);
    assert!(widget.positions.is_empty()); // since image_instances is empty

    widget.set_horizontal_align(HORIZONTAL_ALIGN_CENTER);
    widget.on_change_position(100, 200, false);
    assert!(widget.positions.is_empty());
}

#[test]
fn test_numberline_clear_number() {
    let mut widget = NumberLineWidget::new();
    widget.width_summary = 100;
    widget.number = "123".to_string();
    widget.positions.push((0, 0));
    
    widget.clear_number();
    
    assert_eq!(widget.width_summary, 0);
    assert_eq!(widget.number, "");
    assert!(widget.positions.is_empty());
    assert!(widget.image_instances.is_empty());
}

#[test]
fn test_numberline_clear_number_after_init() {
    let mut widget = NumberLineWidget::new();
    widget.clear_number();
    assert_eq!(widget.width_summary, 0);
    assert_eq!(widget.number, "");
    assert!(widget.positions.is_empty());
    assert!(widget.image_instances.is_empty());
}
