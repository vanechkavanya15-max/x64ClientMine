use m2_client::ui_buff_icons_overlay::{Buff, BuffIconsOverlay, BuffType};

#[test]
fn test_buff_overlay_basic() {
    let mut overlay = BuffIconsOverlay::new(800.0, 600.0);
    assert_eq!(overlay.buffs.len(), 0);

    overlay.add_buff(Buff::new(
        1,
        BuffType::Positive,
        5000,
        "icon1.tga".to_string(),
    ));
    assert_eq!(overlay.buffs.len(), 1);

    overlay.update(2000);
    assert_eq!(overlay.buffs[0].remaining_duration_ms, 3000);

    overlay.update(3000);
    assert_eq!(overlay.buffs.len(), 0);
}
