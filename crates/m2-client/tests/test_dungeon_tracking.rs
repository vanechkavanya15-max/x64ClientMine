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
    
    let (clock_name, last_time) = manager.get_quest_last_time(202).expect("Quest not found");
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
fn test_time_formatting() {
    assert_eq!(format_time(0), "00:00");
    assert_eq!(format_time(59), "00:59");
    assert_eq!(format_time(60), "01:00");
    assert_eq!(format_time(61), "01:01");
    assert_eq!(format_time(600), "10:00");
    assert_eq!(format_time(3599), "59:59");
    assert_eq!(format_time(3600), "60:00");
    assert_eq!(format_time(-10), "00:00"); // Edge case for negative time
}

#[test]
fn test_quest_deletion_and_clear() {
    let mut manager = DungeonTrackingManager::new();
    manager.make_quest(1);
    manager.make_quest(2);
    manager.make_quest(3);
    
    assert_eq!(manager.get_quest_count(), 3);
    
    manager.delete_quest_instance(2);
    assert_eq!(manager.get_quest_count(), 2);
    assert!(!manager.is_quest(2));
    assert!(manager.is_quest(1));
    assert!(manager.is_quest(3));
    
    manager.clear();
    assert_eq!(manager.get_quest_count(), 0);
}
