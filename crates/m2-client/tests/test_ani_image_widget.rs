use m2_client::ui_wndmgr_widget_ani_image::AniImageBoxWidget;

#[test]
fn test_ani_image_delay_and_progression() {
    let mut widget = AniImageBoxWidget::new();
    widget.append_image("frame1.tga");
    widget.append_image("frame2.tga");
    widget.append_image("frame3.tga");
    
    // By default, current_frame is 0 when new is called, but we didn't touch current_frame in append_image anymore.
    // However, it starts at 0 anyway since we instantiate with new().
    widget.reset_frame();
    widget.set_delay(2);
    
    // First update: delay = 1, frame = 0
    let res = widget.update();
    assert_eq!(widget.current_delay, 1);
    assert_eq!(widget.current_frame, 0);
    assert!(!res);
    
    // Second update: delay = 0, frame = 1
    let res = widget.update();
    assert_eq!(widget.current_delay, 0);
    assert_eq!(widget.current_frame, 1);
    assert!(!res);
    
    // Third update: delay = 1, frame = 1
    let res = widget.update();
    assert_eq!(widget.current_delay, 1);
    assert_eq!(widget.current_frame, 1);
    assert!(!res);
    
    // Fourth update: delay = 0, frame = 2
    let res = widget.update();
    assert_eq!(widget.current_delay, 0);
    assert_eq!(widget.current_frame, 2);
    assert!(!res);
}

#[test]
fn test_ani_image_looping() {
    let mut widget = AniImageBoxWidget::new();
    widget.append_image("frame1.tga");
    widget.append_image("frame2.tga");
    
    widget.reset_frame();
    widget.set_delay(1);
    widget.set_loop(true);
    
    // Update 1: frame = 1
    let res = widget.update();
    assert_eq!(widget.current_frame, 1);
    assert!(!res);
    
    // Update 2: frame = 2 -> wraps back to 0, returns true
    let res = widget.update();
    assert_eq!(widget.current_frame, 0);
    assert!(res);
    
    // Update 3: frame = 1 again
    let res = widget.update();
    assert_eq!(widget.current_frame, 1);
    assert!(!res);
}

#[test]
fn test_ani_image_single_play() {
    let mut widget = AniImageBoxWidget::new();
    widget.append_image("frame1.tga");
    widget.append_image("frame2.tga");
    
    widget.reset_frame();
    widget.set_delay(1);
    widget.set_loop(false);
    
    // Update 1: frame = 1
    let res = widget.update();
    assert_eq!(widget.current_frame, 1);
    assert!(!res);
    
    // Update 2: reaches end (frame = 2). Does not loop. current_frame becomes len - 1 (1).
    // is_playing becomes false. returns true.
    let res = widget.update();
    assert_eq!(widget.current_frame, 1);
    assert!(!widget.is_playing);
    assert!(res);
    
    // Update 3: since is_playing is false, nothing happens.
    let res = widget.update();
    assert_eq!(widget.current_frame, 1);
    assert!(!res);
}

#[test]
fn test_ani_image_reset_and_clear() {
    let mut widget = AniImageBoxWidget::new();
    widget.append_image("1.tga");
    widget.append_image("2.tga");
    widget.set_delay(1);
    
    widget.update(); // Frame is 1
    assert_eq!(widget.current_frame, 1);
    
    // Test reset
    widget.reset_frame();
    assert_eq!(widget.current_frame, 0);
    assert!(widget.is_playing);
    
    widget.update(); // Frame is 1 again
    
    // Test clear
    widget.clear_images();
    assert!(widget.images.is_empty());
    assert_eq!(widget.current_frame, 0);
    assert_eq!(widget.current_delay, 0);
    assert!(widget.is_playing);
}
