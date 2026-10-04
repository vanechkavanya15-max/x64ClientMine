use m2_client::game_dungeon_tracking_panel::{DungeonTrackingManager, DungeonQuestInstance, format_time};

#[test]
fn test_quest_registration_and_retrieval() {
    let mut manager = DungeonTrackingManager::new();
    let mut quest = DungeonQuestInstance::default();
    quest.index = 101;
    quest.title = "Find the Key".to_string();
    quest.clock_name = "Time Remaining".to_string();
    quest.counter_name = "Monsters Slain".to_string();
    quest.clock_value = 600; // 10 minutes
    quest.counter_value = 0;
    
    manager.register_quest_instance(quest);
    
    assert_eq!(manager.get_quest_count(), 1);
    assert!(manager.is_quest(101));
    
    let retrieved_quest = manager.get_quest_instance(101).expect("Quest not found");
    assert_eq!(retrieved_quest.title, "Find the Key");
    assert_eq!(retrieved_quest.clock_name, "Time Remaining");
    assert_eq!(retrieved_quest.counter_name, "Monsters Slain");
    assert_eq!(retrieved_quest.clock_value, 600);
}

#[test]
fn test_time_calculations() {
    let mut manager = DungeonTrackingManager::new();
    manager.make_quest(202);
    
    // Test setting clock value
    manager.set_quest_clock_name(202, "Timer");
    manager.set_quest_clock_value(202, 300); // 5 minutes
    
    // Array index should be 0 since it's the only quest
    let (clock_name, last_time) = manager.get_quest_last_time(0).expect("Quest not found");
    assert_eq!(clock_name, "Timer");
    // Since start time is "now" and clock value is 300, last time should be very close to 300.
    // Allow a small delta for test execution time.
    assert!(last_time >= 299 && last_time <= 300);
}

#[test]
fn test_counter_updates() {
    let mut manager = DungeonTrackingManager::new();
    manager.make_quest(303);
    manager.set_quest_counter_name(303, "Metin Stones");
    
    // Initial state
    let quest_before = manager.get_quest_instance(303).unwrap();
    assert_eq!(quest_before.counter_value, 0);
    
    // Update counter
    manager.set_quest_counter_value(303, 5);
    
    let quest_after = manager.get_quest_instance(303).unwrap();
    assert_eq!(quest_after.counter_value, 5);
}

#[test]
fn test_time_formatting_basic_seconds() {
    assert_eq!(format_time(0), "00:00");
    assert_eq!(format_time(9), "00:09");
    assert_eq!(format_time(59), "00:59");
}

#[test]
fn test_time_formatting_minutes() {
    assert_eq!(format_time(60), "01:00");
    assert_eq!(format_time(61), "01:01");
    assert_eq!(format_time(600), "10:00");
    assert_eq!(format_time(3599), "59:59");
}

#[test]
fn test_time_formatting_hours_and_edge_cases() {
    assert_eq!(format_time(3600), "60:00");
    assert_eq!(format_time(3661), "61:01");
    assert_eq!(format_time(-10), "00:00");
    assert_eq!(format_time(-3600), "00:00");
}

#[test]
fn test_multiple_concurrent_counter_updates() {
    let mut manager = DungeonTrackingManager::new();
    
    manager.make_quest(10);
    manager.set_quest_counter_name(10, "Zabite Potwory");
    manager.set_quest_counter_value(10, 45);
    
    manager.make_quest(11);
    manager.set_quest_counter_name(11, "Zniszczone Metiny");
    manager.set_quest_counter_value(11, 2);
    
    let quest_10 = manager.get_quest_instance_by_array_index(0).unwrap();
    assert_eq!(quest_10.counter_value, 45);
    assert_eq!(quest_10.counter_name, "Zabite Potwory");
    
    let quest_11 = manager.get_quest_instance_by_array_index(1).unwrap();
    assert_eq!(quest_11.counter_value, 2);
    assert_eq!(quest_11.counter_name, "Zniszczone Metiny");
    
    // Update both
    manager.set_quest_counter_value(10, 100);
    manager.set_quest_counter_value(11, 5);
    
    assert_eq!(manager.get_quest_instance_by_array_index(0).unwrap().counter_value, 100);
    assert_eq!(manager.get_quest_instance_by_array_index(1).unwrap().counter_value, 5);
}

#[test]
fn test_quest_deletion_and_clear() {
    let mut manager = DungeonTrackingManager::new();
    manager.make_quest(1);
    manager.make_quest(2);
    manager.make_quest(3);
    
    assert_eq!(manager.get_quest_count(), 3);
    
    // After insertion, indices should be: 0->1, 1->2, 2->3
    assert_eq!(manager.get_quest_instance_by_array_index(0).unwrap().index, 1);
    assert_eq!(manager.get_quest_instance_by_array_index(1).unwrap().index, 2);
    assert_eq!(manager.get_quest_instance_by_array_index(2).unwrap().index, 3);
    
    manager.delete_quest_instance(2);
    assert_eq!(manager.get_quest_count(), 2);
    assert!(!manager.is_quest(2));
    assert!(manager.is_quest(1));
    assert!(manager.is_quest(3));
    
    // After deletion of quest 2, array indices should shift
    // indices should be: 0->1, 1->3
    assert_eq!(manager.get_quest_instance_by_array_index(0).unwrap().index, 1);
    assert_eq!(manager.get_quest_instance_by_array_index(1).unwrap().index, 3);
    
    manager.clear();
    assert_eq!(manager.get_quest_count(), 0);
}
