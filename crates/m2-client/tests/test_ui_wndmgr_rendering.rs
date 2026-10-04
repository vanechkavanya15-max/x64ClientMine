#[path = "../src/ui_wndmgr_rendering.rs"]
mod ui_wndmgr_rendering;

use ui_wndmgr_rendering::{UiWindowRendering, RenderingMode};

#[test]
fn test_window_visibility() {
    let mut wnd = UiWindowRendering::new();
    assert_eq!(wnd.is_show(), false);

    wnd.show();
    assert_eq!(wnd.is_show(), true);

    wnd.hide();
    assert_eq!(wnd.is_show(), false);
}

#[test]
fn test_color_and_alpha() {
    let mut wnd = UiWindowRendering::new();
    wnd.set_diffuse_color(0.5, 0.2, 0.8, 1.0);
    assert_eq!(wnd.color.r, 0.5);
    assert_eq!(wnd.color.g, 0.2);
    assert_eq!(wnd.color.b, 0.8);
    assert_eq!(wnd.color.a, 1.0);

    wnd.set_alpha(0.3);
    assert_eq!(wnd.color.r, 0.5);
    assert_eq!(wnd.color.g, 0.2);
    assert_eq!(wnd.color.b, 0.8);
    assert_eq!(wnd.color.a, 0.3);
}

#[test]
fn test_rendering_mode() {
    let mut wnd = UiWindowRendering::new();
    assert_eq!(wnd.rendering_mode, RenderingMode::Normal);

    wnd.set_rendering_mode(RenderingMode::Screen);
    assert_eq!(wnd.rendering_mode, RenderingMode::Screen);

    wnd.set_rendering_mode(RenderingMode::ColorDodge);
    assert_eq!(wnd.rendering_mode, RenderingMode::ColorDodge);

    wnd.set_rendering_mode(RenderingMode::Modulate);
    assert_eq!(wnd.rendering_mode, RenderingMode::Modulate);
}

#[test]
fn test_slot_cool_time_percentage_and_angle() {
    let mut wnd = UiWindowRendering::new();
    let current_time = 100.0;
    
    // Set cooltime of 10 seconds, starting from now (elapsed = 0)
    wnd.set_slot_cool_time(1, 10.0, 0.0, current_time);
    
    // At exactly current_time, 0% complete, 360 degrees fan
    assert_eq!(wnd.get_cool_time_percentage(1, current_time), 0.0);
    assert_eq!(wnd.get_cool_time_angle(1, current_time), 360.0);

    // After 5 seconds, 50% complete, 180 degrees fan
    assert_eq!(wnd.get_cool_time_percentage(1, current_time + 5.0), 0.5);
    assert_eq!(wnd.get_cool_time_angle(1, current_time + 5.0), 180.0);

    // After 10 seconds, 100% complete, 0 degrees fan
    assert_eq!(wnd.get_cool_time_percentage(1, current_time + 10.0), 1.0);
    assert_eq!(wnd.get_cool_time_angle(1, current_time + 10.0), 0.0);

    // After 15 seconds (exceeding cooltime), clamped to 100%, 0 degrees
    assert_eq!(wnd.get_cool_time_percentage(1, current_time + 15.0), 1.0);
    assert_eq!(wnd.get_cool_time_angle(1, current_time + 15.0), 0.0);
}

#[test]
fn test_slot_cool_time_elapsed() {
    let mut wnd = UiWindowRendering::new();
    let current_time = 200.0;
    
    // Set cooltime of 20 seconds, already elapsed 5 seconds
    wnd.set_slot_cool_time(2, 20.0, 5.0, current_time);
    
    // At current_time, it should be 5 seconds in, so 25% complete (5/20)
    assert_eq!(wnd.get_cool_time_percentage(2, current_time), 0.25);
    assert_eq!(wnd.get_cool_time_angle(2, current_time), 360.0 * 0.75); // 270 degrees
    
    // After another 15 seconds, 20 seconds total, 100% complete
    assert_eq!(wnd.get_cool_time_percentage(2, current_time + 15.0), 1.0);
    assert_eq!(wnd.get_cool_time_angle(2, current_time + 15.0), 0.0);
}
