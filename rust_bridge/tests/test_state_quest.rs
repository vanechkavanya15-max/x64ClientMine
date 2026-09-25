use m2_client_bridge::state::quest::*;

#[test]
fn test_quest_manager() {
    let mut manager = QuestManager::new();
    
    assert_eq!(manager.count(), 0);

    let quest1 = QuestEntry {
        quest_index: 101,
        title: "Test Quest 1".to_string(),
        clock_name: "".to_string(),
        clock_value: 0,
    };
    manager.add_quest(quest1);
    
    assert_eq!(manager.count(), 1);

    let quest2 = QuestEntry {
        quest_index: 102,
        title: "Test Quest 2".to_string(),
        clock_name: "Time Left".to_string(),
        clock_value: 3600,
    };
    manager.add_quest(quest2);

    assert_eq!(manager.count(), 2);

    manager.remove_quest(101);
    assert_eq!(manager.count(), 1);

    manager.remove_quest(102);
    assert_eq!(manager.count(), 0);
}

#[test]
fn test_ffi_quest_count() {
    // Najpierw wyczyscmy ewewntualny stan pozostawiony przez inne testy
    {
        let mut global_manager = GLOBAL_QUEST_MANAGER.lock().unwrap();
        // Poniewaz nie mamy metody clear(), usuniemy questy petla lub podmienimy manager.
        // Jednak na potrzeby testu FFI mozna sprawdzic czy po dodaniu cos sie zwieksza.
        *global_manager = QuestManager::new();
    }

    let count_before = unsafe { m2_quest_count() };
    assert_eq!(count_before, 0);

    {
        let mut global_manager = GLOBAL_QUEST_MANAGER.lock().unwrap();
        global_manager.add_quest(QuestEntry {
            quest_index: 1,
            title: "Q1".to_string(),
            clock_name: "".to_string(),
            clock_value: 0,
        });
    }

    let count_after = unsafe { m2_quest_count() };
    assert_eq!(count_after, 1);
}
