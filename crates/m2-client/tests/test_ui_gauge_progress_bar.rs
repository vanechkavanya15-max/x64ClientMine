use m2_client::ui_wndmgr_widget_gauge_bar::*;

#[test]
fn test_gauge_creation_and_types() {
    let handle_hp = m2_ui_gauge_create(0); // HP
    let handle_sp = m2_ui_gauge_create(1); // SP
    let handle_exp = m2_ui_gauge_create(3); // EXP

    assert!(handle_hp >= 1000);
    assert!(handle_sp > handle_hp);
    assert!(handle_exp > handle_sp);

    let p = m2_ui_gauge_get_percentage(handle_hp);
    assert!((p - 1.0).abs() < 0.001);

    m2_ui_gauge_destroy(handle_hp);
    m2_ui_gauge_destroy(handle_sp);
    m2_ui_gauge_destroy(handle_exp);
}

#[test]
fn test_gauge_animation_and_values() {
    let handle = m2_ui_gauge_create(0);
    m2_ui_gauge_set_value(handle, 50.0, 100.0);

    // Initial display_value hasn't updated yet
    let p_before = m2_ui_gauge_get_percentage(handle);
    assert!((p_before - 1.0).abs() < 0.001);

    // Update with dt
    for _ in 0..60 {
        m2_ui_gauge_update(handle, 1.0 / 60.0);
    }

    let p_after = m2_ui_gauge_get_percentage(handle);
    assert!((p_after - 0.5).abs() < 0.05);

    m2_ui_gauge_destroy(handle);
}
