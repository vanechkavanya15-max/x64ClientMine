// test_full-ui-adv_05.rs - Unit tests for AniImageBoxWidget

#[path = "../src/ui_ani_image_box_widget.rs"]
mod ui_ani_image_box_widget;

use ui_ani_image_box_widget::AniImageBoxWidget;

#[test]
fn test_initialization_and_appending() {
    let mut widget = AniImageBoxWidget::new();
    
    // Check initial state
    assert_eq!(widget.images.len(), 0);
    assert_eq!(widget.current_frame, 0);
    assert!(widget.is_playing);
    assert!(widget.loop_enabled);
    assert_eq!(widget.get_current_image(), None);
    
    widget.append_image("frame1.png");
    widget.append_image("frame2.png");
    
    assert_eq!(widget.images.len(), 2);
    assert_eq!(widget.get_current_image().unwrap(), "frame1.png");
}

#[test]
fn test_frame_progression_with_update() {
    let mut widget = AniImageBoxWidget::new();
    widget.append_image("frame1.png");
    widget.append_image("frame2.png");
    widget.append_image("frame3.png");
    widget.set_delay(0.1); // 100ms
    
    // First update, 50ms - not enough to change frame
    widget.update(0.05);
    assert_eq!(widget.current_frame, 0);
    
    // Second update, another 60ms - should trigger frame advance
    widget.update(0.06);
    assert_eq!(widget.current_frame, 1);
    assert_eq!(widget.get_current_image().unwrap(), "frame2.png");
}

#[test]
fn test_animation_looping() {
    let mut widget = AniImageBoxWidget::new();
    widget.append_image("frame1.png");
    widget.append_image("frame2.png");
    widget.set_delay(1.0);
    widget.set_loop(true);
    
    // Advance by 1 second (1 frame)
    widget.update(1.0);
    assert_eq!(widget.current_frame, 1);
    
    // Advance by another 1 second (should loop back to 0)
    widget.update(1.0);
    assert_eq!(widget.current_frame, 0);
    
    // Advance by a large amount (3 seconds)
    widget.update(3.0);
    assert_eq!(widget.current_frame, 1); // 0 + 3 = 3 % 2 = 1
}

#[test]
fn test_animation_no_loop() {
    let mut widget = AniImageBoxWidget::new();
    widget.append_image("frame1.png");
    widget.append_image("frame2.png");
    widget.append_image("frame3.png");
    widget.set_delay(1.0);
    widget.set_loop(false); // Disable loop
    
    // Advance 1 frame
    widget.update(1.0);
    assert_eq!(widget.current_frame, 1);
    assert!(widget.is_playing);
    
    // Advance 2 frames (past the end)
    widget.update(2.0);
    
    // Should stop at the last frame and set is_playing to false
    assert_eq!(widget.current_frame, 2);
    assert_eq!(widget.get_current_image().unwrap(), "frame3.png");
    assert!(!widget.is_playing);
}

#[test]
fn test_pause_play_and_edge_cases() {
    let mut widget = AniImageBoxWidget::new();
    widget.append_image("frame1.png");
    widget.append_image("frame2.png");
    widget.set_delay(0.5);
    
    // Pause animation
    widget.pause();
    widget.update(1.0); // 1s has passed, but animation is paused
    assert_eq!(widget.current_frame, 0); // No progression
    
    // Resume animation
    widget.play();
    widget.update(0.5); // Should advance frame
    assert_eq!(widget.current_frame, 1);
    
    // Test zero delay (should not progress, or should not crash/divide by zero)
    widget.set_delay(0.0);
    widget.update(1.0);
    assert_eq!(widget.current_frame, 1); // Frame didn't change because delay <= 0
    
    // Test empty images update
    let mut empty_widget = AniImageBoxWidget::new();
    empty_widget.update(1.0); // Should not crash
    assert_eq!(empty_widget.current_frame, 0);
    assert_eq!(empty_widget.get_current_image(), None);
}
