#[path = "../src/quest_dialog_controller.rs"]
mod quest_dialog_controller;

use quest_dialog_controller::{QuestDialogController, QuestInstance, TextFragment, DialogLine};

#[test]
fn test_text_parsing_tags() {
    let mut controller = QuestDialogController::new("Hero");
    controller.insert_line("Hello [NAME]![ENTER]Welcome to [COLOR 255;0;0]Metin2[/COLOR].");
    controller.commit_current_line();

    assert_eq!(controller.lines.len(), 2);
    
    // First line
    assert_eq!(controller.lines[0].fragments.len(), 2);
    assert_eq!(controller.lines[0].fragments[0].text, "Hello ");
    assert_eq!(controller.lines[0].fragments[0].color, None);
    assert_eq!(controller.lines[0].fragments[1].text, "Hero!");
    assert_eq!(controller.lines[0].fragments[1].color, None);

    // Second line
    assert_eq!(controller.lines[1].fragments.len(), 3);
    assert_eq!(controller.lines[1].fragments[0].text, "Welcome to ");
    assert_eq!(controller.lines[1].fragments[0].color, None);
    assert_eq!(controller.lines[1].fragments[1].text, "Metin2");
    assert_eq!(controller.lines[1].fragments[1].color, Some((255, 0, 0)));
    assert_eq!(controller.lines[1].fragments[2].text, ".");
    assert_eq!(controller.lines[1].fragments[2].color, None);
}

#[test]
fn test_time_calculation() {
    let mut controller = QuestDialogController::new("Hero");
    let instance = QuestInstance {
        index: 1,
        title: "Test Quest".to_string(),
        clock_name: "남은 시간".to_string(), // Remaining Time
        counter_name: "Count".to_string(),
        clock_value: 3600,
        counter_value: 0,
        start_time: 1000,
        icon_file_name: "".to_string(),
    };
    
    controller.register_quest_instance(instance);
    
    // 1000 start + 3600 limit - 2000 current = 2600 remaining
    let result = controller.get_quest_last_time(1, 2000);
    assert_eq!(result, Some(("남은 시간".to_string(), 2600)));
    
    let formatted = controller.format_left_time_string("남은 시간 : %d초", 2600);
    assert_eq!(formatted, "남은 시간 : 2600초");
}

#[test]
fn test_dialog_select_answer_option_2() {
    let controller = QuestDialogController::new("Hero");
    
    // User selects option 2
    let packet = controller.select_answer(2);
    
    let packet_answer = packet.answer;
    let packet_length = packet.length;
    
    assert_eq!(packet_answer, 2);
    assert_eq!(packet_length, std::mem::size_of::<quest_dialog_controller::TPacketCGScriptAnswer>() as u16);
}

#[test]
fn test_dialog_click_button() {
    let controller = QuestDialogController::new("Hero");
    
    let packet = controller.click_button(12345);
    
    let packet_idx = packet.idx;
    let packet_length = packet.length;
    
    assert_eq!(packet_idx, 12345);
    assert_eq!(packet_length, std::mem::size_of::<quest_dialog_controller::TPacketCGScriptButton>() as u16);
}
