use m2_client::ui_buff_icons_overlay::{Buff, BuffIconsOverlay, BuffType};

#[test]
fn test_buff_add_and_update() {
    let mut overlay = BuffIconsOverlay::new(800.0, 600.0);
    overlay.add_buff(Buff::new(1, BuffType::Positive, 10000, "icon1.tga".to_string()));
    assert_eq!(overlay.buffs.len(), 1);
    
    // Add existing buff (should reset duration)
    overlay.add_buff(Buff::new(1, BuffType::Positive, 5000, "icon1.tga".to_string()));
    assert_eq!(overlay.buffs.len(), 1);
    assert_eq!(overlay.buffs[0].max_duration_ms, 5000);
    assert_eq!(overlay.buffs[0].remaining_duration_ms, 5000);
}

#[test]
fn test_buff_expiration_and_cleanup() {
    let mut overlay = BuffIconsOverlay::new(800.0, 600.0);
    overlay.add_buff(Buff::new(1, BuffType::Positive, 2000, "icon1.tga".to_string()));
    overlay.add_buff(Buff::new(2, BuffType::Negative, 4000, "icon2.tga".to_string()));
    
    overlay.update(2500);
    assert_eq!(overlay.buffs.len(), 1);
    assert_eq!(overlay.buffs[0].id, 2);
    assert_eq!(overlay.buffs[0].remaining_duration_ms, 1500);
}

#[test]
fn test_buff_manual_removal() {
    let mut overlay = BuffIconsOverlay::new(800.0, 600.0);
    overlay.add_buff(Buff::new(1, BuffType::Positive, 10000, "icon1.tga".to_string()));
    overlay.add_buff(Buff::new(2, BuffType::Negative, 10000, "icon2.tga".to_string()));
    
    overlay.remove_buff(1);
    assert_eq!(overlay.buffs.len(), 1);
    assert_eq!(overlay.buffs[0].id, 2);
}

#[test]
fn test_layout_arrangement_and_wrap_around() {
    // Screen width 100. Right margin 10. Icon size 32. Padding 4.
    // First icon: x = 100 - 10 - 32 = 58
    // Second icon: x = 58 - 36 = 22
    // Third icon: x = 22 - 36 = -14 (< 0). So wrap!
    //   -> New row: x = 58, y = 10 (margin) + 32 + 4 = 46.
    
    let mut overlay = BuffIconsOverlay::new(100.0, 600.0);
    overlay.add_buff(Buff::new(1, BuffType::Positive, 10000, "1.tga".to_string()));
    overlay.add_buff(Buff::new(2, BuffType::Positive, 10000, "2.tga".to_string()));
    overlay.add_buff(Buff::new(3, BuffType::Positive, 10000, "3.tga".to_string()));
    
    let layouts = overlay.get_layouts();
    assert_eq!(layouts.len(), 3);
    
    assert_eq!(layouts[0].x, 58.0);
    assert_eq!(layouts[0].y, 10.0);
    
    assert_eq!(layouts[1].x, 22.0);
    assert_eq!(layouts[1].y, 10.0);
    
    assert_eq!(layouts[2].x, 58.0);
    assert_eq!(layouts[2].y, 46.0);
}

#[test]
fn test_progress_calculation() {
    let mut overlay = BuffIconsOverlay::new(800.0, 600.0);
    overlay.add_buff(Buff::new(1, BuffType::Positive, 10000, "1.tga".to_string()));
    
    overlay.update(2500); // 7500 remaining out of 10000 (0.75)
    
    let layouts = overlay.get_layouts();
    assert_eq!(layouts.len(), 1);
    assert!((layouts[0].bar_progress - 0.75).abs() < 0.001);
}
