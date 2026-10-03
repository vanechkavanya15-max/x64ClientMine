
use m2_client::ui_widget_editline_ime::ImeEditLineWidget;
use m2_client::ui_widget_scrollbar::ScrollBarState;
use m2_client::ui_widget_slotwindow::SlotWindow;

use m2_client::full_ui_adv_15::FullUiAdv15Validator;

#[test]
fn test_ime_editline_insert() {
    let validator = FullUiAdv15Validator::new();
    let mut editline = ImeEditLineWidget::new(32, false);
    assert!(validator.validate_ime_editline_insert(&mut editline, "Hello World"));
}

#[test]
fn test_ime_editline_selection() {
    let validator = FullUiAdv15Validator::new();
    let mut editline = ImeEditLineWidget::new(32, false);
    assert!(validator.validate_ime_editline_selection(&mut editline, "Hello World"));
}

#[test]
fn test_scrollbar_scroll() {
    let validator = FullUiAdv15Validator::new();
    let mut scrollbar = ScrollBarState::new();
    assert!(validator.validate_scrollbar_scroll(&mut scrollbar));
}

#[test]
fn test_slotwindow_set_item() {
    let validator = FullUiAdv15Validator::new();
    let mut slotwindow = SlotWindow::new(5, 5, (32, 32));
    assert!(validator.validate_slotwindow_set_item(&mut slotwindow, 0, 100, 1));
}

#[test]
fn test_slotwindow_set_item_invalid() {
    let validator = FullUiAdv15Validator::new();
    let mut slotwindow = SlotWindow::new(5, 5, (32, 32));
    assert!(!validator.validate_slotwindow_set_item(&mut slotwindow, 999, 100, 1));
}
