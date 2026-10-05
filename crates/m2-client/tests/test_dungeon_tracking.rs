use m2_client::game_dungeon_tracking_panel::{DungeonTrackingManager, DungeonQuestInstance, format_time};

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
fn test_countdown_timer() {
    let mut manager = DungeonTrackingManager::new();
    manager.make_quest(1);
    
    // Set 10 minutes
    manager.set_countdown_timer(1, 600);
    
    let time_left = manager.get_time_left(1);
    assert!(time_left >= 599 && time_left <= 600);
    
    let formatted = manager.get_formatted_time_left(1);
    assert!(formatted == "10:00" || formatted == "09:59");
}

#[test]
fn test_monster_progress_tracker() {
    let mut manager = DungeonTrackingManager::new();
    manager.make_quest(2);
    
    // initial
    assert_eq!(manager.get_formatted_monster_progress(2), "0/0 mobow");
    
    // progress
    manager.set_monster_progress(2, 45, 100);
    assert_eq!(manager.get_formatted_monster_progress(2), "45/100 mobow");
    
    let quest = manager.get_quest_instance(2).unwrap();
    assert_eq!(quest.monsters_killed, 45);
    assert_eq!(quest.monsters_total, 100);
}

#[test]
fn test_active_goal_indicator() {
    let mut manager = DungeonTrackingManager::new();
    manager.make_quest(3);
    
    manager.set_active_goal(3, "znajdz klucz");
    assert_eq!(manager.get_active_goal(3), "znajdz klucz");
    
    manager.set_active_goal(3, "zniszcz kamien metin");
    assert_eq!(manager.get_active_goal(3), "zniszcz kamien metin");
    
    manager.set_active_goal(3, "pokonaj bossa");
    assert_eq!(manager.get_active_goal(3), "pokonaj bossa");
}

#[test]
fn test_quest_registration_and_retrieval() {
    let mut manager = DungeonTrackingManager::new();
    let mut quest = DungeonQuestInstance::default();
    quest.index = 101;
    quest.active_goal = "Find the Key".to_string();
    quest.time_limit_seconds = 600;
    quest.monsters_killed = 0;
    quest.monsters_total = 100;
    
    manager.register_quest_instance(quest);
    
    assert_eq!(manager.get_quest_count(), 1);
    assert!(manager.is_quest(101));
    
    let retrieved_quest = manager.get_quest_instance(101).expect("Quest not found");
    assert_eq!(retrieved_quest.active_goal, "Find the Key");
    assert_eq!(retrieved_quest.time_limit_seconds, 600);
}
