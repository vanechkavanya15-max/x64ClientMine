use m2_client::ui_quest_dialog::UiQuestDialog;

#[test]
fn test_integration_ui_quest_dialog() {
    let instance = UiQuestDialog::new();
    assert!(instance.is_ready());
}
