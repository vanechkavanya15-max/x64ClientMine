use m2_client::ui_quest_dialog::{UiQuestDialog, EventType, QuestInstance, ButtonType};
use std::collections::HashMap;
 
 #[test]
fn test_integration_ui_quest_dialog_init() {
     let instance = UiQuestDialog::new();
     assert!(instance.is_ready());
 }

#[test]
fn test_quest_system() {
    let mut dialog = UiQuestDialog::new();

    dialog.make_quest(1);
    assert!(dialog.is_quest(1));
    assert_eq!(dialog.get_quest_count(), 1);

    dialog.set_quest_title(1, "Test Quest".to_string());
    assert_eq!(dialog.quests[0].title, "Test Quest");

    dialog.set_quest_clock_name(1, "Test Clock".to_string());
    assert_eq!(dialog.quests[0].clock_name, "Test Clock");

    dialog.set_quest_counter_name(1, "Test Counter".to_string());
    assert_eq!(dialog.quests[0].counter_name, "Test Counter");

    dialog.set_quest_clock_value(1, 100);
    assert_eq!(dialog.quests[0].clock_value, 100);

    dialog.set_quest_counter_value(1, 50);
    assert_eq!(dialog.quests[0].counter_value, 50);

    dialog.set_quest_icon_file_name(1, "icon.tga".to_string());
    assert_eq!(dialog.quests[0].icon_file_name, "icon.tga");

    dialog.delete_quest_instance(1);
    assert!(!dialog.is_quest(1));
    assert_eq!(dialog.get_quest_count(), 0);

    let instance = QuestInstance {
        index: 2,
        title: "Quest 2".to_string(),
        ..Default::default()
    };
    dialog.register_quest_instance(instance);
    assert!(dialog.is_quest(2));
    assert_eq!(dialog.quests[0].title, "Quest 2");
}

#[test]
fn test_event_manager_script_parsing() {
    let mut dialog = UiQuestDialog::new();

    let mut map_letter = HashMap::new();
    map_letter.insert("value".to_string(), "Hello".to_string());

    let mut map_delay = HashMap::new();
    map_delay.insert("value".to_string(), "50".to_string());

    let script = vec![
        (EventType::Delay, map_delay),
        (EventType::Letter, map_letter),
        (EventType::Enter, HashMap::new()),
        (EventType::Wait, HashMap::new()),
    ];

    let id = dialog.register_event_set_from_script(script) as u32;

    // Process Delay
    // delay gets processed
    dialog.update_event_set(id, 1);
    assert_eq!(dialog.event_sets[&id].waiting_time, 50);
    assert_eq!(dialog.event_sets[&id].command_index, 1);
    assert_eq!(dialog.event_sets[&id].last_delay_time, 0); // delay applies to future steps

    // Process Letter
    dialog.update_event_set(id, 1); 
    assert_eq!(dialog.event_sets[&id].current_line, "Hello");
    assert_eq!(dialog.event_sets[&id].command_index, 2);
    // delay gets reduced by elapsed time which was 1
    assert_eq!(dialog.event_sets[&id].last_delay_time, 49); // delay is 50 - 1 = 49

    // Simulate waiting 50ms to unblock, and processing Enter + Wait
    dialog.update_event_set(id, 50);

    assert_eq!(dialog.event_sets[&id].command_index, 3);
    // After Enter: line inserted, current line cleared.
    // After Wait: Next button added, wait flag set.
    assert_eq!(dialog.event_sets[&id].script_text_line_list.len(), 1);
    assert_eq!(dialog.event_sets[&id].script_text_line_list[0].text, "Hello");
    dialog.update_event_set(id, 1);
    assert_eq!(dialog.event_sets[&id].script_text_line_list.len(), 2);
    assert!(dialog.event_sets[&id].is_wait_flag);
    assert!(dialog.event_sets[&id].is_wait_flag);
}

#[test]
fn test_event_manager_color_and_clear() {
    let mut dialog = UiQuestDialog::new();

    let mut map_color = HashMap::new();
    map_color.insert("r".to_string(), "0.5".to_string());
    map_color.insert("g".to_string(), "0.0".to_string());
    map_color.insert("b".to_string(), "1.0".to_string());

    let script = vec![
        (EventType::Color, map_color),
        (EventType::Clear, HashMap::new()),
    ];

    let id = dialog.register_event_set_from_script(script) as u32;

    // Consume color
    dialog.update_event_set(id, 1);
    assert_eq!(dialog.event_sets[&id].current_color, [0.5, 0.0, 1.0, 1.0]);
    // consume clear
    dialog.update_event_set(id, 1);
    assert!(dialog.event_sets[&id].is_lock);
}

#[test]
fn test_event_manager_skip() {
    let mut dialog = UiQuestDialog::new();
    
    let mut map_delay = HashMap::new();
    map_delay.insert("value".to_string(), "1000".to_string());

    let script = vec![
        (EventType::Delay, map_delay.clone()),
    ];

    let id = dialog.register_event_set_from_script(script) as u32;

    dialog.update_event_set(id, 1);
    assert_eq!(dialog.event_sets[&id].waiting_time, 1000);

    dialog.skip(id);
    assert_eq!(dialog.event_sets[&id].waiting_time, 0);
}
