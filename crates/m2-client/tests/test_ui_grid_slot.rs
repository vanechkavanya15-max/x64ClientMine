use m2_client::ui_grid_slot::UiGridSlot;

#[test]
fn test_integration_ui_grid_slot() {
    let instance = UiGridSlot::new();
    assert!(instance.is_ready());
}
