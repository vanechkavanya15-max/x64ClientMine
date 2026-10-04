use m2_client::ui_wndmgr_widget_ani_image::AniImageBoxWidget;

#[test]
fn test_ani_image_box_basic_append_and_clear() {
    let mut widget = AniImageBoxWidget::new();
    assert_eq!(widget.images.len(), 0);
    
    widget.append_image("frame1.tga");
    widget.append_image("frame2.tga");
    widget.reset_frame(); // Deterministic frame testing
    
    assert_eq!(widget.images.len(), 2);
    assert_eq!(widget.get_current_image().unwrap(), "frame1.tga");
    
    widget.clear_images();
    assert_eq!(widget.images.len(), 0);
    assert!(widget.get_current_image().is_none());
}

#[test]
fn test_ani_image_box_append_randomizes_frame() {
    // Append multiple times and ensure it hits >0 frames at least sometimes
    let mut non_zero_hits = 0;
    for _ in 0..50 {
        let mut widget = AniImageBoxWidget::new();
        for i in 0..10 {
            widget.append_image(&format!("frame{}.tga", i));
        }
        if widget.current_frame > 0 {
            non_zero_hits += 1;
        }
    }
    
    // It is statistically highly improbable to get 0 non_zero_hits over 50 iterations with 10 images
    assert!(non_zero_hits > 0, "Appending images should randomize the current frame");
}

#[test]
fn test_ani_image_box_delay_and_update_looping() {
    let mut widget = AniImageBoxWidget::new();
    widget.append_image("1.tga");
    widget.append_image("2.tga");
    widget.reset_frame(); // Deterministic frame testing
    
    widget.set_delay(2);
    widget.set_loop(true);
    
    // First tick, delay becomes 1, frame stays 0
    let end_frame = widget.update();
    assert_eq!(widget.current_delay, 1);
    assert_eq!(widget.current_frame, 0);
    assert!(!end_frame);
    
    // Second tick, delay becomes 2, which is >= 2. Delay resets to 0, frame goes to 1.
    let end_frame = widget.update();
    assert_eq!(widget.current_delay, 0);
    assert_eq!(widget.current_frame, 1);
    assert!(!end_frame);
    
    // Third tick, delay goes to 1, frame stays 1.
    let end_frame = widget.update();
    assert_eq!(widget.current_delay, 1);
    assert_eq!(widget.current_frame, 1);
    assert!(!end_frame);
    
    // Fourth tick, delay goes to 2, resets to 0, frame goes to 2, which resets to 0. Returns true (end frame).
    let end_frame = widget.update();
    assert_eq!(widget.current_delay, 0);
    assert_eq!(widget.current_frame, 0);
    assert!(end_frame);
}

#[test]
fn test_ani_image_box_delay_and_update_non_looping() {
    let mut widget = AniImageBoxWidget::new();
    widget.append_image("1.tga");
    widget.append_image("2.tga");
    widget.reset_frame(); // Deterministic frame testing
    
    widget.set_delay(1);
    widget.set_loop(false);
    
    // Tick 1 -> Advance to frame 1
    let end_frame = widget.update();
    assert_eq!(widget.current_frame, 1);
    assert!(!end_frame);
    
    // Tick 2 -> Advance to frame 2 (which is out of bounds), stops playing and stays at len() - 1
    let end_frame = widget.update();
    assert_eq!(widget.current_frame, 1);
    assert!(!widget.is_playing);
    assert!(end_frame);
    
    // Tick 3 -> Shouldn't update because not playing
    let end_frame = widget.update();
    assert_eq!(widget.current_frame, 1);
    assert!(!end_frame);
}

#[test]
fn test_ani_image_box_pause_resume() {
    let mut widget = AniImageBoxWidget::new();
    widget.append_image("1.tga");
    widget.append_image("2.tga");
    widget.reset_frame(); // Deterministic frame testing
    
    widget.set_delay(1);
    
    widget.pause();
    widget.update();
    assert_eq!(widget.current_frame, 0);
    
    widget.play();
    widget.update();
    assert_eq!(widget.current_frame, 1);
}

#[test]
fn test_ani_image_box_reset_frame() {
    let mut widget = AniImageBoxWidget::new();
    widget.append_image("1.tga");
    widget.append_image("2.tga");
    widget.reset_frame(); // Deterministic frame testing
    
    widget.set_delay(1);
    widget.update();
    assert_eq!(widget.current_frame, 1);
    
    widget.reset_frame();
    assert_eq!(widget.current_frame, 0);
}
